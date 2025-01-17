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

    header="name,no,total_ops,threads,spent,throughput,latency"
    echo $header > $file

    if [ -s "$file" ]; then
        ARGS+=" --csv-no-headers"
    fi

    threads=( $(seq 1 $3 ) )

    if ! "$BIN" bench -w $1 --threads ${threads[@]} $ARGS --csv 2>>"$file"; then
        bench "$1" "$2" "$3" "$4"
    fi
}

no_threads=$1
capacity=$2
times=$3
operations_stretch=$4

bench ReadHeavy std $no_threads "-c $capacity -t $times --ops-st $operations_stretch"
bench Exchange std $no_threads "-o 0.5 -c $capacity -t $times --ops-st $operations_stretch"
bench RapidGrow std $no_threads "-c $capacity -t $times --ops-st $operations_stretch"
date
