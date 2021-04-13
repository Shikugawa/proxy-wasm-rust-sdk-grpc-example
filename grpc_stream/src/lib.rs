// Copyright 2021 Rei Shimizu
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

mod helloworld;

use helloworld::HelloRequest;
use log::warn;
use protobuf::Message;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> { Box::new(GrpcCallTest) });
}

struct GrpcCallTest;

impl HttpContext for GrpcCallTest {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        let mut req = HelloRequest::new();
        req.set_name("John Smith".to_string());
        let message = req.write_to_bytes().unwrap();

        let mut stream_token = 0;

        match self.create_grpc_stream("test", "helloworld.Greeter", "SayHello", "") {
            Ok(token) => {
                stream_token = token;
                warn!("success token {:?}", stream_token);
            }
            Err(e) => warn!("Failed {:?}", e),
        }

        match self.grpc_send(
            stream_token,
            String::from_utf8(message).unwrap().as_mut(),
            false,
        ) {
            Ok(_) => warn!("success send"),
            Err(e) => warn!("Failed {:?}", e),
        }

        Action::Pause
    }

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        self.set_http_response_header("Powered-By", Some("proxy-wasm"));
        Action::Continue
    }
}

impl Context for GrpcCallTest {
    fn on_grpc_receive(&mut self, token_id: u32, response_size: usize) {
        warn!("grpc receive");
        warn!("{}", token_id.to_string());
        warn!("{}", response_size.to_string());
    }

    fn on_grpc_close(&mut self, _token_id: u32, _status_code: u32) {
        warn!("grpc close");
        warn!("{}", _token_id.to_string());
        warn!("{}", _status_code.to_string());
        // self.resume_http_request()
    }

    fn on_grpc_receive_initial_metadata(&mut self, _token_id: u32, _headers: u32) {
        warn!("grpc initial metadata receive");
        warn!("{}", _token_id.to_string());
        warn!("{}", _headers.to_string())
    }

    fn on_grpc_receive_trailing_metadata(&mut self, _token_id: u32, _traillers: u32) {
        warn!("grpc trailing metadata receive");
        warn!("{}", _token_id.to_string());
        warn!("{}", _traillers.to_string());
        
        // match self.grpc_close(_token_id) {
        //   Ok(_) => warn!("succeded to close {:?}", _token_id),
        //   Err(status) => warn!("failed with status {:?}", status)
        // }
    }
}
