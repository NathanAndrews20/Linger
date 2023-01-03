use crate::{
    parser::{Builtin, SugaredExpr, SugaredStatement},
    tokenizer::Operator,
};

/// A desugared representation of a program in the Linger programming language.
///
/// This is the expanded form of the [SugaredProcedure](crate::parser::SugaredProcedure).
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Procedure<'a> {
    pub name: &'a str,
    pub params: Vec<&'a str>,
    pub body: Vec<Statement<'a>>,
}

/// A desugared representation of a statement in the Linger programming language.
///
/// This is the expanded form of the [SugaredStatement](SugaredStatement).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Statement<'a> {
    Expr(Expr<'a>),
    Let(&'a str, Expr<'a>),
    Assign(&'a str, Expr<'a>),
    If(Expr<'a>, Box<Statement<'a>>, Option<Box<Statement<'a>>>),
    While(Expr<'a>, Box<Statement<'a>>),
    Block(Vec<Statement<'a>>),
    Return(Option<Expr<'a>>),
    Break,
    Continue,
}

/// A desugared representation of a expression in the Linger programming language.
///
/// This is the expanded form of the [SugaredExpr](SugaredExpr).
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expr<'a> {
    Num(i64),
    Bool(bool),
    Str(String),
    Var(&'a str),
    Binary(Operator, Box<Expr<'a>>, Box<Expr<'a>>),
    Unary(Operator, Box<Expr<'a>>),
    PrimitiveCall(Builtin, Vec<Expr<'a>>),
    Call(Box<Expr<'a>>, Vec<Expr<'a>>),
    Lambda(Vec<&'a str>, Vec<Statement<'a>>),
}

/// desugars a list of [SugaredStatements](SugaredStatement) in a list of [Statements](Statement).
pub fn desugar_statements(sugared_statements: Vec<SugaredStatement>) -> Vec<Statement> {
    sugared_statements
        .iter()
        .map(|s| desugar_statement(s.clone()))
        .collect()
}

fn desugar_statement(sugared_statement: SugaredStatement) -> Statement {
    match sugared_statement {
        SugaredStatement::Expr(sugared_expr) => Statement::Expr(desugar_expression(sugared_expr)),
        SugaredStatement::Let(name, sugared_expr) => {
            Statement::Let(name, desugar_expression(sugared_expr))
        }
        SugaredStatement::Assign(name, sugared_expr) => {
            Statement::Assign(name, desugar_expression(sugared_expr))
        }
        SugaredStatement::If(if_cond, then_block, else_ifs, else_option) => {
            let desugared_else_option = match else_option {
                Some(else_block) => Some(desugar_statement(*else_block)),
                None => None,
            };

            let nested_else_ifs = else_ifs.into_iter().rfold(
                desugared_else_option,
                |acc, (cur_sugared_cond_expr, cur_sugared_block)| {
                    return Some(Statement::If(
                        desugar_expression(cur_sugared_cond_expr),
                        Box::new(desugar_statement(cur_sugared_block)),
                        match acc {
                            Some(acc) => Some(Box::new(acc)),
                            None => None,
                        },
                    ));
                },
            );

            let nested_else_ifs = match nested_else_ifs {
                Some(statement) => Some(Box::new(statement)),
                None => None,
            };

            return Statement::If(
                desugar_expression(if_cond),
                Box::new(desugar_statement(*then_block)),
                nested_else_ifs,
            );
        }

        SugaredStatement::Return(sugared_expr_option) => {
            Statement::Return(match sugared_expr_option {
                Some(sugared_expr) => Some(desugar_expression(sugared_expr)),
                None => None,
            })
        }
        SugaredStatement::While(sugared_while_cond, sugared_while_body) => Statement::While(
            desugar_expression(sugared_while_cond),
            Box::new(desugar_statement(*sugared_while_body)),
        ),
        SugaredStatement::For(
            sugared_var_statement,
            sugared_stop_cond,
            sugared_reassign_statement,
            sugared_for_block_statements,
        ) => {
            let desugared_var_statement = desugar_statement(*sugared_var_statement);
            let desugared_stop_cond = desugar_expression(sugared_stop_cond);
            let desugared_reassign_statement = desugar_statement(*sugared_reassign_statement);
            let mut while_block_statements = desugar_statements(sugared_for_block_statements);

            while_block_statements.append(&mut vec![desugared_reassign_statement]);

            let while_statement = Statement::While(
                desugared_stop_cond,
                Box::new(Statement::Block(while_block_statements)),
            );

            return Statement::Block(vec![desugared_var_statement, while_statement]);
        }
        SugaredStatement::Break => Statement::Break,
        SugaredStatement::Continue => Statement::Continue,
        SugaredStatement::Block(sugared_statements) => {
            Statement::Block(desugar_statements(sugared_statements))
        }
    }
}

fn desugar_expression(sugared_expr: SugaredExpr) -> Expr {
    match sugared_expr {
        SugaredExpr::Num(n) => Expr::Num(n),
        SugaredExpr::Bool(b) => Expr::Bool(b),
        SugaredExpr::Str(s) => Expr::Str(s),
        SugaredExpr::Var(id) => Expr::Var(id),
        SugaredExpr::Binary(op, left_sugared_expr, right_sugared_expr) => Expr::Binary(
            op,
            Box::new(desugar_expression(*left_sugared_expr)),
            Box::new(desugar_expression(*right_sugared_expr)),
        ),
        SugaredExpr::Unary(op, expr) => Expr::Unary(op, Box::new(desugar_expression(*expr))),
        SugaredExpr::PrimitiveCall(name, sugared_args) => Expr::PrimitiveCall(
            name,
            sugared_args
                .iter()
                .map(|sugared_arg_expr| desugar_expression(sugared_arg_expr.clone()))
                .collect(),
        ),
        SugaredExpr::Call(sugared_proc_expr, sugared_args) => Expr::Call(
            Box::new(desugar_expression(*sugared_proc_expr)),
            sugared_args
                .iter()
                .map(|sugared_arg_expr| desugar_expression(sugared_arg_expr.clone()))
                .collect(),
        ),
        SugaredExpr::Lambda(params, sugared_body) => {
            Expr::Lambda(params, desugar_statements(sugared_body))
        }
    }
}
