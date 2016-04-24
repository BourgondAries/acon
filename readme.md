# Awk Ideas #
An AWK-compatible markup language.

# Concept #
Awk processes line-by-line. Awk scripts are easy to translate to other languages.

awki  # Register a name
{  # indents increase
	version 0.1.2
	message "hello world"
	an-array
	[
		1 2 3 4
		'there is no escape'
	]
}

