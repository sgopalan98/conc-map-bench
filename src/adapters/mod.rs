// pub use self::{
//     btreemap::RwLockBTreeMapTable, chashmap::CHashMapTable, contrie::ContrieTable,
//     crossbeam_skiplist::CrossbeamSkipMapTable, dashmap::DashMapTable, evmap::EvmapTable,
//     flurry::FlurryTable, std::RwLockStdHashMapTable, server::ServerTable, leapfrog::LeapfrogMapTable
// };
pub use self::server::ServerTable;

// mod btreemap;
// mod chashmap;
// mod contrie;
// mod crossbeam_skiplist;
// mod dashmap;
// mod evmap;
// mod flurry;
// mod std;
mod server;
// mod leapfrog;

type Value = u32;
