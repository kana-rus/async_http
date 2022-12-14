use crate::{
    components::{consts::BUF_SIZE, method::Method, json::JSON},
    response::Response,
    context::Context, result::Result,
};

pub(crate) fn parse_stream(
    buffer: &[u8; BUF_SIZE]
) -> Result<(Method, &str, Context)> {
    let mut lines = std::str::from_utf8(buffer)?
        .trim_end()
        .lines();

    let request_line = lines.next().ok_or_else(|| Response::BadRequest("empty request"))?;
    let (method, path) = parse_request_line(request_line)?;

    while let Some(line) = lines.next() {
        if line.is_empty() {break}
    }

    let request_context = Context {
        pool:  None,
        param: None,
        body:
            if let Some(request_body) = lines.next() {
                Some(JSON::from_str_unchecked(request_body))
            } else {None}
    };

    Ok((method, path, request_context))
}

fn parse_request_line(line: &str) -> Result<(Method, &str)> {
    if line.is_empty() {
        return Err(Response::BadRequest("can't find request status line"))
    }
    let (method, path) = line
        .strip_suffix(" HTTP/1.1")
        .ok_or_else(|| Response::NotImplemented("I can't handle protocols other than `HTTP/1.1`"))?
        .split_once(' ')
        .ok_or_else(|| Response::BadRequest("invalid request line format"))?;
    Ok((Method::parse(method)?, path))
}