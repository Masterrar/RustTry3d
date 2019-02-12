use std::io;

use std::mem;
use std::slice;
use std;
use std::fs::File;
use std::io::Write;

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



fn line(img : &mut Image, x0 : i32, y0 : i32, x1 : i32, y1 : i32, numLine : String, color : Color){
    logLine(img,x0,y0,x1,y1,numLine);
    let mut mx0 = x0;
    let mut my0 = y0;
    let mut mx1 = x1;
    let mut my1 = y1;

    let mut steep = false;

    let delta_X =  abs(x0 - x1);
    let delta_Y =  abs(y0 - y1);

    //println!("2");

    
    
    
    if(delta_X < delta_Y)
    {
        mx0 = mx0 + mx1;
        mx1 = mx0 - mx1;
        mx0 = mx0 - mx1;

        my0 = my0 + my1;
        my1 = my0 - my1;
        my0 = my0 - my1;

        steep = true;
        

    }
    
    //println!("3");
    let mut error = 0;
    let deltaerr = delta_Y;
    let mut y = my0;
    let mut dirY = my1-my0;

    //println!("4");
    if dirY > 0{
        dirY = 1;
    }
    if dirY < 0{
        dirY = -1;
    }
//println!("5");
    if(mx0>mx1)
    {
        mx0 = mx0 + mx1;
        mx1 = mx0 - mx1;
        mx0 = mx0 - mx1;
    }
    for x in mx0..mx1{

       //println!("6");
        //println!("{}:{}",x,y);
        //println!("{}:{}"  ,x.to_string(),y.to_string());
        if(steep == false)
        {
            plot(img,x,y,color);
        }
        else{
            plot(img,y,x,color);
        }

        error = error + deltaerr;

        if 2 * error >= delta_X
        {
            y = y + dirY;
            error = error - delta_X;
        }
        //println!("7");

    }
    //println!("8");
}


fn line_reverseX(img : &mut Image, x0 : i32, y0 : i32, x1 : i32, y1 : i32, gap : i32 ,numLine : String){
    line_reverse(img,x0 + gap,y0,x1 + gap,y1, numLine);
}
fn line_reverseY(img : &mut Image, x0 : i32, y0 : i32, x1 : i32, y1 : i32, gap : i32 , numLine : String){
    line_reverse(img,x0 ,y0 + gap ,x1 ,y1 + gap, numLine);
}
fn line_reverse(img : &mut Image, x0 : i32, y0 : i32, x1 : i32, y1 : i32, numLine : String){
     
    line(img,x1 ,y1,x0 ,y0, numLine , Color(0,0,255));
}

fn logLine(img : &mut Image, x0 : i32, y0 : i32, x1 : i32, y1 : i32, numLine :String){
    println!("---------------------{} x({}, {}) y({}, {})---------------------", numLine,x0,y0,x1,y1);
}

fn plot(img : &mut Image, x : i32 , y : i32, color : Color){
        //let r = ((x ^ y) % 256) as u8;
        //let g = ((x + y) % 256) as u8;
        //let b = ((y.wrapping_sub(x)) % 256) as u8;
        let r = 0 as u8;
        let g = 0 as u8;
        let b = 255 as u8;
        //println!("|{}|{}|{}|",r,g,b);
        img.set_pixel(x,y, color);
}


fn LineTest(img : &mut Image, x0 : i32, y0 : i32, gapX : i32 ,gapY : i32,numLine : String)
{
    
    let xMax = img.width;
    let yMax = img.height;

    let mut x1 = xMax - x0;
    let mut y1 = yMax - y0;
    
    line(img,x0,y0,x1,y1,numLine, Color(255,0,0));
    if(x1 + gapX > xMax)
    {
        x1 = xMax - gapX;
    }
    if(y1 + gapY > yMax)
    {
        y1 = yMax - gapY;
    }

    line(img,x1 + gapX,y1 + gapY,x0 + gapX,y0 + gapY,"reverse".to_string(), Color(0,0,255));
}
/*
void line(int x0, int y0, int x1, int y1, TGAImage &image, TGAColor color) {
    for (float t=0.; t<1.; t+=.01) {
        int x = x0*(1.-t) + x1*t;
        int y = y0*(1.-t) + y1*t;
        image.set(x, y, color);
    }
}
*/
fn main() {
    let xMax = 511;
    let yMax = 511;

    let xMin = 0;
    let yMin = 0;

    let x025 = xMax / 4;
    let y025 = yMax / 4;

    let x05 = xMax / 2;
    let y05 = yMax / 2;

    let gap = 16;
    

    let mut img : Image = Image::new(xMax + 1, yMax + 1);
    
    

    LineTest(&mut img,0         ,yMax               ,gap,0  ,"1".to_string());
    LineTest(&mut img,0         ,0                  ,gap,0  ,"2".to_string());

    LineTest(&mut img,x025 + gap,yMax               ,gap,0  ,"3".to_string());
    LineTest(&mut img,x025 + gap,0                  ,gap,0  ,"4".to_string());

    LineTest(&mut img,0         ,y05 + y025 - gap   ,0  ,gap,"5".to_string());
    LineTest(&mut img,0         ,y05 - y025 + gap   ,0  ,gap,"6".to_string());
    
    LineTest(&mut img,x05       ,yMax               ,gap,0  ,"7".to_string());
    LineTest(&mut img,0         ,y05                ,0  ,gap,"8".to_string());


    img.write_to_tga("line_15.tga").unwrap();
}
