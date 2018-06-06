extern crate pomotool;

const PROGRESS_BAR_WIDTH: usize = 40;
const PROGRESS_TIME_SEC: usize = 3;

use std::time::{Instant};


#[test]
fn get_progressbar() {
    let pb = pomotool::get_progressbar(PROGRESS_BAR_WIDTH, PROGRESS_TIME_SEC);
    assert_eq!(pb.show_speed, false);
    assert_eq!(pb.show_counter, false);
    assert_eq!(pb.show_percent, false);
}

#[test]
fn run_progressbar() {
    let pb = pomotool::get_progressbar(PROGRESS_BAR_WIDTH, PROGRESS_TIME_SEC);
    let start = Instant::now();
    pomotool::run_progressbar(pb);
    assert_eq!(start.elapsed().as_secs(), PROGRESS_TIME_SEC as u64);
}
