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

pub fn val_ast(val: pest::iterators::Pair<Rule>) -> i32 {
  // let valvec: Vec<_> = val.into_inner().collect();
  let mut num = 0;
  for iter in val.into_inner() {
    match iter.as_rule() {
      Rule::num => {
        num = iter.as_str().parse().unwrap();
      },
      Rule::parexpr => {
        num += parexpr_ast(iter);
      }
      _ => {
        unreachable!();
      }
    }
  }
  num
}

/// use only par expr
pub fn parexpr_ast(par_expr: pest::iterators::Pair<Rule>) -> i32 {
  let mut num = 0;
  for iter in par_expr.into_inner() {
    match iter.as_rule() {
      Rule::expr => {
        num += expr_ast(iter);
      }
      _ => {
        unreachable!();
      }
    }
  }
  num
}
