use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref ERROR_CODE_MESSAGE: HashMap<Error, (u16, &'static str)> = HashMap::from([
        // 未知错误
        (Error::UnknownError, (9999, "未知错误，请联系管理员")),
        // 参数错误
        (Error::ParamsError, (2000, "参数错误，请确认您的参数是否符合规范")),
    ]);
}


#[derive(Serialize, Deserialize)]
pub struct Response<D: Serialize> {
    pub code: u16,
    pub message: String,
    pub data: Option<D>,
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum Error {
    UnknownError,
    ParamsError,
}

impl<D: Serialize> Response<D> {
    pub fn success(data: D) -> Self {
        Response {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}