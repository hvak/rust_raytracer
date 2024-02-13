use indicatif::ProgressBar;
mod vec3;
use vec3::Vec3;

fn write_color(pixel_color: Vec3) {
    let ir: usize = (255.999 * pixel_color.x) as usize;
    let ig: usize = (255.999 * pixel_color.y) as usize;
    let ib: usize = (255.999 * pixel_color.z) as usize;

    print!("{} {} {}\n", ir, ig, ib);
}

fn main() {
    // Image
    let image_width: usize = 256;
    let image_height: usize = 256;
    
    //progress bar
    let pb = ProgressBar::new(image_height as u64);
    
    // Render
    print!("P3\n{} {}\n255\n", image_width, image_height);
    for j in 0..image_height {
        pb.inc(1); 
        for i in 0..image_width {
            let col = Vec3::new((i as f64) / ((image_width - 1) as f64), 
                                (j as f64) / ((image_height - 1) as f64), 
                                0.0);
            write_color(col);
        }
    }
}
