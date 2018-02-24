#!/bin/bash

echo
echo "    Extracting archive, this can take a while..."
7z x pwned-passwords-2.0.txt.7z
rm pwned-passwords-2.0.txt.7z # make room

echo "    Sorting database, this can take a while..."
LC_ALL=C sort pwned-passwords-2.0.txt -o pwned-passwords-2.0-sorted.txt
rm pwned-passwords-2.0.txt # make room

echo "    Uniformizing archive, this can take a while..."
cut -d ':' -f 1 < pwned-passwords-2.0-sorted.txt > pwned-passwords-2.0-cut.txt
rm pwned-passwords-2.0-sorted.txt # make room

echo
echo "    Done! Processed database is in `pwned-passwords-2.0-cut.txt`."
echo
