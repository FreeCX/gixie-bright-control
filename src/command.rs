use crate::brightness;
use crate::config::Config;
use crate::connection::Connection;
use crate::suninfo::SunInfo;

pub fn get(config: &Config) -> Result<(), String> {
    let mut connection = Connection::connect(config)?;

    println!("{}", connection.get(config)?);

    Ok(())
}

pub fn set(config: &Config, smooth: bool, new_brightness: u8) -> Result<(), String> {
    let mut connection = Connection::connect(config)?;

    if smooth {
        let _ = transition(config, &mut connection, new_brightness);
    } else {
        if !connection.set(config, new_brightness)? {
            log::warn!("Brightness not changed");
        }
    }

    Ok(())
}

pub fn transition(config: &Config, connection: &mut Connection, new_brightness: u8) -> Result<bool, String> {
    // create smooth transition
    let smooth_transition = |from: u8, to: u8| {
        let step = config.brightness.step.into();
        let mut values: Vec<_> = (from.min(to)..=from.max(to)).step_by(step).collect();
        if from > to {
            values.reverse();
        }
        values
    };

    let cur_brightness = connection.get(config)?;
    log::debug!("Current brightness: {cur_brightness}");

    // nothing to do
    if cur_brightness == new_brightness {
        log::warn!("Brightness not changed");
        return Ok(true);
    }

    log::debug!("Change {cur_brightness} -> {new_brightness}");
    for value in smooth_transition(cur_brightness, new_brightness) {
        if !connection.set(config, value)? {
            log::warn!("Brightness not changed");
            return Ok(false);
        }
    }

    Ok(true)
}

pub fn suninfo(config: &Config) -> Result<(), String> {
    log::debug!("Calculate sun info data");
    let suninfo = SunInfo::try_from(config)?;
    log::debug!("{suninfo:?}");

    println!("sunrise: {}", suninfo.sunrise.format(&config.clock.date_fmt));
    println!(" sunset: {}", suninfo.sunset.format(&config.clock.date_fmt));

    Ok(())
}

pub fn default(config: &Config) -> Result<(), String> {
    log::debug!("Calculate sun info data");
    let suninfo = SunInfo::try_from(config)?;
    log::debug!("{suninfo:?}");

    let new_brightness = brightness::calculate(config, &suninfo);
    log::debug!("Auto brightness: {new_brightness}");

    let mut connection = Connection::connect(config)?;
    log::debug!("Establish websocket connection");

    transition(config, &mut connection, new_brightness)?;

    Ok(())
}
