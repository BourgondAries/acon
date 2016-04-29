#! /usr/bin/awk -f

BEGIN { paths = 0; lists = 0; }
$1 ~ /{/ { path[paths] = $2; ++paths; next; }
$1 ~ /}/ { if (paths == 0) { print "Encountered } without matching begin" | "cat 1>&2"; exit 1; } --paths; next; }
$1 ~ /\[/ { if (NF == 2) {  } else { path[paths] = 0; list[lists] = paths; ++lists; ++paths; } }
// {
	for (i = 0; i < paths; ++i)
		printf "%s.", path[i];
	for (i = 1; i <= NF; ++i)
		printf "%s ", $i;
	printf "\n";
}
END { print "Ending Parsing" | "cat 1>&2"; }
