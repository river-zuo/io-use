extern crate tokio;

use std::borrow::{Borrow, BorrowMut};
use std::ops::Add;
use std::{time, thread};

use tokio::io;
use tokio::net::TcpStream;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() {


    // as_client("127.0.0.1".to_owned(), "6142".to_owned());
    
    let addr = "0.0.0.0:6142".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method. We then define how to process each element in
    // the stream with the `for_each` combinator method
    let server = listener.incoming().for_each(|socket| {
        // TODO: Process socket
        println!("rev...");

        // split the socket stream into readable and writable parts
        let (reader, writer) = socket.split();
        // copy bytes from the reader into the writer
        let amount = io::copy(reader, writer);

        let msg = amount.then(|result| {
            match result {
                Ok((amount, _, _)) => println!("wrote {} bytes", amount),
                Err(e)             => println!("error: {}", e),
            }
            Ok(())
        });

        // spawn the task that handles the client connection socket on to the
        // tokio runtime. This means each client connection will be handled
        // concurrently
        tokio::spawn(msg);

        Ok(())
    })
    .map_err(|err| {
        // Handle error by printing to STDOUT.
        println!("accept error = {:?}", err);
    });

    println!("server running on 0.0.0.0:6142");

    // Start the server
    //
    // This does a few things:
    //
    // * Start the Tokio runtime
    // * Spawns the `server` task onto the runtime.
    // * Blocks the current thread until the runtime becomes idle, i.e. all
    //   spawned tasks have completed.
    tokio::run(server);

}



fn as_client(addr: String, port: String) {
     // Parse the address of whatever server we're talking to

     let mut ad = String::new();
     ad.push_str(addr.as_str());
     ad.push_str(":");
     ad.push_str(port.as_str());
     

     let addr = "127.0.0.1:6142".parse().unwrap();
     // let stream = TcpStream::connect(&addr);
 
     let hello_world = TcpStream::connect(&addr).and_then(|stream| {
         println!("created stream");
     
         //Process stream here
         return io::write_all(stream, "hello world\n").then(|result| {
           println!("wrote to stream; success={:?}", result.is_ok());
           thread::sleep(time::Duration::from_secs(5));
           Ok(())
         });
         
         // Ok(())
     })
     .map_err(|err| {
         // All tasks must have an 'Error' type of '()'. This forces error
         // handing and helps avoid silencing failures.
         println!("connection error = {:?}",err);
     });
     
 
     println!("About to create the stream and write to it...");
     tokio::run(hello_world);
     println!("Stream has been created and written to.");
 
     println!("Hello, world!");
}
