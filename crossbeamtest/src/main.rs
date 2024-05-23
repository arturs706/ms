use crossbeam::channel;
use std::thread;
use std::time::Instant;

#[derive(Debug)]
#[allow(dead_code)]
struct ValueEvent {
    value: usize,
}

fn main() {
    let start_time: Instant = Instant::now();

    let (sender, receiver) = channel::bounded(64);

    let producer_thread = thread::spawn(move || {
        for event_count in 0..100000 {
            let value_event = ValueEvent { value: event_count };
            sender.send(value_event).unwrap();
        }
    });

    let consumer_thread = thread::spawn(move || {
        for _ in 0..100000 {
            receiver.recv().unwrap();
        }
    });

    producer_thread.join().unwrap();
    consumer_thread.join().unwrap();

    let end_time = Instant::now();
    let elapsed_time_in_microseconds = end_time.duration_since(start_time);

    println!("Total execution time: {:?}", elapsed_time_in_microseconds);
}
