use tokio::time::Instant;
use tokio::time::Duration;
use tokio::time::interval as time_interval;

async fn hello() -> String {
    "hello, async fn".to_string()
}


#[tokio::main]
async fn main() {
    let start_at = Instant::now();
    let mut interval = time_interval(Duration::from_millis(100));

    for _ in 0..5 {
        interval.tick().await;
        println!("{:04} [msec]", start_at.elapsed().as_millis());
        let greeting = hello().await;
        println!("{}", greeting);
    }
}
