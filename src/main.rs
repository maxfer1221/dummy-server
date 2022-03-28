use tiny_http::{Server, Response, Request, Method, Header};
use std::io::Cursor;

fn main() {

    // tiny_http
    let server = Server::http("127.0.0.1:8000").unwrap();

    // Serve requests
    for request in server.incoming_requests() {
        let route = request.url().split('?').collect::<Vec<&str>>().to_vec()[0];

        let response: Response<Cursor<Vec<u8>>> = match route {
            "/" => {
                let id = request.url().split('=').collect::<Vec<&str>>().to_vec()[1];

                let mut res = Response::from_string::<String>(
                    format!(
                        "{}\"msg\":\"Hello from server id: {}!\"{}",
                        '{', id, '}'
                    )
                );
                res.add_header(Header::from_bytes(
                    &b"Content-Type"[..], &b"application/json"[..]
                ).unwrap());
                res
            },
            "/ping" => {
                let mut res = Response::from_string::<String>(String::from("pong"));
                res.add_header(Header::from_bytes(
                    &b"Content-Type"[..], &b"text/plain"[..]
                ).unwrap());
                res
            },
            _ => {
                let mut res = Response::from_string::<String>(String::from("route not found"));
                res.add_header(Header::from_bytes(
                    &b"Content-Type"[..], &b"text/plain"[..]
                ).unwrap());
                res
            }
        };

        // respond
        request.respond(response).unwrap();
    }

}
