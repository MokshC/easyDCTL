pub fn create_file(path: PathBuf) {
    let my_str = include_str!("./assets/template_start.cs");
    print!("{my_str}");
}
