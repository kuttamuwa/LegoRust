use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

use lego_config::read::{DataManagementObjects, LegoConfig};
use geo::{LineString, Point, Coordinate, Polygon};
use geo::convexhull::ConvexHull;

// main object which will be used on everywhere
#[derive(Debug)]
pub struct CrossObject {
    pub info: CrossInformation,
    pub data: Vec<Cross>,
}

pub trait ICrossObject {
    fn find_model_frame(&self) -> Extent;

    fn get_cross_by_groupno(&self, group_no: i32) -> Option<&Cross>;

    fn sort_crosses_data(&mut self, axis: Axis);

    fn get_min_axis_value(&self, axis: Axis) -> f64;

    fn remove_crosses_by_group_numbers(&mut self, group_number: Vec<i32>);

    fn get_deeper_than_min_drill(&self, min_drill_value: &f64) -> Vec<i32>;

    fn take_min_axis_coordinate_from_crosses(&self, axis: Axis) -> f64;
}

impl ICrossObject for CrossObject {
    fn find_model_frame(&self) -> Extent {
        // IT HAD TO BE SORTED !

        let min_x = self.data.first().unwrap().give_minimum_x_value().clone();
        let min_y = self.data.first().unwrap().give_minimum_y_value().clone();
        let min_z = self.data.first().unwrap().give_minimum_z_value().clone();

        let max_x = self.data.last().unwrap().give_maximum_x_value().clone();
        let max_y = self.data.last().unwrap().give_maximum_y_value().clone();
        let max_z = self.data.last().unwrap().give_maximum_z_value().clone();

        Extent {
            min_x,
            min_y,
            min_z,

            max_x,
            max_y,
            max_z,
        }
    }

    fn get_cross_by_groupno(&self, group_no: i32) -> Option<&Cross> {
        let mut return_cross: Option<&Cross> = None;

        for i in self.data.iter() {
            if i.group_no == group_no {
                return_cross = Some(i);
            }
        }

        return_cross
    }

    fn sort_crosses_data(&mut self, axis: Axis) {
        self.data.sort_by(|c1, c2| {
            let (c1_max_value, c2_max_value) = (c1.get_minimum_by_axis(&axis), c2.get_minimum_by_axis(&axis));

            c1_max_value.partial_cmp(&c2_max_value).unwrap()
        });
    }

    fn get_min_axis_value(&self, axis: Axis) -> f64 {
        self.data.first().unwrap().get_minimum_by_axis(&axis).clone()
    }

    fn remove_crosses_by_group_numbers(&mut self, group_number: Vec<i32>) {
        // removing
        self.data.retain(|c| group_number.contains(&&c.group_no));
    }

    fn get_deeper_than_min_drill(&self, min_drill_value: &f64) -> Vec<i32> {
        // axis -> Z
        let mut wrong_crosses: Vec<i32> = vec![]; // will store group numbers

        // reading
        for cross in self.data.iter() {
            let min_z_value_of_c = cross.get_minimum_by_axis(&Axis::Z);
            if min_z_value_of_c.le(min_drill_value) {
                wrong_crosses.push(cross.group_no.clone());
            }
        }
        wrong_crosses
    }

    fn take_min_axis_coordinate_from_crosses(&self, axis: Axis) -> f64 {
        // we dont have to accept that Vec<Cross> are already sort between each other
        // according to any axis. We dont have to. Because we will iterate all of them and get
        // minimum value

        // But in any case, we will sort again Cross coordinates in the next steps

        let mut min_axis_value: f64 = self.data.first().unwrap().get_minimum_by_axis(&axis).clone();

        for c in self.data.iter() {
            let c_min_axis_value: f64 = c.get_minimum_by_axis(&axis).clone();

            if c_min_axis_value.le(&min_axis_value) {
                min_axis_value = c_min_axis_value;
            }
        }
        min_axis_value
    }
}

impl CrossObject {
    pub fn new(info: CrossInformation, min_drill_z: Option<f64>) -> CrossObject {
        let data = info.read()
            .expect("Error occured while reading cross section ");

        // creating object. but we need to do some initial things.
        let mut object = CrossObject {
            info,
            data,
        };

        // ordering data by Z axis - single thread context
        object.sort_crosses_data(Axis::Z);

        // is there any duplicates?
        let dups = &object.take_duplicates();
        println!("dups : {:?}", dups);

        // are we going to remove crosses deeper than our drill ?
        match min_drill_z {
            Some(t) => {
                // also setting this value means remove it.
                // removing crosses has z coordinate are bigger than minimum z coordinate of drills
                let wrong_cases = object.get_deeper_than_min_drill(&t);
                println!("wrong cases : {:?}", wrong_cases);
            }
            None => println!("We will not remove crosses deeper than minimum drill value \
            which is not specified ")
        };

        let extent = object.find_model_frame();
        println!("extent : {:?}", extent);
        object
    }

    fn get_cross_by_groupno(&self, group_no: i32) -> Option<&Cross> {
        let mut return_cross: Option<&Cross> = None;

        for i in self.data.iter() {
            if i.group_no == group_no {
                return_cross = Some(i);
            }
        }

        return_cross
    }

    fn take_duplicates(&self) -> Vec<&Cross> {
        let mut duplicates: Vec<&Cross> = vec![];

        for (_, c) in self.data.iter().enumerate() {
            for k in self.data.iter() {
                if c == k {
                    duplicates.push(c);
                }
            }
        }

        duplicates
    }

    pub fn find_model_frame(&self) -> Extent {
        // IT HAD TO BE SORTED !

        let min_x = self.data.first().unwrap().give_minimum_x_value().clone();
        let min_y = self.data.first().unwrap().give_minimum_y_value().clone();
        let min_z = self.data.first().unwrap().give_minimum_z_value().clone();

        let max_x = self.data.last().unwrap().give_maximum_x_value().clone();
        let max_y = self.data.last().unwrap().give_maximum_y_value().clone();
        let max_z = self.data.last().unwrap().give_maximum_z_value().clone();

        Extent {
            min_x,
            min_y,
            min_z,

            max_x,
            max_y,
            max_z,
        }
    }

    fn sort_crosses_data(&mut self, axis: Axis) {
        self.data.sort_by(|c1, c2| {
            let (c1_max_value, c2_max_value) = (c1.get_minimum_by_axis(&axis), c2.get_minimum_by_axis(&axis));

            c1_max_value.partial_cmp(&c2_max_value).unwrap()
        });
    }

    fn remove_crosses_by_group_numbers(&mut self, group_number: Vec<i32>) {
        // removing
        self.data.retain(|c| group_number.contains(&&c.group_no));
    }
}

#[derive(Debug)]
pub struct CrossInformation {
    path: String,
    mining_type: String,
    seperator: String,
    duplicate_avoiding: bool,
}

pub trait ICrossInformation {
    fn new(path: String, mining_type: String, seperator: String) -> CrossInformation {
        CrossInformation {
            path,
            mining_type,
            seperator,
            duplicate_avoiding: true,
        }
    }

    fn new_from_config(config: &LegoConfig) -> CrossInformation {
        // getting mining information
        let path = config.get_cross_section_str_path();
        let mining_type = config.get_mining_type();
        let seperator = config.get_cross_section_seperator();

        CrossInformation::new(path, mining_type, seperator)
    }

    fn read(&self) -> Result<Vec<Cross>, Box<dyn Error>>;
}

impl ICrossInformation for CrossInformation {
    fn read(&self) -> Result<Vec<Cross>, Box<dyn Error>> {
        // our data
        let mut cross_objects: Vec<Cross> = vec![];

        let file = File::open(&self.path)?;
        let reader = BufReader::new(file);
        let mut lines = reader.lines();

        // skip first header line
        lines.next();
        lines.next();

        let mut group_no = 0;
        let mut vertex_id = 0;
        let mut temp_coords: Vec<CrossCoordinate3d> = vec![];

        for line in lines {
            let record = line.expect("Failed to reading line of str file");

            if record.contains(&self.seperator) {
                // cleaning
                group_no += 1;
                vertex_id = 0;

                if temp_coords.len() > 1 {
                    let cross = Cross::new(group_no, temp_coords.clone());

                    if cross_objects.contains(&cross) == false && self.duplicate_avoiding {
                        // avoiding duplicates
                        cross_objects.push(cross);
                    }
                    temp_coords.clear();
                }
            } else {
                vertex_id += 1;
                let record: Vec<String> = record.split(",").map(|s| s.trim().to_string()).collect();

                // println!("record : {:?}", record);
                let x: f64 = record[1].parse().expect("x icin numerik deger çevirilemedi");
                let y: f64 = record[2].parse().expect("y icin numerik deger çevirilemedi");
                let z: f64 = record[3].parse().expect("z icin numerik deger çevirilemedi");
                temp_coords.push(CrossCoordinate3d::new(x, y, z, vertex_id));
            }
        }

        Ok(cross_objects)
    }
}

#[derive(Debug)]
pub struct Extent {
    min_x: f64,
    min_y: f64,
    min_z: f64,

    max_x: f64,
    max_y: f64,
    max_z: f64,

}

#[derive(Debug, PartialEq)]
pub struct Cross {
    pub group_no: i32,
    pub coordinate: Vec<CrossCoordinate3d>,
}

pub trait ICross {
    fn get_min_one<'a>(v1: &'a f64, v2: &'a f64) -> &'a f64 {
        // just get little one
        if v1.le(v2) {
            v1
        } else {
            v2
        }
    }

    fn get_max_one<'a>(v1: &'a f64, v2: &'a f64) -> &'a f64 {
        // just get little one
        if v1.ge(v2) {
            v1
        } else {
            v2
        }
    }

    fn get_minimum_by_axis(&self, axis: &Axis) -> f64 {
        let min_axis_value = match axis {
            Axis::X => {
                self.give_minimum_x_value()
            }

            Axis::Y => {
                self.give_minimum_y_value()
            }

            Axis::Z => {
                self.give_minimum_z_value()
            }
        };

        min_axis_value
    }

    fn give_minimum_z_value(&self) -> f64;

    fn give_minimum_x_value(&self) -> f64;

    fn give_minimum_y_value(&self) -> f64;

    fn give_maximum_z_value(&self) -> f64;

    fn give_maximum_x_value(&self) -> f64;

    fn give_maximum_y_value(&self) -> f64;

    fn find_orient_2d(&self) -> Axis {
        let (min_x, max_x) = (self.give_minimum_x_value(), self.give_maximum_x_value());
        let (min_y, max_y) = (self.give_minimum_y_value(), self.give_maximum_y_value());

        let (delta_x, delta_y) = (f64::abs(max_x - min_x), f64::abs(max_y - min_y));

        if delta_x >= delta_y {
            Axis::X
        } else {
            Axis::Y
        }
    }

    fn difference_self_minimum_axis_value_from_other_cross(&self, other_cross: impl ICross, axis: Axis) -> f64 {
        let self_min_axis: f64;
        let other_min_axis: f64;

        if axis == Axis::X {
            self_min_axis = self.give_minimum_x_value();
            other_min_axis = other_cross.give_minimum_x_value();
        } else if axis == Axis::Y {
            self_min_axis = self.give_minimum_y_value();
            other_min_axis = other_cross.give_minimum_y_value();
        } else {
            self_min_axis = self.give_minimum_z_value();
            other_min_axis = other_cross.give_minimum_z_value();
        }

        self_min_axis - other_min_axis
    }

    fn clone_coordinates_2d(&self) -> Vec<CrossCoordinate2d>;

    fn clone_coordinates_2d_with_points(&self) -> Vec<Coordinate<f64>>;

    fn create_line_string(&self) -> LineString<f64> {
        let coordinates_2d: Vec<Coordinate<f64>> = self.clone_coordinates_2d_with_points();
        LineString(coordinates_2d)
    }

    fn create_polygon_convex_hull(&self) -> Polygon<f64> {
        let line_strings = self.create_line_string();
        let poly = Polygon::new(line_strings, vec![]).convex_hull();  // no interior rings
        poly
    }

    fn find_two_cross_frame<T>(&self, cross_two: impl ICross) -> Extent {
        /*
        It is used to determine one rectangular frame consists two cross

        */
        // minimums
        let min_x = Cross::get_min_one(&self.give_minimum_x_value(), &cross_two.give_minimum_x_value()).clone();
        let min_y = Cross::get_min_one(&self.give_minimum_y_value(), &cross_two.give_minimum_y_value()).clone();
        let min_z = Cross::get_min_one(&self.give_minimum_z_value(), &cross_two.give_minimum_z_value()).clone();

        let max_x = Cross::get_max_one(&self.give_maximum_x_value(), &cross_two.give_maximum_x_value()).clone();
        let max_y = Cross::get_max_one(&self.give_maximum_y_value(), &cross_two.give_maximum_y_value()).clone();
        let max_z = Cross::get_max_one(&self.give_maximum_z_value(), &cross_two.give_maximum_z_value()).clone();

        Extent {
            min_x,
            min_y,
            min_z,

            max_x,
            max_y,
            max_z,
        }
    }

    fn sort_coordinates(v: &mut Vec<&f64>) {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
    }
}

impl Cross {
    fn new(group_no: i32, coordinate: Vec<CrossCoordinate3d>) -> Cross {
        Cross {
            group_no,
            coordinate,
        }
    }
}

impl ICross for Cross {
    fn give_minimum_z_value(&self) -> f64 {
        let mut min_z = self.coordinate.first().unwrap().z_coord.clone();

        for i in self.coordinate.iter() {
            if i.z_coord.le(&min_z) {
                min_z = i.z_coord;
            }
        }
        min_z
    }

    fn give_minimum_x_value(&self) -> f64 {
        let mut min_x = self.coordinate.first().unwrap().x_coord.clone();

        for i in self.coordinate.iter() {
            if i.x_coord.le(&min_x) {
                min_x = i.x_coord;
            }
        }
        min_x
    }

    fn give_minimum_y_value(&self) -> f64 {
        let mut min_y = self.coordinate.first().unwrap().y_coord.clone();

        for i in self.coordinate.iter() {
            if i.y_coord.le(&min_y) {
                min_y = i.y_coord;
            }
        }
        min_y
    }

    fn give_maximum_z_value(&self) -> f64 {
        let mut max_z = self.coordinate.first().unwrap().z_coord.clone();

        for i in self.coordinate.iter() {
            if i.z_coord.le(&max_z) {
                max_z = i.z_coord;
            }
        }
        max_z
    }

    fn give_maximum_x_value(&self) -> f64 {
        let mut max_x = self.coordinate.first().unwrap().x_coord.clone();

        for i in self.coordinate.iter() {
            if i.x_coord.le(&max_x) {
                max_x = i.x_coord;
            }
        }
        max_x
    }

    fn give_maximum_y_value(&self) -> f64 {
        let mut max_y = self.coordinate.first().unwrap().y_coord.clone();

        for i in self.coordinate.iter() {
            if i.y_coord.le(&max_y) {
                max_y = i.y_coord;
            }
        }
        max_y
    }

    fn clone_coordinates_2d(&self) -> Vec<CrossCoordinate2d> {
        let mut cross_coordinate_2d: Vec<CrossCoordinate2d> = vec![];

        for i in self.coordinate.iter() {
            let c = CrossCoordinate2d::new(i.x_coord, i.y_coord, i.vertex_id);
            cross_coordinate_2d.push(c);
        }

        cross_coordinate_2d
    }

    fn clone_coordinates_2d_with_points(&self) -> Vec<Coordinate<f64>> {
        let mut cross_coordinate_2d: Vec<Coordinate<f64>> = vec![];

        for i in &self.coordinate {
            let p = Point::new(i.x_coord.clone(), i.y_coord.clone());
            cross_coordinate_2d.push(p.0);
        }

        cross_coordinate_2d
    }
}

pub trait ICoordinate {
    fn eq_test<T: ICoordinate>(&self, coordinate: &T) -> bool {
        if (self.borrow_x_coord() == coordinate.borrow_x_coord()) && (self.borrow_y_coord() == coordinate.borrow_y_coord()) &&
            (self.borrow_z_coord() == coordinate.borrow_z_coord()) {
            true
        } else {
            false
        }
    }

    fn borrow_x_coord(&self) -> &f64;

    fn borrow_y_coord(&self) -> &f64;

    fn borrow_z_coord(&self) -> &f64;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CrossCoordinate3d {
    pub x_coord: f64,
    pub y_coord: f64,
    pub z_coord: f64,
    pub vertex_id: i32,
}

impl ICoordinate for CrossCoordinate3d {
    fn borrow_x_coord(&self) -> &f64 {
        &self.x_coord
    }

    fn borrow_y_coord(&self) -> &f64 {
        &self.y_coord
    }

    fn borrow_z_coord(&self) -> &f64 {
        &self.z_coord
    }
}

impl CrossCoordinate3d {
    fn new(x: f64, y: f64, z: f64, vertex_id: i32) -> CrossCoordinate3d {
        CrossCoordinate3d {
            x_coord: x,
            y_coord: y,
            z_coord: z,
            vertex_id,
        }
    }
}

impl Display for CrossCoordinate3d {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "x : {} \n\
                   y : {} \n\
                   z : {}", self.x_coord, self.y_coord, self.z_coord)
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CrossCoordinate2d {
    pub x_coord: f64,
    pub y_coord: f64,
    pub vertex_id: i32,
}

impl CrossCoordinate2d {
    pub fn new(x_coord: f64, y_coord: f64, vertex_id: i32) -> CrossCoordinate2d {
        CrossCoordinate2d {
            x_coord,
            y_coord,
            vertex_id,
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl Display for Axis {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match *self {
            Axis::X => f.write_str("X"),
            Axis::Y => f.write_str("Y"),
            Axis::Z => f.write_str("Z")
        }
    }
}


impl Display for Cross {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Cross no : {} \n\
                   coordinate : {:?}", self.group_no, self.coordinate)
    }
}


impl Display for CrossObject {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "info : {} \n\
                   crosses : {:?}", self.info, self.data)
    }
}


impl Display for CrossInformation {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "path: {} \n\
                   mining type : {} \n\
                   seperator : {}", self.path, self.mining_type, self.seperator)
    }
}

#[cfg(test)]
mod tests {
    use lego_config::read::LegoConfig;

    use crate::str::cross_reader::{CrossInformation, CrossObject, ICross, ICrossInformation};
    use ncollide3d;
    use na;

    const TEST_CONFIG_PATH: &str = "/home/umut/CLionProjects/LegoRust/lego_config/test_settings.toml";

    #[test]
    fn read_cross_section_from_config() {
        let config_object = LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let cross_info = CrossInformation::new_from_config(&config_object);
        let cross_object = CrossObject::new(cross_info, None);
        println!("cross sections info : {:?}", &cross_object.info);
        for c in &cross_object.data {
            println!("section : {:?}", c);
        }
        // println!("cross sections data : {:?}", &cross_object.data);
        println!("count of sections : {:?}", &cross_object.data.len());
    }

    #[test]
    fn order_crosses_by_z() {
        let config_object = LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let cross_info = CrossInformation::new_from_config(&config_object);
        let cross_object = CrossObject::new(cross_info, None);

        for i in 1..cross_object.data.len() {
            let c1 = &cross_object.data[i];
            let c2 = &cross_object.data[i - 1];

            // println!("before z: {}", c1.coordinate.z_coord);
            // println!("now z: {}", c2.coordinate.z_coord);

            assert!(c1.give_minimum_z_value().ge(&c2.give_minimum_z_value()));
        }
    }
}
