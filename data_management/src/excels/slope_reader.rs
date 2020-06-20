use std::collections::HashMap;
use std::error::Error;
use csv::{Reader, StringRecord};
use std::hash::Hash;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::string::ToString;

struct SlopeObject {
    info: SlopeInformation,
    data: Vec<Slope>,
}

impl SlopeObject {
    fn new(info: SlopeInformation, data: Vec<Slope>) -> SlopeObject {
        SlopeObject {
            info,
            data,
        }
    }
}

struct SlopeInformation {
    path: String,
    mining_type: String,
    seperator: String,
    columns: HashMap<SlopeColumns, String>,
}

impl SlopeInformation {
    fn new(path: String, mining_type: String, seperator: String,
           columns: HashMap<SlopeColumns, String>) -> SlopeInformation {
        SlopeInformation {
            path,
            mining_type,
            seperator,
            columns,
        }
    }

    fn read(&self) -> Result<Vec<Slope>, Box<dyn Error>> {
        // our data
        let mut Slope_objects: Vec<Slope> = vec![];

        let Slope_csv_path: &String = &self.path;
        let mut reader = Reader::from_path(Slope_csv_path)?;

        // columns
        reader.set_headers(StringRecord::from(vec!["SONDAJNO", "DERINLIK", "DALIM", "AZIMUTH"]));

        let mut results = reader.records();

        // skip first row
        results.next();

        for result in results {
            let record = result?;
            let record: Vec<String> = record[0].to_string().split(";").map(|s| s.to_string()).collect();

            let drill_no = record[0].to_owned();
            let depth: f64 = record[1].parse().expect("DEPTH numerik deger çevirilemedi");
            let dip: f64 = record[2].parse().expect("DIP numerik deger çevirilemedi");
            let azimuth: f64 = record[3].parse().expect("AZIMUTH numerik deger çevirilemedi");

            // todo : bu castinglere bakalım
            let slope_coordinate = DrillSlopeInfo::new(depth, dip as i32, azimuth as i32);
            let d_row = Slope::new(drill_no, slope_coordinate);

            Slope_objects.push(d_row);
        }
        Ok(Slope_objects)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum SlopeColumns {
    DRILLNO,
    DEPTH,
    DIP,
    AZIMUTH,
}

#[derive(Debug)]
struct DrillSlopeInfo {
    depth: f64,
    dalim: i32,
    azimuth: i32,
}

impl DrillSlopeInfo {
    fn new(depth: f64, dalim: i32, azimuth: i32) -> DrillSlopeInfo {
        DrillSlopeInfo {
            depth,
            dalim,
            azimuth,
        }
    }
}

impl Display for DrillSlopeInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "depth : {} \n\
                   dip : {} \n\
                   azimuth : {}", self.depth, self.dalim, self.azimuth)
    }
}

#[derive(Debug)]
struct Slope {
    drill_no: String,
    drill_info: DrillSlopeInfo,
}

impl Slope {
    fn new(drill_no: String, drill_info: DrillSlopeInfo) -> Slope {
        Slope {
            drill_no,
            drill_info,
        }
    }
}

impl Display for Slope {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "drill no : {} \n\
                   drill info : {}", self.drill_no, self.drill_info)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::excels::slope_reader::{SlopeColumns, SlopeInformation, SlopeObject};

    #[test]
    fn running_test() {
        // path
        let slope_csv_path = String::from(r"C:\Users\umut\CLionProjects\LegoRust\tests\data\excels4\hamorneklem.csv");

        // columns
        let mut columns: HashMap<SlopeColumns, String> = HashMap::new();

        columns.insert(SlopeColumns::DRILLNO, "SONDAJNO".to_string());
        columns.insert(SlopeColumns::DEPTH, "DEPTH".to_string());
        columns.insert(SlopeColumns::DIP, "DIP".to_string());
        columns.insert(SlopeColumns::AZIMUTH, "AZIMUTH".to_string());

        // data
        let raw_sample_information = SlopeInformation::new(slope_csv_path,
                                                           "Cu".to_string(), ";".to_string(), columns);
        let raw_sample_objects = raw_sample_information.read().expect("Slope file cannot be read and parsed !");
        println!("Slope rows : {:?}", raw_sample_objects);

        let raw_sample_object = SlopeObject::new(raw_sample_information, raw_sample_objects);
    }
}