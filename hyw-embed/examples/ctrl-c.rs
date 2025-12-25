// Example of handling Ctrl-C signal using compio, taken from https://github.com/compio-rs/compio-website/blob/2e302230723a6b1dea6f95989b8cf6b2e752662e/content/docs/compio/signal.md?plain=1#L6-L20

use futures_util::{FutureExt, select};
use compio::{time::interval, signal::ctrl_c};
use std::{pin::pin, time::Duration};

#[compio::main]
async fn main() {
    let mut interval = interval(Duration::from_secs(1));
    loop {
        let ctrlc = pin!(ctrl_c());
        select! {
            res = ctrlc.fuse() => {
                res.unwrap();
                println!("break");
                break;
            }
            _ = interval.tick().fuse() => println!("ping"),
        }
    }
}
