test:
	./test.sh

parse:
	./parse.awk example

doc:
	pdflatex -output-directory temp report.tex
	evince temp/report.pdf
