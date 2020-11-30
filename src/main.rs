async fn hello() -> String {
    "hello, async fn".to_string()
}

#[tokio::main]
async fn main() {
    let greeting = hello().await;
    println!("{}", greeting);
}
