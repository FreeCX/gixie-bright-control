use crate::brightness::{self, Connection};
use crate::config::Config;
use crate::suninfo::SunInfo;

pub fn get(config: &Config, connection: &mut Connection) -> Result<(), String> {
    println!("{}", connection.get(config)?);

    Ok(())
}

pub fn set(config: &Config, connection: &mut Connection, new_brightness: u8) -> Result<(), String> {
    if !connection.set(config, new_brightness)? {
        log::warn!("Brightness not changed");
    }

    Ok(())
}

pub fn default(config: &Config, connection: &mut Connection) -> Result<(), String> {
    log::debug!("Calculate sun info data");
    let suninfo = SunInfo::try_from(config)?;
    log::debug!("{suninfo:?}");

    let new_brightness = brightness::calculate(&config, &suninfo);
    log::debug!("New brightness: {new_brightness}");

    let cur_brightness = connection.get(config)?;
    log::debug!("Current brightness: {cur_brightness}");

    if new_brightness != cur_brightness {
        log::info!("Change {cur_brightness} -> {new_brightness}");
        if !connection.set(config, new_brightness)? {
            log::warn!("Brightness not changed");
        }
    }

    Ok(())
}
