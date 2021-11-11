use std::str::FromStr;

use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::{digit1 as digit, multispace0 as multispace},
  combinator::{map, map_res},
  multi::many0,
  sequence::{delimited, preceded},
  IResult,
};

#[derive(Debug, PartialEq)]
pub enum Op {
  Add, // +
  Sub, // -
  Mul, // *
  Div, // /
}

/// Expr
///  Value = Number
///  Add = Expr + Expr
///  Sub = Expr - Expr
///  Mul = Expr * Expr
///  Div = Expr / Expr
///  Paren = ( Expr )
#[derive(Debug, PartialEq)]
pub enum Expr {
  Value(i32),
  Add(Box<Expr>, Box<Expr>),
  Sub(Box<Expr>, Box<Expr>),
  Mul(Box<Expr>, Box<Expr>),
  Div(Box<Expr>, Box<Expr>),
  Paren(Box<Expr>),
}

pub fn expr(i: &str) -> IResult<&str, Expr> {
  let (i, initial) = term(i)?;
  let (i, remainder) = many0(alt((
    |i| {
      let (i, add) = preceded(tag("+"), term)(i)?;
      Ok((i, (Op::Add, add)))
    },
    |i| {
      let (i, sub) = preceded(tag("-"), term)(i)?;
      Ok((i, (Op::Sub, sub)))
    },
  )))(i)?;

  Ok((i, fold_exprs(initial, remainder)))
}

pub fn fold_exprs(initial: Expr, remainder: Vec<(Op, Expr)>) -> Expr {
  remainder.into_iter().fold(initial, |acc, pair| {
    let (oper, expr) = pair;
    match oper {
      Op::Add => Expr::Add(Box::new(acc), Box::new(expr)),
      Op::Sub => Expr::Sub(Box::new(acc), Box::new(expr)),
      Op::Mul => Expr::Mul(Box::new(acc), Box::new(expr)),
      Op::Div => Expr::Div(Box::new(acc), Box::new(expr)),
    }
  })
}

pub fn term(i: &str) -> IResult<&str, Expr> {
  let (i, initial) = factor(i)?;
  let (i, remainder) = many0(alt((
    |i| {
      let (i, mul) = preceded(tag("*"), factor)(i)?;
      Ok((i, (Op::Mul, mul)))
    },
    |i| {
      let (i, div) = preceded(tag("/"), factor)(i)?;
      Ok((i, (Op::Div, div)))
    },
  )))(i)?;

  Ok((i, fold_exprs(initial, remainder)))
}

pub fn factor(i: &str) -> IResult<&str, Expr> {
  alt((
    map(
      map_res(delimited(multispace, digit, multispace), FromStr::from_str),
      Expr::Value,
    ),
    parens,
  ))(i)
}

pub fn parens(i: &str) -> IResult<&str, Expr> {
  delimited(
    multispace,
    delimited(tag("("), map(expr, |e| Expr::Paren(Box::new(e))), tag(")")),
    multispace,
  )(i)
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn expr_test() {
    assert_eq!(
      expr("1 - 2 - 3"),
      Ok((
        "",
        Expr::Sub(
          Box::new(Expr::Sub(
            Box::new(Expr::Value(1)),
            Box::new(Expr::Value(2))
          )),
          Box::new(Expr::Value(3))
        ),
      ))
    );
  }
}
