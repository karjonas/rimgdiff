extern crate image;

use image::GenericImageView;

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() != 4 {
        println!("Incorrect arguments provided, example:");
        println!("rimgdiff img0.png img1.png output.png");
        return;
    }

    let file0 = args[1].clone();
    let file1 = args[2].clone();
    let out_file = args[3].clone();

    let img = image::open(&std::path::Path::new(&file0)).unwrap();
    let img1 = image::open(&std::path::Path::new(&file1)).unwrap();

    let dim0 = img.dimensions();
    let dim1 = img1.dimensions();

    if dim0 != dim1 {
        println!("Image dimensions differ {:?} vs {:?}", dim0, dim1);
        return ();
    }

    let max_w = dim0.0;
    let max_h = dim0.1;

    let mut img_diff: image::RgbaImage = image::ImageBuffer::new(max_w, max_h);

    let same_color: [u8; 4] = [255, 255, 255, 255];
    let diff_color: [u8; 4] = [255, 0, 0, 255];

    for h in 0..max_h {
        for w in 0..max_w {
            let pixel0 = img.get_pixel(w, h);
            let pixel1 = img1.get_pixel(w, h);

            let color = if pixel0 == pixel1 {
                same_color
            } else {
                diff_color
            };
            img_diff.put_pixel(w, h, image::Rgba::<u8>(color));
        }
    }

    let _ = img_diff.save(&out_file).unwrap();
}
