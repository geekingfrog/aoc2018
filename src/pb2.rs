
use std::ops::Deref;
use std::fs::File;
use std::io::{BufRead,BufReader};
use fxhash::FxHashMap;

/// One entry, normalized from ascii
#[derive(PartialEq,Eq,Hash,Clone)]
struct ID(Vec<u8>);

impl Deref for ID {
    type Target = Vec<u8>;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl ID {
    fn from_str(s: &str) -> Self {
        let v = s.as_bytes().iter().map(|c| c - ('a' as u8)).collect();
        ID(v)
    }

    /// Split into even elements, and odd elements.
    ///
    /// This is useful because if two IDs differ by only 1 char, at least
    /// one of their event/odd slices will coincide, so we can use a hashmap.
    fn split(&self) -> (ID, ID) {
        let mut v1 = Vec::new();
        let mut v2 = Vec::new();
        for i in 0 .. self.0.len() {
            if i % 2 == 0 {
                v1.push(self.0[i])
            } else {
                v2.push(self.0[i])
            }
        }
        (ID(v1),ID(v2))
    }

    fn to_string(&self) -> String {
        let v = self.0.iter().map(|c| c + ('a' as u8)).collect();
        String::from_utf8(v).unwrap()
    }

    /// If distance from self to other is `1`, return the common part
    fn at_distance_1(&self, other: &Self) -> Option<Self> {
        let mut i_diff = None;

        if self.len() != other.len() { return None }

        for i in 0 .. self.len() {
            if self[i] != other[i] {
                if let Some(_) = i_diff {
                    return None;  // 2 distinct elements
                } else {
                    i_diff = Some(i);
                }
            }
        }

        i_diff.map(|i| {
            let mut v = self.0.clone();
            v.remove(i);
            ID(v)
        })
    }
}

fn parse(arg: &str) -> Vec<ID> {
    let file = File::open(arg).unwrap();
    let reader = BufReader::new(file);

    // parse lines as integers
    reader
        .lines()
        .map(|s| s.unwrap())
        .map(|s| ID::from_str(&s))
        .collect()
}

const N : usize = 26;

/// Map from an ascii char, to its number of occurrences
struct Counts([u8;N]);

impl Counts {
    fn has_dup(&self) -> bool {
        self.0.iter().any(|&x| x == 2)
    }
    fn has_triple(&self) -> bool {
        self.0.iter().any(|&x| x == 3)
    }
}

fn counts(id: &ID) -> Counts {
    let mut seen = [0; N];
    for &c in id.iter() {
        seen[c as usize] += 1;
    }
    Counts(seen)
}

// Find number of strings with duplicate letters (resp. triplicate letters)
fn find_counts(ops: &Vec<ID>) -> (usize, usize) {
    ops.iter().fold((0,0), |(mut n2, mut n3),s| {
        let counts = counts(s);
        if counts.has_dup() { n2 += 1 };
        if counts.has_triple() { n3 += 1 };
        (n2, n3)
    })
}

struct IDSet(Vec<ID>);

impl IDSet {
    fn new() -> Self { IDSet(Vec::new()) }
    fn insert(&mut self, id: &ID) { self.0.push(id.clone()) }
    fn find_at_distance_1(&self, id: &ID) -> Option<(ID,ID,String)> {
        for other in self.0.iter() {
            if let Some(common) = id.at_distance_1(other) {
                return Some((id.clone(),other.clone(),common.to_string()))
            }
        }
        None
    }
}

/// find two IDs that only differ in one position, and also return
/// their common part.
///
/// We keep a multimap from half-slices to their owner IDs, so
/// we can efficiently find collisions
fn find_ids_close_by_1(ops: &Vec<ID>) -> (ID,ID,String) {
    let mut tbl_even : FxHashMap<ID,IDSet> = FxHashMap::default();
    let mut tbl_odds : FxHashMap<ID,IDSet> = FxHashMap::default();

    for id in ops {
        let (id1, id2) = id.split();

        // look for collisions
        if let Some(r) = tbl_even.get(&id1).and_then(|v| v.find_at_distance_1(id)) { return r }
        if let Some(r) = tbl_odds.get(&id2).and_then(|v| v.find_at_distance_1(id)) { return r }

        // no collision, insert into tables
        let e = tbl_even.entry(id1);
        let mut set = e.or_insert(IDSet::new());
        set.insert(id);

        let e = tbl_odds.entry(id2);
        let mut set = e.or_insert(IDSet::new());
        set.insert(id);
    }
    panic!("no pair of IDs found")
}

pub(crate) fn run(args: &[String]) {
    let ops = parse(&args[0]);

    {
        let (n2,n3) = find_counts(&ops);
        println!("checksum is {} (from {}, {})", n2*n3, n2, n3);
    }

    {
        let (id1,id2,diff) = find_ids_close_by_1(&ops);
        println!("common seq: {} (from {} and {})", diff, id1.to_string(), id2.to_string());
    }
}

#[test]
fn test_dup() {
    fn has_dup(s: &str) -> bool { let c = counts(&ID::from_str(s)); c.has_dup() };
    assert!(has_dup("aba"));
    assert!(has_dup("abcdeef"));
    assert!(! has_dup("abcdef"));
}
