use std::fs::OpenOptions;
use std::io::Write;

#[tokio::main]
pub async fn main() {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("output.txt")
        .expect("Unable to open file");

    println!("Ctrl+C to exit.");
    writeln!(file, "Started service.").expect("Unable to write to file");
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for event");
    println!("Received Ctrl+C. Exit.");
    writeln!(file, "Stopped service.").expect("Unable to write to file");
}
