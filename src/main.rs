use std::time::Instant;

use opencv::{core, highgui, imgproc, prelude::*};
use rsautogui::screen::{self};
use image::{DynamicImage, GenericImageView};

fn main() {

    let mut frame_cout = 0;
    let mut start = Instant::now();

    loop {
        let frame_start = Instant::now();

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
        // i want to wait less than 1ms
        highgui::wait_key(1).unwrap();

        // if key == 27 {
        //    break;
        // }

        frame_cout += 1;

        let frame_end = Instant::now();
        let frame_duration = frame_end.duration_since(frame_start);
        let frame_duration = frame_duration.as_secs_f64();
        println!("Frame duration: {:.2}ms", frame_duration * 1000.0);

        if start.elapsed().as_secs() >= 1 {
            println!("FPS: {}", frame_cout);
            frame_cout = 0;
            start = Instant::now();
        }
    }
}

