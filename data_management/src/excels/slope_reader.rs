use std::collections::HashMap;
use std::error::Error;
use csv::{Reader, StringRecord};
use std::hash::Hash;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::string::ToString;
use lego_config::read::{LegoConfig, DataManagementObjects};

#[derive(Debug)]
pub struct SlopeObject {
    info: SlopeInformation,
    data: Vec<Slope>,
}

impl SlopeObject {
    pub(crate) fn new(info: SlopeInformation) -> SlopeObject {
        let data = info.read().expect("Something went wrong while reading slope csv !");
        SlopeObject {
            info,
            data,
        }
    }
}

#[derive(Debug)]
pub struct SlopeInformation {
    path: String,
    mining_type: String,
    seperator: char,
    columns: HashMap<String, String>,  // we'll fix this later
}

impl SlopeInformation {
    fn new(path: String, mining_type: String, seperator: char,
           columns: HashMap<String, String>) -> SlopeInformation {
        SlopeInformation {
            path,
            mining_type,
            seperator,
            columns,
        }
    }

    pub(crate) fn new_from_config(config: &LegoConfig) -> SlopeInformation {
        let path = config.get_slope_csv_path();
        let mining_type = config.get_mining_type();
        let seperator = config.get_x_seperator("slope_csv_seperator");
        let columns = config.get_x_columns("slope_columns");

        SlopeInformation {
            path,
            mining_type,
            seperator,
            columns
        }
    }
    fn read(&self) -> Result<Vec<Slope>, Box<dyn Error>> {
        // our data
        let mut slope_objects: Vec<Slope> = vec![];

        let slope_csv_path: &String = &self.path;
        let mut reader = Reader::from_path(slope_csv_path)?;

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

            slope_objects.push(d_row);
        }
        Ok(slope_objects)
    }
}

#[allow(unused)]
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
    use crate::excels::slope_reader::{SlopeInformation, SlopeObject};
    use lego_config::read::LegoConfig;

    const TEST_CONFIG_PATH: &str = "/home/umut/CLionProjects/LegoRust/lego_config/test_settings.toml";

    #[test]
    fn read_slope_from_config () {
        let config_object = LegoConfig::new(String::from(TEST_CONFIG_PATH));
        let slope_info = SlopeInformation::new_from_config(&config_object);

        let s_object = SlopeObject::new(slope_info);
        println!("slope object : {:?}", s_object);
    }
}