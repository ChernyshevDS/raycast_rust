use std::fs::File;
use std::io::prelude::*;

use super::frame::Frame;

fn clamp_to_byte(v: f32) -> u8 {
    if v > 1.0 { return 255; }
    else if v < 0.0 { return 0; }
    else { return (v * 255.0).round() as u8 }
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

    let header = format!("P3\n{} {}\n255\n", frame.width(), frame.height());
    f.write_all(header.as_bytes())?;
    
    let mut content = String::with_capacity(frame.width() * frame.height() * 10);
    for x in frame.data() {
        let r = clamp_to_byte(x.r);
        let g = clamp_to_byte(x.g);
        let b = clamp_to_byte(x.b);
        let s = format!("{} {} {}\n", r, g, b);
        content.push_str(&s);
    }
    f.write_all(content.as_bytes())?;
    f.sync_all()?;

    Ok(())
}