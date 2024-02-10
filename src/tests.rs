use lopdf::Document;

#[test]
fn can_get_all_the_text_from_a_pdf() {
    let document = Document::load("sample.pdf").unwrap();

    let text = document.extract_text(&[1]).unwrap();

    assert_eq!(text, "intentionally-wrong");
}

#[test]
fn can_get_the_pdf_version() {
    let document = Document::load("sample.pdf").unwrap();

    assert_eq!(document.version, String::from("1.3"));
}

#[test]
fn can_get_value_for_type_in_the_catalog() {
    let document = Document::load("sample.pdf").unwrap();

    let catalog = document.catalog().unwrap();

    let type_value = catalog.get("Type".as_bytes()).unwrap();

    assert_eq!(type_value.as_name().unwrap(), "Catalog".as_bytes());
}
