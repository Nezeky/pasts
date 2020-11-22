#![forbid(unsafe_code)]

use async_std::task;
use pasts::prelude::*;

use std::time::Duration;

fn main() {
    exec!(async {
        println!("Waiting 2 seconds…");
        task::sleep(Duration::new(2, 0)).await;
        println!("Waited 2 seconds.");
    });
}
