use crossbeam_channel::bounded;
use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub fn crossbeam_resize_image() {
    let (snd, rcv) = bounded(2);
    let img =
        image::open("./test/majestic-mountain-peak-tranquil-winter-landscape-generated-by-ai.jpg")
            .unwrap();

    for i in 0..2 {
        let img_clone: Arc<Mutex<image::DynamicImage>> = Arc::new(Mutex::new(img.clone()));
        let snd_clone = snd.clone();
        let rcv_clone = rcv.clone();
        tokio::spawn(async move {
            let resized_img = img_clone.lock().unwrap().resize(
                1920,
                1080,
                image::imageops::FilterType::Triangle,
            );
            snd_clone.send(resized_img).unwrap();
        });
        tokio::spawn(async move {
            let rcv_clone = rcv_clone.recv().unwrap();
            let img_path = format!("./test/output/output_{}.avif", i);
            let received_img_clone = rcv_clone.clone();
            received_img_clone.save(img_path).unwrap();
        });
    }
}
