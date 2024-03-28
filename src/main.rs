use serde_lexpr::to_string;
fn main() {
    // get environment variable
    let pypath = std::env::var("PYTHONPATH");
    let pypath_vec = pypath
        .as_ref()
        .map_or(Vec::new(), |v| v.split(":").collect::<Vec<&str>>());
    println!("PYTHONPATH: {}", to_string(&pypath_vec).unwrap_or_default());
}
