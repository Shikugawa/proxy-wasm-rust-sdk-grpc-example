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

mod route_guide;

use log::warn;
use protobuf::Message;
use proxy_wasm::traits::*;
use proxy_wasm::types::*;
use route_guide::{Point, Rectangle};

#[no_mangle]
pub fn _start() {
    proxy_wasm::set_log_level(LogLevel::Trace);
    proxy_wasm::set_http_context(|_, _| -> Box<dyn HttpContext> {
        Box::new(GrpcStreamTest::new())
    });
}

struct GrpcStreamTest {
    sent_message_counter: u32,
}

impl GrpcStreamTest {
    fn new() -> Self {
        return GrpcStreamTest {
            sent_message_counter: 0,
        };
    }

    fn build_request(&self) -> Vec<u8> {
        let mut req = Rectangle::new();

        let mut lo = Point::new();
        lo.set_latitude(400000000);
        lo.set_longitude(-750000000);

        let mut hi = Point::new();
        hi.set_latitude(400000000);
        hi.set_longitude(-750000000);

        req.set_lo(lo);
        req.set_hi(hi);
        req.write_to_bytes().unwrap()
    }
}

impl HttpContext for GrpcStreamTest {
    fn on_http_request_headers(&mut self, _: usize) -> Action {
        let message = self.build_request();
        let mut stream_token = 0;

        match self.create_grpc_stream("test", "routeguide.RouteGuide", "ListFeatures", "") {
            Ok(token) => {
                stream_token = token;
                warn!("success token {:?}", stream_token);
            }
            Err(e) => warn!("Failed {:?}", e),
        }

        unsafe {
            match self.grpc_send(
                stream_token,
                String::from_utf8_unchecked(message).as_mut(),
                false,
            ) {
                Ok(()) => {
                    warn!("success send");
                    self.sent_message_counter += 1;
                }
                Err(e) => {
                    warn!("Failed {:?}", e);
                    return Action::Continue;
                }
            }
        }

        Action::Pause
    }

    fn on_http_response_headers(&mut self, _: usize) -> Action {
        self.set_http_response_header("Powered-By", Some("proxy-wasm"));
        Action::Continue
    }
}

impl Context for GrpcStreamTest {
    fn on_grpc_receive(&mut self, token_id: u32, _response_size: usize) {
        if self.sent_message_counter >= 5 {
            match self.grpc_close(token_id) {
                Ok(_) => warn!("succeeded to cancel stream {:?}", token_id),
                Err(e) => warn!("Failed {:?}", e),
            }
        } else {
            let message = self.build_request();

            unsafe {
                match self.grpc_send(
                    token_id,
                    String::from_utf8_unchecked(message).as_mut(),
                    false,
                ) {
                    Ok(()) => {
                        warn!("success send");
                        self.sent_message_counter += 1;
                    }
                    Err(e) => warn!("Failed {:?}", e),
                }
            }
        }
    }

    fn on_grpc_close(&mut self, _token_id: u32, _status_code: u32) {
        warn!("grpc close");
        self.resume_http_request()
    }

    fn on_grpc_receive_initial_metadata(&mut self, _token_id: u32, _headers: u32) {
        warn!("grpc initial metadata receive");
    }

    fn on_grpc_receive_trailing_metadata(&mut self, _token_id: u32, _traillers: u32) {
        warn!("grpc trailing metadata receive");
    }
}
