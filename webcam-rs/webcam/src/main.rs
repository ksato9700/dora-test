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

    let camera_index_envvar = std::env::var("CAMERA_INDEX");
    println!("CAMERA_INDEX env var: {:?}", camera_index_envvar);

    let camera_index_envvar = std::env::var("USER");
    println!("USER env var: {:?}", camera_index_envvar);

    // get camera index from env, parsed as integer
    let camera_index: i32 = std::env::var("CAMERA_INDEX")
        .unwrap_or_else(|_| "0".to_string())
        .parse()
        .expect("CAMERA_INDEX must be an integer");

    println!("Opening camera with index {}", camera_index);
    let mut video_capture = VideoCapture::new(camera_index, opencv::videoio::CAP_ANY)?;
    video_capture.set(opencv::videoio::CAP_PROP_FRAME_WIDTH, CAMERA_WIDTH)?;
    video_capture.set(opencv::videoio::CAP_PROP_FRAME_HEIGHT, CAMERA_HEIGHT)?;

    let mut frame = Mat::default();
    // let params: Vector<i32> = vec![opencv::imgcodecs::IMWRITE_JPEG_QUALITY, 95].into();

    // wait for the camera to be ready
    std::thread::sleep(std::time::Duration::from_millis(500));

    let (mut node, mut events) = DoraNode::init_from_env()?;

    while let Some(event) = events.recv() {
        if let Event::Input {
            id: _,
            metadata,
            data: _,
        } = event
        {
            let result = video_capture.read(&mut frame)?;
            if result {
                println!("Frame {:?}", frame);
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
    }

    Ok(())
}
