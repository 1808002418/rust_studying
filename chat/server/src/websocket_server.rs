use std::collections::HashMap;
use std::io::Error;
use std::net::SocketAddr;
use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

const SOCKET_SERVER: Token = Token(111);
const CLIENT_SERVER: Token = Token(112);

pub fn run_socket_server(addr:SocketAddr){
    loop_event(addr);
}

fn loop_event(addr:SocketAddr)->Result<(),Error>{
    // Create a poll instance.
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(128);
    // Setup the server socket.
    let mut server = TcpListener::bind(addr)?;
    // Start listening for incoming connections.
    poll.registry()
        .register(&mut server, SOCKET_SERVER, Interest::READABLE)?;

    // Start an event loop.
    // let mut map = HashMap::new();
/*    loop {
        // Poll Mio for events, blocking until we get an event.
        poll.poll(&mut events, None)?;
        // Process each event.
        for event in events.iter() {
            // We can use the token we previously provided to `register` to
            // determine for which socket the event is.
            match event.token() {
                SOCKET_SERVER => {
                    let mut connection = server.accept()?;
                    poll.registry().register(&mut connection,CLIENT_SERVER, Interest::READABLE)?;
                    map.insert(event.token(),connection);
                }
                CLIENT_SERVER=>{
                    let x = map.get(&event.token()).unwrap();
                }
                // We don't expect any events with tokens other than those we provided.
                _ => unreachable!(),
            }
        }
    }*/
    return Ok(());
}

fn handle(){

}
