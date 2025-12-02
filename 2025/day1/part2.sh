#!/usr/bin/env bash

dial=50

zero_count=0

while read -r instruction; do
	dir="${instruction:0:1}"

	length=$((${#instruction}-1))

	amount="${instruction:1:length}"

	scale_factor=$((1 + (amount / 100)))

	echo "moving $dir by $amount"

	if [[ $dir == 'L' ]]; then
		dial_start=$dial
		((dial=((dial-amount) + 100) % 100))

		# we went around the dial at least once
		if (( dial_start < dial )); then
			echo "went around dial $scale_factor times"
			((zero_count+=scale_factor))
		fi

	elif [[ $dir == 'R' ]]; then
		dial_start=$dial
		((dial=((dial + amount) + 100) % 100))
		# we went around the dial at least once
		if (( dial_start > dial )); then
			echo "went around dial $scale_factor times"
			((zero_count+=scale_factor))
		fi
	fi
done

echo $zero_count
