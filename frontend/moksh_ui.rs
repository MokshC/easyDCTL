use gtk::prelude::*;
use gtk::{glib, Application, Button, DropDown, Box, Orientation, StringList};

const APP_ID: &str = "org.moksh.easyDCTL";



fn main() -> glib::ExitCode {
    // Create a new application
    let app = Application::builder()
    	.application_id(APP_ID)
    	.build();

    // Connect to "activate" signal of `app`
    app.connect_activate(build_ui);

    // Run the application
    app.run()
}

// shortcut to create boxes either horizontally or vertically
fn box_shortcut(orientation: gtk::Orientation, align: gtk::Align, spacing: i32) -> gtk::Box {
    // Set up box
    let gtk_box = Box::builder()
    	.margin_top(1)
        .margin_bottom(1)
        .margin_start(12)
        .margin_end(12)
        .spacing(spacing)
        .halign(align)
        .valign(gtk::Align::Center)
        .orientation(orientation)
        .build();
	gtk_box
}

fn build_ui(app: &Application) {
	
	// Build an app window
	let window = gtk::ApplicationWindow::new(app);
	window.set_title(Some("easyDCTL"));
	// window.set_default_size(100, 10);
	
	
	// Create Labels for everything
	let ics_label = gtk::Label::default();
    ics_label.set_text("Input Color Space:");
	let ig_label = gtk::Label::default();
    ig_label.set_text("Input Gamma:");
	let ocs_label = gtk::Label::default();
    ocs_label.set_text("Output Color Space:");
	let og_label = gtk::Label::default();
    og_label.set_text("Output Gamma:");
    
	
	// Create a StringList of color spaces and gammas for the dropdowns
	let colorspaces_vec = vec!["ACES AP0", "ARRI Wide Gamut 3", "RED Wide Gamut RGB", "Sony S-Gamut3.Cine"];
	let colorspaces_strlist = StringList::new(colorspaces_vec.clone().as_slice());
	let gamma_vec = vec!["Linear", "ARRI LogC3", "RED Log3G10", "Sony S-Log3"];
	let gamma_strlist = StringList::new(gamma_vec.clone().as_slice());

	// Create expression (not sure why but we need it)
	let exp = gtk::PropertyExpression::new(
    	gtk::StringObject::static_type(),
    	None::<gtk::Expression>,
    	"string",
	);
	
	// create dropdown menu
	let ics_dropdown = DropDown::new(Some(colorspaces_strlist.clone()), Some(exp.clone()));
	let ig_dropdown = DropDown::new(Some(gamma_strlist.clone()), Some(exp.clone()));
	let ocs_dropdown = DropDown::new(Some(colorspaces_strlist), Some(exp.clone()));
	let og_dropdown = DropDown::new(Some(gamma_strlist), Some(exp));

	
    // Create a button with label and margins
    let create_button = Button::builder()
        .label("Create DCTL!")
        .margin_top(5)
        .margin_bottom(12)
        .margin_start(12)
        .margin_end(12)
        .build();

	// create swap button
	let swap_button = Button::builder()
		.label("Swap (not working)")
        .margin_top(12)
        .margin_bottom(0)
        .margin_start(12)
        .margin_end(12)
        .build();
    
    let label_column = box_shortcut(Orientation::Vertical, gtk::Align::Start, 27);
    label_column.append(&ics_label);
    label_column.append(&ig_label);
    label_column.append(&ocs_label);
    label_column.append(&og_label);
    
    let drop_column = box_shortcut(Orientation::Vertical, gtk::Align::End, 12);
    drop_column.append(&ics_dropdown);
    drop_column.append(&ig_dropdown);
    drop_column.append(&ocs_dropdown);
    drop_column.append(&og_dropdown); 

	let rows_box = box_shortcut(Orientation::Horizontal, gtk::Align::Fill, 12);
	rows_box.append(&label_column);
	rows_box.append(&drop_column);

	let gtk_box = box_shortcut(Orientation::Vertical, gtk::Align::Center, 12);
	gtk_box.append(&rows_box);
	gtk_box.append(&swap_button);
	gtk_box.append(&create_button);
	

    // add box to window
	window.set_child(Some(&gtk_box));

    // Present window
    window.present();
    
    /* 

	I HAVE NO IDEA HOW TO DO THIS

    // give swap functionality to button
    swap_button.connect_clicked(|ics_dropdown| {
    	let in_color = ics_dropdown.selected();
    	let out_color = ocs_dropdown.selected();
    	ocs_dropdown.set_selected(in_color);
    	ics_dropdown.set_selected(out_color);
    });
	*/
	    
    // Connect to "clicked" signal of `button`
    create_button.connect_clicked(move |create_button| {
        // Set the label to "Created!" after the button has been clicked on
        create_button.set_label("Creating...");
        println!("{:#?}", ics_dropdown.selected_item().unwrap());
    });

}
