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

fn main() {
    let proto_files = vec!["./protos/route_guide.proto"];

    protoc_rust::Codegen::new()
        .out_dir("./src")
        .inputs(proto_files)
        .run()
        .expect("running protoc failed");
}
