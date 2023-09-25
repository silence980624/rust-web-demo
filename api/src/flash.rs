/// Cookie 处理
/// 
use axum::http::{header, HeaderMap, HeaderValue, StatusCode};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};

#[derive(Deserialize)]
struct ValuedMessage<T> {
    #[serde(rename = "_")]
    value: T,
}

#[derive(Serialize)]
struct ValuedMessageRef<'a, T> {
    #[serde(rename = "_")]
    value: &'a T,
}

// cookie标识
const FLASH_COOKIE_NAME: &str = "_flash";

// 获取HTTP请求cookie
pub fn get_flash_cookie<T>(cookies: &Cookies) -> Option<T>
where
    T: DeserializeOwned,
{
    cookies.get(FLASH_COOKIE_NAME).and_then(|flash_cookie| {
        if let Ok(ValuedMessage::<T> { value }) = serde_json::from_str(flash_cookie.value()) {
            Some(value)
        } else {
            None
        }
    })
}

//Http Post响应
pub type PostResponse = (StatusCode, HeaderMap);

pub fn post_response<T>(cookies: &mut Cookies, data: T) -> PostResponse
where
    T: Serialize
{
    let value_ref = ValuedMessageRef{value: &data};
    let mut cookie = Cookie::new(
            FLASH_COOKIE_NAME, 
            serde_json::to_string(&value_ref).unwrap()
        );

    cookie.set_path("/");
    cookies.add(cookie);

    let mut headers = HeaderMap::new();
    headers.insert(header::LOCATION, HeaderValue::from_static("/"));
    (StatusCode::SEE_OTHER, headers)
}