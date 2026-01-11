use super::*;

#[test]
fn dataview_fields_basic() {
    html_opts!(
        [extension.dataview_field],
        concat!("full line:: inline field"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-full-line\">",
            "<span class=\"dataview-key\">full line</span>",
            "<span class=\"dataview-value\"> inline field</span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("field in the [key:: value] bracket"),
        concat!(
            "<p>field in the <div class=\"dataview-field dataview-field-bracket\">",
            "<span class=\"dataview-key\">key</span>",
            "<span class=\"dataview-value\"> value</span>",
            "</div> bracket</p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("field in the (key:: value) parenthesis"),
        concat!(
            "<p>field in the <div class=\"dataview-field dataview-field-parenthesis\">",
            "<span class=\"dataview-key\">key</span>",
            "<span class=\"dataview-value\"> value</span>",
            "</div> parenthesis</p>\n"
        ),
        no_roundtrip,
    );
}

#[test]
fn dataview_fields_empty_key_value() {
    html_opts!(
        [extension.dataview_field],
        concat!("::"),
        concat!("<p>::</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("[::]"),
        concat!("<p>[::]</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("(::)"),
        concat!("<p>(::)</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("::value with empty key are not allowed"),
        concat!("<p>::value with empty key are not allowed</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("[::value with empty key are not allowed]"),
        concat!("<p>[::value with empty key are not allowed]</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("( ::value with empty key are not allowed)"),
        concat!("<p>( ::value with empty key are not allowed)</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("key with empty value are allowed::"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-full-line\">",
            "<span class=\"dataview-key\">key with empty value are allowed</span>",
            "<span class=\"dataview-value\"></span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("[key with empty value are allowed::]"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-bracket\">",
            "<span class=\"dataview-key\">key with empty value are allowed</span>",
            "<span class=\"dataview-value\"></span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("(key with empty value are allowed::   )"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-parenthesis\">",
            "<span class=\"dataview-key\">key with empty value are allowed</span>",
            "<span class=\"dataview-value\">   </span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );
}

#[test]
fn dataview_fields_() {
    html_opts!(
        [extension.dataview_field],
        concat!("[a::(b::c]"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-bracket\">",
            "<span class=\"dataview-key\">a</span>",
            "<span class=\"dataview-value\">(b::c</span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );
    html_opts!(
        [extension.dataview_field],
        concat!("[a::(b::c]"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-bracket\">",
            "<span class=\"dataview-key\">a</span>",
            "<span class=\"dataview-value\">(b::c</span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("[a::(b::c)"),
        concat!(
            "<p>[a::<div class=\"dataview-field dataview-field-parenthesis\">",
            "<span class=\"dataview-key\">b</span>",
            "<span class=\"dataview-value\">c</span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("a::]"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-full-line\">",
            "<span class=\"dataview-key\">a</span>",
            "<span class=\"dataview-value\">]</span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("a::[]"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-full-line\">",
            "<span class=\"dataview-key\">a</span>",
            "<span class=\"dataview-value\">[]</span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("[a::[]]"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-bracket\">",
            "<span class=\"dataview-key\">a</span>",
            "<span class=\"dataview-value\">[]</span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("[[a::b]]"),
        concat!(
            "<p>[<div class=\"dataview-field dataview-field-bracket\">",
            "<span class=\"dataview-key\">a</span>",
            "<span class=\"dataview-value\">b</span>",
            "</div>]</p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("((a::b))"),
        concat!(
            "<p>(<div class=\"dataview-field dataview-field-parenthesis\">",
            "<span class=\"dataview-key\">a</span>",
            "<span class=\"dataview-value\">b</span>",
            "</div>)</p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("(a::::b)"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-parenthesis\">",
            "<span class=\"dataview-key\">a</span>",
            "<span class=\"dataview-value\">::b</span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("hello\na::b\nc::d\nworld"),
        concat!(
            "<p>hello\n",
            "<div class=\"dataview-field dataview-field-full-line\"><span class=\"dataview-key\">a</span><span class=\"dataview-value\">b</span></div>\n",
            "<div class=\"dataview-field dataview-field-full-line\"><span class=\"dataview-key\">c</span><span class=\"dataview-value\">d</span></div>\n",
            "world</p>\n",
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("hello\n[a::b]\nc::d\nworld\n(e::f)"),
        concat!(
            "<p>hello\n",
            "<div class=\"dataview-field dataview-field-bracket\"><span class=\"dataview-key\">a</span><span class=\"dataview-value\">b</span></div>\n",
            "<div class=\"dataview-field dataview-field-full-line\"><span class=\"dataview-key\">c</span><span class=\"dataview-value\">d</span></div>\n",
            "world\n",
            "<div class=\"dataview-field dataview-field-parenthesis\"><span class=\"dataview-key\">e</span><span class=\"dataview-value\">f</span></div>",
            "</p>\n",
        ),
        no_roundtrip,
    );
}

#[test]
fn dataview_fields_malformed() {
    html_opts!(
        [extension.dataview_field],
        concat!("[a::"),
        concat!("<p>[a::</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("(a[b::)"),
        concat!("<p>(a[b::)</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("[a)b::]"),
        concat!("<p>[a)b::]</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("[a()b::]"),
        concat!("<p>[a()b::]</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("[a[]b::]"),
        concat!("<p>[a[]b::]</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("(a[]b::)"),
        concat!("<p>(a[]b::)</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("(a[]b::)"),
        concat!("<p>(a[]b::)</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("[a()]b::]"),
        concat!("<p>[a()]b::]</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("(a::(b)"),
        concat!("<p>(a::(b)</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("[a::[b]"),
        concat!("<p>[a::[b]</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("[a\n::b]"),
        concat!("<p>[a\n::b]</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("(a::b\nc)"),
        concat!("<p>(a::b\nc)</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("a\n::b)"),
        concat!("<p>a\n::b)</p>\n"),
        no_roundtrip,
    );
}

#[test]
fn dataview_fields_nest_inline() {
    html_opts!(
        [extension.dataview_field],
        concat!("key::[link](url)"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-full-line\">",
            "<span class=\"dataview-key\">key</span>",
            "<span class=\"dataview-value\"><a href=\"url\">link</a></span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("[*key*::[**link**](url)] "),
        concat!(
            "<p><div class=\"dataview-field dataview-field-bracket\">",
            "<span class=\"dataview-key\"><em>key</em></span>",
            "<span class=\"dataview-value\"><a href=\"url\"><strong>link</strong></a></span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("(key::[*link*](url))"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-parenthesis\">",
            "<span class=\"dataview-key\">key</span>",
            "<span class=\"dataview-value\"><a href=\"url\"><em>link</em></a></span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("[link](url)::value"),
        concat!("<p><a href=\"url\">link</a>::value</p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("[[link](url)::value]"),
        concat!("<p>[<a href=\"url\">link</a>::value]</p>\n"),
        no_roundtrip,
    );
}

#[test]
fn dataview_fields_in_block() {
    html_opts!(
        [extension.dataview_field],
        concat!(
            "- key::value\n",
            "- [key2::value2]\n",
            "  - (key3::value3) abc"
        ),
        concat!(
            "<ul>\n",
            "<li><div class=\"dataview-field dataview-field-full-line\"><span class=\"dataview-key\">key</span><span class=\"dataview-value\">value</span></div></li>\n",
            "<li><div class=\"dataview-field dataview-field-bracket\"><span class=\"dataview-key\">key2</span><span class=\"dataview-value\">value2</span></div>\n",
            "<ul>\n",
            "<li><div class=\"dataview-field dataview-field-parenthesis\"><span class=\"dataview-key\">key3</span><span class=\"dataview-value\">value3</span></div> abc</li>\n",
            "</ul>\n",
            "</li>\n",
            "</ul>\n",
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field],
        concat!("> key::value\n>> [key::value]"),
        concat!(
            "<blockquote>\n",
            "<p><div class=\"dataview-field dataview-field-full-line\"><span class=\"dataview-key\">key</span><span class=\"dataview-value\">value</span></div></p>\n",
            "<blockquote>\n",
            "<p><div class=\"dataview-field dataview-field-bracket\"><span class=\"dataview-key\">key</span><span class=\"dataview-value\">value</span></div></p>\n",
            "</blockquote>\n",
            "</blockquote>\n",
        ),
        no_roundtrip,
    );
}

#[test]
fn dataview_fields_in_table() {
    html_opts!(
        [extension.dataview_field, extension.table],
        concat!(
            "| col1 | col2 |\n",
            "| ---- | ---- |\n",
            "| [key::value] | key::value |\n",
        ),
        concat!(
            "<table>\n",
            "<thead>\n",
            "<tr>\n",
            "<th>col1</th>\n",
            "<th>col2</th>\n",
            "</tr>\n",
            "</thead>\n",
            "<tbody>\n",
            "<tr>\n",
            "<td><div class=\"dataview-field dataview-field-bracket\"><span class=\"dataview-key\">key</span><span class=\"dataview-value\">value</span></div></td>\n",
            "<td><div class=\"dataview-field dataview-field-full-line\"><span class=\"dataview-key\">key</span><span class=\"dataview-value\">value</span></div></td>\n",
            "</tr>\n",
            "</tbody>\n",
            "</table>\n",
        ),
        no_roundtrip,
    );
}

#[test]
fn dataview_fields_in_tasklist() {
    // FIXME: fix this
    // html_opts!(
    //     [extension.dataview_field, extension.tasklist],
    //     concat!(
    //         "- [ ] key :: value\n",
    //         "  - [x] [key :: value]\n",
    //         "- [x] key :: value\n",
    //     ),
    //     concat!(
    //         "<ul>\n",
    //         "<li><input type=\"checkbox\" disabled=\"\" />  <div class=\"dataview-field dataview-field-full-line\"><span class=\"dataview-key\">key </span><span class=\"dataview-value\"> value</span></div>\n",
    //         "<ul>\n",
    //         "<li><input type=\"checkbox\" checked=\"\" disabled=\"\" /> <div class=\"dataview-field dataview-field-bracket\"><span class=\"dataview-key\">key </span><span class=\"dataview-value\"> value</span></div></li>\n",
    //         "</ul>\n",
    //         "</li>\n",
    //         "<li><input type=\"checkbox\" checked=\"\" disabled=\"\" /> key :: value</li>\n",
    //         "</ul>\n",
    //     ),
    //     no_roundtrip,
    // );
}

#[test]
fn dataview_fields_with_wikilink() {
    html_opts!(
        [
            extension.dataview_field,
            extension.wikilinks_title_after_pipe
        ],
        concat!("[[key::value]]"),
        concat!("<p><a href=\"key::value\" data-wikilink=\"true\">key::value</a></p>\n"),
        no_roundtrip,
    );

    html_opts!(
        [
            extension.dataview_field,
            extension.wikilinks_title_after_pipe
        ],
        concat!("[key::[[a::b]]]"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-bracket\">",
            "<span class=\"dataview-key\">key</span>",
            "<span class=\"dataview-value\"><a href=\"a::b\" data-wikilink=\"true\">a::b</a></span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [
            extension.dataview_field,
            extension.wikilinks_title_after_pipe
        ],
        concat!("[[[a::b]]::value]"),
        concat!(
            "<p>[[<div class=\"dataview-field dataview-field-bracket\">",
            "<span class=\"dataview-key\">a</span>",
            "<span class=\"dataview-value\">b</span>",
            "</div>]::value]</p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [
            extension.dataview_field,
            extension.wikilinks_title_after_pipe
        ],
        concat!("(key::[[a::b]])"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-parenthesis\">",
            "<span class=\"dataview-key\">key</span>",
            "<span class=\"dataview-value\"><a href=\"a::b\" data-wikilink=\"true\">a::b</a></span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [
            extension.dataview_field,
            extension.wikilinks_title_after_pipe
        ],
        concat!("([[a::b]]::value)"),
        concat!("<p>(<a href=\"a::b\" data-wikilink=\"true\">a::b</a>::value)</p>\n"),
        no_roundtrip,
    );
}

#[test]
fn dataview_fields_with_shortcode() {
    html_opts!(
        [extension.dataview_field, extension.shortcodes],
        concat!(":smile::smile::::smile::smile:"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-full-line\">",
            "<span class=\"dataview-key\">😄😄</span>",
            "<span class=\"dataview-value\">😄😄</span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field, extension.shortcodes],
        concat!(":smile:[:smile::::smile:]:smile:"),
        concat!(
            "<p>😄<div class=\"dataview-field dataview-field-bracket\">",
            "<span class=\"dataview-key\">😄</span>",
            "<span class=\"dataview-value\">😄</span>",
            "</div>😄</p>\n"
        ),
        no_roundtrip,
    );
}

#[test]
fn dataview_fields_with_cjk_friendly_emphasis() {
    html_opts!(
        [extension.dataview_field, extension.cjk_friendly_emphasis],
        concat!("これは**私のやりたかったこと**::だからするの。\n"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-full-line\">",
            "<span class=\"dataview-key\">これは<strong>私のやりたかったこと</strong></span>",
            "<span class=\"dataview-value\">だからするの。</span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );

    html_opts!(
        [extension.dataview_field, extension.cjk_friendly_emphasis],
        concat!("[__注意__::__事項__]"),
        concat!(
            "<p><div class=\"dataview-field dataview-field-bracket\">",
            "<span class=\"dataview-key\"><strong>注意</strong></span>",
            "<span class=\"dataview-value\"><strong>事項</strong></span>",
            "</div></p>\n"
        ),
        no_roundtrip,
    );
}

#[test]
fn sourcepos() {
    assert_ast_match!(
        [extension.dataview_field],
        "full line:: inline field",
        (document (1:1-1:24) [
            (paragraph (1:1-1:24) [
                (dataview_field (1:1-1:24) [
                    (dataview_key (1:1-1:9) [
                        (text (1:1-1:9) "full line")
                    ])
                    (dataview_value (1:12-1:24) [
                        (text (1:12-1:24) " inline field")
                    ])
                ])
            ])
        ])
    );

    assert_ast_match!(
        [extension.dataview_field],
        "field in the [ key :: value ] bracket",
        (document (1:1-1:37) [
            (paragraph (1:1-1:37) [
                (text (1:1-1:13) "field in the ")
                (dataview_field (1:14-1:29) [
                    (dataview_key (1:15-1:19) [
                        (text (1:15-1:19) " key ")
                    ])
                    (dataview_value (1:22-1:28) [
                        (text (1:22-1:28) " value ")
                    ])
                ])
                (text (1:30-1:37) " bracket")
            ])
        ])
    );

    assert_ast_match!(
        [extension.dataview_field],
        "field in the (key:: value) parenthesis",
        (document (1:1-1:38) [
            (paragraph (1:1-1:38) [
                (text (1:1-1:13) "field in the ")
                (dataview_field (1:14-1:26) [
                    (dataview_key (1:15-1:17) [
                        (text (1:15-1:17) "key")
                    ])
                    (dataview_value (1:20-1:25) [
                        (text (1:20-1:25) " value")
                    ])
                ])
                (text (1:27-1:38) " parenthesis")
            ])
        ])
    );
}
