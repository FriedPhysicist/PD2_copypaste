extern crate clipboard;
use raylib::prelude::*;
use std::io::Read;
use wl_clipboard_rs::{paste::{get_contents, ClipboardType, MimeType, Seat}};
use enigo::{Enigo, Keyboard, Settings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (mut rl, thread) = raylib::init()
        .size(115, 50)
        .title("PD2_copypaste")
        .build();

    let mut time : f32 = 0.0;
    let mut bwill_paste : bool = false;
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        if d.gui_button(Rectangle::new(0.0, 20.0, 115.0, 30.0), "Paste") {
            bwill_paste = true;
        }

        if bwill_paste {
            time = time + d.get_frame_time();
        }

        if time >= 2.0 {
            let result = get_contents(ClipboardType::Regular, Seat::Unspecified, MimeType::Text);

            match result {
                Ok((mut pipe, _)) => {
                    let mut contents = vec![];
                    pipe.read_to_end(&mut contents)?;
                    let mut enigo = Enigo::new(&Settings::default()).unwrap();
                    enigo.text(&String::from_utf8_lossy(&contents)).unwrap();
                    println!("Pasted: {}", String::from_utf8_lossy(&contents));
                }
                _ => println!("Possible error.")
            }

            bwill_paste = false;
            time = 0.0;
        }

        d.clear_background(Color::BLACK);
    }

    Ok(())
}
