use daipendency::Language as UpstreamLanguage;

/// The languages supported by Daipendency.
#[napi]
pub enum Language {
    Rust,
}

impl Language {
    pub fn from_upstream(lang: UpstreamLanguage) -> Self {
        match lang {
            UpstreamLanguage::Rust => Self::Rust,
        }
    }

    pub fn to_upstream(self) -> UpstreamLanguage {
        match self {
            Self::Rust => UpstreamLanguage::Rust,
        }
    }
}
