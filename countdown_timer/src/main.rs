use std::{fs::File, io, io::BufReader, thread, time::Duration};
use rodio::{Decoder, OutputStream, Sink};

fn main() {
    println!("Enter countdown time in seconds:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    let seconds: u64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!(" Please enter a valid number.");
            return;
        }
    };

    println!("Starting timer for {} seconds...\n", seconds);

    for i in (1..=seconds).rev() {
        println!("Time left: {} seconds", i);
        thread::sleep(Duration::from_secs(1));
    }

    println!("\n Time's up!");
    // Play sound
    if let Ok((_stream, stream_handle)) = OutputStream::try_default() {
        if let Ok(sink) = Sink::try_new(&stream_handle) {
            let file = BufReader::new(File::open("alarm.wav").unwrap());
            let source = Decoder::new(file).unwrap();
            sink.append(source);
            sink.sleep_until_end(); // Wait until sound finishes
        }
    }
    
}
