use tonic::{transport::Server, Request, Response, Status};

use hello_world::greeter_server::{Greeter, GreeterServer};
use hello_world::{HelloReply, HelloRequest, SumReply, SumRequest};

pub mod hello_world {
    tonic::include_proto!("helloworld");
}

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, r: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        println!("Got a request: {:?}", r);

        let reply = hello_world::HelloReply {
            message: format!("Hello {}!", r.into_inner().name).into(),
        };

        Ok(Response::new(reply))
    }

    async fn sum(&self, request: Request<SumRequest>) -> Result<Response<SumReply>, Status> {
        println!("Got a request: {:?}", request);
        let data = request.into_inner();
        let reply = hello_world::SumReply {
            total: (data.first + data.second),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
