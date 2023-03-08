use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Har {
    pub log: Log,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Log {
    pub version: String,
    pub creator: Creator,
    pub browser: Browser,
    pub pages: Vec<Page>,
    pub entries: Vec<Entry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Creator {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Browser {
    pub name: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    pub started_date_time: String,
    pub id: String,
    pub title: String,
    pub page_timings: PageTimings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PageTimings {
    pub on_content_load: i64,
    pub on_load: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub pageref: String,
    pub started_date_time: String,
    pub request: Request,
    pub response: Response,
    pub cache: Cache,
    pub timings: Timings,
    pub time: i64,
    #[serde(rename = "_securityState")]
    pub security_state: Option<String>,
    #[serde(rename = "serverIPAddress")]
    pub server_ipaddress: Option<String>,
    pub connection: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    pub body_size: i64,
    pub method: String,
    pub url: String,
    pub http_version: String,
    pub headers: Vec<Header>,
    pub cookies: Vec<Value>,
    pub query_string: Vec<Value>,
    pub headers_size: Option<i64>,
    pub post_data: Option<PostData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostData {
    pub mime_type: String,
    pub params: Vec<Value>,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub status: i64,
    pub status_text: String,
    pub http_version: String,
    pub headers: Vec<Header>,
    pub cookies: Vec<Value>,
    pub content: Content,
    #[serde(rename = "redirectURL")]
    pub redirect_url: String,
    pub headers_size: Option<i64>,
    pub body_size: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub mime_type: String,
    pub size: Option<i64>,
    pub text: Option<String>,
    pub encoding: Option<String>,
    pub comment: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cache {
    pub after_request: Option<AfterRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AfterRequest {
    pub expires: String,
    pub last_fetched: String,
    pub fetch_count: String,
    #[serde(rename = "_dataSize")]
    pub data_size: String,
    #[serde(rename = "_lastModified")]
    pub last_modified: String,
    #[serde(rename = "_device")]
    pub device: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Timings {
    pub blocked: Option<i64>,
    pub dns: Option<i64>,
    pub connect: Option<i64>,
    pub ssl: Option<i64>,
    pub send: Option<i64>,
    pub wait: Option<i64>,
    pub receive: Option<i64>,
}

