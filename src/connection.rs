use std::net::TcpStream;

use crate::config::Config;
use crate::error::{AppError, Result};

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

impl Connection {
    pub fn connect(config: &Config) -> Result<Connection> {
        let url = Url::parse(&config.clock.server)?;
        let (mut socket, response) = connect(url)?;

        log::debug!("Response HTTP code: {}", response.status());
        log::debug!("Response contains the following headers:");
        for (ref header, value) in response.headers() {
            log::debug!("  {}: {:?}", header, value);
        }

        // read empty line and "Connected"
        for _ in 0..2 {
            let data = socket.read()?;
            log::debug!("recv data: `{data}`");
        }

        Ok(Connection { socket })
    }

    fn request(&mut self, request: String) -> Result<String> {
        log::debug!("send data: {request}");
        self.socket.send(Message::Text(request))?;

        // read json response
        let response = self.socket.read()?.to_string();
        log::debug!("recv data: {response}");

        Ok(response)
    }

    pub fn get(&mut self, config: &Config) -> Result<u8> {
        let request = Request { cmd_type: CmdType::Get, cmd_num: config.brightness.num, ..Default::default() };
        log::debug!("{request:?}");
        let request = serde_json::to_string(&request)?;

        let json_data = self.request(request)?;
        let response: Response = serde_json::from_str(&json_data)?;
        log::debug!("{response:?}");

        response.data.ok_or(AppError::MissingData.into())
    }

    pub fn set(&mut self, config: &Config, value: u8) -> Result<bool> {
        let request =
            Request { cmd_type: CmdType::Set, cmd_num: config.brightness.num, cmd_ctx: Some(CmdCtx { value }) };
        log::debug!("{request:?}");
        let request = serde_json::to_string(&request)?;

        let json_data = self.request(request)?;
        let response: Response = serde_json::from_str(&json_data)?;
        log::debug!("{response:?}");

        Ok(response.res_code == 200)
    }
}
