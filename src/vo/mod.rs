use std::fmt::Debug;

use rbatis::rbdc::db::ExecResult;
use rbatis::rbdc::Error;
use serde::Serialize;

pub mod menu_vo;
pub mod role_vo;
pub mod user_vo;

// 统一返回vo
#[derive(Serialize, Debug, Clone)]
pub struct BaseResponse<T>
where
    T: Serialize + Debug,
{
    pub code: i32,
    pub msg: String,
    pub data: Option<T>,
}

// 处理统一返回
pub fn handle_result(result: Result<ExecResult, Error>) -> BaseResponse<String> {
    match result {
        Ok(_u) => ok_result(),
        Err(err) => err_result_msg(err.to_string()),
    }
}

pub fn ok_result() -> BaseResponse<String> {
    BaseResponse {
        msg: "操作成功".to_string(),
        code: 200,
        data: None,
    }
}

pub fn ok_result_msg(msg: String) -> BaseResponse<String> {
    BaseResponse {
        msg: msg.to_string(),
        code: 0,
        data: None,
    }
}

pub fn ok_result_code(code: i32, msg: String) -> BaseResponse<String> {
    BaseResponse {
        msg: msg.to_string(),
        code,
        data: None,
    }
}

pub fn ok_result_data<T: Serialize + Debug>(data: T) -> BaseResponse<T> {
    BaseResponse {
        msg: "操作成功".to_string(),
        code: 200,
        data: Some(data),
    }
}

pub fn err_result_msg(msg: String) -> BaseResponse<String> {
    BaseResponse {
        msg: msg.to_string(),
        code: 100,
        data: None,
    }
}

pub fn err_result_code(code: i32, msg: String) -> BaseResponse<String> {
    BaseResponse {
        msg: msg.to_string(),
        code,
        data: None,
    }
}

// 统一返回分页
#[derive(Serialize, Debug, Clone)]
pub struct ResponsePage<T>
where
    T: Serialize + Debug,
{
    pub code: i32,
    pub msg: String,
    pub total: u64,
    pub success: bool,
    pub data: Option<T>,
}

pub fn ok_result_page<T: Serialize + Debug>(data: T, total: u64) -> ResponsePage<T> {
    ResponsePage {
        msg: "操作成功".to_string(),
        code: 0,
        success: true,
        data: Some(data),
        total,
    }
}

pub fn err_result_page<T: Serialize + Debug>(data: T, msg: String) -> ResponsePage<T> {
    ResponsePage {
        msg: msg.to_string(),
        code: 1,
        success: false,
        data: Some(data),
        total: 0,
    }
}
