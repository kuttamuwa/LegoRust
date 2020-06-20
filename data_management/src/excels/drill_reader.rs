use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::hash::Hash;
use std::string::ToString;

use csv::{Reader, StringRecord};

pub struct DrillObject {
    info: DrillInformation,
    data: Vec<Drill>,
}

impl DrillObject {
    fn new(info: DrillInformation, data: Vec<Drill>) -> DrillObject {
        DrillObject {
            info,
            data,
        }
    }
}


struct DrillInformation {
    path: String,
    mining_type: String,
    seperator: String,
    columns: HashMap<DrillColumns, String>,
}

impl DrillInformation {
    fn new(path: String, mining_type: String, seperator: String,
           columns: HashMap<DrillColumns, String>) -> DrillInformation {
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

impl Display for Drill {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "drill no : {} \n\
                   coordinate : {}", self.drill_no, self.coordinate)
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

#[cfg(test)]
mod tests {
    use crate::excels::drill_reader::{DrillColumns, DrillInformation, DrillObject};
    use std::collections::HashMap;

    #[test]
    fn running_test() {
        // path
        let drill_csv_path = String::from(r"/home/umut/CLionProjects/LegoRust/tests/data/excels4/sondaj.csv");

        // columns
        let mut columns: HashMap<DrillColumns, String> = HashMap::new();

        columns.insert(DrillColumns::DRILLNO, "SONDAJNO".to_string());
        columns.insert(DrillColumns::X, "X".to_string());
        columns.insert(DrillColumns::Y, "Y".to_string());
        columns.insert(DrillColumns::Z, "Z".to_string());
        columns.insert(DrillColumns::DEPTH, "DERINLIK".to_string());

        // data
        let drill_information = DrillInformation::new(drill_csv_path,
                                                      "Cu".to_string(), ";".to_string(), columns);
        let drill_vectors = drill_information.read().expect("Drill file cannot be read and parsed !");
        println!("drill rows : {:?}", drill_vectors);

        let drill_object = DrillObject::new(drill_information, drill_vectors);

        // println!("Drill object : {}", drill_object);
    }
}