use std::fs::File;
use std::io::prelude::*;
use cgmath::{Vector3};

const OUTPUT_WIDTH: u32 = 256;
const OUTPUT_HEIGHT: u32 = 256;
const MAX_HEIGHT: f32 = 8.0;
const EPSILON: f32 = 0.1;
const MAX_ATTEMPS: u32 = 100;

fn main() -> std::io::Result<()> {
    let mut file = File::create("foo.ppm")?;
    file.write_all(b"P2 \n")?;
    file.write(format!("{} {} {} \n",OUTPUT_WIDTH,OUTPUT_HEIGHT,255).as_bytes())?;
    for x in 0..OUTPUT_WIDTH {
	for y in 0..OUTPUT_WIDTH {
	    let height_at_position: f32 = get_dist(Vector3::new(x as f32,0.0,y as f32),0);
	    let color: f32 = (height_at_position/MAX_HEIGHT) * 255.0;
	    file.write(format!("{} \n",color as i32).as_bytes())?;
	}
    }
    Ok(())
}

fn get_dist(p:Vector3<f32>,i:u32) -> f32 {
    if i == MAX_ATTEMPS || p.y >= MAX_HEIGHT {
	return p.y;
    }

    let dist: f32 = sdf(&p);
    if dist >= 0.1 {
	return p.y;
    }
    
    get_dist(p+Vector3::new(0.0,EPSILON,0.0),i+1)
}

fn sdf(p:&Vector3<f32>) -> f32 {
    p.y-2.0
}

fn min(x: f32,y: f32) -> f32 {
    if x > y {
	return x;
    }

    return y;
}
