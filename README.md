# conc-map-bench

Build:

cargo build --release

To evaluate Dashmap (example):

./target/release/conc-map-bench bench --maptype Dashmap -c 20 --client-threads 1 --workload ReadHeavy --ops-per-req 100

To evaluate Delegation (example):

./target/release/conc-map-bench bench --maptype Delegation -c 20 --client-threads 1 --workload ReadHeavy --ops-per-req 100 --server-threads 1

