use std::sync::{Arc, Mutex};
use tokio::task;

pub async fn tokio_test() {
    let img = image::open("./test/majestic-mountain-peak-tranquil-winter-landscape-generated-by-ai.jpg").unwrap();
    let img = Arc::new(Mutex::new(img));

    let mut handles = vec![];

    for i in 0..10 {
        let img_clone = Arc::clone(&img);
        let t1 = task::spawn(async move {
            let resized_img = img_clone.lock().unwrap().resize(1920, 1080, image::imageops::FilterType::Triangle);
            let img_path = format!("./test/output/output_{}.avif", i);
            resized_img.save(img_path).unwrap();
        });
        handles.push(t1);
    }

    // Await all spawned tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
}
