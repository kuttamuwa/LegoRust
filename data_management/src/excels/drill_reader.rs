use std::collections::HashMap;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::hash::Hash;
use std::string::ToString;

use csv::{Reader, StringRecord};
use lego_config::read::{LegoConfig, DataManagementObjects};
use plotly::{Plot, Scatter};
use crate::excels::excel_traits::WebDraw;
use plotly::{Surface, Layout};
use plotly::surface::{Lighting, PlaneContours, PlaneProject, SurfaceContours};

pub struct DrillObject {
    info: DrillInformation,
    data: Vec<Drill>,
}

impl DrillObject {
    pub(crate) fn new(info: DrillInformation) -> DrillObject {
        let data = info.read().unwrap();
        DrillObject {
            info,
            data,
        }
    }
}

impl WebDraw for DrillObject {
    fn generate_topograpy(&self, auto_show: bool) {
        let x_coords: Vec<f64> = self.data.iter().map(|e| e.coordinate.x_coord).collect();
        let y_coords: Vec<f64> = self.data.iter().map(|e| e.coordinate.y_coord).collect();
        let z_coords: Vec<f64> = self.data.iter().map(|e| e.coordinate.z_coord).collect();

        // todo: burayı halledemedim.
        let mut z_coords_v2: Vec<Vec<f64>> = Vec::new();
        for i in 0..x_coords.len() {
            let mut iz: Vec<f64> = Vec::new();
            for k in 0..x_coords.len() {
                // let xf = (xi as f64) / n as f64;
                // let yf = (yi as f64) / n as f64;
                let cz: f64 = self.data.get(i).unwrap().coordinate.z_coord;
                iz.push(cz);
            }
            z_coords_v2.push(iz);
        }

        let trace = Surface::new(z_coords_v2).x(x_coords).y(y_coords).visible(true)
            .hide_surface(false).lighting(Lighting::new())
            .contours(SurfaceContours::new().z(PlaneContours::new().show(true)
                                                   .use_colormap(true)
                                                   .project(PlaneProject::new().z(true))));


        let mut plot = Plot::new();
        plot.set_layout(Layout::new());
        plot.add_trace(trace);

        if auto_show {
            plot.show();
        }
    }
}

pub struct DrillInformation {
    path: String,
    mining_type: String,
    seperator: char,
    columns: HashMap<String, String>,  // todo sonra hallederim
}

impl DrillInformation {
    fn new(path: String, mining_type: String, seperator: char,
           columns: HashMap<String, String>) -> DrillInformation {
        DrillInformation {
            path,
            mining_type,
            seperator,
            columns,
        }
    }

    fn read(&self) -> Result<Vec<Drill>, Box<dyn Error>> {
        // our data
        let mut drill_objects: Vec<Drill> = vec![];

        let drill_csv_path: &String = &self.path;
        let mut reader = Reader::from_path(drill_csv_path)?;

        // columns
        reader.set_headers(StringRecord::from(vec!["SONDAJNO", "X", "Y", "Z", "DERINLIK"]));

        let mut results = reader.records();

        // skip first row
        results.next();

        for result in results {
            let record = result?;
            let record: Vec<String> = record[0].to_string().split(";").map(|s| s.to_string()).collect();

            let drill_no = record[0].to_owned();
            let x: f64 = record[1].parse().expect("numerik deger çevirilemedi");
            let y: f64 = record[2].parse().expect("numerik deger çevirilemedi");
            let z: f64 = record[3].parse().expect("numerik deger çevirilemedi");
            let depth: f64 = record[4].parse().expect("numerik deger çevirilemedi");

            let drill_coordinate = DrillCoordinate::new(x, y, z, depth);
            let d_row = Drill::new(drill_no, drill_coordinate);

            drill_objects.push(d_row);
        }
        Ok(drill_objects)
    }

    pub(crate) fn new_from_config(config: &LegoConfig) -> DrillInformation {
        // getting mining information
        let path = config.get_drill_csv_path();
        let mining_type = config.get_mining_type();
        let seperator = config.get_x_seperator("drill_csv_seperator");
        let columns = config.get_x_columns("drill_columns");

        DrillInformation {
            path,
            mining_type,
            seperator,
            columns
        }
    }
}

#[allow(unused)]
#[derive(Debug, PartialEq, Eq, Hash)]
enum DrillColumns {
    DRILLNO,
    X,
    Y,
    Z,
    DEPTH,
}

#[derive(Debug)]
struct Drill {
    drill_no: String,
    coordinate: DrillCoordinate,
}

impl Drill {
    fn new(drill_no: String, coordinate: DrillCoordinate) -> Drill {
        Drill {
            drill_no,
            coordinate,
        }
    }
}

#[derive(Debug)]
struct DrillCoordinate {
    x_coord: f64,
    y_coord: f64,
    z_coord: f64,
    depth: f64,
}

impl DrillCoordinate {
    fn new(x: f64, y: f64, z: f64, depth: f64) -> DrillCoordinate {
        DrillCoordinate {
            x_coord: x,
            y_coord: y,
            z_coord: z,
            depth,
        }
    }
}

impl Display for DrillCoordinate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x : {} \n\
                   y : {} \n\
                   z : {}", self.x_coord, self.y_coord, self.z_coord)
    }
}

impl Display for DrillInformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "path : {} \n\
                   mining type : {} \n\
                   seperator : {} \n\
                   columns: {:?}", self.path, self.mining_type, self.seperator, self.columns)
    }
}

impl Display for DrillObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Information : {} \n\
                   Drills : {:?}", self.info, self.data)
    }
}

impl Display for Drill {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "drill no : {} \n\
                   coordinate : {}", self.drill_no, self.coordinate)
    }
}

#[cfg(test)]
mod tests {
    use crate::excels::drill_reader::{DrillInformation, DrillObject};
    use lego_config::read::{LegoConfig};
    use crate::excels::excel_traits::WebDraw;

    const TEST_CONFIG_PATH: &str = "/home/umut/CLionProjects/LegoRust/lego_config/test_settings.toml";

    #[test]
    fn creating_drill_object_from_config() {
        // config path
        let config_object = LegoConfig::new(String::from(TEST_CONFIG_PATH));
        let d_info = DrillInformation::new_from_config(&config_object);

        let drill_object = DrillObject::new(d_info);
        println!("drill object : {}", drill_object);

    }

    #[test]
    fn create_topo () {
        // config path
        let config_object = LegoConfig::new(String::from(TEST_CONFIG_PATH));
        let d_info = DrillInformation::new_from_config(&config_object);

        let drill_object = DrillObject::new(d_info);
        drill_object.generate_topograpy(true);

    }
}