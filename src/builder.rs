use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use easy_dctl::ColorProfile;

fn clean(dirty: String) -> String {
	dirty.replace(" ", "").replace("-", "").replace(".", "")
}

fn get_transform(input: ColorProfile, output: ColorProfile) {

	println!("Finding Transform");

	let ics = format!("./assets/{}_to_ACESAP0.cs", clean(input.colorspace));
	let ocs = format!("./assets/ACESAP0_to_{}.cs", clean(output.colorspace));
	let ig = format!("./assets/{}_to_Linear.cs", clean(input.gamma));
	let og = format!("./assets/Linear_to_{}.cs", clean(output.gamma));

	let mut transform = include_str!(ig.as_str());

}

pub fn create_file(path: PathBuf, input: ColorProfile, output: ColorProfile) {

	println!("Creating DCTL");					// print for user

    let bytes = include_bytes!("./assets/template_start.cs");
    get_transform(input, output);
    
    // println!("{new}");
    
	let file = File::create(path);
	let _ = file.expect("Couldn't open file").write_all(bytes);
	
	
 
}
