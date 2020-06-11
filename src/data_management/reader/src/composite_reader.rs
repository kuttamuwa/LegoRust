use std::collections::HashMap;

pub struct CompositeObject {
    info: CompositeInformation,
    data: Vec<Composite>,
}

impl CompositeObject {
    pub fn new(info: CompositeInformation, data: Vec<Composite>) -> CompositeObject {
        CompositeObject {
            info,
            data,
        }
    }
}

// others

struct CompositeInformation {
    pub path: String,
    pub mining_type: String,
    pub date: String,
    // will be changed
    pub seperator: String,
    pub columns: HashMap,
}

struct Composite {
    // tek satır yani

    groupno: i8,
    coordinate: CompositeCoordinate,
}

struct CompositeCoordinate {
    x_coord: f64,
    y_coord: f64,
    z_coord: f64,
}


// implementing traits
impl CompositeCoordinate {
    fn new(x: f64, y: f64, z: f64) -> CompositeCoordinate {
        CompositeCoordinate {
            x_coord: x,
            y_coord: y,
            z_coord: z,
        }
    }
}

impl Composite {
    fn new(groupno: i8, coordinate: CompositeCoordinate) -> Composite {
        Composite {
            groupno,
            coordinate,
        }
    }
}

impl CompositeInformation {
    fn new(path: String, mining_type: String, date: String, seperator: String, columns: HashMap) -> CompositeInformation {
        CompositeInformation {
            path,
            mining_type,
            date,
            seperator,
            columns,
        }
    }
}