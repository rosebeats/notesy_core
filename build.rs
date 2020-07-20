fn main() {
    prost_build::compile_protos(&["protobuf/threading.proto"],
                                &["protobuf/"]).unwrap();
}
