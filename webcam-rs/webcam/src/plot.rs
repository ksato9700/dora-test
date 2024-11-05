use dora_node_api::{DoraNode, Event};
use std::error::Error;

use opencv::core::{Mat, Size, CV_8UC3};
use opencv::highgui::{imshow, wait_key};
// use opencv::imgcodecs::{imwrite, IMREAD_COLOR};
use opencv::prelude::*;

const CAMERA_WIDTH: i32 = 640;
const CAMERA_HEIGHT: i32 = 480;

fn main() -> Result<(), Box<dyn Error>> {
    let (mut _node, mut events) = DoraNode::init_from_env()?;

    // let params: Vector<i32> = vec![opencv::imgcodecs::IMWRITE_JPEG_QUALITY, 95].into();

    while let Some(event) = events.recv() {
        if let Event::Input {
            id,
            metadata: _,
            data,
        } = event
        {
            match id.as_str() {
                "image" => {
                    // let data = data.clone();
                    let data = data.clone().to_data();
                    let data: &[u8] = data.buffer(0);

                    let frame = unsafe {
                        let mut frame = Mat::new_size(
                            Size {
                                width: CAMERA_WIDTH,
                                height: CAMERA_HEIGHT,
                            },
                            CV_8UC3,
                        )?;
                        frame.set_data(data.as_ptr());
                        frame
                    };

                    println!("Frame {:?}", frame);
                    imshow("frame", &frame)?;
                    wait_key(1)?;
                    // imwrite("frame.jpg", &frame, &params)?;
                }
                other => println!("Unknown data id: {}", other),
            }
        }
    }

    Ok(())
}
