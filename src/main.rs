
extern crate image;
use std::mem;
use std::slice;
use std;
use std::fs::File;
use std::io::Write;
use std::io;

use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::string::String;
use std::error::Error;
use std::mem::swap;
use std::str::FromStr;
mod wavefront;
#[derive(Clone, Copy)]
struct Color(u8, u8, u8);

struct Image {
    width: i32,
    height: i32,
    data: Vec<Color>,
}

unsafe fn struct_to_u8_slice<T>(s: &T) -> &[u8] {
    let data_ptr: *const u8 = mem::transmute(s);
    slice::from_raw_parts(data_ptr, mem::size_of::<T>())
}

unsafe fn slice_to_u8_slice<T>(s: &[T]) -> &[u8] {
    let data_ptr: *const u8 = mem::transmute(&s[0]);
    slice::from_raw_parts(data_ptr, mem::size_of::<T>() * s.len())
}

impl Image {
    fn new(width: i32, height: i32) -> Image {
        let v = vec![Color(0,0,0);(width*height) as usize];
        let result = Image {
            width: width,
            height: height,
            data: v,
        };

        result
    }

    fn apply_gamma(self: &mut Image, gamma: f32) {
        for c in self.data.iter_mut() {
            let Color(r, g, b) = *c;
            let fr = ((r as f32) / 255.0).powf(gamma);
            let fg = ((g as f32) / 255.0).powf(gamma);
            let fb = ((b as f32) / 255.0).powf(gamma);
            c.0 = (fr * 255.0) as u8;
            c.1 = (fg * 255.0) as u8;
            c.2 = (fb * 255.0) as u8;
        }
    }

    fn set_pixel(self: &mut Image, x: i32, y: i32, c: Color) {
        self.data[(x + y * self.width) as usize] = c;
    }

    fn write_to_tga(self: &Image, filename: &str) -> io::Result<()> {
        #[repr(C, packed)]
        #[derive(Default)]
        struct Header {
            id_length: u8,
            color_map_type: u8,
            image_type: u8,
            c_map_start: u16,
            c_map_length: u16,
            c_map_depth: u8,
            x_offset: u16,
            y_offset: u16,
            width: u16,
            height: u16,
            pixel_depth: u8,
            image_descriptor: u8,
        }
        let h = Header {
            image_type: 2,
            width: self.width as u16,
            height: self.height as u16,
            pixel_depth: 24,
            ..Header::default()
        };

        let mut f = r#try!(File::create(filename));
        unsafe {
            r#try!(f.write_all(struct_to_u8_slice(&h)));
            r#try!(f.write_all(slice_to_u8_slice(&self.data[..])));
        }
        Ok(())
    }
}

fn abs(num : i32) -> i32
{
    let mut return_num = num;
    if num < 0{
            return_num = num * -1;
    }
    return return_num;
}




fn line(img : &mut Image, x0 : i32, y0 : i32, x1 : i32, y1 : i32,  color : Color){
    //logLine(img,x0,y0,x1,y1,numLine);
    let mut mx0 = x0;
    let mut my0 = y0;
    let mut mx1 = x1;
    let mut my1 = y1;

    let mut steep = false;

    

    //println!("2");

    let mut steep = (x0 - x1).abs() < (y0 - y1).abs();
    
    
    if(steep)
    {
        swap(&mut mx0, &mut my0);
        swap(&mut mx1, &mut my1);
        
    }

    if(mx0>mx1)
    {
        swap(&mut mx0, &mut mx1);
        swap(&mut my0, &mut my1);
    }
    let delta_X =  mx1-mx0;
    let delta_Y =  my1-my0;
    
    let mut error2 = 0;
    let deltaerr2 = (delta_Y * 2).abs();
    let mut y = my0;
    let mut dirY = my1-my0;
    if dirY > 0{
        dirY = 1;
    }
    if dirY < 0{
        dirY = -1;
    }
    
    for x in mx0..=mx1{
        
        if(steep)
        {
            img.set_pixel(y,x,color);
        }
        else{
            img.set_pixel(x,y,color);
        }

        error2 = error2 + deltaerr2;

        if  error2 >= delta_X
        {
            y = y + dirY;
            error2 = error2 - 2 *delta_X;
        }


    }

}

fn draw_line(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32,
             buffer: &mut image::RgbImage, color: image::Rgb<u8>) {

    let steep = (x0 - x1).abs() < (y0 - y1).abs();

    if steep {
        swap(&mut x0, &mut y0);
        swap(&mut x1, &mut y1);
    }

    if x0 > x1 {
        swap(&mut x0, &mut x1);
        swap(&mut y0, &mut y1);
    }

    let delta_x = x1 - x0;
    let delta_y = y1 - y0;
    let delta_error = (delta_y * 2).abs();
    let mut y = y0;
    let mut error = 0;

    for x in x0 ..= x1 {
        if steep {
            buffer.put_pixel(y as u32, x as u32, color);
        } else {
            buffer.put_pixel(x as u32, y as u32, color);
        }
        error += delta_error;
        if error > delta_x {
            y += if y1 > y0 { 1 } else { -1 };
            error -= delta_x * 2;
        }
    }
}

fn main() {
    let xMax = 1000;
    let yMax = 1000;

    
    let mut buffer = image::ImageBuffer::new(xMax + 1, yMax + 1);

    let mut img : Image = Image::new((xMax + 1) as i32, (yMax + 1) as i32);
    

    let vects_faces_res = model_create();
    
    let vects_faces = vects_faces_res.unwrap();
    let vects = vects_faces.0;
    let faces = vects_faces.1;

    for i in 0..faces.len()
    {
        let face = &faces[i];
        for j in 0..3
        {
            
            let v0 = &vects[face[j] as usize];
            let v1 = &vects[face[(j+1)%3] as usize];

            let x0 = ((v0[0] as f64 + 1.0) * (xMax as f64/2.0)) as i32;
            let y0 = ((v0[1] as f64 + 1.0) * (yMax as f64/2.0)) as i32;
            let x1 = ((v1[0] as f64 + 1.0) * (xMax as f64/2.0)) as i32;
            let y1 = ((v1[1] as f64 + 1.0) * (yMax as f64/2.0)) as i32;
            line(&mut img,x0,y0, x1,y1,Color(255,255,255));
            draw_line(x0, y0, x1, y1, &mut buffer, image::Rgb([255, 255, 255]))
        }
    }
    


    


    let coordinates = wavefront::Object::new("C:\\Users\\Administrator\\Documents\\Rust_Projects\\Bres\\target\\debug\\african_head.obj");

    let mut buffer1 = image::ImageBuffer::new(xMax + 1, yMax + 1);
    let mut img1 : Image = Image::new((xMax + 1) as i32, (yMax + 1) as i32);
    for face in coordinates.geometric_faces {
        for i in 0..3 {
            let v0 = coordinates.geometric_vertices[(face[i]) as usize];
            let v1 = coordinates.geometric_vertices[(face[(i+1) % 3]) as usize];

            let x0 = ((v0.x + 1.0) * buffer1.width() as f64 / 2.0).min(buffer1.width() as f64 - 1.0);
            let y0 = ((v0.y + 1.0) * buffer1.height() as f64 / 2.0).min(buffer1.height() as f64 - 1.0);

            let x1 = ((v1.x + 1.0) * buffer1.width() as f64 / 2.0).min(buffer1.width() as f64 - 1.0);
            let y1 = ((v1.y + 1.0) * buffer1.height() as f64 / 2.0).min(buffer1.height() as f64 - 1.0);
            line(&mut img1,x0 as i32,y0 as i32, x1 as i32,y1 as i32,Color(255,255,255));
            draw_line(x0 as i32, y0 as i32, x1 as i32, y1 as i32,
                      &mut buffer1, image::Rgb([255, 255, 255]));
        }
    }
    img.write_to_tga("render_211.tga").unwrap();
    let ref mut render = File::create("output21.png").unwrap();

    image::ImageRgb8(buffer).flipv()
                            .save(render, image::PNG)
                            .unwrap();
    img1.write_to_tga("render_221.tga").unwrap();
    let ref mut render = File::create("output221.png").unwrap();

    image::ImageRgb8(buffer1).flipv()
                            .save(render, image::PNG)
                            .unwrap();

    
}
fn model_create()-> std::io::Result<(Vec<Vec<f32>>,Vec<Vec<i32>>)>{
    let mut vects = Vec::new();
    let mut faces = Vec::new();
    
    let path = std::path::Path::new("C:\\Users\\Administrator\\Documents\\Rust_Projects\\Bres\\target\\debug\\african_head.obj");
    let display = path.display();
    println!("Haha");
    let  file = match File::open(&path){
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file
    };
    
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    for lineResult in lines {
        
        //println!("{}","let work_line = lineResult;");
        let work_line = lineResult;
        //println!("{}","let mut dataStr = work_line.unwrap();");
        let  dataStr = work_line.unwrap();
        
        //println!("{}","let mut dataStr = dataStr.split_whitespace();");

        
        let mut dataStr = dataStr.split_whitespace();
        
        //println!("{}","let mut f_str = dataStr.next().unwrap();");
        let f_strNext = dataStr.next();
        match f_strNext{
            None => continue,
            _ => {}
        }
        let f_str = f_strNext.unwrap();
        //println!("{}","if (f_str.starts_with())");
        if (f_str.starts_with("#"))
        {
            let ara = f_str;
            println!("{}",f_str);

        }
        else{
        
            if(f_str.starts_with("v"))
            {
                let mut vec  = Vec::new();

                
                let xstr = dataStr.next().unwrap();
                let ysrt = dataStr.next().unwrap();


                vec.push(f32::from_str(xstr).unwrap());
                vec.push(f32::from_str(ysrt).unwrap());


                vects.push(vec);
            }
            else if(f_str.starts_with("f"))
            {
                let mut vec = Vec::new();
                
                
                    // Один из наборов, состоящих из 3 номеров
                    // Пример: f |3/3/3| 3/3/3 3/3/3 , где между |...| - один из кусков
                    let part1 : Vec<&str> = dataStr.next().unwrap().split('/').collect();
                    let part2 : Vec<&str> = dataStr.next().unwrap().split('/').collect();
                    let part3 : Vec<&str> = dataStr.next().unwrap().split('/').collect();
                    vec.push(i32::from_str(part1[0]).unwrap());
                    vec.push(i32::from_str(part2[0]).unwrap());
                    vec.push(i32::from_str(part3[0]).unwrap());
                

                faces.push(vec);
                
            }
        }

        
    }
    Ok((vects,faces))
    
}
fn parseObjString(str : String){

}
