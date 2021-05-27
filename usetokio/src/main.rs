use tokio::io;
use tokio::time;
use tokio::task;
use tokio::process::Command;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // for _ in 1..=10 {
    //     Command::new("echo").arg("Hello, world!").spawn()?.await?;
    // }
    let date_loop = task::spawn(date_loop());
    let copying = task::spawn(copying());

    date_loop.await??;
    copying.await??;

    Ok(())
}

// async fn date_loop() -> Result<(), std::io::Error> {
//     let mut interval = time::interval(Duration::from_secs(1));
//     loop {
//         interval.tick().await;
//         Command::new("date").spawn()?.await?;
//     }
// }

async fn date_loop() -> Result<(), std::io::Error> {
    loop {
        task::spawn_blocking(|| std::thread::sleep(Duration::from_secs(1))).await;
        Command::new("date").spawn()?.await?;
    }
}

async fn copying() -> Result<(), std::io::Error> {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    io::copy(&mut stdin, &mut stdout).await?;

    Ok(())
}