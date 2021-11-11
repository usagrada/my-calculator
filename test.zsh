assert() {
  expected="$1"
  input="$2"

  rm test.s a.out
  cargo run -- "$input" > test.s
  gcc test.s
  ./a.out

  actual="$?"

  if [ "$actual" = "$expected" ]; then
    echo "$input => $actual"
  else
    echo "$input => $expected expected, but got $actual"
    exit 1
  fi
}

assert 0 0
assert 42 42
assert 6 "1 + 2 + 3"
assert 0 "5 - 2 - 3"
assert 4 "10 - 2 * 3"
assert 1 "5 / 2 - 1"

echo OK