use std::sync::{Arc, Mutex};
use threadpool::ThreadPool;

pub fn multi_thread() {
    let img =
        image::open("./test/majestic-mountain-peak-tranquil-winter-landscape-generated-by-ai.jpg")
            .unwrap();
    let img = Arc::new(Mutex::new(img));

    let resize_pool = ThreadPool::new(10);
    let saving_pool = ThreadPool::new(10);

    for i in 0..10 {
        let img_clone = Arc::clone(&img);
        let saving_pool_clone = saving_pool.clone();
        resize_pool.execute(move || {
            let img = img_clone.lock().unwrap();
            let resized_img = img.resize(1920, 1080, image::imageops::FilterType::Triangle);
            let img_path = format!("./test/output/output_{}.avif", i);
            saving_pool_clone.execute(move || {
                resized_img.save(img_path).unwrap();
            });
        });
    }

    resize_pool.join();
    saving_pool.join();
}
