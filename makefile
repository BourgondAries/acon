test:
	./test.sh

parse:
	./acon example

doc:
	pdflatex -output-directory temp report.tex
	evince temp/report.pdf

example:
	cat tests/0.input | ./acon
