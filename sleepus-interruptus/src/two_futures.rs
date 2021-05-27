use async_std::task::{sleep, spawn};
use std::time::Duration;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::future::Future;
use pin_project_lite::pin_project;

pin_project! {
    struct TwoFutures<Fut1, Fut2> {
        first_done: bool,
        #[pin]
        first: Fut1,
        #[pin]
        second: Fut2
    }
}

impl<Fut1: Future, Fut2: Future> Future for TwoFutures<Fut1, Fut2> {
    type Output = Fut2::Output;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context) -> Poll<Self::Output> {
        let this = self.project();

        if !*this.first_done {
            if let Poll::Ready(_) = this.first.poll(ctx) {
                *this.first_done = true;
            }
        }
        
        if *this.first_done {
            this.second.poll(ctx)
        } else {
            Poll::Pending
        }
    }
}

// fn sleepus() -> impl Future<Output=()> {
//     TwoFutures {
//         first_done: false,
//         first: sleep(Duration::from_millis(3000)),
//         second: async { println!("Hello TwoFutures") }
//     }
// }

// Exercise 11
async fn sleepus() {
    println!("Sleepus 1");
    sleep(Duration::from_millis(500)).await;
    println!("Sleepus 2");
    sleep(Duration::from_millis(500)).await;
    println!("Sleepus 3");
    sleep(Duration::from_millis(500)).await;
    println!("Sleepus 4");
}

async fn interruptus() {
    for i in 1..=5 {
        println!("Interruptus {}", i);
        sleep(Duration::from_millis(1000)).await;
    }
}

// #[async_std::main]
// pub async fn main() {
//     let sleepus = spawn(sleepus());
//     interruptus().await;

//     sleepus.await;
// }

// Exercise 12
pub fn main() {
    async_std::task::block_on(async {
        let sleepus = spawn(sleepus());
        interruptus().await;

        sleepus.await;
    })
    // let sleepus = spawn(sleepus());
    // async_std::task::block_on(interruptus());

    // async_std::task::block_on(sleepus);
}