#![deny(warnings)]
use common::api;
use std::time::Instant;

#[tokio::main]
async fn main() {
    let now = Instant::now();
    let result = api::get_stories().await;
    dbg!(&result);
    dbg!(result.expect("must have a result"));
    println!(
        "Getting best stories took {:.2} ms",
        now.elapsed().as_millis()
    );
}
