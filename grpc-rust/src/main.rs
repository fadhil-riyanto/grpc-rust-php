use tonic::{transport::Server, Request, Response, Status};
use std::net::{SocketAddr, IpAddr, Ipv4Addr, Ipv6Addr};

pub mod helloworld {
    tonic::include_proto!("helloworld");
}

use helloworld::greeter_server::{Greeter, GreeterServer};
use helloworld::{HelloReply, HelloRequest};

#[derive(Debug, Default)]
struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self,
        request: tonic::Request<HelloRequest>) -> std::result::Result<tonic::Response<HelloReply>, tonic::Status> {
        
        println!("got a request, {:?}", request);

        let reply = HelloReply{
            message: "hello too".to_string()
        };

        Ok(Response::new(reply))
    }
}


#[tokio::main]
async fn main()-> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 8998);
    let greeter = MyGreeter::default();
    
    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
