use crate::template::run_single::run_single;
use crate::template::Day;

pub fn handle(day: Day) {
    let _timing = run_single(day, true, true).unwrap();
}