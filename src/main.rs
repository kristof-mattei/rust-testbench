use std::sync::LazyLock;
use std::time::Duration;

use tokio::time::Instant;

static START: LazyLock<Instant> = LazyLock::new(Instant::now);

fn dump(args: impl std::fmt::Display) {
    println!("{} {}", START.elapsed().as_secs(), args);
}

static VALUE: u32 = 5;

fn main() {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .build()
        .unwrap();

    let _r = runtime.block_on(async { tokio::task::spawn(spawn()).await });
}

async fn spawn() {
    let (sender, mut receiver) = tokio::sync::watch::channel(VALUE);

    let left = tokio::task::spawn(async move {
        while let Ok(()) = receiver.changed().await {
            dump("Changed");
        }

        dump("The end");
    });

    let right = tokio::task::spawn(async move {
        for _ in 0..10 {
            dump("Woke up, sending");
            let _r = sender.send(VALUE);
            dump("Sent");

            dump("Sleeping");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    let _r = tokio::join!(left, right);
}
