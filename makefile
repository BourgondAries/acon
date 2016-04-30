json2acon:
	cargo build
	cat example | ./target/debug/json2acon
test:
	./test.sh

parse:
	./acon example

doc:
	pdflatex -output-directory temp report.tex
	evince temp/report.pdf
