use config::{Config, File, Value};
use std::collections::HashMap;
use std::fmt::{Debug, Formatter, Error, Display};
use std::fmt;

const TEST_CONFIG_PATH: &str = "/home/umut/CLionProjects/LegoRust/lego_config/test/test_settings.toml";

#[cfg(test)]
mod tests {
    use crate::{CSVPath, LConfig, TEST_CONFIG_PATH};

    #[test]
    fn create_lconfig_object() {
        let lc = LConfig::new(TEST_CONFIG_PATH);
    }

    #[test]
    fn create_csvpath_object() {
        let lc = LConfig::new(TEST_CONFIG_PATH);
        let csv_object = CSVPath::new(&lc);

        println!("csv object : {:?}", csv_object);
    }
}

#[derive(Debug)]
struct CSVPath {
    slope_csv_path: String,
    lythology_csv_path: String,
    rawsample_csv_path: String,
    drill_csv_path: String,
}

impl CSVPath {
    fn new(object: &LConfig) -> CSVPath {
        let (slope_csv_path,
            lythology_csv_path,
            rawsample_csv_path,
            drill_csv_path) = object.get_all_csvs_from_path_table();

        CSVPath {
            slope_csv_path,
            lythology_csv_path,
            rawsample_csv_path,
            drill_csv_path,
        }
    }
}

#[derive(Debug)]
struct LConfig {
    object: Config,
    path_table: HashMap<String, Value>,
}

impl LConfig {
    fn new(path: &str) -> LConfig {
        let object = crate::LConfig::create_config_object(path);
        let path_table = object.get_table("paths")
            .expect(format!("{} config file has to have PATHS section !", TEST_CONFIG_PATH).as_ref());
        LConfig {
            object,
            path_table,
        }
    }

    fn create_config_object(config_path: &str) -> Config {
        let mut s = Config::new();
        s.merge(File::with_name(config_path));

        s
    }

    fn get_all_csvs_from_path_table(&self) -> (String, String, String, String) {
        (self.path_table.get("slope_csv_path").unwrap().kind.to_string(),
         self.path_table.get("lythology_csv_path").unwrap().kind.to_string(),
         self.path_table.get("rawsample_csv_path").unwrap().kind.to_string(),
         self.path_table.get("drill_csv_path").unwrap().kind.to_string(),
        )
    }
}

impl Display for LConfig {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "path table : {:?} ", self.path_table)
    }
}