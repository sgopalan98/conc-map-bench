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

    threads=( $(seq 1 $3 ) )

    if ! "$BIN" bench -w $1 --threads ${threads[@]} $ARGS --skip $skip --csv 2>>"$file"; then
        bench "$1" "$2" "$3" "$4"
    fi
}

no_threads=168
capacity=22
times=10
bench ReadHeavy fx $no_threads "-c $capacity -t $times"
bench Exchange fx $no_threads "-o 0.5 -c $capacity -t $times" # because of OOM in case of `flurry`
bench RapidGrow fx $no_threads "-c $capacity -t $times"

bench ReadHeavy std $no_threads "-c $capacity -t $times"
bench Exchange std $no_threads "-o 0.5 -c $capacity -t $times"
bench RapidGrow std $no_threads "-c $capacity -t $times"
date
