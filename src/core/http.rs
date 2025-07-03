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

pub enum Http4xx {
    BadRequest,
    UserNotFound,
}

impl HttpCode for Http4xx {
    fn status(&self) -> StatusCode {
        match self {
            Http4xx::BadRequest => StatusCode::BAD_REQUEST,
            Http4xx::UserNotFound => StatusCode::NOT_FOUND,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Http4xx::BadRequest => "F001",
            Http4xx::UserNotFound => "F002",
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Http4xx::BadRequest => "파라미터 에러",
            Http4xx::UserNotFound => "사용자를 찾을 수 없습니다.",
        }
    }
}
