use reqwest::Error;
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let url = "https://jsonplaceholder.typicode.com/posts/1";
    let starttime = Instant::now();

    let _ = reqwest::get(url).await?;

    let elapsed_time = starttime.elapsed();
    println!("Time elapsed: {:?}", elapsed_time);

    Ok(())
}
