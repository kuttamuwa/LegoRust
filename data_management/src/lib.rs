#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }


}

pub mod excels {
    pub mod drill_reader;

    pub mod lytho_reader;

    pub mod rawsample_reader;

    pub mod slope_reader;
}

pub mod str {
    pub mod cross_reader;

    pub mod composite_reader;
}

// pub mod read;


