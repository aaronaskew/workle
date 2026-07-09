#!/bin/env bash

for i in {0..23}; do
    wget http://storage.googleapis.com/books/ngrams/books/20200217/eng/1-"$(printf "%05d" "$i")"-of-00024.gz
done
