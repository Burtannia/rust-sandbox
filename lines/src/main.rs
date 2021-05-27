use tokio::prelude::*;
use tokio::io::AsyncBufReadExt;
use tokio::fs::File;
use futures::future::try_join_all;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let mut args = std::env::args();
    args.next();

    let count: u32 =
        try_join_all(args.map(|path| tokio::spawn(count_lines(path))))
        .await?
        .into_iter()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .fold(0, |acc, x| acc + x);

    println!("Lines in files: {}", count);
    Ok(())
}

async fn count_lines(path: String) -> Result<u32, std::io::Error> {
    let mut count = 0u32;
    let file = File::open(path).await?;
    let file = io::BufReader::new(file);
    let mut lines = file.lines();
    while let Some(_) = lines.next_line().await? {
        count += 1;
    }
    Ok(count)
}