use opencv::{core, highgui, imgproc, prelude::*};
use rsautogui::screen::{self};
use image::{DynamicImage, GenericImageView};

fn main() {
    loop {
        // Capture a screenshot
        let screenshot = screen::screenshot(0, 0, 800, 800);
        // println!("{:?}", screenshot);

        // Convert the image to a Vec<u8> containing the pixel data
        let image_rgb8 = screenshot.to_rgb8();
        let (width, height) = image_rgb8.dimensions();
        let data = image_rgb8.into_raw();

        // Create an OpenCV Mat from the pixel data
        let mat = Mat::from_slice(&data).unwrap();
        let mat = mat.reshape(3, height as i32).unwrap();

        // Convert the Mat from RGB to BGR (because OpenCV expects BGR)
        let mut mat_bgr = Mat::default();
        imgproc::cvt_color(&mat, &mut mat_bgr, imgproc::COLOR_RGB2BGR, 0).unwrap();

        // Display the image using OpenCV's highgui
        highgui::imshow("Screenshot", &mat_bgr).unwrap();
        let key = highgui::wait_key(30).unwrap();

        if key == 27 {
            break;
        }
    }
}

