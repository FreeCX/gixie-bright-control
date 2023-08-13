use std::net::TcpStream;

use crate::config::Config;
use crate::suninfo::SunInfo;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use tungstenite::stream::MaybeTlsStream;
use tungstenite::{connect, Message, WebSocket};
use url::Url;

#[derive(Debug, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum CmdType {
    #[default]
    Get = 0,
    Set = 1,
}

#[derive(Serialize, Debug)]
struct CmdCtx {
    value: u8,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Response {
    #[serde(rename = "resCode")]
    res_code: u16,
    #[serde(rename = "cmdType")]
    cmd_type: CmdType,
    #[serde(rename = "cmdNum")]
    cmd_num: u8,
    data: Option<u8>,
}

#[derive(Debug, Default, Serialize)]
struct Request {
    #[serde(rename = "cmdType")]
    cmd_type: CmdType,
    #[serde(rename = "cmdNum")]
    cmd_num: u8,
    #[serde(rename = "cmdCtx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    cmd_ctx: Option<CmdCtx>,
}

#[derive(Debug)]
pub struct Connection {
    socket: WebSocket<MaybeTlsStream<TcpStream>>,
}

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

impl Connection {
    pub fn connect(config: &Config) -> Result<Connection, String> {
        let url = Url::parse(&config.clock.server).map_err(|_| "Cannot parse url")?;
        let (mut socket, _) = connect(url).map_err(|_| "Cannot connect to server")?;

        // FIX: read empty line and "Connected"
        for _ in 0..2 {
            let data = socket.read().map_err(|_| "Cannot read message")?;
            log::debug!("recv data: `{data}`");
        }

        Ok(Connection { socket })
    }

    fn request(&mut self, request: String) -> Result<String, String> {
        log::debug!("send data: {request}");
        self.socket.send(Message::Text(request)).map_err(|_| "Cannot send message")?;

        // read json response
        let response = self.socket.read().map_err(|_| "Cannot read message")?.to_string();
        log::debug!("recv data: {response}");

        Ok(response)
    }

    pub fn get(&mut self, config: &Config) -> Result<u8, String> {
        let request = Request { cmd_type: CmdType::Get, cmd_num: config.brightness.num, ..Default::default() };
        log::debug!("{request:?}");
        let request = serde_json::to_string(&request).map_err(|_| "Cannot serialize json")?;

        let json_data = self.request(request)?;
        let response: Response = serde_json::from_str(&json_data).map_err(|_| "Cannot parse json response")?;
        log::debug!("{response:?}");

        response.data.ok_or("Missing data".to_string())
    }

    pub fn set(&mut self, config: &Config, value: u8) -> Result<bool, String> {
        let request =
            Request { cmd_type: CmdType::Set, cmd_num: config.brightness.num, cmd_ctx: Some(CmdCtx { value }) };
        log::debug!("{request:?}");
        let request = serde_json::to_string(&request).map_err(|_| "Cannot serialize json")?;

        let json_data = self.request(request)?;
        let response: Response = serde_json::from_str(&json_data).map_err(|_| "Cannot parse json response")?;
        log::debug!("{response:?}");

        Ok(response.res_code == 200)
    }
}
