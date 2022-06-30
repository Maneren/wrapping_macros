#![crate_type = "dylib"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::ToTokens;
use syn::{
    parse_macro_input, parse_quote,
    visit_mut::{self, VisitMut},
    BinOp, Expr, ExprAssignOp, ExprBinary, ExprParen, ExprUnary, Ident, UnOp,
};

#[proc_macro]
pub fn wrapping(input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as Expr);
    LiteralReplacer.visit_expr_mut(&mut input);
    input.into_token_stream().into()
}

struct LiteralReplacer;

impl VisitMut for LiteralReplacer {
    fn visit_expr_mut(&mut self, i: &mut Expr) {
        match i {
            Expr::Unary(ExprUnary {
                op: UnOp::Neg(..),
                expr,
                ..
            }) => {
                // Recurse in sub-expressions
                self.visit_expr_mut(expr);

                // remove parenthesis to prevent clippy warning
                let expr = try_unwrap_parenthesis(expr);

                // Rewrite `-a` to `a.wrapping_neg()`
                let method = Ident::new("wrapping_neg", Span::call_site());
                *i = parse_quote!( #expr.#method() );
            }
            Expr::Binary(ExprBinary {
                left, op, right, ..
            }) => {
                // Recurse in sub-expressions
                self.visit_expr_mut(left);
                self.visit_expr_mut(right);

                // remove parenthesis to prevent clippy warning
                let left = try_unwrap_parenthesis(left);
                let right = try_unwrap_parenthesis(right);

                // Rewrite e.g. `a + b` to `a.wrapping_add(b)`
                if let Some(method) = wrapping_method(op) {
                    *i = parse_quote!( #left.#method(#right) );
                }
            }
            Expr::AssignOp(ExprAssignOp {
                left, op, right, ..
            }) => {
                // Recurse in sub-expressions
                self.visit_expr_mut(right);

                // remove parenthesis to prevent clippy warning
                let right = try_unwrap_parenthesis(right);

                // Rewrite e.g. `a += b` to `a = a.wrapping_add(b)`
                if let Some(method) = wrapping_method(op) {
                    *i = parse_quote!( #left = #left.#method(#right) );
                }
            }
            _ => {
                // Recurse in sub-expressions
                visit_mut::visit_expr_mut(self, i);
            }
        }
    }
}

/// Returns the wrapping version of an operator, if applicable.
fn wrapping_method(op: &BinOp) -> Option<Ident> {
    let name = match op {
        BinOp::Add(..) | BinOp::AddEq(..) => "wrapping_add",
        BinOp::Sub(..) | BinOp::SubEq(..) => "wrapping_sub",
        BinOp::Mul(..) | BinOp::MulEq(..) => "wrapping_mul",
        BinOp::Div(..) | BinOp::DivEq(..) => "wrapping_div",
        BinOp::Rem(..) | BinOp::RemEq(..) => "wrapping_rem",
        BinOp::Shl(..) | BinOp::ShlEq(..) => "wrapping_shl",
        BinOp::Shr(..) | BinOp::ShrEq(..) => "wrapping_shr",
        _ => return None,
    };
    Some(Ident::new(name, Span::call_site()))
}

fn try_unwrap_parenthesis(expr: &mut Expr) -> &mut Expr {
    if let Expr::Paren(ExprParen { expr, .. }) = expr {
        expr
    } else {
        expr
    }
}
