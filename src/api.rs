use std::cell::RefCell;
// use std::io;

// use futures::{Future, Stream};
use hyper::client::HttpConnector;
use hyper::Client;
use hyper_tls::HttpsConnector;
use serde::*;
use tokio_core::reactor::Core;
use url::Url;

pub struct WebHDFSUrl {
    schema: String,
    host: String,
    port: u8,
    user: String,
}

/// type alias for a custom hyper client, configured for HTTPS
/// instead of HTTP.
type HttpsClient = Client<HttpsConnector<HttpConnector>, hyper::Body>;

/// The top level interface for interacting with the remote service.
pub struct WebHDFSClient {
    /// The `UriMaker` we built in Part 1 of the series.
    //uri_maker: UriMaker,
    uri: Url,
    /// tokio "core" to run our requests in.
    core: RefCell<Core>,
    /// hyper http client to build requests with.
    http: HttpsClient,
}

impl WebHDFSClient {
    pub fn new(url_str: &str) -> WebHDFSClient {
        let uri = Url::parse(url_str).unwrap();

        let core = Core::new().unwrap();

        let http = {
            let connector = HttpsConnector::new(4).unwrap();
            Client::builder().build(connector)
        };

        WebHDFSClient {
            uri,
            core: RefCell::new(core),
            http,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum FsType {
    File,
    Directory,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileStatus {
    pub access_time: u64,
    pub block_size: u64,
    pub group: String,
    pub length: u64,
    pub modification_time: u64,
    pub owner: String,
    pub path_suffix: String,
    pub permission: String,
    pub replication: u8,
    #[serde(rename = "type")]
    pub fstype: FsType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileStatuses {
    pub file_status: Vec<FileStatus>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ContentSummary {
    pub directory_count: u64,
    pub file_count: u64,
    pub length: u64,
    pub quota: i8,
    pub space_consumed: u64,
    pub space_quota: i8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FileChecksum {
    pub algorithm: String,
    pub bytes: String,
    pub length: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub url_string: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ExceptionType {
    IllegalArgumentException,
    SecurityException,
    AccessControlException,
    FileNotFoundException,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Exception {
    exception: ExceptionType,
    java_class_name: String,
    message: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Responce {
    FileStatuses(FileStatuses),
    FileStatus(FileStatus),
    ContentSummary(ContentSummary),
    FileChecksum(FileChecksum),
    Path(String),
    #[serde(rename = "boolean")]
    Replication(bool),
    Token(Token),
    #[serde(rename = "long")]
    Expiration(u64),
    RemoteException(Exception),
}
