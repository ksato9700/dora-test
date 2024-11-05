use dora_node_api::{dora_core::config::DataId, DoraNode, Event, IntoArrow};
use std::error::Error;

use opencv::core::Mat;
use opencv::videoio::VideoCapture;
// use opencv::imgcodecs::imwrite;
// use opencv::highgui::imshow;

use opencv::prelude::*;

const CAMERA_WIDTH: f64 = 640.0;
const CAMERA_HEIGHT: f64 = 480.0;

fn main() -> Result<(), Box<dyn Error>> {
    let mut video_capture = VideoCapture::new(1, opencv::videoio::CAP_ANY)?;
    video_capture.set(opencv::videoio::CAP_PROP_FRAME_WIDTH, CAMERA_WIDTH)?;
    video_capture.set(opencv::videoio::CAP_PROP_FRAME_HEIGHT, CAMERA_HEIGHT)?;

    let mut frame = Mat::default();
    // let params: Vector<i32> = vec![opencv::imgcodecs::IMWRITE_JPEG_QUALITY, 95].into();

    // wait for the camera to be ready
    std::thread::sleep(std::time::Duration::from_millis(500));

    let (mut node, mut events) = DoraNode::init_from_env()?;

    while let Some(event) = events.recv() {
        match event {
            Event::Input {
                id: _,
                metadata,
                data: _,
            } => {
                let result = video_capture.read(&mut frame)?;
                println!("Frame read: {}", result);
                if result {
                    // let mut buffer = Vector::<u8>::new();
                    // imencode(".jpg", &frame, &mut buffer, &params)?;
                    // let array:Vec<u8> = buffer.as_ref().into();
                    let array: Vec<u8> = frame.data_bytes()?.to_vec();
                    node.send_output(
                        DataId::from("image".to_owned()),
                        metadata.parameters,
                        array.into_arrow(),
                    )?;
                } else {
                    println!("Error reading frame");
                }
            }
            _ => {}
        }
    }

    Ok(())
}
