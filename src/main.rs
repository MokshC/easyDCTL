use easy_dctl::ColorProfile;
use easy_dctl::get_name;

fn main() {

	let input: ColorProfile = easy_dctl::get_color("input");	// create input color profile
	let output: ColorProfile = easy_dctl::get_color("output");	// create output color profile
	
	println!("Input  || {input}\nOutput || {output}");			// print profiles for user
	
	let filename = get_name(&input, &output);
	
	println!("{filename}");
	
	// let directory = dirs::download_dir;			// brings in directory path of downloads folder
	// let path = directory.join(filename);
	

}

