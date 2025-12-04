#!/usr/bin/env bash

invalid_ids=()

while read -r input; do
	IFS=, read -r -a id_ranges <<< "$input"

	for id_range in "${id_ranges[@]}"; do
		IFS=- read -r -a range_ends <<< "$id_range"

		range_start="${range_ends[0]}"
		range_end="${range_ends[1]}"
		
		echo "Range starts at $range_start and ends at $range_end"

		for (( i = range_start; i <= range_end; i++ )); do
			len=${#i}

			if (( len % 2 != 0 )); then
				continue
			fi

			first_half=${i:0:((len/2))}
			second_half=${i:((len/2))}

			if [[ $first_half == $second_half ]]; then
				invalid_ids+=($i)
			fi
		done
	done
done

id_total=0
for invalid_id in "${invalid_ids[@]}"; do
	(( id_total += invalid_id ))
done

echo "$id_total"
