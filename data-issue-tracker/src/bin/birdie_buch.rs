// Birdie-Buch Generator mit automatischem Download der Bahn-Skizzen
// Voraussetzungen: docx-rs = "0.4", reqwest = { version = "0.11", features = ["blocking"] }, scraper = "0.18"

use docx_rs::*;
use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::fs::File;
use std::io::Write;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. HTML der Seite laden
    let resp = get("https://www.golfclub-hannover.de/anlage/spielbahnen.html")?.text()?;
    println!("Response length: {}", resp.len());
    let document = Html::parse_document(&resp);
    let selector = Selector::parse("img").unwrap();
    println!("Selector: {:?}", selector);
    // 2. Bild-URLs sammeln (nur Skizzen)
    let mut image_urls = Vec::new();
    for element in document.select(&selector) {
        if let Some(src) = element.value().attr("src") {
            if src.contains("Bahn") {
                image_urls.push(format!("https://www.golfclub-hannover.de{}", src));
            }
        }
    }

    // 3. Bilder herunterladen und speichern

    // 4. Word-Dokument mit Bildern und Informationen erzeugen
    let mut doc = Docx::new();
    for (i, url) in image_urls.iter().enumerate() {
        // Add a top-level heading for each Bahn, starting on a new page
        doc = doc.add_paragraph(
            Paragraph::new()
                .style("Heading1")
                .add_run(Run::new().add_text(format!("Bahn {}", i + 1))),
        );

        // // Add predefined Birdie-Buch information for each Bahn
        // doc = doc.add_paragraph(
        //     Paragraph::new().add_run(Run::new().add_text("Par: 4, Länge: 350m, Handicap: 12")),
        // );
        // doc = doc.add_paragraph(
        //     Paragraph::new().add_run(Run::new().add_text(
        //         "Beschreibung: Eine herausfordernde Bahn mit einem Wasserhindernis.",
        //     )),
        // );

        // Read the image file and convert it into a Pic object
        let resp = get(url)?;
        let bytes = resp.bytes()?;
        let pic = Pic::new(&bytes);
        let (w, h) = pic.size;

        // Add the image to the document, resizing it to half its original size
        doc = doc
            .add_paragraph(Paragraph::new().add_run(Run::new().add_image(pic.size(w / 2, h / 2))));
    }

    let mut file = File::create("birdie_buch.docx")?;
    doc.build().pack(&mut file)?;
    println!("Fertig! Bilder und Word-Dokument wurden erstellt.");
    Ok(())
}
