use crate::commons::editor::ErrorKind;
use core::{
    descriptor::message::MethodMessage, MethodDescriptor, ProtoDescriptor, ProtoTuiConfig,
    ServiceDescriptor,
};
use http::Uri;
use std::error::Error;

/// The [CoreClient] calls the proto descriptor and gRPC client of the
/// core package.  
#[derive(Debug, Clone)]
pub struct CoreClient {
    /// The proto descriptor
    desc: ProtoDescriptor,
    /// Config to create a new gRPC client
    grpc: GrpcClientConfig,
}

#[derive(Debug, Clone)]
struct GrpcClientConfig(ProtoTuiConfig);

impl CoreClient {
    pub fn new(cfg: ProtoTuiConfig) -> Result<Self, Box<dyn Error>> {
        let desc = ProtoDescriptor::from_config(&cfg)?;
        let grpc = GrpcClientConfig(cfg);
        Ok(Self { desc, grpc })
    }

    /// Return the proto Services
    pub fn get_services(&self) -> Vec<ServiceDescriptor> {
        self.desc.get_services()
    }

    /// Returns the proto methods of a given service
    pub fn get_methods(&self, service: &ServiceDescriptor) -> Vec<MethodDescriptor> {
        self.desc.get_methods(service)
    }

    /// Returns the proto request of a given method
    pub fn get_request(&self, method: &MethodDescriptor) -> MethodMessage {
        self.desc.get_request(method)
    }

    /// Returns the default address as defined in the config.json
    pub fn get_default_address(&self) -> String {
        self.grpc.0.address.clone()
    }

    /// Makes a unary grpc call with a given Message and Method which is
    /// defined in ProtoMessage
    pub fn call_unary(
        &self,
        req: &MethodMessage,
        address: &str,
    ) -> Result<MethodMessage, ErrorKind> {
        let uri = Uri::try_from(address).map_err(|_| ErrorKind {
            kind: "ParseAddressError".to_string(),
            msg: "".to_string(),
        })?;
        let resp = core::call_unary_blocking(&self.grpc.0, uri, req)?;
        Ok(resp)
    }
}
