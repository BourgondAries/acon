#! /usr/bin/awk -f

BEGIN { paths = 0; lists = 0; printf "Beginning Parsing" | "cat 1>&2"; }
function printPath() { for (i = 0; i < paths - 1; ++i) if (path[i] != "") printf "%s.", path[i]; printf "%s", path[i]; }
function isInArray() { if (lists - 1 >= 0) { return list[lists - 1] == paths - 1; } return false; }
function printElements() { for (i = 0; i < NF; ++i) printf " %s", $i; }
function appendPath(argument) { path[paths] = argument; ++paths; }
$1 ~ /{/ { if (NF == 2) { path[paths] = $2; ++paths; } if (isInArray()) { appendPath(""); } next; }
$1 ~ /}/ { if (paths == 0) { print "Encountered } without matching begin" | "cat 1>&2"; exit 1; } --paths; next; }
$1 ~ /\[/ { if (NF == 2) { path[paths] = $2; ++paths; } else { unnamed[paths] = true; } path[paths] = 0; list[lists] = paths; ++lists; ++paths; next; }
$1 ~ /\]/ { --lists; --paths; if (unnamed[paths]) --paths; if (isInArray()) ++path[paths-1]; next; }
// {
	printPath();
	if (isInArray()) { ++path[paths-1]; printElements(); }
	else { printf ".%s", $1; }
	printf " ";
	printf "\n";
}

END { print "Ending Parsing" | "cat 1>&2"; }
