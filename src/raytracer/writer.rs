use std::fs::File;
use std::io::prelude::*;

use super::frame::Frame;

fn clamp_to_byte(v: f32) -> u8 {
    (v * 255.0).round().max(0.0).min(255.0) as u8
}

#[test]
fn test_clamp_to_byte() {
    assert_eq!(clamp_to_byte(0.0), 0);
    assert_eq!(clamp_to_byte(1.0), 255);
    
    assert_eq!(clamp_to_byte(0.5), 128);
    assert_eq!(clamp_to_byte(0.7), 179);

    assert_eq!(clamp_to_byte(-15.0), 0);
    assert_eq!(clamp_to_byte(12345.0), 255);

    assert_eq!(clamp_to_byte(f32::NAN), 0);
    assert_eq!(clamp_to_byte(f32::INFINITY), 255);
    assert_eq!(clamp_to_byte(f32::NEG_INFINITY), 0);
}

pub fn save_as_ppm(frame: &Frame, filepath: &str) -> std::io::Result<()>{
    let mut f = File::create(filepath)?;

    let header = format!("P6\n{} {}\n255\n", frame.width(), frame.height());
    f.write_all(header.as_bytes())?;
    
    let mut buf = [0; 3];
    for x in frame.data().iter() {
        buf[0] = clamp_to_byte(x.r);
        buf[1] = clamp_to_byte(x.g);
        buf[2] = clamp_to_byte(x.b);
        f.write(&buf)?;
    }
    f.sync_all()?;

    Ok(())
}