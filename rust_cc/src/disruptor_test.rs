use disruptor::{Builder, BusySpin};
use image::{DynamicImage, ImageOutputFormat::Avif};
use std::fs::File;
use std::sync::{Arc, Mutex};

pub struct Calculation {
    img: Arc<Mutex<DynamicImage>>,
}

pub fn disruptor_fn() {
    let img = Arc::new(Mutex::new(
        image::open("./test/majestic-mountain-peak-tranquil-winter-landscape-generated-by-ai.jpg")
            .expect("Failed to open image"),
    ));

    let factory = || Calculation { img: img.clone() };
    let processor = move |event: &Calculation, sequence: i64, _end_of_batch: bool| {
        let resized_img =
            event
                .img
                .lock()
                .unwrap()
                .resize(1920, 1080, image::imageops::FilterType::Triangle);
        let output_path = format!("./test/output/output_{}.avif", sequence);
        let mut output_file = File::create(&output_path).expect("Failed to create output file");
        resized_img
            .write_to(&mut output_file, Avif)
            .expect("Failed to save resized image");
    };

    let size = 2048;
    let mut producer =
        Builder::new(size, factory, processor, BusySpin).create_with_multi_producer();

    for _ in 0..10 {
        producer.publish(|_event| {});
    }
}
