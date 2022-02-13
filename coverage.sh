#!/bin/sh

non_empty=$(cat test/* | grep -ve '^$')
lines=$(echo "$non_empty" | wc -l)
uncommented=$(echo "$non_empty" | grep -cv "^\s*//")

echo Coverage: $(( uncommented * 100 / lines )) %
