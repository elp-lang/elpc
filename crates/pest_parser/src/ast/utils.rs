use pest::Span;

pub(crate) fn span_into_str(span: Span) -> &str {
    span.as_str()
}

pub(crate) fn span_into_str_option(span: Span) -> Option<&str> {
    Some(span.as_str())
}

pub(crate) fn span_into_vec_str(span: pest::Span) -> Vec<&str> {
    span.as_str().split(',').map(|s| s.trim()).collect()
}
