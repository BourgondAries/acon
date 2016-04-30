json2acon:
	cargo build
	cat example | ./target/debug/json2acon
test:
	./test.sh

parse:
	./parse.awk example

doc:
	pdflatex -output-directory temp report.tex
	evince temp/report.pdf
