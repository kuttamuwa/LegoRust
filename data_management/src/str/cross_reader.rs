use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

use lego_config::read::{DataManagementObjects, LegoConfig};

#[derive(Debug)]
pub struct CrossObject {
    info: CrossInformation,
    data: Vec<Cross>,
}

impl CrossObject {
    pub(crate) fn new(info: CrossInformation) -> CrossObject {
        let mut data = info.read()
            .expect("Error occured while reading cross section ");

        // ordering data
        CrossObject::order_data(&mut data);

        // is there any duplicates?
        let dups = CrossObject::take_duplicates(&data);
        println!("dups : {:?}", dups);

        // take minimum z drill
        println!("min drill : {}", CrossObject::take_min_drill(&data));

        // is there any data which z coordinate is deeper than min one?
        CrossObject::delete_deeper_than_min_drill(&mut data);

        let extent = Cross::find_model_frame(&data);
        println!("extent : {:?}", extent);

        for i in 0..data.len() {
            println!("{}", &data[i]);
        }

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

    fn take_duplicates(crosses: &Vec<Cross>) -> Vec<&Cross> {
        let mut duplicates: Vec<&Cross> = vec![];

        for (_, c) in crosses.iter().enumerate() {
            for k in crosses.iter() {
                if c == k {
                    duplicates.push(c);
                }
            }
        }

        duplicates
    }

    fn take_min_drill(crosses: &Vec<Cross>) -> &Cross {
        crosses.get(0).unwrap()
    }

    fn delete_deeper_than_min_drill(crosses: &mut Vec<Cross>) {
        let min_drill_z_coords = CrossObject::take_min_drill(&crosses)
            .coordinate.z_coord;

        let mut to_be_delete_index: i32 = -1;  // dummy var

        for (index, e) in crosses.iter().enumerate() {
            if e.coordinate.z_coord < min_drill_z_coords {
                println!("deeper than min drill : {:?}", e);
                to_be_delete_index = index as i32;
            }
        }

        if to_be_delete_index as i32 != -1 {
            crosses.remove(to_be_delete_index as usize);
        }
    }
}

#[derive(Debug)]
pub struct CrossInformation {
    path: String,
    mining_type: String,
    seperator: String,
    duplicate_avoiding: bool,
}

impl CrossInformation {
    fn new(path: String, mining_type: String, seperator: String) -> CrossInformation {
        CrossInformation {
            path,
            mining_type,
            seperator,
            duplicate_avoiding: true,
        }
    }

    pub(crate) fn new_from_config(config: &LegoConfig) -> CrossInformation {
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
        let mut cross_id = 0;

        for line in lines {
            let record = line.expect("Failed to reading line of str file");
            cross_id += 1;

            if record.contains(&self.seperator) {
                group_no += 1;
            } else {
                let record: Vec<String> = record.split(",").map(|s| s.trim().to_string()).collect();

                // println!("record : {:?}", record);
                let x: f64 = record[1].parse().expect("x icin numerik deger çevirilemedi");
                let y: f64 = record[2].parse().expect("y icin numerik deger çevirilemedi");
                let z: f64 = record[3].parse().expect("z icin numerik deger çevirilemedi");

                let coord = CrossCoordinate::new(x, y, z);
                let cross = Cross::new(group_no, coord, cross_id);

                if cross_objects.contains(&cross) == false && self.duplicate_avoiding {
                    // avoiding duplicates
                    cross_objects.push(cross);
                }
            }
        }

        Ok(cross_objects)
    }
}

#[derive(Debug)]
struct Extent {
    min_x: f64,
    min_y: f64,
    min_z: f64,

    max_x: f64,
    max_y: f64,
    max_z: f64,

}

#[derive(Debug, PartialEq)]
struct Cross {
    group_no: i32,
    coordinate: CrossCoordinate,
    cross_id: i32,
}

impl Cross {
    fn new(group_no: i32, coordinate: CrossCoordinate, cross_id: i32) -> Cross {
        Cross {
            group_no,
            coordinate,
            cross_id,
        }
    }

    fn find_model_frame(crosses: &Vec<Cross>) -> Extent
    {
        let mut xs = Cross::give_all_xs(crosses);
        xs.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut ys = Cross::give_all_ys(crosses);
        ys.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut zs = Cross::give_all_zs(crosses);
        zs.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let min_x = xs.first().unwrap().clone();
        let min_y = ys.first().unwrap().clone();
        let min_z = zs.first().unwrap().clone();

        let max_x= xs.last().unwrap().clone();
        let max_y = ys.last().unwrap().clone();
        let max_z = zs.last().unwrap().clone();

        Extent {
            min_x,
            min_y,
            min_z,

            max_x,
            max_y,
            max_z
        }

    }

    fn find_two_cross_frame(&self, cross_two: &Cross) -> Extent {
        /*
        It is used to determine one rectangular frame consists two cross

        */
        let min_x: f64 = if self.coordinate.x_coord.le(&cross_two.coordinate.x_coord) {
            self.coordinate.x_coord.clone()
        } else {
            cross_two.coordinate.x_coord.clone()
        };

        let min_y: f64 = if self.coordinate.y_coord.le(&cross_two.coordinate.y_coord) {
            self.coordinate.y_coord.clone()
        } else {
            cross_two.coordinate.y_coord.clone()
        };
        let min_z: f64 = if self.coordinate.z_coord.le(&cross_two.coordinate.z_coord) {
            self.coordinate.z_coord.clone()
        } else {
            cross_two.coordinate.z_coord.clone()
        };

        let max_x: f64 = if self.coordinate.x_coord.ge(&cross_two.coordinate.x_coord) {
            self.coordinate.x_coord.clone()
        } else {
            cross_two.coordinate.x_coord.clone()
        };
        let max_y: f64 = if self.coordinate.y_coord.ge(&cross_two.coordinate.y_coord) {
            self.coordinate.y_coord.clone()
        } else {
            cross_two.coordinate.y_coord.clone()
        };
        let max_z: f64 = if self.coordinate.z_coord.ge(&cross_two.coordinate.z_coord) {
            self.coordinate.z_coord.clone()
        } else {
            cross_two.coordinate.z_coord.clone()
        };


        Extent {
            min_x,
            min_y,
            min_z,

            max_x,
            max_y,
            max_z
        }
    }

    fn give_all_xs(crosses: &Vec<Cross>) -> Vec<f64> {
        let mut v: Vec<f64> = vec![];
        for i in crosses {
            v.push(i.coordinate.x_coord)
        }

        v
    }

    fn give_all_ys(crosses: &Vec<Cross>) -> Vec<f64> {
        let mut v: Vec<f64> = vec![];
        for i in crosses {
            v.push(i.coordinate.y_coord)
        }

        v
    }

    fn give_all_zs(crosses: &Vec<Cross>) -> Vec<f64> {
        let mut v: Vec<f64> = vec![];
        for i in crosses {
            v.push(i.coordinate.z_coord)
        }

        v
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
            z_coord: z,
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

            // println!("before z: {}", c1.coordinate.z_coord);
            // println!("now z: {}", c2.coordinate.z_coord);

            assert!(c1.coordinate.z_coord >= c2.coordinate.z_coord);
        }
    }
}
