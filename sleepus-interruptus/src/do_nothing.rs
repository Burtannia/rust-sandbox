use async_std::task::{sleep, spawn};
use std::time::Duration;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;

struct DoNothing;

// fn sleepus() -> impl Future<Output=()> {
//     for i in 1..=10 {
//         println!("Sleepus {}", i);
//         sleep(Duration::from_millis(500));
//     }
//     DoNothing
// }

async fn sleepus() {
    DoNothing.await
}

impl Future for DoNothing {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _ctx: &mut Context) -> Poll<Self::Output> {
        Poll::Ready(())
    }
}

async fn interruptus() {
    for i in 1..=5 {
        println!("Interruptus {}", i);
        sleep(Duration::from_millis(1000)).await;
    }
}

#[async_std::main]
pub async fn main() {
    let sleepus = spawn(sleepus());
    interruptus().await;

    sleepus.await;
}