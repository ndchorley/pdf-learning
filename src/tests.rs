use lopdf::Document;

#[test]
fn can_get_all_the_text_from_a_pdf() {
    let document = Document::load("sample.pdf").unwrap();

    let text = document.extract_text(&[1]).unwrap();

    assert_eq!(text, "intentionally-wrong");
}
