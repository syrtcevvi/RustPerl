use enum_as_inner::EnumAsInner;

use crate::interpreter::lexer::Ident;

#[derive(Debug, EnumAsInner)]
pub enum Ast<'src> {
    Stmts(Vec<Ast<'src>>),

    VariableDefinition { name: Ident<'src>, init_value: Expr },

    Expr(Expr),
}

pub type Expr = i64;

impl<'src> Ast<'src> {
    pub fn push_stmt(&mut self, stmt: Ast<'src>) {
        if let Some(stmts) = self.as_stmts_mut() {
            stmts.push(stmt);
        }
    }
}
