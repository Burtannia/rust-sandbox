use async_std::task::{sleep, spawn};
use std::time::Duration;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;

struct SleepPrint<Fut> {
    sleep: Fut
}

impl<Fut: Future<Output=()>> Future for SleepPrint<Fut> {
    type Output = ();

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let sleep: Pin<&mut Fut> = unsafe { self.map_unchecked_mut(|s| &mut s.sleep) };

        match sleep.poll(ctx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(()) => {
                println!("Inside SleepPrint");
                Poll::Ready(())
            }
        }
    }
}

fn sleepus() -> impl Future<Output=()> {
    SleepPrint {
        sleep: sleep(Duration::from_millis(3000))
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