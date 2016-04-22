## koml ##

Kev's Obvious Minimal Language.

Simple markup language. Mix of toml, json, and sgml.

## Key-Value Pairs ##
Key-value pairs are separated by spaces.

All non-whitespace characters are allowed in keys.

Values can be of the types array, boolean, integers, floats, strings, and tables.

	my_key 100
	the?key|right? 'This is my string'

## Tables ##

A table is simply a value starting with a bare `{`.

	key {
		sub-key1 10
		sub-key2 20
		sub-key3 30
		other-key 'This is my other key, it is key.other-key'
	}
	key.inside 'A key that is part of the key table'

This allows you to specify keys inside tables.

