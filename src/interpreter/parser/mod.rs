mod ast;

use std::{iter::Peekable, ops::Range};

use crate::interpreter::{
    error::{InterpreterError, ParsingError},
    lexer::{Ident, Token, TokenKind},
    parser::ast::Ast,
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
        let mut lhs = self.parse_stmt()?;

        if self.peek_next_token().is_none() {
            return Ok(lhs);
        }

        while self.peek_next_token().is_some() && self.match_token(TokenKind::Semicolon)? {
            // Makes trailing semicolon optional
            if self.peek_next_token().is_none()
                || matches!(
                    self.peek_next_token().unwrap().kind,
                    TokenKind::CurlyBrace("}")
                )
            {
                return Ok(lhs);
            }

            let rhs = self.parse_stmt()?;

            if lhs.is_stmts() {
                lhs.push_stmt(rhs);
            } else {
                lhs = Ast::Stmts(vec![lhs, rhs]);
            }
        }

        Ok(lhs)
    }

    fn parse_stmt(&mut self) -> Result<Ast<'src>, InterpreterError> {
        match self.take_next_token()?.kind {
            TokenKind::My => {
                let variable_name = self.match_ident()?;
                self.match_token(TokenKind::Equal)?;
                let expr = self.parse_expr()?;
                return Ok(Ast::VariableDefinition {
                    name: variable_name,
                    init_value: expr.into_expr().unwrap(),
                });
            }
            _ => todo!(),
        }
        todo!()
    }

    fn parse_expr(&mut self) -> Result<Ast, InterpreterError> {
        self.match_number().map(Ast::Expr)
    }

    fn match_any_token(&mut self, choices: &[Token<'src>]) {
        todo!()
    }

    fn match_token(&mut self, token_kind: TokenKind) -> Result<bool, InterpreterError> {
        if matches!(self.take_next_token()?.kind, token_kind) {
            return Ok(true);
        }

        Err(InterpreterError::new_parsing(
            ParsingError::SyntaxError("todo!".to_owned()),
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
            ParsingError::SyntaxError("ident expected".to_owned()),
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
            Err(InterpreterError::new_unexpected_eoi(
                self.last_token_span.clone(),
            ))
        }
    }
}
