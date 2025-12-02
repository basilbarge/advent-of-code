#!/usr/bin/env bash

dial=50

zero_count=0

echo "starting at $dial"

while read -r instruction; do
	dir="${instruction:0:1}"

	length=$((${#instruction}-1))

	amount="${instruction:1:length}"

	scale_factor=$((1 + (amount / 100)))

	echo "moving $dir by $amount"

	prev_dial=$dial

	if [[ $dir == 'L' ]]; then
		new_dial=$((dial-amount))
		((dial=((new_dial) + 100) % 100))

		# we went around the dial at least once
		if (( new_dial <= 0 && prev_dial != 0 )); then
			echo "went around dial $scale_factor times"
			((zero_count+=scale_factor))
		fi

	elif [[ $dir == 'R' ]]; then
		new_dial=$((dial+amount))
		((dial=((new_dial) + 100) % 100))
		# we went around the dial at least once
		if (( new_dial >= 99 )); then
			echo "went around dial $scale_factor times"
			((zero_count+=scale_factor))
		fi
	fi

	echo "Dial is now at $dial"
done

echo $zero_count
