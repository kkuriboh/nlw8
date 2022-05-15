fn main() {
    #[cfg(debug_assertions)]
    std::process::Command::new("yarn")
        .arg("tailwindcss")
        .arg("-c")
        .arg("./tailwind.config.js")
        .arg("-o")
        .arg("./tailwind.css")
        .output().unwrap();

    #[cfg(not(debug_assertions))]
    std::process::Command::new("yarn")
        .env("NODE_ENV", "production")
        .arg("tailwindcss")
        .arg("-c")
        .arg("./tailwind.config.js")
        .arg("-o")
        .arg("./tailwind.css")
        .arg("--minify")
        .output().unwrap();
}
