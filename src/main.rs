extern crate image;
extern crate img_hash;

fn main() {
    // wget "https://pbs.twimg.com/media/CT6cDD3UkAEnP8Y.jpg:large" -O ygg_jp.jpg
    // wget "https://pbs.twimg.com/media/CfuZgxLUkAArdGe.jpg:large" -O ygg_en.jpg
    let hash1 = hash(include_bytes!("../ygg_jp.jpg"), "ygg_jp");
    let hash2 = hash(include_bytes!("../ygg_en.jpg"), "ygg_en");

    assert_eq!(hash1.dist(&hash2), 0);
}

fn hash(bytes: &[u8], name: &str) -> img_hash::ImageHash {
    use image::GenericImage;

    let mut img = image::load_from_memory(bytes).expect(name);
    let (w, h) = img.dimensions();
    img.crop(0, 0, w, h * 3 / 4);

    img_hash::ImageHash::hash(&img, 8, img_hash::HashType::Gradient)
}
