use std::collections::HashMap;
use std::error::Error;
use csv::{Reader, StringRecord};
use std::hash::Hash;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::string::ToString;
use lego_config::read::{LegoConfig, DataManagementObjects};

pub struct RawSampleObject {
    info: RawSampleInformation,
    data: Vec<RawSample>,
}

impl RawSampleObject {
    pub(crate) fn new(info: RawSampleInformation) -> RawSampleObject {
        let data = info.read().expect("Error occured while reading raw sample !");
        RawSampleObject {
            info,
            data,
        }
    }
}

pub struct RawSampleInformation {
    path: String,
    mining_type: String,
    seperator: char,
    columns: HashMap<String, String>,  // we'l change here
}

impl RawSampleInformation {
    fn new(path: String, mining_type: String, seperator: char,
           columns: HashMap<String, String>) -> RawSampleInformation {
        RawSampleInformation {
            path,
            mining_type,
            seperator,
            columns,
        }
    }

    pub(crate) fn new_from_config(config: &LegoConfig) -> RawSampleInformation {
        // getting mining information
        let path = config.get_rawsample_csv_path();
        let mining_type = config.get_mining_type();
        let seperator = config.get_x_seperator("rawsample_csv_seperator");
        let columns = config.get_x_columns("rawsample_columns");

        RawSampleInformation {
            path,
            mining_type,
            seperator,
            columns,
        }
    }

    fn read(&self) -> Result<Vec<RawSample>, Box<dyn Error>> {
        // our data
        let mut raw_sample_objects: Vec<RawSample> = vec![];

        let raw_sample_csv_path: &String = &self.path;
        let mut reader = Reader::from_path(raw_sample_csv_path)?;

        // columns
        reader.set_headers(StringRecord::from(vec!["SONDAJNO", "FROM", "TO", "PERCENT"]));

        let mut results = reader.records();

        // skip first row
        results.next();

        for result in results {
            let record = result?;
            let record: Vec<String> = record[0].to_string().split(";").map(|s| s.to_string()).collect();

            let drill_no = record[0].to_owned();
            let start: f64 = record[1].parse().expect("numerik deger çevirilemedi");
            let end: f64 = record[2].parse().expect("numerik deger çevirilemedi");
            let percent: f64 = record[3].parse().expect("numerik deger çevirilemedi");

            let rawsample_coordinate = RawSampleCoordinate::new(start, end, percent);
            let d_row = RawSample::new(drill_no, rawsample_coordinate);

            raw_sample_objects.push(d_row);
        }
        Ok(raw_sample_objects)
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Hash)]
enum RawSampleColumns {
    DRILLNO,
    FROM,
    TO,
    PERCENT,
}

#[derive(Debug)]
struct RawSample {
    drill_no: String,
    coordinate: RawSampleCoordinate,
}

impl RawSample {
    fn new(drill_no: String, coordinate: RawSampleCoordinate) -> RawSample {
        RawSample {
            drill_no,
            coordinate,
        }
    }
}

#[derive(Debug)]
struct RawSampleCoordinate {
    start: f64,
    end: f64,
    percent: f64,
}

impl RawSampleCoordinate {
    fn new(start: f64, end: f64, percent: f64) -> RawSampleCoordinate {
        RawSampleCoordinate {
            start,
            end,
            percent,
        }
    }
}

impl Display for RawSampleCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "start : {} \n\
                   end : {} \n\
                   percent: {}", self.start, self.end, self.percent)
    }
}

impl Display for RawSample {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "drill no : {} \n\
                   coordinate : {}", self.drill_no, self.coordinate)
    }
}

impl Display for RawSampleInformation {
    /*
    path: String,
    mining_type: String,
    seperator: char,
    columns: HashMap<String, String>,
    */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "path : {} \n\
                   mining type : {} \n\
                   seperator : {} \n\
                   columns : {:?}", self.path, self.mining_type, self.seperator, self.columns)
    }
}

impl Display for RawSampleObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "info: {} \n\
                   data : {:?}", self.info, self.data)
    }
}

#[cfg(test)]
mod tests {
    use crate::excels::rawsample_reader::{RawSampleInformation, RawSampleObject};
    use lego_config::read::LegoConfig;

    const TEST_CONFIG_PATH: &str = "/home/umut/CLionProjects/LegoRust/lego_config/test_settings.toml";

    #[test]
    fn read_rawsample_from_config() {
        let config_object = LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let rawsample_info = RawSampleInformation::new_from_config(&config_object);
        let l_object = RawSampleObject::new(rawsample_info);
        println!("lytology : {}", l_object);
    }
}