use crate::str::cross_reader::info::CrossInformation;
use lego_config::read::{LegoConfig, DataManagementObjects};
use std::error::Error;
use geo::{Coordinate, LineString, Polygon};
use geo::convexhull::ConvexHull;
use crate::str::cross_reader::common::{Extent, Axis};
use crate::str::cross_reader::cross::Cross;
use crate::str::cross_reader::coordinate::{CrossCoordinate2d, CrossCoordinate3d};
use plotly::common::PlotType::Scatter;
use plotly::Plot;

pub trait ICrossObject {
    fn find_model_frame(&self) -> Extent;

    fn get_cross_by_groupno(&self, group_no: i32) -> Option<&Cross>;

    fn sort_crosses_data(&mut self, axis: Axis);

    fn get_min_axis_value(&self, axis: Axis) -> f64;

    fn remove_crosses_by_group_numbers(&mut self, group_number: Vec<i32>);

    fn get_deeper_than_min_drill(&self, min_drill_value: &f64) -> Vec<i32>;

    fn take_min_axis_coordinate_from_crosses(&self, axis: Axis) -> f64;
}


pub trait ICrossInformation {
    fn new_from_config(config: &LegoConfig) -> CrossInformation {
        // getting mining information
        let path = config.get_cross_section_str_path();
        let mining_type = config.get_mining_type();
        let seperator = config.get_cross_section_seperator();

        CrossInformation::new(path, mining_type, seperator)
    }

    fn read(&self) -> Result<Vec<Cross>, Box<dyn Error>>;
}


pub trait ICross {
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

    fn give_above_one(&self) -> Vec<&CrossCoordinate3d>;

    fn give_below_one(&self) -> Vec<&CrossCoordinate3d>;

    fn give_minimum_z_value(&self) -> f64;

    fn give_minimum_x_value(&self) -> f64;

    fn give_minimum_y_value(&self) -> f64;

    fn give_maximum_z_value(&self) -> f64;

    fn give_maximum_x_value(&self) -> f64;

    fn give_maximum_y_value(&self) -> f64;

    fn find_vertex_id_via_value_of_axis(&self, value: &f64, axis: &Axis) -> Option<i32>;

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
    fn find_two_cross_frame(&self, cross_two: impl ICross) -> Extent {
        /*
        It is used to determine one rectangular frame consists two cross

        */
        // minimums
        let min_x: f64 = Cross::get_min_one(&self.give_minimum_x_value(), &cross_two.give_minimum_x_value()).clone();
        let min_y: f64 = Cross::get_min_one(&self.give_minimum_y_value(), &cross_two.give_minimum_y_value()).clone();
        let min_z: f64 = Cross::get_min_one(&self.give_minimum_z_value(), &cross_two.give_minimum_z_value()).clone();

        let max_x: f64 = Cross::get_max_one(&self.give_maximum_x_value(), &cross_two.give_maximum_x_value()).clone();
        let max_y: f64 = Cross::get_max_one(&self.give_maximum_y_value(), &cross_two.give_maximum_y_value()).clone();
        let max_z: f64 = Cross::get_max_one(&self.give_maximum_z_value(), &cross_two.give_maximum_z_value()).clone();

        Extent::new(min_x, min_y, min_z, max_x, max_y, max_z)
    }

    fn sort_coordinates(v: &mut Vec<&f64>) {
        v.sort_by(|a, b| a.partial_cmp(b).unwrap());
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


pub trait DrawOnWeb {
    fn draw_points(&self, group_no: i32, auto_show: bool);


}