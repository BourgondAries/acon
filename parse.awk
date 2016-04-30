#! /usr/bin/awk -f

BEGIN { paths = 0; lists = 0; printf "Beginning Parsing" | "cat 1>&2"; }
function printElements(start) { for (i = start; i <= NF; ++i) printf " %s", $i; }
function isInArray() { return lists > 0 && topList() == paths; }
function printPath() { for (i = 0; i < paths - 1; ++i) if (path[i] != "") { printf "%s.", path[i]; } if (path[paths - 1] != "") printf "%s", path[paths - 1]; }
function pushPath(argument) { path[paths] = argument; ++paths; }
function popPath() { if (paths <= 0) { printf "Can not pop path, already empty" | "cat 1>&2"; exit 1; } else { --paths; return path[paths]; }}
function pushList(argument) { list[lists] = argument; ++lists; }
function popList() { if (lists <= 0) { printf "Can not pop list, already empty" | "cat 1>&2"; exit 1; } else { --lists; return list[lists]; }}
function topList() { if (lists <= 0) { printf "Can not get top element, list is empty" | "cat 1>&2"; exit 1; } else { return list[lists - 1]; }}
function incrementIfInList() { if (paths == list[lists - 1]) ++path[paths - 1]; }
function isNamed() { if (paths == 0 || path[paths - 1] == "") return ""; else return "."; }
function printUnitTitle() { if (lists <= 0 || lists > 0 && isInArray() == 0) { if ($1 != "") printf "%s%s", isNamed(), $1; } }
$1 ~ /{/ { pushPath($2); next; }
$1 ~ /}/ { popPath(); next; }
$1 ~ /\[/ { pushPath($2); pushPath(0); pushList(paths); next; }
$1 ~ /\]/ { popPath(); popPath(); popList(); next; }
// {
	printPath();
	printUnitTitle();
	printElements((isInArray() == 0) + 1);
	printf "\n";
	incrementIfInList();
}
END { print "Ending Parsing" | "cat 1>&2"; }
