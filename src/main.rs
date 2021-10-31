#[macro_use]
extern crate pest_derive;
use pest::Parser;
use std::fs;
mod parser;
use parser::{CalcParser, *};
mod interpreter;

fn main() {
  let unparsed_file = fs::read_to_string("calc.txt").expect("cannot read file");

  let input = CalcParser::parse(Rule::file, &unparsed_file)
    .expect("unsuccessful parse") // unwrap the parse result
    .next()
    .unwrap();

  match input.as_rule() {
    Rule::file => {
      for iter in input.clone().into_inner() {
        match iter.as_rule() {
          Rule::expr => {
            let num = interpreter::expr_ast(iter);
            println!("result: {}", num);
          }
          _ => {}
        }
      }
    }
    _ => {}
  }
}
