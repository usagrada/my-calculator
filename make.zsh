rm test.s a.out
cargo run -- "1" > test.s
gcc test.s
./a.out
echo $?