use std::fs::OpenOptions;
use std::io::Write;

#[tokio::main]
pub async fn main() {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("output.txt")
        .expect("Unable to open file");

    writeln!(file, "Hello, file!").expect("Unable to write to file");
    println!("Ctrl+C to exit.");
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for event");
    println!("Received Ctrl+C. Exit.");
    writeln!(file, "Goodbye, file!").expect("Unable to write to file");
}
