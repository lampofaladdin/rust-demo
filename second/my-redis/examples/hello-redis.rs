use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("localhost:3333").await?;
    let b = client.set("hello", "world".into()).await?;
    println!("{:?}", b);
    let result = client.get("hello").await;
    println!("从服务器端获取到结果={:?}", result);
    Ok(())
}
