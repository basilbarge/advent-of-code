#!/usr/bin/env bash

dial=50

zero_count=0

echo "starting at $dial"

while read -r instruction; do
	dir="${instruction:0:1}"

	length=$((${#instruction}-1))

	amount="${instruction:1}"

	if [[ $dir == 'L' ]]; then

		for (( i =  0; i < amount; i++)); do
			(( dial -= 1 ))

			if (( dial < 0 )); then
				dial=99
			fi

			if (( dial == 0 )); then
				(( zero_count++ ))
			fi
		done

	elif [[ $dir == 'R' ]]; then

		for (( i =  0; i < amount; i++)); do
			(( dial += 1 ))

			if (( dial > 99 )); then
				dial=0
			fi

			if (( dial == 0 )); then
				(( zero_count++ ))
			fi
		done

	fi

done

echo $zero_count
