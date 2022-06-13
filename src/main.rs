// Код написан по мотивам статьи: 
// https://habr.com/ru/post/248153/

// Снимок репозитория: 
// https://github.com/ssloy/tinyrenderer/tree/f6fecb7ad493264ecd15e230411bfb1cca539a12

use std;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::error::Error;
use std::mem::swap;
use std::str::FromStr;
use std::path::Path;


mod TGAWriter;
use TGAWriter::Image;
use TGAWriter::Color;



fn main() {
    let xMax = 1000;
    let yMax = 1000;
    let xMax = 1000;
    let yMax = 1000;
asdfasdf
    let mut img : Image = Image::new((xMax + 1) as i32, (yMax + 1) as i32);
    

    let vects_faces_res = model_create("C:\\Users\\Administrator\\Documents\\Rust_Projects\\Bres\\target\\debug\\african_head.obj".to_string());
    
    let vects_faces = vects_faces_res.unwrap();
    let vects = vects_faces.0;
    let faces = vects_faces.1;

    for i in 0..faces.len()
    {
        let face = &faces[i];
        for j in 0..3
        {
            
            let v0 = &vects[(face[j] - 1) as usize];
            let v1 = &vects[(face[(j+1)%3] - 1) as usize];

            let x0 = ((v0[0] as f64 + 1.0) * (xMax as f64/2.0)) as i32;
            let y0 = ((v0[1] as f64 + 1.0) * (yMax as f64/2.0)) as i32;
            let x1 = ((v1[0] as f64 + 1.0) * (xMax as f64/2.0)) as i32;
            let y1 = ((v1[1] as f64 + 1.0) * (yMax as f64/2.0)) as i32;
            line(&mut img,x0,y0, x1,y1,Color::new(255,255,255));
        }
    }
    

    img.write_to_tga("render_1.tga").unwrap();
 
}


fn model_create(strPathToObjFile : String)-> std::io::Result<(Vec<Vec<f32>>,Vec<Vec<i32>>)>{
    let mut vects = Vec::new();
    let mut faces = Vec::new();
    
    let path = std::path::Path::new(&strPathToObjFile);
    let display = path.display();

    let  file = match File::open(&path){
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file
    };
    
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    for lineResult in lines {
        

        let work_line = lineResult;
        
 
        let  dataStr = match work_line{
            Ok(line) => line,
            Error => continue
            
        };
        
        
        if (dataStr.starts_with("#"))
        {}
        else{
            if(dataStr.starts_with("v "))
            {
                let mut dataStr = dataStr.split_whitespace();
        
                
                dataStr.next();
                let mut vec  = Vec::new();

                
                let xstr = dataStr.next().unwrap();
                let ysrt = dataStr.next().unwrap();


                vec.push(f32::from_str(xstr).unwrap());
                vec.push(f32::from_str(ysrt).unwrap());


                vects.push(vec);
            }
            else if(dataStr.starts_with("f "))
            {
                let mut dataStr = dataStr.split_whitespace();
                dataStr.next();
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


fn line(img : &mut TGAWriter::Image, x0 : i32, y0 : i32, x1 : i32, y1 : i32,  color : Color){
 
    let mut mx0 = x0;
    let mut my0 = y0;
    let mut mx1 = x1;
    let mut my1 = y1;

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

