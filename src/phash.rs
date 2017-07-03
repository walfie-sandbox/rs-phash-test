use image::{DynamicImage, FilterType};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImageHash(i64);

const SIZE: usize = 32;
const SMALL_SIZE: usize = 8;

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

    let dct_vals = apply_dct(&vals);

    let dct_slice = dct_vals
        .iter()
        .take(SMALL_SIZE)
        .flat_map(|arr| &arr[0..SMALL_SIZE])
        .cloned()
        .collect::<Vec<f64>>();

    let total: f64 = dct_slice.iter().skip(1).sum();

    let average = total / (SMALL_SIZE * SMALL_SIZE - 1) as f64;

    let hash = dct_slice.into_iter().enumerate().skip(1).fold(
        0,
        |acc, (i, v)| {
            if v > average { acc | (1 << i) } else { acc }
        },
    );

    hash
}

fn apply_dct(f: &[[f64; SIZE]; SIZE]) -> [[f64; SIZE]; SIZE] {
    use std::f64::consts::{FRAC_1_SQRT_2, PI};

    let mut out = [[0.0; SIZE]; SIZE];

    for u in 0..SIZE {
        for v in 0..SIZE {
            let mut sum = 0.0;

            for i in 0..SIZE {
                for j in 0..SIZE {
                    sum += f[i][j] *
                        (PI * u as f64 * (2 * i + 1) as f64 / (2.0 * SIZE as f64)).cos() *
                        (PI * v as f64 * (2 * j + 1) as f64 / (2.0 * SIZE as f64)).cos();
                }
            }

            if u == 0 {
                sum *= FRAC_1_SQRT_2
            }

            if v == 0 {
                sum *= FRAC_1_SQRT_2
            }

            out[u][v] = sum * 0.25;
        }
    }

    out
}

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

        let h1 = ImageHash::new(&load_and_crop(en).unwrap());
        let h2 = ImageHash::new(&load_and_crop(jp).unwrap());

        assert_eq!(h1, h2);
    }
}
