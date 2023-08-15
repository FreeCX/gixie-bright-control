use crate::config::Config;
use crate::suninfo::SunInfo;

pub fn calculate(config: &Config, suninfo: &SunInfo) -> u8 {
    //     min         max          min
    // .......... | .......... | ..........
    //         sunrise       sunset
    //      1           2            3

    let sunrise_diff = (suninfo.sunrise - suninfo.now).num_seconds();
    log::debug!("sunrise_diff: {sunrise_diff}");

    let sunset_diff = (suninfo.sunset - suninfo.now).num_seconds();
    log::debug!("sunset_diff: {sunset_diff}");

    match (sunrise_diff > 0, sunset_diff > 0) {
        // 1
        (true, true) => config.brightness.min,
        // 2
        (false, true) => config.brightness.max,
        // 3
        (false, false) => config.brightness.min,
        _ => unreachable!(),
    }
}
