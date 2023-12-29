mod server;
mod http_server;
mod websocket_server;

use std::error::Error;
use std::time::Duration;

use mio::net::{TcpListener, TcpStream};
use mio::{Events, Interest, Poll, Token};

// Some tokens to allow us to identify which event is for which socket.

const CLIENT: Token = Token(1);
const SERVER: Token = Token(12);

fn main() -> Result<(), Box<dyn Error>> {
    // Create a poll instance.
    let mut poll = Poll::new()?;
    // Create storage for events.
    let mut events = Events::with_capacity(128);

    // Setup the server socket.
    let addr = "127.0.0.1:7788".parse()?;
    let mut server = TcpListener::bind(addr)?;
    // Start listening for incoming connections.
    poll.registry()
        .register(&mut server, SERVER, Interest::READABLE)?;

    // Setup the client socket.
    // let mut client = TcpStream::connect(addr)?;
    // Register the socket.
    // poll.registry()
    //     .register(&mut client, CLIENT, Interest::READABLE | Interest::WRITABLE)?;

    // Start an event loop.
    println!("start");
    loop {
        // Poll Mio for events, blocking until we get an event.
        println!("loop");
        let result = poll.poll(&mut events, None);

        println!("poll");
        // Process each event.
        for event in events.iter() {
            println!("1");
            // We can use the token we previously provided to `register` to
            // determine for which socket the event is.
            match event.token() {
                SERVER => {
                    // If this is an event for the server, it means a connection
                    // is ready to be accepted.
                    //
                    // Accept the connection and drop it immediately. This will
                    // close the socket and notify the client of the EOF.
                    match server.accept() {
                        Ok(stream) => {
                            println!("{}", stream.1);
                        }
                        Err(err) => {
                            println!("{}", err);
                        }
                    }
                    println!("SERVER");
                }
                CLIENT => {
                    if event.is_writable() {
                        // We can (likely) write to the socket without blocking.
                    }

                    if event.is_readable() {
                        // We can (likely) read from the socket without blocking.
                    }

                    // Since the server just shuts down the connection, let's
                    // just exit from our event loop.
                    println!("B");
                    // return Ok(());
                }
                // We don't expect any events with tokens other than those we provided.
                _ => {
                    println!("C")
                },
            }
        }
        println!("OV");
    }


}