use crate::webpage_build_imports::*; // Import all items from webpage_build_imports

// Define a template context struct
#[derive(Template)]
#[template(path = "game_list.html")]
pub struct IndexTemplate<'a> {
    pub content: &'a str,
}

pub fn render_template() {
    // Create an instance of the template context
    let context = IndexTemplate {
        content: "This is a static HTML page generated by Rust.",
    };

    // Render the template to a string
    let rendered = context.render().expect("Failed to render template");

    // Define the output directory and file path
    let output_dir = "webpage";
    let output_file = format!("{}/index.html", output_dir);

    // Create the output directory if it doesn't exist
    if !Path::new(output_dir).exists() {
        fs::create_dir(output_dir).expect("Unable to create directory");
    }

    // Save the rendered HTML to a file
    fs::write(&output_file, rendered).expect("Unable to write file");
}