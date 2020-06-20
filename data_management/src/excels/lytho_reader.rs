use std::collections::HashMap;
use std::error::Error;
use csv::{Reader, StringRecord};
use std::hash::Hash;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::string::ToString;

struct LythologyObject {
    info: LythologyInformation,
    data: Vec<Lythology>,
}

impl LythologyObject {
    fn new(info: LythologyInformation, data: Vec<Lythology>) -> LythologyObject {
        LythologyObject {
            info,
            data,
        }
    }
}

struct LythologyInformation {
    path: String,
    mining_type: String,
    seperator: String,
    columns: HashMap<LythologyColumns, String>,
}

impl LythologyInformation {
    fn new(path: String, mining_type: String, seperator: String,
           columns: HashMap<LythologyColumns, String>) -> LythologyInformation {
        LythologyInformation {
            path,
            mining_type,
            seperator,
            columns,
        }
    }

    fn read(&self) -> Result<Vec<Lythology>, Box<dyn Error>> {
        // our data
        let mut Lythology_objects: Vec<Lythology> = vec![];

        let Lythology_csv_path: &String = &self.path;
        let mut reader = Reader::from_path(Lythology_csv_path)?;

        // columns
        reader.set_headers(StringRecord::from(vec!["SONDAJNO", "FROM", "TO", "LYTHO"]));

        let mut results = reader.records();

        // skip first row
        results.next();

        for result in results {
            let record = result?;
            let record: Vec<String> = record[0].to_string().split(";").map(|s| s.to_string()).collect();

            let drill_no = record[0].to_owned();
            let start: f64 = record[1].parse().expect("numerik deger çevirilemedi");
            let end: f64 = record[2].parse().expect("numerik deger çevirilemedi");
            let LYTHO: f64 = record[3].parse().expect("numerik deger çevirilemedi");

            let Lythology_coordinate = LythologyCoordinate::new(start, end, LYTHO);
            let d_row = Lythology::new(drill_no, Lythology_coordinate);

            Lythology_objects.push(d_row);
        }
        Ok(Lythology_objects)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum LythologyColumns {
    DRILLNO,
    FROM,
    TO,
    LYTHO,
}

#[derive(Debug)]
struct Lythology {
    drill_no: String,
    coordinate: LythologyCoordinate,
}

impl Lythology {
    fn new(drill_no: String, coordinate: LythologyCoordinate) -> Lythology {
        Lythology {
            drill_no,
            coordinate,
        }
    }
}

impl Display for Lythology {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "drill no : {} \n\
                   coordinate : {}", self.drill_no, self.coordinate)
    }
}

#[derive(Debug)]
struct LythologyCoordinate {
    start: f64,
    end: f64,
    lytho: f64,
}

impl LythologyCoordinate {
    fn new(start: f64, end: f64, lytho: f64) -> LythologyCoordinate {
        LythologyCoordinate {
            start,
            end,
            lytho,
        }
    }
}

impl Display for LythologyCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "start : {} \n\
                   end : {} \n\
                   LYTHO: {}", self.start, self.end, self.lytho)
    }
}

#[cfg(test)]
mod tests {
    use crate::excels::lytho_reader::{LythologyColumns, LythologyInformation, LythologyObject};
    use std::collections::HashMap;

    #[test]
    fn running_test() {
        // path
        let Lythology_csv_path = String::from(r"C:\Users\umut\CLionProjects\LegoRust\tests\data\excels4\hamorneklem.csv");

        // columns
        let mut columns: HashMap<LythologyColumns, String> = HashMap::new();

        columns.insert(LythologyColumns::DRILLNO, "SONDAJNO".to_string());
        columns.insert(LythologyColumns::FROM, "FROM".to_string());
        columns.insert(LythologyColumns::TO, "TO".to_string());
        columns.insert(LythologyColumns::LYTHO, "LYTHO".to_string());

        // data
        let raw_sample_information = LythologyInformation::new(Lythology_csv_path,
                                                               "Cu".to_string(), ";".to_string(), columns);
        let raw_sample_objects = raw_sample_information.read().expect("Lythology file cannot be read and parsed !");
        println!("Lythology rows : {:?}", raw_sample_objects);

        let raw_sample_object = LythologyObject::new(raw_sample_information, raw_sample_objects);
    }
}