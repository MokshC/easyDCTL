use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use easy_dctl::ColorProfile;

fn get_transform(input: ColorProfile, output: ColorProfile) {

	input.

}

pub fn create_file(path: PathBuf, input: ColorProfile, output: ColorProfile) {

    let bytes = include_bytes!("./assets/template_start.cs");
	let file = File::create(path);	
	
	let _ = file.expect("Couldn't open file").write_all(bytes);
 
}