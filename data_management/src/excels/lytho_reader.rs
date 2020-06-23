use std::collections::HashMap;
use std::error::Error;
use csv::{Reader, StringRecord};
use std::hash::Hash;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::string::ToString;
use lego_config::read::{LegoConfig, DataManagementObjects};

pub struct LythologyObject {
    info: LythologyInformation,
    data: Vec<Lythology>,
}

impl LythologyObject {
    fn new(info: LythologyInformation) -> LythologyObject {
        let data = info.read().expect("Lythology excel okunurken bir hata oluştu !");

        LythologyObject {
            info,
            data,
        }
    }
}

struct LythologyInformation {
    path: String,
    mining_type: String,
    seperator: char,
    columns: HashMap<String, String>, // todo: we will change here to enum
}

impl LythologyInformation {
    fn new(path: String, mining_type: String, seperator: char,
           columns: HashMap<String, String>) -> LythologyInformation {
        LythologyInformation {
            path,
            mining_type,
            seperator,
            columns,
        }
    }

    fn new_from_config(config: &LegoConfig) -> LythologyInformation {
        // getting mining information
        let path = config.get_lythology_csv_path();
        let mining_type = config.get_mining_type();
        let seperator = config.get_x_csv_seperator("lythology_csv_path");
        let columns = config.get_x_columns("lythology_columns");

        LythologyInformation {
            path,
            mining_type,
            seperator,
            columns
        }
    }

    fn read(&self) -> Result<Vec<Lythology>, Box<dyn Error>> {
        // our data
        let mut lythology_objects: Vec<Lythology> = vec![];

        let lythology_csv_path: &String = &self.path;
        let mut reader = Reader::from_path(lythology_csv_path)?;

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
            let lytho: String = record[3].parse().expect("numerik deger çevirilemedi");

            let lythology_coordinate = LythologyCoordinate::new(start, end, lytho);
            let d_row = Lythology::new(drill_no, lythology_coordinate);

            lythology_objects.push(d_row);
        }
        Ok(lythology_objects)
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



#[derive(Debug)]
struct LythologyCoordinate {
    start: f64,
    end: f64,
    lytho: String,
}

impl LythologyCoordinate {
    fn new(start: f64, end: f64, lytho: String) -> LythologyCoordinate {
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

impl Display for Lythology {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "drill no : {} \n\
                   coordinate : {}", self.drill_no, self.coordinate)
    }
}

impl Display for LythologyObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "info : {} \n\
                   data : {:?}", self.info, self.data)
    }
}

impl Display for LythologyInformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "path : {} \n\
                   mining type : {} \n\
                   seperator : {} \n\
                   columns : {:?}", self.path, self.mining_type, self.seperator, self.columns)
    }
}

#[cfg(test)]
mod tests {
    use crate::excels::lytho_reader::{LythologyColumns, LythologyInformation, LythologyObject};
    use std::collections::HashMap;
    use lego_config::read::LegoConfig;

    const TEST_CONFIG_PATH: &str = "/home/umut/CLionProjects/LegoRust/lego_config/test_settings.toml";

    #[test]
    fn read_lythology_from_config() {
        let config_object = LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let lytho_info = LythologyInformation::new_from_config(&config_object);
        let l_object = LythologyObject::new(lytho_info);
        println!("lytology : {}", l_object);

    }
}