#! /bin/bash

# Probably I should install anothre spell checker...

for src in $(find . -type f -name '*.rs') ; do
    TMP_FILE=$(mktemp)
    grep '//' "$src" | hunspell -d en_US -l | sort | uniq > "$TMP_FILE"
    if [[ '0' != $(du -hs "$TMP_FILE" | cut -f 1) ]] ; then
        echo "$src"
        while read LINE ; do
            echo -e "\t$LINE"
        done < "$TMP_FILE"
    fi
done
