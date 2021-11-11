use crate::ast;
mod helper;

pub fn cogen(ast: ast::Expr) -> String {
  let header = "	.arch armv8-a
	.text
	.align	2
	.globl _main
_main:
"
  .to_owned();
  let mut result = vec![header];

  result.push(cogen_expr(ast));
  result.push(helper::sp_add(16));
  result.push(format!("\tldr	w0, [sp, 12]\n"));

  let ret = "\tret\n".to_owned();
  result.push(ret);
  result.concat()
}

pub fn cogen_expr(ast: ast::Expr) -> String {
  let mut result = vec![];
  match ast {
    ast::Expr::Value(i) => {
      result.push(format!("\tmov	w0, {}\n", i));
      result.push(format!("\tstr	w0, [sp, 12]\n"));
      result.push(helper::sp_sub(16));
    }
    ast::Expr::Add(l, r) => {
      result.push(cogen_expr(*l));
      result.push(cogen_expr(*r));

      result.push(helper::calc("add"));
    }
    ast::Expr::Sub(l, r) => {
      result.push(cogen_expr(*l));
      result.push(cogen_expr(*r));

      result.push(helper::calc("sub"));
    }
    ast::Expr::Mul(l, r) => {
      result.push(cogen_expr(*l));
      result.push(cogen_expr(*r));

      result.push(helper::calc("mul"));
    }
    ast::Expr::Div(l, r) => {
      result.push(cogen_expr(*l));
      result.push(cogen_expr(*r));
      result.push(helper::calc("udiv"));
    }
    ast::Expr::Paren(e) => {
      result.push(cogen_expr(*e));
    }
  }

  result.concat()
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn it_works() {
    // #![feature(asm)]
    unsafe {
      asm!("nop");
    }
    let (_, expr) = ast::expr("1").expect("error");
    let n = cogen_expr(expr);
    unsafe {
    }
  }
}
