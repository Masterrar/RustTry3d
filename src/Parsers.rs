
    use nalgebra::core::{Vector2, Vector3};
    pub struct Point<T>{
        X : T,
        Y : T,
        Z : T,
        Vector : Vector3
    }
    
    pub struct ObjModel{
        vects : std::vec::Vec<Point<f32>>,
        faces : std::vec::Vec<Vector3<i32>>
    }



    impl <T> Point<T> {
        pub fn new(x : T, y : T, z : T) -> Point{
            let X = x;
            let Y = y;
            let Z = z;
            let Vector = Vector3<T>::new(X,Y,Z);


            Point 
            { 
                X:X,
                Y:Y,
                Z:Z,
                Vector:Vector
            }
        }
    }

    

    impl ObjModel{


        pub fn new(strPathToObjFile : String)-> ObjModel{
            let mut vects : Vec<Point<f32>> = Vec::new();
            let mut faces : Vec<Vector3<i32>> = Vec::new();
            
            
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
                        
                        let xstr = dataStr.next().unwrap();
                        let ystr = dataStr.next().unwrap();
                        let zstr = dataStr.next().unwrap();

                        vects.push  (Point::new (
                                                f32::from_str(xstr).unwrap(),
                                                f32::from_str(ystr).unwrap(),
                                                f32::from_str(zstr).unwrap()
                                                )
                                    );
                    }
                    else if(dataStr.starts_with("f "))
                    {
                        let mut dataStr = dataStr.split_whitespace();
                        dataStr.next();

                        // Один из наборов, состоящих из 3 номеров
                        // Пример: f |3/3/3| 3/3/3 3/3/3 , где между |...| - один из кусков
                        let part1 : Vec<&str> = dataStr.next().unwrap().split('/').collect();
                        let part2 : Vec<&str> = dataStr.next().unwrap().split('/').collect();
                        let part3 : Vec<&str> = dataStr.next().unwrap().split('/').collect();
                        

                        faces.push  (Vector3::new   (
                                                    i32::from_str(part1[0]).unwrap(),
                                                    i32::from_str(part2[0]).unwrap(),
                                                    i32::from_str(part3[0]).unwrap()
                                                    )
                                    );
                        
                    }
                }
            }
            ObjModel 
            {   
                vects : vects ,
                faces : faces
            }


        }
        