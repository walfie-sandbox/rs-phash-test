use image::{DynamicImage, FilterType};
use stream_dct;

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
    // Resize to 32x32 and get the grayscale values into a Vec of length 32*32
    let pixels = img.resize_exact(SIZE as u32, SIZE as u32, FilterType::Nearest)
        .to_luma()
        .pixels()
        .map(|p| p.data[0] as f64)
        .collect::<Vec<f64>>();

    assert_eq!(pixels.len(), 32 * 32);

    // Compute the discrete cosine transform, returning a Vec of length 32*32
    let dct = stream_dct::dct_2d(&pixels, SIZE);

    for row in dct.chunks(SIZE) {
        for v in row {
            print!("{}, ", *v as i64);
        }
        println!("");
    }

    assert_eq!(dct.len(), 32 * 32);

    // Take the top left 8x8 section of the matrix
    let dct_slice = dct.chunks(SIZE)
        .take(SMALL_SIZE)
        .flat_map(|row| &row[0..SMALL_SIZE])
        .cloned()
        .collect::<Vec<f64>>();

    /*
    let dct_slice = (0..SMALL_SIZE)
        .flat_map(|i| &dct[SIZE * i..SIZE * i + SMALL_SIZE])
        .cloned()
        .collect::<Vec<f64>>();
        */

    assert_eq!(dct_slice.len(), 8 * 8);

    println!("first: {}", dct_slice[0]);

    // Compute the average, skipping the top-leftmost element of the matrix
    let total = dct_slice.iter().skip(1).sum::<f64>();
    let average = total / (SMALL_SIZE * SMALL_SIZE - 1) as f64;

    println!("total = {}\naverage = {}", total, average);

    // Compute the hash based on whether each element in the matrix is
    // greater than or less than the average
    let hash = dct_slice.into_iter().enumerate().skip(1).fold(
        0,
        |acc, (i, v)| {
            if v > average { acc | (1 << i) } else { acc }
        },
    );

    hash
}


//================================================================================
fn get_hash_old(img: &DynamicImage) -> i64 {
    let gray = img.resize_exact(SIZE as u32, SIZE as u32, FilterType::Nearest)
        .to_luma();

    let mut vals = [[0.0; SIZE]; SIZE];
    for (x, y, p) in gray.enumerate_pixels() {
        //vals[x as usize][y as usize] = p.data[0] as f64;
        vals[y as usize][x as usize] = p.data[0] as f64;
    }

    let dct_vals = apply_dct(&vals);

    // TODO
    for i in 0..SIZE {
        for j in 0..SIZE {
            print!("{}, ", dct_vals[i][j] as i64)
        }
        println!("");
    }

    let mut total: f64 = dct_vals.iter().take(SMALL_SIZE).fold(0.0, |acc, &arr| {
        acc + arr.iter().take(SMALL_SIZE).sum::<f64>()
    });

    println!("first: {}", dct_vals[0][0]);
    total -= dct_vals[0][0];

    let average = total / (SMALL_SIZE * SMALL_SIZE - 1) as f64;

    println!("total = {}\naverage = {}", total, average);


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

//================================================================================

#[cfg(test)]
mod test {
    use super::*;
    use error::*;
    use image::{self, GenericImage};

    #[test]
    fn medusa() {
        // This test depends on the following images existing in the root dir
        //
        // ```
        // curl "https://pbs.twimg.com/media/CT6ccesVEAAy_Kx.jpg:large" >medusa_jp.jpg
        // curl "https://pbs.twimg.com/media/CfqXEh_UsAEb9dw.jpg:large" >medusa_en.jpg
        // ```
        let en = include_bytes!("../nezha_en.jpg");
        let jp = include_bytes!("../medusa_jp.jpg");

        fn load_and_crop(bytes: &[u8]) -> Result<DynamicImage> {
            let mut img = image::load_from_memory(bytes).chain_err(
                || "failed to load image",
            )?;

            let (w, h) = img.dimensions();
            img = img.crop(0, 0, w, h * 3 / 4);
            Ok(img)
        }

        let h1a = get_hash(&load_and_crop(en).unwrap());
        println!("\n\n");
        let h1b = get_hash_old(&load_and_crop(en).unwrap());
        println!("======");

        let h2a = get_hash(&load_and_crop(jp).unwrap());
        println!("\n\n");
        let h2b = get_hash_old(&load_and_crop(jp).unwrap());
        println!("======");

        println!("medusa\nnew: {:b}\nold: {:b}", h1a, h1b);

        println!("nezha\nnew: {:b}\nold: {:b}", h2a, h2b);

        println!("old not eq");
        assert!(h1b != h2b);

        assert_eq!(h1a, h1b);
        assert_eq!(h2a, h2b);


        //assert!(h1 != h2);
    }
}
