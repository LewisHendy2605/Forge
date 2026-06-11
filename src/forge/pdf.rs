use super::error::ForgeError;
use genpdf::fonts;

#[derive(rust_embed::RustEmbed)]
#[folder = "assets\\fonts\\LiberationSans"]
struct Fonts;

pub fn write(path: &str, records: u32) -> Result<(), ForgeError> {
    let regular_bytes = Fonts::get("LiberationSans-Regular.ttf").unwrap();
    let bold_bytes = Fonts::get("LiberationSans-Bold.ttf").unwrap();
    let italic_bytes = Fonts::get("LiberationSans-Italic.ttf").unwrap();
    let bold_italic_bytes = Fonts::get("LiberationSans-BoldItalic.ttf").unwrap();

    let regular = genpdf::fonts::FontData::new(regular_bytes.data.to_vec(), None)
        .expect("Failed to load font");
    let bold =
        genpdf::fonts::FontData::new(bold_bytes.data.to_vec(), None).expect("Failed to load font");
    let italic = genpdf::fonts::FontData::new(italic_bytes.data.to_vec(), None)
        .expect("Failed to load font");
    let bold_italic = genpdf::fonts::FontData::new(bold_italic_bytes.data.to_vec(), None)
        .expect("Failed to load font");

    let font_family = fonts::FontFamily {
        regular,
        bold,
        italic,
        bold_italic,
    };

    // Create a document and set the default font family
    let mut doc = genpdf::Document::new(font_family);

    // Change the default settings
    doc.set_title("Output Pdf");

    // Customize the pages
    let mut decorator = genpdf::SimplePageDecorator::new();
    decorator.set_margins(10);
    doc.set_page_decorator(decorator);

    for i in 0..records {
        if i > 0 {
            doc.push(genpdf::elements::PageBreak::new());
        }
        doc.push(genpdf::elements::Paragraph::new(format!(
            "Record {}",
            i + 1
        )));
    }

    // Render the document and write it to a file
    doc.render_to_file(path).expect("Failed to write PDF file");

    Ok(())
}
