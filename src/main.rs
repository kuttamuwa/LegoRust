use lego_config::read::LConfig;
use data_management;

const TEST_CONFIG_PATH: &str = "/home/umut/CLionProjects/LegoRust/lego_config/path_config/test_settings.toml";

fn main () {
    let csv_section_name = String::from("excel_paths");
    let str_section_name = String::from("str_paths");

    let config_object = LConfig::new(TEST_CONFIG_PATH, csv_section_name, str_section_name);

    let csv_object = config_object.create_and_get_csv_object();
    let str_object = config_object.create_and_get_str_object();

    println!("csv object : {}", csv_object);
    println!("str object : {}", str_object);

}

