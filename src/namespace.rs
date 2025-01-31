use daipendency::Namespace as UpstreamNamespace;

use super::Symbol;

#[napi]
#[derive(Clone)]
pub struct Namespace {
    pub name: String,
    pub symbols: Vec<Symbol>,
    pub doc_comment: Option<String>,
}

impl Namespace {
    pub fn from_upstream(namespace: &UpstreamNamespace) -> Self {
        Self {
            name: namespace.name.clone(),
            symbols: namespace
                .symbols
                .iter()
                .map(Symbol::from_upstream)
                .collect(),
            doc_comment: namespace.doc_comment.clone(),
        }
    }
}
