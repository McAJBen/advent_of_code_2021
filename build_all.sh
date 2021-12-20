start=`date +%s`

cargo build --release --bin puzzle_1_1
cargo build --release --bin puzzle_1_2
cargo build --release --bin puzzle_2_1
cargo build --release --bin puzzle_2_2
cargo build --release --bin puzzle_3_1
cargo build --release --bin puzzle_3_2
cargo build --release --bin puzzle_4_1
cargo build --release --bin puzzle_4_2
cargo build --release --bin puzzle_5_1
cargo build --release --bin puzzle_5_2
cargo build --release --bin puzzle_6_1
cargo build --release --bin puzzle_6_2
cargo build --release --bin puzzle_7_1
cargo build --release --bin puzzle_7_2
cargo build --release --bin puzzle_8_1
cargo build --release --bin puzzle_8_2
cargo build --release --bin puzzle_9_1
cargo build --release --bin puzzle_9_2
cargo build --release --bin puzzle_10_1
cargo build --release --bin puzzle_10_2
cargo build --release --bin puzzle_11_1
cargo build --release --bin puzzle_11_2
cargo build --release --bin puzzle_12_1
cargo build --release --bin puzzle_12_2
cargo build --release --bin puzzle_13_1
cargo build --release --bin puzzle_13_2
cargo build --release --bin puzzle_14_1
cargo build --release --bin puzzle_14_2
cargo build --release --bin puzzle_15_1
cargo build --release --bin puzzle_15_2
cargo build --release --bin puzzle_16_1
cargo build --release --bin puzzle_16_2
cargo build --release --bin puzzle_17_1
cargo build --release --bin puzzle_17_2
cargo build --release --bin puzzle_18_1
cargo build --release --bin puzzle_18_2
cargo build --release --bin puzzle_19_1
cargo build --release --bin puzzle_19_2
cargo build --release --bin puzzle_20_1
cargo build --release --bin puzzle_20_2

end=`date +%s`
echo "took $(expr $end - $start)s"

read
