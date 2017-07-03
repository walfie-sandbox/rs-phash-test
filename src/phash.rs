use image::{DynamicImage, FilterType};
use std::f64::consts::PI;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImageHash(i64);

impl ImageHash {
    pub fn new(img: &DynamicImage) -> Self {
        ImageHash(get_hash(img))
    }
}

fn get_hash(img: &DynamicImage) -> i64 {
    let gray = img.resize_exact(SIZE as u32, SIZE as u32, FilterType::Nearest)
        .to_luma();

    let mut vals = [[0.0; SIZE]; SIZE];
    for (x, y, p) in gray.enumerate_pixels() {
        vals[x as usize][y as usize] = p.data[0] as f64;
    }

    let dct_vals = apply_dct(vals);

    let mut total: f64 = dct_vals.iter().take(SMALL_SIZE).fold(0.0, |acc, &arr| {
        acc + arr.iter().take(SMALL_SIZE).sum::<f64>()
    });

    total -= dct_vals[0][0];

    let average = total / (SMALL_SIZE * SMALL_SIZE - 1) as f64;

    let mut hash: i64 = 0;

    for i in 0..(SMALL_SIZE) {
        for j in 0..(SMALL_SIZE) {
            if dct_vals[i][j] > average {
                hash |= 1 << (i * SMALL_SIZE + j);
            }
        }
    }

    hash |= 1;

    hash
}

fn apply_dct(f: [[f64; SIZE]; SIZE]) -> [[f64; SIZE]; SIZE] {
    let mut out = [[0.0; SIZE]; SIZE];

    for u in 0..SIZE {
        let u_index = u;
        let u = u as f64;

        for v in 0..SIZE {
            let v_index = v;
            let v = v as f64;

            let mut sum = 0.0;

            for i in 0..SIZE {
                for j in 0..SIZE {
                    sum += ((2 * i + 1) as f64 * u * PI / (2.0 * SIZE_F64)).cos() *
                        ((2 * j + 1) as f64 * v * PI / (2.0 * SIZE_F64)).cos() *
                        f[i][j];
                }
            }

            sum *= COEFFICIENTS[u_index] * COEFFICIENTS[v_index] / 4.0;
            out[u_index][v_index] = sum;
        }
    }

    out
}

const SIZE: usize = 32;
const SIZE_F64: f64 = SIZE as f64;
const SMALL_SIZE: usize = 8;
const COEFFICIENTS: [f64; SIZE] = [
    0.707106781186547524400844362104849039284835937688474036588,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
    1.0,
];

#[cfg(test)]
mod test {
    use super::*;
    use error::*;
    use image::{self, GenericImage};

    #[test]
    fn ygg() {
        // This test depends on the following images existing in the root dir
        //
        // ```
        // curl "https://pbs.twimg.com/media/CfuZgxLUkAArdGe.jpg:large" >ygg_en.jpg
        // curl "https://pbs.twimg.com/media/CT6cDD3UkAEnP8Y.jpg:large" >ygg_jp.jpg
        // ```
        let en = include_bytes!("../ygg_en.jpg");
        let jp = include_bytes!("../ygg_jp.jpg");

        fn load_and_crop(bytes: &[u8]) -> Result<DynamicImage> {
            let mut img = image::load_from_memory(bytes).chain_err(
                || "failed to load image",
            )?;

            let (w, h) = img.dimensions();
            img = img.crop(0, 0, w, h * 3 / 4);
            Ok(img)
        }

        let h1 = ImageHash::new(load_and_crop(en).unwrap());
        let h2 = ImageHash::new(load_and_crop(jp).unwrap());

        assert_eq!(h1, h2);
    }
}
