use fv_plugin::ServicePlugin;
use axum::Router;
use tonic::{transport::Server, Request, Response, Status};
use async_trait::async_trait;
use tracing::info;

pub mod greeter {
    tonic::include_proto!("greeter");
}

#[derive(Default)]
struct GreeterService;

#[async_trait]
impl greeter::greeter_server::Greeter for GreeterService {
    async fn say_hello(
        &self,
        request: Request<greeter::HelloRequest>,
    ) -> Result<Response<greeter::HelloReply>, Status> {
        let reply = greeter::HelloReply {
            message: format!("Hello {}", request.into_inner().name),
        };
        Ok(Response::new(reply))
    }
}

pub struct GreeterPlugin;

#[async_trait]
impl ServicePlugin for GreeterPlugin {
    fn name(&self) -> &'static str { "greeter-plugin" }

    async fn routes(&self) -> Router {
        Router::new()
    }

    async fn init(&self, _registry: &service_registry::LocalServiceRegistry) -> anyhow::Result<()> {
        info!("Initializing GreeterPlugin");
        Ok(())
    }

    fn register_grpc(self: Box<Self>, server: Server) -> Server {
        server.add_service(greeter::greeter_server::GreeterServer::new(GreeterService::default()))
    }
}

#[no_mangle]
pub extern "C" fn finalverse_plugin_entry() -> *mut dyn ServicePlugin {
    Box::into_raw(Box::new(GreeterPlugin))
}
