use mr_mono::MrMono;
use tokio::time::{sleep, Duration};
mod mr_mono;
#[tokio::main]
async fn main() {
    let mut mr = MrMono::new();

    spawn_speaker(mr.clone(), "Alice".to_string());
    sleep(Duration::from_millis(15)).await;

    spawn_speaker(mr.clone(), "Bob".to_string());
    sleep(Duration::from_millis(200)).await;

    spawn_speaker(mr.clone(), "Charlie".to_string());
    sleep(Duration::from_millis(55)).await;

    spawn_speaker(mr.clone(), "Diana".to_string());
    sleep(Duration::from_millis(20)).await;

    println!("> Stopping current task");
    mr.stop_current();

    // Sleep to make sure all closure resolve
    sleep(Duration::from_millis(100)).await;
}

pub fn spawn_speaker(mut mr: MrMono, message: String) {
    tokio::spawn(async move {
        println! {"> Starting {message}"}
        let result = mr.say(message.clone()).await;
        println! {"> {message} resolved in {:?}", result}
    });
}
