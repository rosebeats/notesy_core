fn main() {
    prost_build::compile_protos(&["protobufs/threading.proto"],
                                &["protobufs/"]).unwrap();
}
