pub mod nwc;

use axum::response::IntoResponse;
use derive_builder::Builder;
use serde::Serialize;
use std::borrow::Cow;
// #[derive(Default, Debug, Serialize, Builder)]
// #[builder(setter(into))]
// pub struct ResponseData<'a, T: Serialize> {
//     success: &'a str,
//     message: Cow<'a, str>,
//     data: T,
//     error: Cow<'a, str>,
// }
#[derive(Default, Debug, Serialize, Builder)]
#[builder(setter(into))]
pub struct ResponseData<'a, T: Serialize> {
    success: &'a str,
    message: Cow<'a, str>,
    data: Vec<T>,
    error: Cow<'a, str>,
}

pub fn into_axum_error_response(error_message: &str) -> axum::http::Response<axum::body::Body> {
    let data_vec = vec!["".to_string()];
    let response = ResponseDataBuilder::<String>::default()
        .success("false")
        .error(error_message)
        .message("")
        .data(data_vec)
        .build()
        .unwrap();
    axum::response::Json(response).into_response()
}

pub fn into_axum_success_response<T: Serialize>(
    data: Vec<T>,
) -> axum::http::Response<axum::body::Body>
where
    T: Serialize + Clone,
{
    let response = ResponseDataBuilder::<T>::default()
        .success("true")
        .error("")
        .message("")
        .data(data)
        .build()
        .unwrap();
    axum::response::Json(response).into_response()
}
