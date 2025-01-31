use super::Language;
use super::Namespace;
use daipendency::generate_markdown_documentation;
use daipendency::Library as UpstreamLibrary;

#[napi]
pub struct Library(UpstreamLibrary);

#[napi]
impl Library {
    #[napi]
    pub async fn load_dependency(
        name: String,
        path: String,
        language: Option<Language>,
    ) -> napi::Result<Self> {
        tokio::task::spawn_blocking(move || {
            // Convert Language enum if needed
            let lang = language.map(|l| l.to_upstream());

            UpstreamLibrary::load_dependency(&name, path.as_ref(), lang)
                .map(Library)
                .map_err(|e| napi::Error::from_reason(e.to_string()))
        })
        .await
        .map_err(|e| napi::Error::from_reason(e.to_string()))?
    }

    #[napi]
    pub async fn load(path: String, language: Option<Language>) -> napi::Result<Self> {
        let lang = language.map(|l| l.to_upstream());

        UpstreamLibrary::load(path.as_ref(), lang)
            .map(Library)
            .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    #[napi(getter)]
    pub fn name(&self) -> String {
        self.0.name.clone()
    }

    #[napi(getter)]
    pub fn version(&self) -> Option<String> {
        self.0.version.clone()
    }

    #[napi(getter)]
    pub fn documentation(&self) -> String {
        self.0.documentation.clone()
    }

    #[napi(getter)]
    pub fn language(&self) -> Language {
        Language::from_upstream(self.0.language)
    }

    #[napi(getter)]
    pub fn namespaces(&self) -> Vec<Namespace> {
        self.0
            .namespaces
            .iter()
            .map(Namespace::from_upstream)
            .collect()
    }

    #[napi]
    pub fn generate_markdown_documentation(&self) -> String {
        generate_markdown_documentation(&self.0)
    }
}
