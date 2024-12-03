use pest::Span;

pub(crate) fn span_into_str(span: Span) -> &str {
    println!("span_into_str: {:?}", span.as_str());
    span.as_str()
}

pub(crate) fn span_into_str_option(span: Span) -> Option<&str> {
    println!("span_into_str_option: {:?}", span.as_str());
    Some(span.as_str())
}
