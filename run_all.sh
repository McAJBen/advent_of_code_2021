overall_start=`date +%s%N`

for day in {1..21}
do
  for part in 1 2
  do
    start=`date +%s%N`
    cargo run --release --bin puzzle_${day}_${part}
    echo "  took $(expr $(expr $(date +%s%N) - $start) / 1000000) ms"
  done
done

echo "  overall took $(expr $(expr $(date +%s%N) - $overall_start) / 1000000) ms"

read
