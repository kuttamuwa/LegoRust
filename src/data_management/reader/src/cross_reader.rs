pub struct CrossObject {
    info: CrossInfo,
    data: Vec<Cross>,
}

impl CrossObject {
    fn new(info: CrossInfo, data: Vec<Cross>) -> CrossObject {
        CrossObject {
            info,
            data,
        }
    }
}

// others
struct CrossInfo {
    // general information
    path: String,
    mining_type: String,
    date: String,
}

impl CrossInfo {
    fn new(path: String, mining_type: String, date: String) -> CrossInfo {
        CrossInfo {
            path,
            mining_type,
            date,
        }
    }
}

struct Cross {
    // tek satır
    groupno: i8,
    coordinate: CrossCoordinate,
}

impl Cross {
    fn new(groupno: i8, coordinate: CrossCoordinate) -> Cross {
        Cross {
            groupno,
            coordinate,
        }
    }
}

struct CrossCoordinate {
    x_coords: f64,
    y_coords: f64,
    z_coords: f64,
}

impl CrossCoordinate {
    fn new(x: f64, y: f64, z: f64) -> CrossCoordinate {
        CrossCoordinate {
            x_coords: x,
            y_coords: y,
            z_coords: z,
        }
    }
}


