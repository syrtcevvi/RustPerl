mod ast;

use std::{iter::Peekable, mem, ops::Range};

use crate::interpreter::{
    error::{InterpreterError, InterpreterErrorKind, ParsingError},
    lexer::{Ident, Token, TokenKind},
    parser::ast::{Ast, Elsif, If, While},
};

pub struct Parser<I>
where
    I: Iterator,
{
    tokens: Peekable<I>,
    last_token_span: Range<usize>,
}

impl<'src, I> Parser<I>
where
    I: Iterator<Item = Token<'src>>,
{
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
            last_token_span: 0..0,
        }
    }

    pub fn parse(&mut self) -> Result<Ast<'src>, InterpreterError> {
        self.parse_program()
    }

    fn parse_program(&mut self) -> Result<Ast<'src>, InterpreterError> {
        self.parse_stmts()
    }

    fn parse_stmts(&mut self) -> Result<Ast<'src>, InterpreterError> {
        // Program can be completely empty or contains only semicolons, btw
        let mut lhs = Ast::Empty;
        if let Some(next_token) = self.peek_next_token() {
            lhs = self.parse_stmt_or_stmt_with_block()?;

            while lhs.does_not_need_separating_semicolon()
                || self.peek_next_token().map(|t| t.kind) == Some(TokenKind::Semicolon)
            {
                // Allows trailing semicolons
                while self.peek_next_token().map(|t| t.kind) == Some(TokenKind::Semicolon) {
                    let _ = self.take_next_token();
                }

                if self.peek_next_token().is_none()
                    || matches!(
                        self.peek_next_token().unwrap().kind,
                        TokenKind::CurlyBrace("}")
                    )
                {
                    return Ok(lhs);
                }

                let rhs = self.parse_stmt_or_stmt_with_block()?;

                if lhs.is_stmts() {
                    lhs.push_stmt(rhs);
                } else {
                    lhs = Ast::Stmts(vec![lhs, rhs]);
                }
            }

            if let Some(next_token) = self.peek_next_token() && next_token.kind != TokenKind::CurlyBrace("}") {
                return Err(InterpreterError {
                    kind: InterpreterErrorKind::Parsing(ParsingError::SyntaxError(
                        "Missing ';'".to_owned(),
                    )),
                    span: (self.last_token_span.start + 1..self.last_token_span.end + 1),
                });
            }

            return Ok(lhs);
        }
        Ok(lhs)
    }

    fn parse_stmt_or_stmt_with_block(&mut self) -> Result<Ast<'src>, InterpreterError> {
        if let Some(next_token) = self.peek_next_token() {
            match next_token.kind {
                TokenKind::If | TokenKind::Unless | TokenKind::While | TokenKind::Until => {
                    return self.parse_stmt_with_block();
                }
                TokenKind::My => {
                    return self.parse_stmt();
                }
                // This must be empty to allow empty {} blocks within condition and cycle statements
                _ => {}
            }
        }

        Ok(Ast::Empty)
    }

    fn parse_stmt_with_block(&mut self) -> Result<Ast<'src>, InterpreterError> {
        match self.take_next_token()?.kind {
            // TODO invert condition in case of unless || until
            TokenKind::If => {
                let (condition, block) = self.parse_condition_and_block()?;
                let mut if_ = If {
                    condition,
                    block,
                    elsif_blocks: vec![],
                    else_block: None,
                };

                while let Some(maybe_elsif) = self.peek_next_token()
                    && maybe_elsif.kind == TokenKind::Elsif
                {
                    self.take_next_token().unwrap();
                    let (condition, block) = self.parse_condition_and_block()?;
                    if_.elsif_blocks.push(Elsif { block, condition });
                }

                if let Some(maybe_else) = self.peek_next_token()
                    && maybe_else.kind == TokenKind::Else
                {
                    self.take_next_token().unwrap();
                    let else_block = self.parse_block_stmt()?;
                    if_.else_block = Some(else_block);
                }

                Ok(Ast::If(Box::new(if_)))
            }
            TokenKind::While => {
                let (condition, block) = self.parse_condition_and_block()?;

                Ok(Ast::While(Box::new(While { condition, block })))
            }
            _ => unreachable!(),
        }
    }

    fn parse_condition_and_block(&mut self) -> Result<(Ast<'src>, Ast<'src>), InterpreterError> {
        self.match_token_by_value(&TokenKind::RoundParen("("))?;
        let condition = self.parse_expr()?;
        self.match_token_by_value(&TokenKind::RoundParen(")"))?;
        let block = self.parse_block_stmt()?;

        Ok((condition, block))
    }

    fn parse_block_stmt(&mut self) -> Result<Ast<'src>, InterpreterError> {
        self.match_token_by_value(&TokenKind::CurlyBrace("{"))?;
        let block = self.parse_stmts()?;
        self.match_token_by_value(&TokenKind::CurlyBrace("}"))?;
        Ok(block)
    }

    fn parse_stmt(&mut self) -> Result<Ast<'src>, InterpreterError> {
        match self.take_next_token()?.kind {
            TokenKind::My => {
                let variable_name = self.match_ident()?;
                self.match_token_by_kind(&TokenKind::Equal)?;
                let expr = self.parse_expr()?;
                Ok(Ast::VariableDefinition {
                    name: variable_name,
                    init_value: expr.into_expr().unwrap(),
                })
            }
            _ => unreachable!(),
        }
    }

    fn parse_expr(&mut self) -> Result<Ast<'src>, InterpreterError> {
        self.match_number().map(Ast::Expr)
    }

    fn match_any_token(&mut self, choices: &[Token<'src>]) {
        todo!()
    }

    fn match_token_by_kind(
        &mut self,
        expected_token_kind: &TokenKind,
    ) -> Result<bool, InterpreterError> {
        let current_token_kind = &self.take_next_token()?.kind;
        if mem::discriminant(current_token_kind) == mem::discriminant(expected_token_kind) {
            return Ok(true);
        }

        Err(InterpreterError::new_parsing(
            ParsingError::SyntaxError(format!(
                "Expected '{expected_token_kind}', but found: '{current_token_kind}'"
            )),
            self.last_token_span.clone(),
        ))
    }

    fn match_token_by_value(
        &mut self,
        expected_token_kind: &TokenKind,
    ) -> Result<bool, InterpreterError> {
        let current_token_kind = self.take_next_token()?.kind;
        if current_token_kind == *expected_token_kind {
            return Ok(true);
        }

        Err(InterpreterError::new_parsing(
            ParsingError::SyntaxError(format!(
                "Expected '{expected_token_kind}', but found: '{current_token_kind}'"
            )),
            self.last_token_span.clone(),
        ))
    }

    fn match_token_sequence(&mut self, token_kinds: &[TokenKind]) -> bool {
        for token_kind in token_kinds {
            let current_token = self.tokens.next();
            if current_token.is_none() {
                return false;
            } else if let Some(token) = current_token {
                if matches!(token.kind, token_kind) {
                    continue;
                } else {
                    return false;
                }
            }
        }
        true
    }

    fn match_ident(&mut self) -> Result<Ident<'src>, InterpreterError> {
        let maybe_ident = self.take_next_token()?;
        if matches!(maybe_ident.kind, TokenKind::Ident(_)) {
            return Ok(maybe_ident.kind.into_ident().unwrap());
        }
        Err(InterpreterError::new_parsing(
            ParsingError::SyntaxError("identifier expected".to_owned()),
            self.last_token_span.clone(),
        ))
    }

    fn match_number(&mut self) -> Result<i64, InterpreterError> {
        let maybe_number = self.take_next_token()?;
        if matches!(maybe_number.kind, TokenKind::Number(_)) {
            return Ok(maybe_number.kind.into_number().unwrap());
        }
        Err(InterpreterError::new_parsing(
            ParsingError::SyntaxError("number expected".to_owned()),
            self.last_token_span.clone(),
        ))
    }

    fn peek_next_token(&mut self) -> Option<&Token<'src>> {
        self.tokens.peek()
    }

    fn take_next_token(&mut self) -> Result<Token<'src>, InterpreterError> {
        if let Some(next_token) = self.tokens.next() {
            self.last_token_span = next_token.span.clone();
            Ok(next_token)
        } else {
            Err(self.get_eof_error())
        }
    }

    fn get_eof_error(&self) -> InterpreterError {
        InterpreterError::new_unexpected_eoi(self.last_token_span.clone())
    }
}
