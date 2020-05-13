use std::path::PathBuf;

pub fn get_resources_folder(module_path: &str) -> PathBuf {
    let mut output = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../resources");
    for module in module_path.split("::").skip(1) {
        output.push(module);
    }

    output
}
