extern crate servo_media;
extern crate servo_media_auto;

use servo_media::ServoMedia;
use std::sync::Arc;
use std::{thread, time};
use objc2_app_kit::NSApplication;

fn run_example(servo_media: Arc<ServoMedia>) {
    if let Some(stream) = servo_media.create_videoinput_stream(Default::default()) {
        let mut output = servo_media.create_stream_output();
        output.add_stream(&stream);
        thread::sleep(time::Duration::from_millis(6000));
    } else {
        print!("No video input elements available");
    }
}

fn main() {
    #[cfg(target_os = "macos")]
    {
        use objc2::MainThreadMarker;
        use objc2_app_kit::NSApplication;
        let mtm = MainThreadMarker::new().unwrap();
        let ns = NSApplication::new(mtm);
        // leak the NSApplication to keep it alive for the duration of the program
        let _leaked_ns = Box::leak(Box::new(ns));
    }
    ServoMedia::init::<servo_media_auto::Backend>();
    let servo_media = ServoMedia::get();
    run_example(servo_media);
}
