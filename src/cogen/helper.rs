pub fn sp_add(n: usize) -> String {
  format!("\tadd	sp, sp, {}\n", n)
}

pub fn sp_sub(n: usize) -> String {
  format!("\tsub	sp, sp, #{}\n", n)
}

/// op には演算命令が入る
pub fn calc(op: &str) -> String {
  let mut res = vec![];
  // val1
  res.push(sp_add(16));
  res.push(format!("\tldr	w0, [sp, 12]\n"));

  // val2
  res.push(sp_add(16));
  res.push(format!("\tldr	w1, [sp, 12]\n"));

  // calc
  res.push(format!("\t{}	w0, w1, w0\n", op));

  // store
  res.push(format!("\tstr	w0, [sp, 12]\n"));
  res.push(sp_sub(16));

  // return
  res.join("")
}
