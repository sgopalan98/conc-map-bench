use std::collections::hash_map::RandomState;
use std::{fmt::Debug, io, thread::sleep, time::Duration};

use bustle::*;
use fxhash::FxBuildHasher;
use structopt::StructOpt;

use crate::{adapters::*, record::Record, workloads};

#[derive(Debug, StructOpt)]
pub struct Options {
    #[structopt(short, long)]
    pub workload: workloads::WorkloadKind,
    #[structopt(short, long, default_value = "1")]
    pub operations: f64,
    #[structopt(short, default_value = "15")]
    pub capacity: u8,
    #[structopt(short, long, default_value = "1")]
    pub times: u8,
    #[structopt(short, long, default_value = "1")]
    pub ops_per_req: u8,
    #[structopt(short, long, default_value = "1")]
    pub server_threads: u8,
    #[structopt(long, default_value = "")]
    pub maptype: String,
    #[structopt(long)]
    pub client_threads: u8,
    #[structopt(long)]
    pub use_std_hasher: bool,
    #[structopt(long, default_value = "2000")]
    pub gc_sleep_ms: u64,
    #[structopt(long)]
    pub skip: Option<Vec<String>>, // TODO: use just `Vec<String>`.
    #[structopt(long)]
    pub complete_slow: bool,
    #[structopt(long)]
    pub csv: bool,
    #[structopt(long)]
    pub csv_no_headers: bool,
}

fn gc_cycle(options: &Options) {
    sleep(Duration::from_millis(options.gc_sleep_ms));
    let mut new_guard = crossbeam_epoch::pin();
    new_guard.flush();
    for _ in 0..32 {
        new_guard.repin();
    }
}

type Handler = Box<dyn FnMut(&str, u32, &Measurement)>;

fn case<C>(name: &str, options: &Options, handler: &mut Handler, additional_params: Option<Vec<u8>>)
where
    C: Collection,
    <C::Handle as CollectionHandle>::Key: Send + Debug,
{
    if options
        .skip
        .as_ref()
        .and_then(|s| s.iter().find(|s| s == &name))
        .is_some()
    {
        println!("-- {} [skipped]", name);
        return;
    } else {
        println!("-- {}", name);
    }

    let threads = options.client_threads;

    // let mut first_throughput = None;

    // let times = options.times;

    
    let m = workloads::create(options).run_silently::<C>(additional_params);
    handler(name, threads as u32, &m);


    gc_cycle(options);
    println!();
}

fn construct_shared_memory_params(options: &Options) -> Vec<u8> {
    let mut params = vec![];
    params.push(options.capacity);
    params.push(options.client_threads);
    params.push(options.ops_per_req);
    return params;
}

fn construct_delegation_params(options: &Options) -> Vec<u8> {
    let mut params = vec![];
    params.push(options.capacity);
    params.push(options.client_threads);
    params.push(options.ops_per_req);
    params.push(options.server_threads);
    return params;
}

fn run(options: &Options, h: &mut Handler) {
    match options.maptype.as_str() {
        "Dashmap" => {
            let params = construct_shared_memory_params(options);
            case::<ServerTable<u64>>("DashMapServer", options, h, Some(params));
        },
        "Delegation" => {
            let params = construct_delegation_params(options);
            case::<ServerTable<u64>>("Delegation Server", options, h, Some(params));
        },
        &_ => {
            
        }
    }
}

pub fn bench(options: &Options) {
    println!("== {:?}", options.workload);

    let mut handler = if options.csv {
        let mut wr = csv::WriterBuilder::new()
            .has_headers(!options.csv_no_headers)
            .from_writer(io::stderr());

        Box::new(move |name: &str, n, m: &Measurement| {
            wr.serialize(Record {
                name: name.into(),
                total_ops: m.total_ops,
                threads: n,
                spent: m.spent.as_secs_f64(),
                throughput: m.throughput / 10f64.powi(6),
                latency: m.latency,
            })
            .expect("cannot serialize");
            wr.flush().expect("cannot flush");
        }) as Handler
    } else {
        Box::new(|_: &str, n, m: &Measurement| {
            eprintln!(
                "total_ops={}\tthreads={}\tspent={:.1?}\tlatency={:?}\tthroughput={:.0}op/s",
                m.total_ops, n, m.spent, m.latency, m.throughput,
            )
        }) as Handler
    };

    run(&options, &mut handler);
}
