# acon #

An Awk-compatible markup language.

# Concept #

Awk processes line-by-line. Awk scripts are easy to translate to other languages. The language is simple to process. This command line util can search for a regular expression as a path.

	{
		version 0.1.2
		message "hello world"
		an-array
		[
			1 2 3 4
			this is the second element in the array
		]
	}

# Usage #

You can use acon to annotate every line with its path.

	cat your-file | acon

It is also possible to filter the paths by a regular expression.

	cat your-file | acon 'my-regex'

Try running one of the files under test `*.input` as your-file.
