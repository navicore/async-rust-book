use reqwest::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let starttime = Instant::now();

    let _ = reqwest::get(url).await?;

    let first = reqwest::get(url);
    let second = reqwest::get(url);
    let thrird = reqwest::get(url);
    let fourth = reqwest::get(url);
    let _first = first.await?;
    let _second = second.await?;
    let _third = thrird.await?;
    let _fourth = fourth.await?;

    let elapsed_time = starttime.elapsed();
    println!("Time elapsed: {:?}", elapsed_time);

    Ok(())
}
