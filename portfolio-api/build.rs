fn main() {
    println!("cargo:rerun-if-changed=proto/portfolio.proto");
    tonic_build::compile_protos("proto/portfolio.proto").unwrap();
}
