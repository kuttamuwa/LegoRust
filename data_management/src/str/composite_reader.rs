use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::ToString;

use lego_config::read::{DataManagementObjects, LegoConfig};

struct CompositeObject {
    info: CompositeInformation,
    data: Vec<Composite>,
}

impl CompositeObject {
    fn new(info: CompositeInformation) -> CompositeObject {
        let data = info.read().expect(
            "Error occurs while reading composite !"
        );
        CompositeObject {
            info,
            data,
        }
    }
}

struct CompositeInformation {
    path: String,
    mining_type: String,
    seperator: char,
}

impl CompositeInformation {
    fn new(path: String, mining_type: String, seperator: char) -> CompositeInformation {
        CompositeInformation {
            path,
            mining_type,
            seperator,
        }
    }

    fn new_from_config(config: &LegoConfig) -> CompositeInformation {
        // getting mining information
        let path = config.get_composite_str_path();
        let mining_type = config.get_mining_type();
        let seperator = config.get_x_seperator("composite_str_seperator");

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
           cut_taken: f64, coordinate: CompositeCoordinate) -> Composite {
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
        /*
         group_no: i32,
    tenor: f64,
    drill_no: String,
    cut_from: f64,
    cut_end: f64,
    cut_taken: f64,
    coordinate: CompositeCoordinate,
    */

        write!(f, "group no: {} \n\
                   tenor : {} \n\
                   drill no : {} \n\
                   cut from : {} \n\
                   cut end : {} \n\
                   cut taken : {} \n\
                   coordinate : {}", self.group_no, self.tenor, self.drill_no, self.cut_from,
               self.cut_end, self.cut_taken, self.coordinate)
    }
}

impl Display for CompositeInformation {
    /*
    path: String,
    mining_type: String,
    seperator: String,
    */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "path: {} \n\
                   mining type : {} \n\
                   seperator : {}", self.path, self.mining_type, self.seperator)
    }
}

impl Display for CompositeObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        //     info: CompositeInformation,
        //     data: Vec<Composite>,

        write!(f, "info: {} \n\
                   data : {:?}", self.info, self.data)
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

impl Display for CompositeCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x : {} \n\
                   y : {} \n\
                   z : {}", self.x_coord, self.y_coord, self.z_coord)
    }
}


#[cfg(test)]
mod tests {
    use lego_config::read::LegoConfig;

    use crate::str::composite_reader::{CompositeInformation, CompositeObject};

    const TEST_CONFIG_PATH: &str = "/home/umut/CLionProjects/LegoRust/lego_config/test_settings.toml";

    #[test]
    fn read_composite_from_config() {
        let config_object = LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let composite_info = CompositeInformation::new_from_config(&config_object);
        let l_object = CompositeObject::new(composite_info);
        println!("composite : {}", l_object);
    }
}
