#!/bin/bash

awk '{
	print length
	sum += length
} 
END {
	print "mean=", sum/NR
}' input.txt
