extern crate image;
extern crate img_hash;
 
use img_hash::{HasherConfig};
use std::env;

fn main() {
      
    let args: Vec<String> = env::args().collect();
    let pciture_1_path = args.get(1).unwrap();
    let picture_2_path = args.get(2).unwrap();

    let image1 = image::open(pciture_1_path).unwrap();
    let image2 = image::open(picture_2_path).unwrap();
     
    let hasher = HasherConfig::new().to_hasher();

    let hash1 = hasher.hash_image(&image1);
    let hash2 = hasher.hash_image(&image2);

    if hash1.dist(&hash2) == 0 {
        println!("Pictures are the same");
    } else {
        println!("Pictures are different");
    }
}