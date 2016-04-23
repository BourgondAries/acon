## koml ##

Kev's Obvious Minimal Language.

Simple markup language. Mix of toml, json, and sgml.

## Key-Value Pairs ##
Key-value pairs are separated by spaces.

All non-whitespace characters are allowed in keys.

Values can be of the types array, boolean, integers, floats, strings, and tables.

	my_key 100
	the?key|right? 'This is my string'

## Grammar ##
The grammar of koml is given by the following ebnf:

	START ::= ( KEY VALUE )* ;

	KEY ::= non_space+ ;
	VALUE ::=
		ARRAY
		| BOOLEAN
		| INTEGER
		| FLOAT
		| STRING
		| TABLE ;

	ARRAY ::= '[' VALUE* ']';
	BOOLEAN ::= 'false' | 'true';
	INTEGER ::= \d+;
	FLOAT ::= \d+'.'\d+;
	STRING ::= '\''.*'\'';
	TABLE ::= '{' START '}';

## Space-separation ##
Space separation of the keys, symbols, and variables is chosen because it makes
the code easy to write. It also removes the need for shady edge cases.

## Simple Key-Value Pair ##
	key 'value'

Keys are always unique. Multiplicities are erroneous.

## Types ##

	key-one [ 1 2 3 'is an array' ]  # An array
	key-two true  # A boolean
	key-three 1  # An integer
	key-four 3.14159  # A float
	key-five 'value'  # A String
	key-six {  # A table
		sub-key 'value'
	}

## Arrays ##
Arrays can contain any value type. They are heterogeneous.
Arrays can contain tables.

	key [ [ true ] false 0 1e9 'value' { subkey 'other' } ]

Space separation ensures correct behaviour.
What about mixing text with other stuff?
