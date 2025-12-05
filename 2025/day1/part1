#!/usr/bin/env bash

dial=50

zero_count=0

while read -r instruction; do
	dir="${instruction:0:1}"

	length=$((${#instruction}-1))

	amount="${instruction:1:length}"

	if [[ $dir == 'L' ]]; then
		((dial=((dial-amount) + 100) % 100))
	elif [[ $dir == 'R' ]]; then
		((dial=((dial + amount) + 100) % 100))
	fi

	if (( dial == 0 )); then
		((zero_count+=1))
	fi
done

echo $zero_count
