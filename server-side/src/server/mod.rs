use std::env::Args;

use crate::error::NoSettingFoundError;


pub struct Server {
    port: u16,
    server_name: String,
}

impl Server {
    pub fn new(args: Args, default_settings: String) -> Result<Server, NoSettingFoundError> {
        let port = default_settings
            .lines().find(|&x| x.contains("port=")).ok_or(NoSettingFoundError("port"))?;
        let port = port.strip_prefix("port=").unwrap().to_string();

        let server_name = default_settings
            .lines().find(|&x| x.contains("server_name=")).ok_or(NoSettingFoundError("server name"))?;
        let server_name = server_name.strip_prefix("server_name=").unwrap().to_string();

        for arg in args {

        };

        Ok(Server {
            port,
            server_name: String::from("D"),
        }) 
    }
}