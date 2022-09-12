extern crate micromark;
use micromark::{micromark, micromark_with_options, Constructs, Options};
use pretty_assertions::assert_eq;

#[test]
fn fuzz() -> Result<(), String> {
    assert_eq!(
        micromark("[\n~\na\n-\n\n"),
        "<p>[\n~\na</p>\n<ul>\n<li></li>\n</ul>\n",
        "1: label, blank lines, and code"
    );

    assert_eq!(
        // The first link is stopped by the `+` (so it’s `a@b.c`), but the next
        // link overlaps it (`b.c+d@e.f`).
        micromark_with_options(
            "a@b.c+d@e.f",
            &Options {
                constructs: Constructs::gfm(),
                gfm_tagfilter: true,
                ..Options::default()
            }
        )?,
        "<p><a href=\"mailto:a@b.c\">a@b.c</a><a href=\"mailto:+d@e.f\">+d@e.f</a></p>",
        "2: gfm: email autolink literals running into each other"
    );

    Ok(())
}
