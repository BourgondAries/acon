#! /bin/bash

if ! [ -d temp ]; then
	mkdir temp
fi
successes=0
failures=0
for i in tests/*.input; do
	out="${i%.input}".output
	out=temp/"${out#tests/}"
	./acon "$i" > "$out"
	if ! diff "${i%.input}.correct" "$out"; then
		echo -e "\e[31mFailure for $i\e[0m"
		: $((++failures))
	else
		echo -e "\e[32mSuccess for $i\e[0m"
		: $((++successes))
	fi
done
echo $successes 'tests succeeded'
echo $failures 'tests failed'
