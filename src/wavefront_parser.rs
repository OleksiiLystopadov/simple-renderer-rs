use std::fs::File;
use std::str::FromStr;
use std::io::{BufReader, BufRead};

pub fn read(str_path_to_obj_file: String) -> std::io::Result<(Vec<Vec<f32>>, Vec<Vec<i32>>)>{
    let mut vectors = Vec::new();
    let mut faces = Vec::new();

    let path = std::path::Path::new(&str_path_to_obj_file);
    let display = path.display();

    let  file = match File::open(&path){
        Err(why) => panic!("couldn't open {}: {}", display,
                           why.to_string()),
        Ok(file) => file
    };

    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    for lineResult in lines {


        let work_line = lineResult;


        let data_str = match work_line{
            Ok(line) => line,
            Error => continue

        };


        if (data_str.starts_with("#"))
        {}
        else{
            if(data_str.starts_with("v "))
            {
                let mut data_string = data_str.split_whitespace();


                data_string.next();
                let mut vec  = Vec::new();


                let xstr = data_string.next().unwrap();
                let ysrt = data_string.next().unwrap();
                let zsrt = data_string.next().unwrap();


                vec.push(f32::from_str(xstr).unwrap());
                vec.push(f32::from_str(ysrt).unwrap());
                vec.push(f32::from_str(zsrt).unwrap());


                vectors.push(vec);
            }
            else if(data_str.starts_with("f "))
            {
                let mut dataStr = data_str.split_whitespace();
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
    Ok((vectors, faces))
}
