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

    pub mod dat_management_traits;
}

pub mod lego_tests {
    use lego_config::read::LegoConfig;
    use crate::excels::drill_reader::{DrillObject, DrillInformation};
    use crate::excels::lytho_reader::{LythologyObject, LythologyInformation};
    use crate::excels::rawsample_reader::{RawSampleObject, RawSampleInformation};
    use crate::excels::slope_reader::{SlopeObject, SlopeInformation};
    use crate::str::cross_reader::{CrossObject, CrossInformation, ICrossInformation};
    use crate::str::composite_reader::{CompositeObject, CompositeInformation};

    const TEST_CONFIG_PATH: &str = "/home/umut/CLionProjects/LegoRust/lego_config/test_settings.toml";

    pub fn give_me_test_drill () -> DrillObject {
        let legoconfig: LegoConfig =  LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let d_info = DrillInformation::new_from_config(&legoconfig);

        let drill_object = DrillObject::new(d_info);
        drill_object
    }

    pub fn give_me_test_lytho () -> LythologyObject {
        let legoconfig: LegoConfig =  LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let lytho_info = LythologyInformation::new_from_config(&legoconfig);
        let l_object = LythologyObject::new(lytho_info);

        l_object
    }

    pub fn give_me_test_rawsample () -> RawSampleObject {
        let legoconfig: LegoConfig =  LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let rawsample_info = RawSampleInformation::new_from_config(&legoconfig);
        let r_object = RawSampleObject::new(rawsample_info);

        r_object

    }

    pub fn give_me_slope () -> SlopeObject {
        let legoconfig: LegoConfig =  LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let slope_info = SlopeInformation::new_from_config(&legoconfig);
        let s_object = SlopeObject::new(slope_info);

        s_object
    }

    pub fn give_me_cross () -> CrossObject {
        let legoconfig: LegoConfig =  LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let cross_info = CrossInformation::new_from_config(&legoconfig);
        let cross_object = CrossObject::new(cross_info, None);

        cross_object
    }

    pub fn give_me_composite () -> CompositeObject {
        let legoconfig: LegoConfig =  LegoConfig::new(String::from(TEST_CONFIG_PATH));

        let composite_info = CompositeInformation::new_from_config(&legoconfig);
        let l_object = CompositeObject::new(composite_info);

        l_object

    }
}



