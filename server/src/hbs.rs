//! Defines custom structs for the variables used by the HBS templates.

/// Variables used by templates/header.hbs.
#[derive(Serialize)]
pub struct Header {
    /// Length of the "entries" table.
    pub num_entries: i64,

    /// Length of the "meets" table.
    pub num_meets: i64,
}

/// Variables used by templates/desktop-base.hbs.
#[derive(Serialize)]
pub struct Base<'a> {
    /// The page title, in the HTML <head>.
    pub title: &'a str,

    pub header: Header,
}

/// Variables used by templates/faq.html.hbs.
#[derive(Serialize)]
pub struct FaqContext<'a> {
    pub base: Base<'a>,
}

/// Variables used by templates/lifter.html.hbs.
#[derive(Serialize)]
pub struct LifterContext<'a> {
    /// Lifter name with possible Instagram link, as HTML.
    pub lifter_nameurl_html: &'a str,

    pub base: Base<'a>,
}
