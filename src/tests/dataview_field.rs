use super::*;

#[test]
fn dataview_fields() {
    html_opts!(
        [extension.dataview_field],
        concat!("full line key:: and the value"),
        concat!(
            "<p>",
            "<div class=\"dataview-field dataview-field-full-line\">",
            "<span class=\"dataview-key\">full line key</span>",
            "<span class=\"dataview-value\"> and the value</span>",
            "</div>",
            "</p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("field in the [key:: value] bracket"),
        concat!(
            "<p>field in the ",
            "<div class=\"dataview-field dataview-field-bracket\">",
            "<span class=\"dataview-key\">key</span>",
            "<span class=\"dataview-value\"> value</span>",
            "</div>",
            " bracket</p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("field in the (key:: value) parenthesis"),
        concat!(
            "<p>field in the ",
            "<div class=\"dataview-field dataview-field-parenthesis\">",
            "<span class=\"dataview-key\">key</span>",
            "<span class=\"dataview-value\"> value</span>",
            "</div>",
            " parenthesis</p>\n"
        ),
        no_roundtrip,
    );
}

// #[test]
// fn sourcepos() {
//     assert_ast_match!(
//         [extension.wikilinks_title_after_pipe],
//         "This [[http://example.com|link label]] that\n",
//         (document (1:1-1:43) [
//             (paragraph (1:1-1:43) [
//                 (text (1:1-1:5) "This ")
//                 (wikilink (1:6-1:38) [
//                     (text (1:27-1:36) "link label")
//                 ])
//                 (text (1:39-1:43) " that")
//             ])
//         ])
//     );
// }
