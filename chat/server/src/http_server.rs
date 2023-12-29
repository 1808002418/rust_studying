use std::collections::LinkedList;
use std::net::TcpStream;

struct WebSocketHandler {
    handlers:LinkedList<SocketHandler>,
    counter: usize,
}
struct SocketHandler{
    stream: TcpStream,
}

struct CodeStates {
    code: u16,
    description: String,
}

enum  Methods{
    GET,
    POST,
    DELETE,
    PUT,
    OPTIONS
}

struct Url{

}

struct Header{

}