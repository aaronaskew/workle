#!/bin/env bash

# filter list to 5-letter words only
rg -NIz '^[a-zA-Z]{5}(_[A-Z]+)*\s' 1-*.gz >five-letter-n-grams.txt

# remove all entries that include _POS (the parts of speech) annotations
rg '^[^_]*$' five-letter-n-grams.txt >five-letter-n-grams-no-pos.txt

# sort by word ignoring case
sort -f five-letter-n-grams-no-pos.txt >five-letter-n-grams-sorted.txt

# run parse_ngrams to combine all counts and words
cargo run -p parse_ngrams -- -i five-letter-n-grams-sorted.txt -o five-letter-n-grams-combined.txt

# filter n-gram list to only wordle words
