use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::hash::Hash;
use std::string::ToString;

use csv::{Reader, StringRecord};
use lego_config::read::{LegoConfig, DataManagementObjects};

pub struct DrillObject {
    info: DrillInformation,
    data: Vec<Drill>,
}

impl DrillObject {
    fn new(info: DrillInformation) -> DrillObject {
        let data = info.read().unwrap();
        DrillObject {
            info,
            data,
        }
    }
}


struct DrillInformation {
    path: String,
    mining_type: String,
    seperator: char,
    columns: HashMap<String, String>,  // todo sonra hallederim
}

impl DrillInformation {
    fn new(path: String, mining_type: String, seperator: char,
           columns: HashMap<String, String>) -> DrillInformation {
        DrillInformation {
            path,
            mining_type,
            seperator,
            columns,
        }
    }

    fn read(&self) -> Result<Vec<Drill>, Box<dyn Error>> {
        // our data
        let mut drill_objects: Vec<Drill> = vec![];

        let drill_csv_path: &String = &self.path;
        let mut reader = Reader::from_path(drill_csv_path)?;

        // columns
        reader.set_headers(StringRecord::from(vec!["SONDAJNO", "X", "Y", "Z", "DERINLIK"]));

        let mut results = reader.records();

        // skip first row
        results.next();

        for result in results {
            let record = result?;
            let record: Vec<String> = record[0].to_string().split(";").map(|s| s.to_string()).collect();

            let drill_no = record[0].to_owned();
            let x: f64 = record[1].parse().expect("numerik deger çevirilemedi");
            let y: f64 = record[2].parse().expect("numerik deger çevirilemedi");
            let z: f64 = record[3].parse().expect("numerik deger çevirilemedi");
            let depth: f64 = record[4].parse().expect("numerik deger çevirilemedi");

            let drill_coordinate = DrillCoordinate::new(x, y, z, depth);
            let d_row = Drill::new(drill_no, drill_coordinate);

            drill_objects.push(d_row);
        }
        Ok(drill_objects)
    }

    fn new_from_config(config: &LegoConfig) -> DrillInformation {
        // getting mining information
        let path = config.get_drill_csv_path();
        let mining_type = config.get_mining_type();
        let seperator = config.get_x_seperator("drill_csv_seperator");
        let columns = config.get_x_columns("drill_columns");

        DrillInformation {
            path,
            mining_type,
            seperator,
            columns
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum DrillColumns {
    DRILLNO,
    X,
    Y,
    Z,
    DEPTH,
}

#[derive(Debug)]
struct Drill {
    drill_no: String,
    coordinate: DrillCoordinate,
}

impl Drill {
    fn new(drill_no: String, coordinate: DrillCoordinate) -> Drill {
        Drill {
            drill_no,
            coordinate,
        }
    }
}

#[derive(Debug)]
struct DrillCoordinate {
    x_coord: f64,
    y_coord: f64,
    z_coord: f64,
    depth: f64,
}

impl DrillCoordinate {
    fn new(x: f64, y: f64, z: f64, depth: f64) -> DrillCoordinate {
        DrillCoordinate {
            x_coord: x,
            y_coord: y,
            z_coord: z,
            depth,
        }
    }
}

impl Display for DrillCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x : {} \n\
                   y : {} \n\
                   z : {}", self.x_coord, self.y_coord, self.z_coord)
    }
}

impl Display for DrillInformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "path : {} \n\
                   mining type : {} \n\
                   seperator : {} \n\
                   columns: {:?}", self.path, self.mining_type, self.seperator, self.columns)
    }
}

impl Display for DrillObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Information : {} \n\
                   Drills : {:?}", self.info, self.data)
    }
}

impl Display for Drill {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "drill no : {} \n\
                   coordinate : {}", self.drill_no, self.coordinate)
    }
}

#[cfg(test)]
mod tests {
    use crate::excels::drill_reader::{DrillColumns, DrillInformation, DrillObject};
    use std::collections::HashMap;
    use lego_config::read::{LegoConfig, DataManagementObjects};

    const TEST_CONFIG_PATH: &str = "/home/umut/CLionProjects/LegoRust/lego_config/test_settings.toml";

    #[test]
    fn creating_drill_object() {
        // path
        let drill_csv_path = String::from(r"/home/umut/CLionProjects/LegoRust/tests/data/excels4/sondaj.csv");

        // columns
        let mut columns: HashMap<String, String> = HashMap::new();

        columns.insert("DRILLNO".to_string(), "SONDAJNO".to_string());
        columns.insert("X".to_string(), "X".to_string());
        columns.insert("Y".to_string(), "Y".to_string());
        columns.insert("Z".to_string(), "Z".to_string());
        columns.insert("DEPTH".to_string(), "DERINLIK".to_string());

        // data
        let drill_information = DrillInformation::new(drill_csv_path,
                                                      "Cu".to_string(), ';', columns);

        let drill_object = DrillObject::new(drill_information);

        println!("Drill object : {}", drill_object);
    }

    #[test]
    fn creating_drill_object_from_config() {
        // config path
        let config_object = LegoConfig::new(String::from(TEST_CONFIG_PATH));
        let d_info = DrillInformation::new_from_config(&config_object);

        let drill_object = DrillObject::new(d_info);
        println!("drill object : {}", drill_object);

    }
}