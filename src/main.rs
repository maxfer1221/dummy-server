use tiny_http::{Server, Response, Header};
use std::io::Cursor;

fn main() {

    let ip = String::from("0.0.0.0:8000");

    // tiny_http
    let server = Server::http("0.0.0.0:8000").unwrap();

    println!("Server listening on {}", ip);

    // Serve requests
    for request in server.incoming_requests() {
        let route = request.url().split('?').collect::<Vec<&str>>().to_vec()[0];

        let response: Response<Cursor<Vec<u8>>> = match route {
            "/" => {
                match request.url().split('=').collect::<Vec<&str>>().to_vec().get(1) {
                    Some(id) => {
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
                    _ => {
                        let mut res = Response::from_string::<String>(String::from("route not found"));
                        res.add_header(Header::from_bytes(
                            &b"Content-Type"[..], &b"text/plain"[..]
                        ).unwrap());
                        res
                    }
                }
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
