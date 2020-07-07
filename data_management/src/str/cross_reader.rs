use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

use lego_config::read::{DataManagementObjects, LegoConfig};
use geo::{LineString, Point, Coordinate, Polygon};
use geo::convexhull::ConvexHull;
use crate::str::str_traits::ICrossObject;

pub mod cross_main {
    use crate::str::str_traits::{ICrossObject, ICrossInformation, ICross, DrawOnWeb};
    use std::fmt::{Display, Formatter, Result};
    use crate::str::cross_reader::info::CrossInformation;
    use crate::str::cross_reader::cross::Cross;
    use crate::str::cross_reader::common::{Extent, Axis};
    use plotly::{Plot, Scatter};

    // main object which will be used on everywhere
    #[derive(Debug)]
    pub struct CrossObject {
        pub info: CrossInformation,
        pub data: Vec<Cross>,
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

            Extent::new(min_x, min_y, min_z, max_x, max_y, max_z)
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

            Extent::new(min_x, min_y, min_z, max_x, max_y, max_z)
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

    impl DrawOnWeb for CrossObject {
        fn draw_points(&self, group_no: i32, auto_show: bool) {
            let cross: &Cross = self.data.get(group_no as usize)
                .expect("group no cannot be found ?!!");

            let x_coords = cross.coordinate.iter().map(|ele| ele.x_coord).collect();
            let y_coords = cross.coordinate.iter().map(|ele| ele.y_coord).collect();

            let mut plot = Plot::new();
            let trace = Scatter::new(x_coords, y_coords);
            plot.add_trace(trace);

            if auto_show {
                plot.show();
            }

        }
    }
    impl Display for CrossObject {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "info : {} \n\
                   crosses : {:?}", self.info, self.data)
        }
    }
}

pub mod info {
    use std::io::{BufReader, BufRead};
    use std::fs::File;
    use lego_config::read::{LegoConfig, DataManagementObjects};
    use std::error::Error;
    use std::fmt::{Display, Formatter};
    use crate::str::cross_reader::cross::Cross;
    use crate::str::cross_reader::coordinate::CrossCoordinate3d;
    use std::fmt;
    use crate::str::str_traits::ICrossInformation;

    #[derive(Debug)]
    pub struct CrossInformation {
        path: String,
        mining_type: String,
        seperator: String,
        pub(crate) duplicate_avoiding: bool,
    }

    impl CrossInformation {
        pub(crate) fn new(path: String, mining_type: String, seperator: String) -> CrossInformation {
            CrossInformation {
                path,
                mining_type,
                seperator,
                duplicate_avoiding: true,
            }
        }
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

    impl Display for CrossInformation {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "path: {} \n\
                   mining type : {} \n\
                   seperator : {}", self.path, self.mining_type, self.seperator)
        }
    }
}

pub mod common {
    use std::fmt::{Display, Formatter, Result};

    #[derive(Debug)]
    pub struct Extent {
        min_x: f64,
        min_y: f64,
        min_z: f64,

        max_x: f64,
        max_y: f64,
        max_z: f64,

    }

    impl Extent {
        pub fn new(min_x: f64, min_y: f64, min_z: f64, max_x: f64, max_y: f64, max_z: f64) -> Extent {
            Extent {
                min_x,
                min_y,
                min_z,

                max_x,
                max_y,
                max_z,
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
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            match *self {
                Axis::X => f.write_str("X"),
                Axis::Y => f.write_str("Y"),
                Axis::Z => f.write_str("Z")
            }
        }
    }
}

pub mod cross {
    use crate::str::cross_reader::coordinate::{CrossCoordinate3d, CrossCoordinate2d};
    use std::fmt::{Display, Formatter, Result};
    use geo::{Point, Coordinate, Polygon, LineString};
    use crate::str::cross_reader::common::{Extent, Axis};
    use geo::convexhull::ConvexHull;
    use crate::str::str_traits::ICross;

    #[derive(Debug, PartialEq)]
    pub struct Cross {
        pub group_no: i32,
        pub coordinate: Vec<CrossCoordinate3d>,
    }

    impl Cross {
        pub fn new(group_no: i32, coordinate: Vec<CrossCoordinate3d>) -> Cross {
            Cross {
                group_no,
                coordinate,
            }
        }
    }

    impl ICross for Cross {
        fn give_above_one(&self) -> Vec<&CrossCoordinate3d> {
            // let min_vertex_id = one_cross
            //     .find_vertex_id_via_value_of_axis(&one_cross.give_minimum_x_value(), &axis).unwrap();
            // let max_vertex_id = one_cross
            //     .find_vertex_id_via_value_of_axis(&one_cross.give_maximum_x_value(), &axis).unwrap();

            // self.coordinate[max_vertex_id as usize..min_vertex_id as usize]
            //     .to_vec()
            vec![]
        }

        fn give_below_one(&self) -> Vec<&CrossCoordinate3d> {
            unimplemented!()
        }

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
                if i.z_coord.ge(&max_z) {
                    max_z = i.z_coord;
                }
            }
            max_z
        }

        fn give_maximum_x_value(&self) -> f64 {
            let mut max_x = self.coordinate.first().unwrap().x_coord.clone();

            for i in self.coordinate.iter() {
                if i.x_coord.ge(&max_x) {
                    max_x = i.x_coord;
                }
            }
            max_x
        }

        fn give_maximum_y_value(&self) -> f64 {
            let mut max_y = self.coordinate.first().unwrap().y_coord.clone();

            for i in self.coordinate.iter() {
                if i.y_coord.ge(&max_y) {
                    max_y = i.y_coord;
                }
            }
            max_y
        }

        fn find_vertex_id_via_value_of_axis(&self, value: &f64, axis: &Axis) -> Option<i32> {
            let mut vertex_id: Option<i32> = None;
            let iter = self.coordinate.iter();

            // it doesn't matter that even if there is multiple vertices

            if Axis::X.eq(axis) {
                for c in iter {
                    if c.x_coord.eq(value) {
                        vertex_id = Some(c.vertex_id);
                    }
                }
            } else if Axis::Y.eq(axis) {
                for c in iter {
                    if c.y_coord.eq(value) {
                        vertex_id = Some(c.vertex_id);
                    }
                }
            } else if Axis::Z.eq(axis) {
                for c in iter {
                    if c.z_coord.eq(value) {
                        vertex_id = Some(c.vertex_id);
                    }
                }
            } else {
                panic!("DEVELOPER ERROR ! \n\
                Axis set for X, Y, Z ! ")
            }

            vertex_id
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

    impl Display for Cross {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
            write!(f, "Cross no : {} \n\
                   coordinate : {:?}", self.group_no, self.coordinate)
        }
    }
}

pub mod coordinate {
    use std::fmt::{Display, Formatter, Result};
    use crate::str::str_traits::ICoordinate;

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
        pub(crate) fn new(x: f64, y: f64, z: f64, vertex_id: i32) -> CrossCoordinate3d {
            CrossCoordinate3d {
                x_coord: x,
                y_coord: y,
                z_coord: z,
                vertex_id,
            }
        }
    }

    impl Display for CrossCoordinate3d {
        fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
}


// TEST TIME !
#[cfg(test)]
mod tests {
    use lego_config::read::LegoConfig;
    // use ncollide3d;
    // use na;
    use crate::str::cross_reader::cross_main::CrossObject;
    use crate::str::str_traits::{ICrossInformation, ICross, DrawOnWeb};
    use plotly::{Scatter, Plot};
    use plotly::common::Mode;
    use crate::str::cross_reader::info::CrossInformation;

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

    #[test]
    fn draw_plotly_test() {
        let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
            .name("trace1")
            .mode(Mode::Markers);
        let trace2 = Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9])
            .name("trace2")
            .mode(Mode::Lines);
        let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12]).name("trace3");

        let mut plot = Plot::new();
        plot.add_trace(trace1);
        plot.add_trace(trace2);
        plot.add_trace(trace3);
        plot.show();
    }

    #[test]
    fn draw_cross () {
        let config_object = LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let cross_info = CrossInformation::new_from_config(&config_object);
        let cross_object = CrossObject::new(cross_info, None);

        for (index, c) in cross_object.data.iter().enumerate() {
            cross_object.draw_points(index as i32, true);
        }
    }
}
