extern crate clipboard;
use rdev::{listen, simulate, Key, EventType};
use enigo::{
    Enigo, Keyboard, Settings
};
use std::{thread, time};

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

fn main() {
    let mut v : String = String::from("");
    listen(move |event| {
        match event.name {
            Some(_) => {
                v.push_str(event.clone().name.unwrap().as_str());

                let len : usize = v.len();
                let key : &str = "!@#";

                if len == key.len() {
                    if v == key{
                        let mut clipboard: ClipboardContext = ClipboardProvider::new().unwrap();
                        let mut enigo = Enigo::new(&Settings::default()).unwrap();
                        let clipboard_text : String = clipboard.get_contents().unwrap().clone();
                        
                        if !clipboard_text.starts_with("/w *") {
                            return;
                        }

                        thread::spawn(move || {
                            for _ in 0..key.len() {
                                thread::sleep(time::Duration::from_millis(50));
                                simulate(&EventType::KeyPress(Key::Backspace)).unwrap();
                            }
                            thread::sleep(time::Duration::from_millis(500));
                            enigo.text(&clipboard_text).unwrap();

                            println!("Pasted! {:?}", &clipboard_text);
                        }
                        );
                    }
                }

                if len >= key.len() {
                    v.remove(0);
                }
            },
            None => (),
        }
    }).unwrap();
}
