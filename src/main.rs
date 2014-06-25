extern crate image;

use std::os;
use std::io::File;

use image::Image;
use image::{PNG, JPEG, PPM};

fn main() {
	let file = if os::args().len() == 2 {
		os::args().as_slice()[1].clone()
	} else {
		fail!("Please enter a file")
	};

	let im = Image::open(&Path::new(file.clone())).unwrap();

	println!("dimensions {}", im.dimensions());
	println!("{}", im.colortype());
	println!("{} bytes", im.raw_pixels().len());

	let t = im.clone();
	spawn(proc() {
		let fout = File::create(&Path::new(format!("{}.jpg", os::args().as_slice()[1]))).unwrap();
		let _    = t.save(fout, JPEG);
	});

	let t = im.clone();
	spawn(proc() {
		let fout = File::create(&Path::new(format!("{}.ppm", os::args().as_slice()[1]))).unwrap();
		let _    = t.save(fout, PPM);
	});

	let t = im.clone();
	spawn(proc() {
		let fout = File::create(&Path::new(format!("{}.png", os::args().as_slice()[1]))).unwrap();
		let g = t.resize_exact(1200, 1200, image::Nearest);
		let _    = g.save(fout, PNG);
	});

	let t = im.clone();
	spawn(proc() {
		let fout = File::create(&Path::new(format!("{}3.png", os::args().as_slice()[1]))).unwrap();

		let g = t.rotate90();

		let _    = g.save(fout, PNG);
	});
}