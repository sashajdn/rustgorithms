use std::collections::{BTreeMap, HashMap};
use std::rc::Rc;
use std::sync::Arc;

// 24 bytes to store (ptr, cap. len) on 64 bit machine.
#[derive(Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct Id(String);

// 16 bytes to store (ptr, cap. len) on 64 bit machine.
#[derive(Hash, Eq, PartialEq, Clone, Ord, PartialOrd)]
struct BetterId(Arc<str>);

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct SomeObject(i32);

fn arc_str() {
    let mut hm: HashMap<Id, SomeObject> = Default::default();
    let mut bm: BTreeMap<Id, SomeObject> = Default::default();

    let mut hm2: HashMap<BetterId, SomeObject> = Default::default();
    let mut bm2: BTreeMap<BetterId, SomeObject> = Default::default();

    let id1: Id = Id(String::from("id1"));
    let id2: BetterId = BetterId(Arc::from("id2"));

    // bad
    hm.insert(id1.clone(), SomeObject(1));
    bm.insert(id1.clone(), SomeObject(1));

    // better since we're just incrementing reference counters than storing the whole data.
    // this is also better for cache locality, since the same Id in this case is being used repeatedly.
    // NOTE: We don't need Arc<T> if we don't need to send between threads.
    hm2.insert(id2.clone(), SomeObject(1));
    bm2.insert(id2.clone(), SomeObject(1));
}
