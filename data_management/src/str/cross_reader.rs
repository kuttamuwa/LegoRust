use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::fmt::{Display, Formatter};
use std::fmt;
use lego_config::read::{LegoConfig, DataManagementObjects};
use std::collections::HashSet;
use std::borrow::BorrowMut;
use std::ptr::eq;
use std::borrow::Cow::Borrowed;

#[derive(Debug)]
pub struct CrossObject {
    info: CrossInformation,
    data: Vec<Cross>,
}

impl CrossObject {
    fn new(info: CrossInformation) -> CrossObject {
        let mut data = info.read()
            .expect("Error occured while reading cross section ");

        CrossObject::order_data(&mut data);

        CrossObject {
            info,
            data,
        }
    }

    fn order_data(crosses: &mut Vec<Cross>) {
        // sorting by z

        crosses.sort_by(|c1, c2|
            c1.coordinate.z_coord.partial_cmp(&c2.coordinate.z_coord).unwrap());
    }

}

#[derive(Debug)]
struct CrossInformation {
    path: String,
    mining_type: String,
    seperator: String,
    duplicate_avoiding: bool
}

impl CrossInformation {
    fn new(path: String, mining_type: String, seperator: String) -> CrossInformation {
        CrossInformation {
            path,
            mining_type,
            seperator,
            duplicate_avoiding: true
        }
    }

    fn new_from_config(config: &LegoConfig) -> CrossInformation {
        // getting mining information
        let path = config.get_cross_section_str_path();
        let mining_type = config.get_mining_type();
        let seperator = config.get_cross_section_seperator();

        CrossInformation::new(path, mining_type, seperator)
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
                group_no += 1;

            } else {
                let record: Vec<String> = record.split(",").map(|s| s.trim().to_string()).collect();

                // println!("record : {:?}", record);
                let x: f64 = record[1].parse().expect("x icin numerik deger çevirilemedi");
                let y: f64 = record[2].parse().expect("y icin numerik deger çevirilemedi");
                let z: f64 = record[3].parse().expect("z icin numerik deger çevirilemedi");

                let coord = CrossCoordinate::new(x, y, z);
                let cross = Cross::new(group_no, coord);

                if cross_objects.contains(&cross) == false && self.duplicate_avoiding {
                    // avoiding duplicates
                    cross_objects.push(cross);
                }

            }


        }

        Ok(cross_objects)
    }
}

#[derive(Debug, PartialEq)]
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

#[derive(Debug, PartialEq)]
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

    pub fn eq_test(&self, coordinate: &CrossCoordinate) -> bool {
        if (self.x_coord == coordinate.x_coord) && (self.y_coord == coordinate.y_coord) &&
            (self.z_coord == coordinate.z_coord) {
           true
        } else {
            false
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


impl Display for Cross {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Cross no : {} \n\
                   coordinate : {}", self.group_no, self.coordinate)
    }
}


impl Display for CrossObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "info : {} \n\
                   crosses : {:?}", self.info, self.data)
    }
}


impl Display for CrossInformation {
    // path: String,
    //     mining_type: String,
    //     seperator: char

    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "path: {} \n\
                   mining type : {} \n\
                   seperator : {}", self.path, self.mining_type, self.seperator)
    }
}

#[cfg(test)]
mod tests {
    use lego_config::read::LegoConfig;

    use crate::str::cross_reader::{CrossInformation, CrossObject};

    const TEST_CONFIG_PATH: &str = "/home/umut/CLionProjects/LegoRust/lego_config/test_settings.toml";

    #[test]
    fn read_cross_section_from_config() {
        let config_object = LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let cross_info = CrossInformation::new_from_config(&config_object);
        let cross_object = CrossObject::new(cross_info);
        println!("cross sections info : {:?}", &cross_object.info);
        for c in &cross_object.data {
            println!("section : {:?}", c);
        }
        // println!("cross sections data : {:?}", &cross_object.data);
        println!("count of sections : {:?}", &cross_object.data.len());
    }

    #[test]
    fn order_crosses_by_z() {
        let config_object = LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let cross_info = CrossInformation::new_from_config(&config_object);
        let cross_object = CrossObject::new(cross_info);

        for i in 1..cross_object.data.len() {
            let c1 = &cross_object.data[i];
            let c2 = &cross_object.data[i - 1];

            println!("before z: {}", c1.coordinate.z_coord);
            println!("now z: {}", c2.coordinate.z_coord);

            assert!(c1.coordinate.z_coord >= c2.coordinate.z_coord);
        }
    }
}
