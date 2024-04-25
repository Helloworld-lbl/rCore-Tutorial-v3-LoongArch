static TARGET_PATH: &str = "../user/target/loongarch64-unknown-none/release/";

fn main() {
    println!("cargo:rerun-if-changed=../user/src/");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
}
