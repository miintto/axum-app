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
    Unauthenticated,
    PermissionDenied,
    InvalidParameter,
    UserNotFound,
    PasswordMismatched,
    DuplicatedEmail,
    AuthenticationFail,
}

impl HttpCode for Http4xx {
    fn status(&self) -> StatusCode {
        match self {
            Http4xx::BadRequest => StatusCode::BAD_REQUEST,
            Http4xx::Unauthenticated => StatusCode::UNAUTHORIZED,
            Http4xx::PermissionDenied => StatusCode::FORBIDDEN,
            Http4xx::InvalidParameter => StatusCode::UNPROCESSABLE_ENTITY,
            Http4xx::UserNotFound => StatusCode::NOT_FOUND,
            Http4xx::PasswordMismatched => StatusCode::NOT_FOUND,
            Http4xx::DuplicatedEmail => StatusCode::NOT_FOUND,
            Http4xx::AuthenticationFail => StatusCode::UNPROCESSABLE_ENTITY,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Http4xx::BadRequest => "F001",
            Http4xx::Unauthenticated => "F002",
            Http4xx::PermissionDenied => "F003",
            Http4xx::InvalidParameter => "F004",
            Http4xx::UserNotFound => "F005",
            Http4xx::PasswordMismatched => "F006",
            Http4xx::DuplicatedEmail => "F007",
            Http4xx::AuthenticationFail => "F008",
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Http4xx::BadRequest => "잘못된 요청",
            Http4xx::Unauthenticated => "인증 실패",
            Http4xx::PermissionDenied => "권한이 없습니다",
            Http4xx::InvalidParameter => "파라미터 에러",
            Http4xx::UserNotFound => "사용자를 찾을 수 없습니다.",
            Http4xx::PasswordMismatched => "패스워드가 서로 일치하지 않습니다.",
            Http4xx::DuplicatedEmail => "이미 사용중인 이메일입니다.",
            Http4xx::AuthenticationFail => "이메일 혹은 비밀번호가 일치하지 않습니다.",
        }
    }
}

pub enum Http5xx {
    ServerError,
}

impl HttpCode for Http5xx {
    fn status(&self) -> StatusCode {
        match self {
            Http5xx::ServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn code(&self) -> &'static str {
        match self {
            Http5xx::ServerError => "E001",
        }
    }

    fn message(&self) -> &'static str {
        match self {
            Http5xx::ServerError => "서버 에러",
        }
    }
}
