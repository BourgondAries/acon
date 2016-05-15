test:
	./test.sh

parse:
	./acon example

doc:
	pdflatex -output-directory temp report.tex
	evince temp/report.pdf
