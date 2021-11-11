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
  result.push("\tldr	w0, [sp, 12]\n".to_owned());

  let ret = "\tret\n".to_owned();
  result.push(ret);
  result.concat()
}

pub fn cogen_expr(ast: ast::Expr) -> String {
  let mut result = vec![];
  match ast {
    ast::Expr::Value(i) => {
      result.push(format!("\tmov	w0, {}\n", i));
      result.push("\tstr	w0, [sp, 12]\n".to_owned());
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
  use std::str::FromStr;
  #[test]
  fn test() {
    let s = "-1";
    let i: i32 = FromStr::from_str(s).expect("error");
    assert_eq!(-1, i);
  }

  #[test]
  fn test2() {
    let s = "+1";
    let i: i32 = FromStr::from_str(s).expect("error");
    assert_eq!(1, i);
  }
}