use axum::http::StatusCode;

pub trait HttpCode {
    fn status(&self) -> StatusCode;
    fn code(&self) -> &'static str;
    fn message(&self) -> &'static str;
}

pub enum Http2xx {
    Ok,
    Created,
}

impl HttpCode for Http2xx {
    fn status(&self) -> StatusCode {
        match self {
            Http2xx::Ok => StatusCode::OK,
            Http2xx::Created => StatusCode::CREATED,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Http2xx::Ok => "S001",
            Http2xx::Created => "S002",
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Http2xx::Ok => "성공",
            Http2xx::Created => "생성 완료",
        }
    }
}
