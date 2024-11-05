use dora_node_api::{dora_core::config::DataId, DoraNode, Event, IntoArrow};
use std::error::Error;

use opencv::core::Vector;
use opencv::videoio::VideoCapture;
use opencv::imgcodecs::imwrite;
use opencv::highgui::imshow;
use opencv::prelude::*;


const CAMERA_WIDTH: f64 = 640.0;
const CAMERA_HEIGHT: f64 = 480.0;

fn main() -> Result<(), Box<dyn Error>> {
    let mut video_capture = VideoCapture::new(0, opencv::videoio::CAP_ANY)?;
    video_capture.set(opencv::videoio::CAP_PROP_FRAME_WIDTH, CAMERA_WIDTH)?;
    video_capture.set(opencv::videoio::CAP_PROP_FRAME_HEIGHT, CAMERA_HEIGHT)?;
    // video_capture.set(opencv::videoio::CAP_PROP_BRIGHTNESS, 0.9)?;
    // video_capture.set(opencv::videoio::CAP_PROP_CONTRAST, 0.9)?;
    let mut image = opencv::core::Mat::default();
    let result = video_capture.read(&mut image)?;

    println!("Result: {:?}", result);
    println!("Image: {:?}", image);

    let params: Vector<i32> = vec![opencv::imgcodecs::IMWRITE_JPEG_QUALITY, 95].into();
    imwrite("image.jpg", &image, &params)?;
    // let result = imshow("image", &image);
    // println!("Result: {:?}", result);

    // sleep for 5 seconds
    // std::thread::sleep(std::time::Duration::from_secs(5));

    // let (mut node, mut events) = DoraNode::init_from_env()?;

    // while let Some(event) = events.recv() {
    //     match event {
    //         Event::Input {
    //             id,
    //             metadata,
    //             data: _,
    //         } => {
    //             match video_capture.read(&mut opencv::core::Mat::default())? {
    //                 Ok(frame) => {
    //                     let mut buffer = Vec::new();
    //                     opencv::imgcodecs::imencode(".jpg", &frame, &mut buffer, &opencv::types::VectorOfint::new())?;
    //                     node.send_output(
    //                         DataId::from("image".to_owned()),
    //                         metadata.parameters,
    //                         buffer.into_arrow(),
    //                     )?;
    //                 }
    //                 Err(e) => {
    //                     eprintln!("Error reading frame: {}", e);
    //                 }

    //         }
    //     }
    // }

    Ok(())
}
