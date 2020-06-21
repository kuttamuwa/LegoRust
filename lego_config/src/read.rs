#[allow(unused_imports)]

use std::collections::HashMap;
use std::fmt;
use config::{Config, File, Value};

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    const TEST_CONFIG_PATH: &str = "/home/umut/CLionProjects/LegoRust/lego_config/path_config/test_settings.toml";
    use crate::read::{LConfig, ConfigPathEnums};
    use config::{Config, File, Value};

   //  #[test]
   //  fn create_and_read_sections() {
   //      let csv_section_name = String::from("excel_paths");
   //      let str_section_name = String::from("str_paths");
   // }

    #[test]
    fn create_csv_object() {
       let csv_section_name = String::from("excel_paths");
       let str_section_name = String::from("str_paths");
       let csv_object = LConfig::new(TEST_CONFIG_PATH, csv_section_name, str_section_name);

       csv_object.create_csvpath_object();

   }
}

pub struct CSVPath {
    // this naming convention will be exactly same with config !
    slope_csv_path: String,
    lythology_csv_path: String,
    rawsample_csv_path: String,
    drill_csv_path: String
}

impl CSVPath {
    pub fn new(drill_csv_path: String, lythology_csv_path: String, slope_csv_path: String, rawsample_csv_path: String,) -> CSVPath {
        CSVPath {
            slope_csv_path,
            lythology_csv_path,
            rawsample_csv_path,
            drill_csv_path
        }
    }
}
pub struct STRPath {
    // this naming convention will be exactly same with config !
    cross_section_path: String,
    composite_path: String
}

impl STRPath {
    fn new(cross_section_path: String, composite_path: String) -> STRPath {
        STRPath {
            cross_section_path,
            composite_path
        }
    }
}

pub trait ConfigPathEnums {
    fn new(path: &str, csv_section_name: String, str_section_name: String) -> Self;

    fn read_section(&self, section_name: &str) -> HashMap<String, Value>;

    fn create_rust_config_object(path: &str) -> Config;  // we will not code config library, customize instead.

    // fn read_excel_and_str_config(&self) -> (CSVPath, STRPath);
    // I added for easy usage. You will add more function like below.

    // new data types functions will be added here !
    fn create_csvpath_object(&self)  -> CSVPath;

    fn create_strpath_object(&self) -> STRPath;
}

// trait ConfigErrors {
//     fn csvpath_cannot_be_empty () -> String {
//         "CSVPATH CANNOT BE EMPTY".to_string()
//     }
//
//     fn strpath_cannot_be_empty() -> String {
//         "STRPATH CANNOT BE EMPTY".to_string()
//     }
//
//     fn section_cannot_be_found(section_name: &str) -> &'static str {
//         format!("{} SECTION CANNOT BE FOUND. PLEASE CHECK YOUR CONFIG !", section_name).as_str()
//         // "SECTION CANNOT BE FOUND. PLEASE CHECK YOUR CONFIG !"
//     }
//
//     fn path_cannot_be_found(variable_name: &str) -> String {
//         format!("{} PATH CANNOT BE FOUND. PLEASE CHECK YOUR CONFIG !", variable_name)
//     }
//
//     fn path_must_be_string() -> String {
//         "PATH MUST BE STRING !".to_string()
//     }
// }


#[derive(Debug)]
struct LConfig {
    object: Config,
    csv_section_name: String,
    str_section_name: String
}

impl ConfigPathEnums for LConfig {
    fn new(path: &str, csv_section_name: String, str_section_name: String) -> LConfig {
        let config_object = LConfig::create_rust_config_object(path);

        LConfig {
            object: config_object,
            csv_section_name,
            str_section_name
        }
    }

    fn read_section(&self, section_name: &str) -> HashMap<String, Value> {
        self.object.get_table(section_name)
            .expect("")
    }

    fn create_rust_config_object(path: &str) -> Config {
        let mut s = Config::new();
        s.merge(File::with_name(path));

        s
    }

    // fn read_excel_and_str_config(&self) -> (CSVPath, STRPath){
    //     (self.create_csvpath_object(), self.create_strpath_object())
    // }

    fn create_csvpath_object(&self) -> CSVPath
    {
        // todo: This will be fixed after ConfigErrors trait will be done
        let csv_path_table = &self.object.get_table(&self.csv_section_name)
            .unwrap();

        // slope_csv_path: String, lythology_csv_path: String, rawsample_csv_path: String,
        //     drill_csv_path: String
        println!("csv path table : {:?}", csv_path_table);
        let drill_csv_path = csv_path_table.get("drill_csv_path")
            .expect("asdasda").kind.to_string();

        let lythology_csv_path = csv_path_table.get("lythology_csv_path")
           .expect("asdasda").kind.to_string();

        let slope_csv_path = csv_path_table.get("slope_csv_path")
            .expect("asdasda").kind.to_string();

        let rawsample_csv_path = csv_path_table.get("rawsample_csv_path")
            .expect("asdasda").kind.to_string();

        CSVPath::new(
            drill_csv_path,
            lythology_csv_path,
            slope_csv_path,
            rawsample_csv_path
        )
    }

    fn create_strpath_object(&self) -> STRPath {
        let str_path_table = &self.object.get_table(&self.str_section_name)
            .unwrap();

        // cross_section_path: String, composite_path: String
        let cross_section_path = str_path_table.get("cross_section_path")
            .expect("asdasda").kind.to_string();

        let composite_path = str_path_table.get("composite_path")
            .expect("asdasda").kind.to_string();

        STRPath::new(
            cross_section_path,
            composite_path
        )
    }
}


impl fmt::Display for LConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "str section name: {:?} \n\
                   csv section name: {:?} \n\
                   config object : ", self.str_section_name, self.csv_section_name)
    }
}