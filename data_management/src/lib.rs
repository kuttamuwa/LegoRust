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

    pub mod cross_management_traits;
}

pub mod lego_tests {
    use lego_config::read::LegoConfig;
    use crate::excels::drill_reader::{DrillObject, DrillInformation};
    use crate::excels::lytho_reader::{LythologyObject, LythologyInformation};
    use crate::excels::rawsample_reader::{RawSampleObject, RawSampleInformation};
    use crate::excels::slope_reader::{SlopeObject, SlopeInformation};

    const TEST_CONFIG_PATH: &str = "/home/umut/CLionProjects/LegoRust/lego_config/test_settings.toml";
    // const LEGOCONFIG: LegoConfig =  LegoConfig::new(String::from(TEST_CONFIG_PATH));

    pub fn give_me_test_drill () -> DrillObject {
        let LEGOCONFIG: LegoConfig =  LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let d_info = DrillInformation::new_from_config(&LEGOCONFIG);

        let drill_object = DrillObject::new(d_info);
        drill_object
    }

    pub fn give_me_test_lytho () -> LythologyObject {
        let LEGOCONFIG: LegoConfig =  LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let lytho_info = LythologyInformation::new_from_config(&LEGOCONFIG);
        let l_object = LythologyObject::new(lytho_info);

        l_object
    }

    pub fn give_me_test_rawsample () -> RawSampleObject {
        let LEGOCONFIG: LegoConfig =  LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let rawsample_info = RawSampleInformation::new_from_config(&LEGOCONFIG);
        let r_object = RawSampleObject::new(rawsample_info);

        r_object

    }

    pub fn give_me_slope () -> SlopeObject {
        let LEGOCONFIG: LegoConfig =  LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let slope_info = SlopeInformation::new_from_config(&LEGOCONFIG);
        let s_object = SlopeObject::new(slope_info);

        s_object
    }
}



