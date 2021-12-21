overall_start=`date +%s%N`

for day in {1..21}
do
  for part in 1 2
  do
    cargo build --release --bin puzzle_${day}_${part}
  done
done

echo "  overall took $(expr $(expr $(date +%s%N) - $overall_start) / 1000000) ms"

read
