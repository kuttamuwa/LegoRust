use std::collections::HashMap;
use std::error::Error;
use csv::{Reader, StringRecord, ErrorKind};
use std::hash::Hash;
use std::fmt::{Display, Formatter};
use std::{fmt, fs};
use std::string::ToString;
use std::fs::File;
use std::io::{BufReader, BufRead};

struct CompositeObject {
    info: CompositeInformation,
    data: Vec<Composite>,
}

impl CompositeObject {
    fn new(info: CompositeInformation, data: Vec<Composite>) -> CompositeObject {
        CompositeObject {
            info,
            data,
        }
    }
}

struct CompositeInformation {
    path: String,
    mining_type: String,
    seperator: String,
}

impl CompositeInformation {
    fn new(path: String, mining_type: String, seperator: String) -> CompositeInformation {
        CompositeInformation {
            path,
            mining_type,
            seperator,
        }
    }

    fn read(&self) -> Result<Vec<Composite>, Box<dyn Error>> {
        // our data
        let mut composite_objects: Vec<Composite> = vec![];

        let composite_str_path: &String = &self.path;

        let file = File::open(composite_str_path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // skip first rows
        lines.next();

        for line in lines {
            let record = line.expect("Failed to reading line of str file");

            let record: Vec<String> = record.split(",").map(|s| s.trim().to_string()).collect();

            let regular_no: i32 = record[0].parse().expect("regular no icin numerik deger cevirilemedi");

            if regular_no != 0 {
                let x: f64 = record[1].parse().expect("x icin numerik deger çevirilemedi");
                let y: f64 = record[2].parse().expect("y icin numerik deger çevirilemedi");
                let z: f64 = record[3].parse().expect("z icin numerik deger çevirilemedi");
                let tenor: f64 = record[4].parse().expect("tenor orani için numerik deger çevirilemedi");
                let drill_no: String = record[5].parse().expect("drill no için text değer çevirilemedi");
                let cut_from: f64 = record[6].parse().expect("kesim başlangıç değeri numeriğe çevirilemedi");
                let cut_end: f64 = record[7].parse().expect("Kesim bitiş değeri numerik değere çevirilemedi");
                let cut_taken: f64 = record[8].parse().expect("Son sütun olan kesim alım değeri numerik değere çevirilemedi");

                let coord = CompositeCoordinate::new(x, y, z);
                let composite = Composite::new(regular_no, tenor, drill_no,
                cut_from, cut_end, cut_taken, coord);

                composite_objects.push(composite);
            }
        }

        Ok(composite_objects)
    }
}

#[derive(Debug)]
struct Composite {
    group_no: i32,
    tenor: f64,
    drill_no: String,
    cut_from: f64,
    cut_end: f64,
    cut_taken: f64,
    coordinate: CompositeCoordinate,
}

impl Composite {
    fn new(composite_no: i32, tenor: f64, drill_no: String, cut_from: f64, cut_end: f64,
           cut_taken: f64,coordinate: CompositeCoordinate) -> Composite {
        Composite {
            group_no: composite_no,
            tenor,
            drill_no,
            cut_from,
            cut_end,
            cut_taken,
            coordinate,
        }
    }
}

impl Display for Composite {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "group no : {} \n\
                   coordinate : {}", self.group_no, self.coordinate)
    }
}

#[derive(Debug)]
struct CompositeCoordinate {
    x_coord: f64,
    y_coord: f64,
    z_coord: f64,
}

impl CompositeCoordinate {
    fn new(x: f64, y: f64, z: f64) -> CompositeCoordinate {
        CompositeCoordinate {
            x_coord: x,
            y_coord: y,
            z_coord: z,
        }
    }
}
// let tenor: f64 = record[4].parse().expect("tenor orani için numerik deger çevirilemedi");
//                 let drill_no: String = record[5].parse().expect("drill no için text değer çevirilemedi");
//                 let cut_from: f64 = record[6].parse().expect("kesim başlangıç değeri numeriğe çevirilemedi");
//                 let cut_end: f64 = record[7].parse().expect("Kesim bitiş değeri numerik değere çevirilemedi");
//                 let cut_taken: f64 = record[8].parse().expect("Son sütun olan kesim alım değeri numerik değere çevirilemedi");

impl Display for CompositeCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x : {} \n\
                   y : {} \n\
                   z : {}", self.x_coord, self.y_coord, self.z_coord)
    }
}

fn main() {
    // path
    let composite_csv_path = String::from(r"C:\Users\umut\CLionProjects\LegoRust\tests\data\halkalar\cu_composite1.str");

    // data
    let composite_information = CompositeInformation::new(composite_csv_path,
                                                          "Cu".to_string(), ";".to_string());
    let composites_data = composite_information.read().expect("Composite file cannot be read and parsed !");
    println!("Composite rows : {:?}", composites_data);

    let composite_object = CompositeObject::new(composite_information, composites_data);
}