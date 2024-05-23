pub fn single_thread() {
    let img =
        image::open("./test/majestic-mountain-peak-tranquil-winter-landscape-generated-by-ai.jpg")
            .unwrap();
    for i in 0..10 {
        let resized_img = img.resize(1920, 1080, image::imageops::FilterType::Triangle);
        resized_img
            .save(format!("./test/output/output_{}.avif", i))
            .unwrap();
    }
}
