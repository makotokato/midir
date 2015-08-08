extern crate midir;

use std::thread::sleep_ms;

use midir::{MidiApi, MidiInApi};
use midir::winmm::MidiInWinMM;

fn main() {
    let mut midi = MidiInWinMM::new("My Test", 100).unwrap();
    let count = midi.get_port_count();
    println!("Number of input devices: {}", count);
    for i in 0..count {
        println!("{}: {}", i, midi.get_port_name(i).unwrap());
    }
    println!("Opening port");
    midi.open_port(0, "RtMidi").unwrap();
    println!("Port open");
    
    let mut message = Vec::new();
    
    for i in 0..500 {
          // switch to using a callback (and back)
        if i == 150 {
            midi.set_callback(|stamp, message| {
                println!("{}: {:?} (from callback)", stamp, message);
            });
        } else if i == 350 {
            midi.cancel_callback();
        }
        let stamp = midi.get_message(&mut message);
        if (message.len() > 0) {
            println!("{}: {:?}", stamp, message);
        }
        // Sleep for 10 milliseconds ... platform-dependent.
        sleep_ms(10);
    }
    
    sleep_ms(1000);
    
    println!("Closing port");
    midi.close_port();
    println!("Port closed");
}