use std::collections::HashMap;
use std::error::Error;
use csv::{Reader, StringRecord, ErrorKind};
use std::hash::Hash;
use std::fmt::{Display, Formatter};
use std::{fmt, fs};
use std::string::ToString;
use std::fs::File;
use std::io::{BufReader, BufRead};

struct CrossObject {
    info: CrossInformation,
    data: Vec<Cross>,
}

impl CrossObject {
    fn new(info: CrossInformation, data: Vec<Cross>) -> CrossObject {
        CrossObject {
            info,
            data,
        }
    }
}

struct CrossInformation {
    path: String,
    mining_type: String,
    seperator: String
}

impl CrossInformation {
    fn new(path: String, mining_type: String, seperator: String) -> CrossInformation {
        CrossInformation {
            path,
            mining_type,
            seperator
        }
    }

    fn read(&self) -> Result<Vec<Cross>, Box<dyn Error>> {
        // our data
        let mut cross_objects: Vec<Cross> = vec![];

        let cross_str_path: &String = &self.path;

        let file = File::open(cross_str_path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // skip first header line
        lines.next();
        lines.next();

        let mut group_no = 0;

        for line in lines {
            let record = line.expect("Failed to reading line of str file");

            if record.contains(&self.seperator) {
                println!("seperator included");
                group_no += 1;

            } else {
                let record: Vec<String> = record.split(",").map(|s| s.trim().to_string()).collect();

                println!("record : {:?}", record);
                let x: f64 = record[1].parse().expect("x icin numerik deger çevirilemedi");
                let y: f64 = record[2].parse().expect("y icin numerik deger çevirilemedi");
                let z: f64 = record[3].parse().expect("z icin numerik deger çevirilemedi");

                let coord = CrossCoordinate::new(x, y, z);
                let cross = Cross::new(group_no, coord);

                cross_objects.push(cross);

            }


        }

        Ok(cross_objects)
    }
}

#[derive(Debug)]
struct Cross {
    group_no: i32,
    coordinate: CrossCoordinate,
}

impl Cross {
    fn new(group_no: i32, coordinate: CrossCoordinate) -> Cross {
        Cross {
            group_no,
            coordinate,
        }
    }
}

impl Display for Cross {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Cross no : {} \n\
                   coordinate : {}", self.group_no, self.coordinate)
    }
}

#[derive(Debug)]
struct CrossCoordinate {
    x_coord: f64,
    y_coord: f64,
    z_coord: f64,
}

impl CrossCoordinate {
    fn new(x: f64, y: f64, z: f64) -> CrossCoordinate {
        CrossCoordinate {
            x_coord: x,
            y_coord: y,
            z_coord: z
        }
    }
}

impl Display for CrossCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x : {} \n\
                   y : {} \n\
                   z : {}", self.x_coord, self.y_coord, self.z_coord)
    }
}

fn main() {
    // path
    let cross_csv_path = String::from(r"C:\Users\umut\CLionProjects\LegoRust\tests\data\halkalar\cu_enkesit.str");

    // data
    let cross_information = CrossInformation::new(cross_csv_path,
                                                  "Cu".to_string(), "0, 0.000, 0.000, 0.000,".to_string());
    let crosses_data = cross_information.read().expect("Cross file cannot be read and parsed !");
    println!("Cross rows : {:?}", crosses_data);

    let cross_object = CrossObject::new(cross_information, crosses_data);
}