// pub use self::{
//     btreemap::RwLockBTreeMapTable, chashmap::CHashMapTable, contrie::ContrieTable,
//     crossbeam_skiplist::CrossbeamSkipMapTable, dashmap::DashMapTable, evmap::EvmapTable,
//     flurry::FlurryTable, std::RwLockStdHashMapTable,
// };
pub use self::dashmap::DashMapTable;

// mod btreemap;
// mod chashmap;
// mod contrie;
// mod crossbeam_skiplist;
mod dashmap;
// mod evmap;
// mod flurry;
// mod std;
// pub mod striped;

type Value = u32;
