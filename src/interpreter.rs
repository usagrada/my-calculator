use super::Rule;

pub fn expr_ast(expr: pest::iterators::Pair<Rule>) -> i32 {
  let mut exprvec: Vec<_> = expr.into_inner().collect();
  exprvec.reverse();
  let mut num1 = if let Some(n) = exprvec.pop() {
    mul_ast(n)
  } else {
    unreachable!()
  };

  loop {
    let top = exprvec.pop();
    if let Some(v) = top {
      match v.as_rule() {
        Rule::op => {
          let num2 = exprvec.pop();
          if let Some(num2) = num2 {
            let num2 = match num2.as_rule() {
              Rule::mul => mul_ast(num2),
              _ => expr_ast(num2),
            };
            match v.as_str() {
              "+" => {
                num1 += num2;
              }
              "-" => {
                num1 -= num2;
              }
              _ => {}
            }
          }
        }
        _ => {}
      }
    } else {
      break;
    }
  }
  num1
}

pub fn mul_ast(mul: pest::iterators::Pair<Rule>) -> i32 {
  let mut mulvec: Vec<_> = mul.into_inner().collect();
  mulvec.reverse();
  let mut num1 = if let Some(n) = mulvec.pop() {
    val_ast(n)
  } else {
    unreachable!()
  };

  loop {
    let top = mulvec.pop();
    if let Some(v) = top {
      match v.as_rule() {
        Rule::expr => {
          return expr_ast(v);
        }
        Rule::mul => {
          return mul_ast(v);
        }
        Rule::mulop => {
          let num2 = mulvec.pop();
          if let Some(num2) = num2 {
            let num2 = val_ast(num2);
            match v.as_str() {
              "*" => {
                num1 *= num2;
              }
              "/" => {
                num1 /= num2;
              }
              _ => {}
            }
          }
        }
        _ => {}
      }
    } else {
      break;
    }
  }
  num1
}

/// use for value
pub fn val_ast(val: pest::iterators::Pair<Rule>) -> i32 {
  val.into_inner().fold(0, |sum, iter| {
    let value = match iter.as_rule() {
      Rule::num => iter.as_str().parse().unwrap(),
      Rule::parexpr => parexpr_ast(iter),
      _ => unreachable!(),
    };
    sum + value
  })
}

/// use only par expr
pub fn parexpr_ast(par_expr: pest::iterators::Pair<Rule>) -> i32 {
  par_expr.into_inner().fold(0, |sum, iter| {
    let value = match iter.as_rule() {
      Rule::expr => expr_ast(iter),
      _ => unreachable!(),
    };
    sum + value
  })
}

#[cfg(test)]
mod test {
  use super::super::parser::*;
  use super::*;
  use pest::Parser;
  #[test]
  fn test1() {
    let input = CalcParser::parse(Rule::expr, "1+1")
      .expect("unsuccessful parse") // unwrap the parse result
      .next()
      .unwrap();
    let num = expr_ast(input);
    assert_eq!(num, 2);
  }

  #[test]
  fn test2() {
    let input = CalcParser::parse(Rule::expr, "1*1")
      .expect("unsuccessful parse") // unwrap the parse result
      .next()
      .unwrap();
    let num = expr_ast(input);
    assert_eq!(num, 1);
  }
}
