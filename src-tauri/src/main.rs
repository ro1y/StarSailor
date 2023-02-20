#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]
use openssl::ssl;
use std::{
    io::{Read, Write},
    net,
};
use url::Url;

#[derive(Debug, thiserror::Error)]
enum Error {
  #[error("URL Parsing Error")]
  ParseError(#[from] url::ParseError),
  #[error("Handshake Error")]
  HandshakeError(#[from] ssl::HandshakeError<net::TcpStream>),
  #[error("IO Error")]
  IoError(#[from] std::io::Error)
}
impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

#[tauri::command]
fn fetch(url: &str) -> Result<String, Error> {
    let url = Url::parse(url)?;
    let addr = format!(
        "{}:{}",
        url.host().expect("URL must contain a host"),
        url.port().or(Some(1965)).unwrap().to_string()
    );

    let mut builder =
        ssl::SslConnector::builder(ssl::SslMethod::tls()).expect("Failed to register builder");
    builder.set_verify(ssl::SslVerifyMode::NONE);
    let connector = builder.build();

    let stream = net::TcpStream::connect(&addr)?;
    let mut stream = connector.connect(&addr, stream)?;

    stream.write_all(format!("{}\r\n", url.to_string()).as_bytes())?;
    let mut res = vec![];
    stream.read_to_end(&mut res)?;

    Ok(String::from_utf8_lossy(&res).into_owned())
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![fetch])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
