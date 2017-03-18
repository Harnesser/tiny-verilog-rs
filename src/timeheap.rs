///
/// Min-heap pointing to the next procedure to run at the closest time
///

use std::cmp::Ordering;
use std::collections::BinaryHeap;

use procedure::{Time, ProcId};

#[derive(Eq, PartialEq)]
struct Entry {
    time: Time,
    proc_id: ProcId,
}

impl Ord for Entry {
    fn cmp(&self, other: &Entry) -> Ordering {
        // note we flip the ordering here
        other.time.cmp(&self.time)
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


pub struct TimeHeap {
    heap: BinaryHeap<Entry>,
}

impl TimeHeap {

    pub fn new() -> TimeHeap {
        TimeHeap{
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, proc_id: ProcId, time: Time) {
        self.heap.push( Entry {time: time, proc_id: proc_id } );
    }


    // activate all procedures which share the next lowest time
    pub fn activate(&mut self) -> (Option<Time>, Vec<ProcId>) {
        let mut time: Option<Time> = None;
        let mut proc_ids: Vec<ProcId> = vec![];

        // pop the first entry
        if let Some(entry) = self.heap.pop() { 
            time = Some(entry.time);
            proc_ids.push(entry.proc_id);
        }

        // now get any others that activate at the same time
        while let Some(entry) = self.heap.pop() {
            if entry.time == time.unwrap() {
                proc_ids.push(entry.proc_id);
            } else {
                self.heap.push(entry);
                break;
            }
        }

        (time, proc_ids)
    }

}

