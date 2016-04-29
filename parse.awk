#! /usr/bin/awk -f

BEGIN { comment = 0; current = 0; listn = 0; print "Beginning Parsing"; }
$1 ~ /##/ { comment = comment == 1 ? 0 : 1; next; }
// { if (comment == 1) { next; } }
$1 ~ /#/ { next; }
$1 ~ /list/ { ++listn; next; }
$1 ~ /begin/ { list[current] = $2; ++current; next; }
$1 ~ /end/ {
	if (current == 0) {
		print "Encountered end without matching begin" | "cat 1>&2";
		exit 1;
	}
	--current;
	next;
}
! $1 ~ /begin|end|list/ {
	for (i = 0; i < current; ++i)
		printf "%s.", list[i];
	for (i = 1; i <= NF; ++i)
		printf "%s ", $i;
	printf "\n";
}
END { print "Ending Parsing"; }
