use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Mutex, MutexGuard},
    thread,
    time::Duration,
};

use lazy_static::lazy_static;
use log::debug;
use tokio::spawn;
use tokio::time::{interval, Sleep};

use crate::models::advertisement::Advertisement;

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct ShowStatistic {
    pub uid: i64,
    pub cid: i64,
    pub os: i8,
    pub browser: i8,
    pub country: i64,
    pub sub_acc: i64,
    pub operator: i64,
    pub adv_type: Advertisement,
    pub device: i8,
}

lazy_static! {
    static ref TOTAL_STATS: Mutex<HashMap<ShowStatistic, i32>> = Mutex::new(HashMap::new());
}

pub fn increment(stats: ShowStatistic) {
    debug!("Increment stats {:?}", stats);

    let mut map = TOTAL_STATS.lock().expect("Failed to lock Mutex");

    let mut counter = map.get(&stats).or_else(|| Some(&0)).unwrap();

    let counter = counter + 1;

    map.insert(stats, counter);

    debug!("Current stats is {:?}", map);
}

/// Runs stats flusher in separated thread.
///
/// # Panics
///
/// This function panics if `report_url` is empty or `duration` is less then 1 second.
pub fn run_interval_flusher(report_url: String, duration: Duration) {
    assert_ne!(report_url, "", "report_url must not be empty");

    assert!(
        duration >= Duration::from_secs(1),
        "Duration must be equal or greater then 1s"
    );

    debug!(
        "Flush stats to {:} in interval of {:?}",
        report_url, duration
    );

    spawn(async move {
        let mut interval = interval(duration);

        loop {
            debug!("Next iteration");
            interval.tick().await;
            debug!("Ticked");
            flush();
        }
    });
}

fn flush() {
    let mut map = TOTAL_STATS.lock().unwrap();

    if map.is_empty() {
        debug!("Nothing to flush");
        return ();
    }

    debug!("flush {:?}", map);
    thread::sleep(Duration::from_millis(300));
    debug!("flush done");

    map.clear();
}
