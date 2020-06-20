use std::collections::HashMap;
use std::error::Error;
use csv::{Reader, StringRecord};
use std::hash::Hash;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::string::ToString;

struct RawSampleObject {
    info: RawSampleInformation,
    data: Vec<RawSample>,
}

impl RawSampleObject {
    fn new(info: RawSampleInformation, data: Vec<RawSample>) -> RawSampleObject {
        RawSampleObject {
            info,
            data,
        }
    }
}

struct RawSampleInformation {
    path: String,
    mining_type: String,
    seperator: String,
    columns: HashMap<RawSampleColumns, String>,
}

impl RawSampleInformation {
    fn new(path: String, mining_type: String, seperator: String,
           columns: HashMap<RawSampleColumns, String>) -> RawSampleInformation {
        RawSampleInformation {
            path,
            mining_type,
            seperator,
            columns,
        }
    }

    fn read(&self) -> Result<Vec<RawSample>, Box<dyn Error>> {
        // our data
        let mut RawSample_objects: Vec<RawSample> = vec![];

        let RawSample_csv_path: &String = &self.path;
        let mut reader = Reader::from_path(RawSample_csv_path)?;

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

            RawSample_objects.push(d_row);
        }
        Ok(RawSample_objects)
    }
}

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

impl Display for RawSample {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "drill no : {} \n\
                   coordinate : {}", self.drill_no, self.coordinate)
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

#[cfg(test)]
mod tests {
    use crate::excels::rawsample_reader::{RawSampleColumns, RawSampleInformation, RawSampleObject};
    use std::collections::HashMap;

    #[test]
    fn running_test() {
        // path
        let RawSample_csv_path = String::from(r"C:\Users\umut\CLionProjects\LegoRust\tests\data\excels4\hamorneklem.csv");

        // columns
        let mut columns: HashMap<RawSampleColumns, String> = HashMap::new();

        columns.insert(RawSampleColumns::DRILLNO, "SONDAJNO".to_string());
        columns.insert(RawSampleColumns::FROM, "FROM".to_string());
        columns.insert(RawSampleColumns::TO, "TO".to_string());
        columns.insert(RawSampleColumns::PERCENT, "PERCENT".to_string());

        // data
        let raw_sample_information = RawSampleInformation::new(RawSample_csv_path,
                                                               "Cu".to_string(), ";".to_string(), columns);
        let raw_sample_objects = raw_sample_information.read().expect("RawSample file cannot be read and parsed !");
        println!("RawSample rows : {:?}", raw_sample_objects);

        let raw_sample_object = RawSampleObject::new(raw_sample_information, raw_sample_objects);
    }
}