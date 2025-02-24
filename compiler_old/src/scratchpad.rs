use sexpr;
use sexpr_derive::SExpr;

#[derive(SExpr)]
enum Expr {
    Add(u32, u32),
    Sub(u32, u32),
}
