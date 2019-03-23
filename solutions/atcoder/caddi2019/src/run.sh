#!/bin/bash

cat example/in.txt | cargo run > out.txt

cat example/in.txt | cargo run out.txt > /tmp/hoge

cat /tmp/hoge >> results.txt
cat /tmp/hoge

