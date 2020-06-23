use config::{Config, File, Value};
use std::collections::HashMap;
use std::path::Path;

pub struct LegoConfig {
    path: String,
    config_object: Config
}

impl LegoConfig {
    pub fn new(path: String) -> LegoConfig {
        let _b = std::path::Path::new(&path).exists();

        match Path::new(&path).exists() {
            true => {
                let mut config_object = Config::new();
                config_object.merge(File::with_name(&path));

                LegoConfig {
                    path,
                    config_object
                }

            } false => {
                panic!("Config file cannot be found !")
            }
        }

    }

    fn refresh_config(&mut self) {
        &self.config_object.refresh();
    }
}

pub trait DataManagementObjects {
    fn get_section(&self, section_name: &str) -> HashMap<String, Value>;

    fn get_mining_information(&self) -> HashMap<String, Value> {
        let mining_information_section: HashMap<String, Value> = self.get_section("mining_information");
        mining_information_section
    }

    fn get_excel_paths(&self) -> HashMap<String, Value> {
        let excel_path_section: HashMap<String, Value> = self.get_section("excel_paths");
        excel_path_section
    }

    fn get_str_paths(&self) -> HashMap<String, Value> {
        let str_path_section: HashMap<String, Value> = self.get_section("str_paths");
        str_path_section
    }

    fn get_drill_column_section(&self) -> HashMap<String, Value> {
        let column_section: HashMap<String, Value> = self.get_section("drill_columns");
        column_section
    }

    fn get_drill_csv_path(&self) -> String {
        let excel_section = self.get_excel_paths();
        let drill_csv_path = excel_section.get("drill_csv_path")
            .expect("drill csv path cannot be found !").kind.to_string();

        drill_csv_path
    }

    fn get_lythology_csv_path(&self) -> String {
        let excel_section = self.get_excel_paths();
        let lyth_csv_path = excel_section.get("lythology_csv_path")
            .expect("lythology csv path cannot be found !").kind.to_string();

        lyth_csv_path
    }

    fn get_rawsample_csv_path(&self) -> String {
        let excel_section = self.get_excel_paths();
        let rawsample_csv_path = excel_section.get("rawsample_csv_path")
            .expect("rawsample csv path cannot be found !").kind.to_string();

        rawsample_csv_path
    }

    fn get_slope_csv_path(&self) -> String {
        let excel_section = self.get_excel_paths();
        let slope_csv_path = excel_section.get("slope_csv_path")
            .expect("slope csv path cannot be found !").kind.to_string();

        slope_csv_path
    }

    fn get_x_columns(&self, section_name: &str) -> HashMap<String, String> {
        let column_section = self.get_section(section_name);

        // convert Value to String
        let mut columns: HashMap<String, String> = HashMap::new();
        for (k, v) in column_section.iter() {
            columns.insert(k.clone(), v.kind.to_string());
        }

        columns
    }

    fn get_mining_type(&self) -> String {
        let m_section = self.get_mining_information();
        let mining_type = m_section.get("mining_type")
            .expect("Mining type cannot be found !").kind.to_string();

        mining_type
    }

    fn get_x_csv_seperator(&self, excel_name: &str) -> char {
        // default is ;
        let m_section = &self.get_mining_information();
        let csv_seperator = m_section.get(excel_name);

        match csv_seperator {
            Some(t) => {
                let s = t.kind.to_string();
                let k = s.chars().last().unwrap();
                k
            },  // length cannot be more than 1 => char !
            None => ';'
        }
    }


}

impl DataManagementObjects for LegoConfig {
    fn get_section(&self, section_name: &str) -> HashMap<String, Value> {
        let table = &self.config_object.get_table(section_name)
            .expect(format!("section cannot be found ! {}", section_name).as_str());
        table.clone()
    }
}


mod tests {
    use super::{LegoConfig, DataManagementObjects};
    const TEST_SETTING: &str = r"/home/umut/CLionProjects/LegoRust/lego_config/test_settings.toml";

    #[test]
    fn create_config() {
        let p = String::from(TEST_SETTING);
        let c = LegoConfig::new(p);

        // excel section
        let excel_section = c.get_excel_paths();
        println!("excel section : {:?}", excel_section);

        // str section
        let str_section = c.get_str_paths();
        println!("str section : {:?}", str_section);

        // mining information
        let mining_section = c.get_mining_information();
        println!("mining section : {:?}", mining_section);
        let scp = excel_section.get("slope_csv_path").unwrap();

        // slope csv path
        println!("scp : {}", scp.kind);

    }
}