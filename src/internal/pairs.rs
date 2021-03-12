use super::super::*;

pub enum Pair {
    ContentMd5(String),
    ContentType(String),
    ContinuationToken(String),
    Credential(String),
    Endpoint(String),
    Expire(isize),
    Interceptor(Interceptor),
    IoCallback(IoCallback),
    ListMode(ListMode),
    Location(String),
    MultipartId(String),
    Name(String),
    Offset(i64),
    PairPolicy(PairPolicy),
    Size(i64),
    WorkDir(String),
}