pub use self::{
    btreemap::RwLockBTreeMapTable, chashmap::CHashMapTable, contrie::ContrieTable,
    crossbeam_skiplist::CrossbeamSkipMapTable, dashmap::DashMapTable, evmap::EvmapTable,
    flurry::FlurryTable, leapfrog::LeapfrogMapTable, std::RwLockStdHashMapTable, striped::StripedMapTable
};

mod btreemap;
mod chashmap;
mod contrie;
mod crossbeam_skiplist;
mod dashmap;
mod evmap;
mod flurry;
mod leapfrog;
mod std;
mod striped;

type Value = u32;
