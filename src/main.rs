use mylang;

fn main() {
  let args: Vec<String> = std::env::args().collect();
  // println!("{:?}", args);
  let input = &args[1];
  let (_, exp) = mylang::ast::expr(input).expect("");
  let code = mylang::cogen::cogen(exp);
  println!("{}", code);
}
