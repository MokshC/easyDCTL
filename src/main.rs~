use easy_dctl::ColorProfile;
use easy_dctl::get_name;
use crate::builder::create_file;

pub mod builder;

fn main() {

	let input: ColorProfile = easy_dctl::get_color("input");	// create input color profile
	let output: ColorProfile = easy_dctl::get_color("output");	// create output color profile
	
	println!("Input  || {input}\nOutput || {output}");			// print profiles for user
	
	let filename = get_name(&input, &output);					// get filename based on color profiled
	let directory = dirs::download_dir().unwrap();				// get downloads folder
	let path = directory.join(filename);						// add filename to downloads folder path
	
	println!("{}", &path.display());
	
	create_file(path, input, output);
	

}

