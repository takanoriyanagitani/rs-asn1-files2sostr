#!/bin/sh

oname="./sample.d/sequence-of-octet-string-values.asn1.der.dat"

geninput(){

	echo generating input files...

	mkdir -p sample.d

	printf hw0 > ./sample.d/file0.txt
	printf hw1 > ./sample.d/file1.txt
	printf hwii > ./sample.d/file2.txt
	printf hwiii > ./sample.d/file3.txt
	printf hwiv > ./sample.d/file4.txt

}

test -f "./sample.d/file0.txt" || geninput
test -f "./sample.d/file1.txt" || geninput
test -f "./sample.d/file2.txt" || geninput
test -f "./sample.d/file3.txt" || geninput
test -f "./sample.d/file4.txt" || geninput

find \
	./sample.d \
	-type f \
	-name '*.txt' |
	sort |
	sed \
		-n \
		-e 's/^.//' \
		-e 's/sample/guest/' \
		-e p |
	wazero \
		run \
		-mount "${PWD}/sample.d:/guest.d:ro" \
		./rs-asn1-files2sostr.wasm |
	dd \
		if=/dev/stdin \
		of="${oname}" \
		bs=1048576 \
		status=none

cat "${oname}" |
	fq \
		-d asn1_ber \
		'.constructed[].value'
