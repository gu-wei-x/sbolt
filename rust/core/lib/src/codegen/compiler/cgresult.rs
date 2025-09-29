use proc_macro2::TokenStream;

pub(crate) struct CodeGenResult {
    pub code: Option<TokenStream>,
    pub imports: Option<TokenStream>,
    pub layout: Option<TokenStream>,
}

impl CodeGenResult {
    pub fn new() -> Self {
        Self {
            code: None,
            imports: None,
            layout: None,
        }
    }

    pub fn with_code(mut self, code: Option<TokenStream>) -> Self {
        self.code = code;
        self
    }

    pub fn with_imports(mut self, imports: Option<TokenStream>) -> Self {
        self.imports = imports;
        self
    }

    pub fn with_layout(mut self, layout: Option<TokenStream>) -> Self {
        self.layout = layout;
        self
    }
}
