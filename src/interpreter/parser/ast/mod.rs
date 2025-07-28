use enum_as_inner::EnumAsInner;

use crate::interpreter::lexer::Ident;

#[derive(Debug, EnumAsInner)]
pub enum Ast<'src> {
    Stmts(Vec<Ast<'src>>),

    VariableDefinition { name: Ident<'src>, init_value: Expr },
    If(Box<If<'src>>),
    While(Box<While<'src>>),

    Expr(Expr),

    Empty,
}

#[derive(Debug)]
pub struct While<'src> {
    pub(crate) condition: Ast<'src>,
    pub(crate) block: Ast<'src>,
}

#[derive(Debug)]
pub struct If<'src> {
    pub(crate) condition: Ast<'src>,
    pub(crate) block: Ast<'src>,
    pub(crate) elsif_blocks: Vec<Elsif<'src>>,
    pub(crate) else_block: Option<Ast<'src>>,
}

#[derive(Debug)]
pub struct Elsif<'src> {
    pub(crate) condition: Ast<'src>,
    pub(crate) block: Ast<'src>,
}

pub type Expr = i64;

impl<'src> Ast<'src> {
    pub fn push_stmt(&mut self, stmt: Ast<'src>) {
        if let Some(stmts) = self.as_stmts_mut() {
            stmts.push(stmt);
        }
    }

    pub fn does_not_need_separating_semicolon(&self) -> bool {
        match self {
            Self::Empty | Self::If(_) | Self::While(_) => true,
            Self::Stmts(stmts) => stmts[stmts.len() - 1].does_not_need_separating_semicolon(),
            _ => false,
        }
    }
}
