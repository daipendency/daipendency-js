use daipendency::Symbol as UpstreamSymbol;

#[napi(object)]
#[derive(Clone)]
pub struct Symbol {
    pub name: String,
    pub source_code: String,
}

impl Symbol {
    pub fn from_upstream(symbol: &UpstreamSymbol) -> Self {
        Self {
            name: symbol.name.clone(),
            source_code: symbol.source_code.clone(),
        }
    }
}
