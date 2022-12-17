#!/usr/bin/env bash

set -x

BIN=./target/release/conc-map-bench
OUT=./results

cargo build --release
mkdir -p "$OUT"

function bench {
    ARGS=$4

    if [ "$2" = "std" ]; then
        ARGS+=" --use-std-hasher"
    fi

    date

    file="$OUT/$1.$2.csv"

    if [ -s "$file" ]; then
        ARGS+=" --csv-no-headers"
    fi

    skip=$(cat "$file" | cut -d, -f1 | uniq | paste -sd ' ' -)

    if ! "$BIN" bench -w $1 -c $3 $ARGS --skip $skip --csv 2>>"$file"; then
        bench "$1" "$2" "$3" "$4"
    fi
}

bench ReadHeavy fx 22
bench Exchange fx 22 '-o 0.5' # because of OOM in case of `flurry`
bench RapidGrow fx 22

bench ReadHeavy std 22
bench Exchange std 22 '-o 0.5'
bench RapidGrow std 22
date
