#![allow(clippy::needless_borrows_for_generic_args, dead_code)]
#![allow(unused_imports)]
mod multi_thread;

use multi_thread::multi_thread;
use single_thread::single_thread;
mod disruptor_test;
mod single_thread;
use disruptor_test::disruptor_fn;
use std::time::Instant;
mod crossbeam_test;
use crossbeam_test::crossbeam_resize_image;
use csv::WriterBuilder;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
mod tokio_test;
use tokio_test::tokio_test;



#[tokio::main]

async fn main() {
    let start_time = Instant::now();
    // disruptor_fn();
    // tokio_test().await;
    // multi_thread();
    // single_thread();
    crossbeam_resize_image();
    println!(
        "Time taken to process {:?}",
        start_time.elapsed().as_secs_f64()
    );
    if let Err(err) = delete_all_images("./test/output") {
        eprintln!("Failed to delete images: {}", err);
    }

}

fn delete_all_images(folder_path: &str) -> Result<(), std::io::Error> {
    for entry in fs::read_dir(folder_path)? {
        let entry = entry?;
        let path = entry.path();
        if let Some(extension) = path.extension() {
            if extension == "avif" {
                fs::remove_file(path)?;
            }
        }
    }
    Ok(())
}
