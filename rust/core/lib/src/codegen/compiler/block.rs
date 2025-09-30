use crate::codegen::{
    cgresult, consts,
    parser::template::{Block, Kind},
};
use crate::types::error;
use proc_macro2::TokenStream;
use quote::quote;

impl<'a> Block<'a> {
    pub(crate) fn generate_code(&self) -> Result<cgresult::CodeGenResult, error::Error> {
        let mut result = cgresult::CodeGenResult::new();
        let imports = Self::generate_imports(self)?;
        result = result.with_imports(imports);

        let layout = Self::generate_layout(self)?;
        result = result.with_layout(layout);

        let render = Self::generate_render(self)?;
        result = result.with_code(Some(render));

        Ok(result)
    }

    pub(crate) fn generate_main(&self, from: Option<Kind>, output: &mut String) {
        match from {
            Some(from_kind) => {
                match (from_kind, self.kind()) {
                    (Kind::CODE, Kind::CODE) => {
                        // TODO: put in a closure
                        output.push_str(self.content());
                    }
                    (Kind::CODE, Kind::CONTENT) => {
                        // TODO: from code to content.put in a closure
                        // todo: inlined content sth like <View test="@code" .../>
                        // or content block.
                        println!("???????????????????exepected: {}", self.content());
                        output
                            .push_str(&format!(r####"output.write(r#"{}"#);"####, self.content()));
                    }
                    (Kind::CONTENT, Kind::INLINEDCODE) => {
                        // code inside content.
                        // todo: check whether type is inlined.
                        // or block: block should be closure with output as parameter.
                        output.push_str(&format!(
                            r####"output.writefn(||({}).to_string());"####,
                            self.content()
                        ));
                    }
                    (Kind::CONTENT, Kind::CONTENT) => {
                        // from content to content.
                        output
                            .push_str(&format!(r####"output.write(r#"{}"#);"####, self.content()));
                    }
                    (_, _) => {
                        println!(
                            "+++++++++++++++++++++++++++++exepected:{from_kind:?}, {:?}",
                            self.content()
                        )
                    }
                }
            }
            None => {
                // from root.
                match self.kind() {
                    Kind::CODE => {
                        if !self.has_blocks() {
                            //pure code block.
                            output.push_str(self.content());
                        } else {
                            // TODO: mixed code block. should be FnOnce with output as parameter.
                            for block in self.blocks() {
                                block.generate_main(Some(Kind::CODE), output);
                            }
                        }
                    }
                    Kind::CONTENT => {
                        if !self.has_blocks() {
                            //pure content block.
                            output.push_str(&format!(
                                r####"output.write(r#"{}"#);"####,
                                self.content()
                            ));
                        } else {
                            // TODO: mixed content block. should be FnOnce with output as parameter.
                            for block in self.blocks() {
                                block.generate_main(Some(Kind::CONTENT), output);
                            }
                        }
                    }
                    Kind::INLINEDCODE => {
                        output.push_str(&format!(
                            r####"output.writefn(||({}).to_string());"####,
                            self.content()
                        ));
                    }
                    _ => {}
                }
            }
        }
    }

    fn generate_imports(block: &Block) -> Result<Option<TokenStream>, error::Error> {
        let imports_content = match block.kind() {
            Kind::CODE if block.name() == Some(&consts::DIRECTIVE_KEYWORD_USE.to_string()) => {
                format!("{} {};", consts::DIRECTIVE_KEYWORD_USE, block.content())
            }
            Kind::CONTENT if block.has_blocks() => block
                .blocks()
                .iter()
                .filter(|b| {
                    matches!(b.kind(), Kind::CODE)
                        && b.name() == Some(&consts::DIRECTIVE_KEYWORD_USE.to_string())
                })
                .map(|block| {
                    let import_content = block.content();
                    format!("{} {};", consts::DIRECTIVE_KEYWORD_USE, import_content)
                })
                .collect::<Vec<String>>()
                .join(""),
            _ => "".to_string(),
        };

        if imports_content.is_empty() {
            return Ok(None);
        }

        let ts = imports_content.parse::<TokenStream>();
        match ts {
            Ok(ts) => Ok(Some(ts)),
            Err(e) => Err(error::Error::CodeGen(format!(
                "Failed to parse imports: {}",
                e
            ))),
        }
    }

    fn generate_layout(block: &Block) -> Result<Option<TokenStream>, error::Error> {
        let none_layout = "fn layout() -> Option<String> {None}".to_string();
        let layout_content = match block.kind() {
            Kind::CODE if block.name() == Some(&consts::DIRECTIVE_KEYWORD_LAYOUT.to_string()) => {
                none_layout
            }
            Kind::CONTENT if block.has_blocks() => {
                let items = block
                    .blocks()
                    .iter()
                    .filter(|b| b.name() == Some(&consts::DIRECTIVE_KEYWORD_LAYOUT.to_string()))
                    .collect::<Vec<_>>();
                let layout_count = items.len();
                match layout_count {
                    1 => {
                        let layout_block = items[0];
                        let layout_name = layout_block.content();
                        format!(
                            r#"fn layout() -> Option<String> {{Some("{}".to_string())}}"#,
                            layout_name
                        )
                    }
                    _ => {
                        return Err(error::Error::CodeGen(
                            "Multiple layout directives found".to_string(),
                        ));
                    }
                }
            }
            _ => none_layout,
        };

        let ts = layout_content.parse::<TokenStream>();
        match ts {
            Ok(ts) => Ok(Some(ts)),
            Err(e) => Err(error::Error::CodeGen(format!(
                "Failed to parse layout: {}",
                e
            ))),
        }
    }

    fn generate_render(block: &Block) -> Result<TokenStream, error::Error> {
        // block can only be from root.
        if !matches!(block.kind(), Kind::ROOT) {
            return Err(error::Error::CodeGen(
                "Only root block can be rendered".to_string(),
            ));
        }

        let mut content = String::new();
        match block.has_blocks() {
            false => {
                content.push_str(&format!(r####"output.write(r#"{}"#);"####, block.content()));
            }
            true => {
                // use, content, put layout to context.
                for b in block.blocks() {
                    if b.name().is_none() {
                        b.generate_main(None, &mut content);
                    }
                }
            }
        }

        let main_ts = content.parse::<TokenStream>();
        match main_ts {
            Ok(ts) => Ok(quote! {
                fn render(&self, output: &mut impl disguise::types::Writer) {
                    #ts
                }
            }),
            Err(e) => Err(error::Error::CodeGen(format!(
                "Failed to parse main content: {}",
                e
            ))),
        }
    }
}
