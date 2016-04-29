#! /usr/bin/awk -f

# key - The list of keys to this entry
# ikey - The top index of the key
# list - The list of array depths
# ilist - The current top index of the list
# com - Current comment
BEGIN { comment = 0; ikey = 0; list = 0; }
$1 ~ /\[/ { key[ikey]=$2; ++ikey; ilist[list]=ikey; ++list; key[ikey]=0; ++ikey;next;}
$1 ~ /\]/ { --list;--ikey;next;}
$1 ~ /{/ {key[ikey] = $2; ++ikey; next;}
$1 ~ /}/ {
	if (ikey == 0) {
		print "Encountered end without matching begin" | "cat 1>&2";
		exit 1;
	}
	--ikey;
	next;
}
// {
	for (i = 0; i < ikey; ++i)
		printf "%s.", key[i];
	for (i = 1; i <= NF; ++i)
		printf "%s ", $i;
	printf "\n";
	if (list-1 >= 0 && ilist[list-1] == ikey-1) {
		++key[ikey-1];
	}
}
END { print "Ending Parsing" | "cat 1>&2"; }
