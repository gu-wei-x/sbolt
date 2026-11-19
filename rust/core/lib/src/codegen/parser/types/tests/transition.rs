#![cfg(test)]
use crate::codegen::CompilerOptions;
use crate::codegen::parser::tokenizer::Tokenizer;
use crate::codegen::parser::types::context::{Kind, ParseContext};
use crate::codegen::types::Block;
use crate::types::result;
use crate::types::template;
use winnow::stream::{Stream as _, TokenSlice};

#[test]
fn parse_transition_block() -> result::Result<()> {
    let source = r#"@test"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let options = CompilerOptions::default();
    let mut context = ParseContext::new(Kind::KCODE, template::Kind::KHTML, &options, source);
    let block = Block::parse_transition_block(&mut token_stream, &mut context)?;
    assert!(matches!(block, Block::KCODE(_)));
    Ok(())
}

#[test]
#[should_panic]
fn parse_transition_block_starts_without_transition_symbol() {
    let source = r#"@test"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let options = CompilerOptions::default();
    let mut context = ParseContext::new(Kind::KCODE, template::Kind::KHTML, &options, source);
    // change the first token to not be '@'
    token_stream.next_token();
    Block::parse_transition_block(&mut token_stream, &mut context).unwrap();
}

#[test]
#[should_panic]
fn parse_transition_block_empty_stream() {
    let source = r#""#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let options = CompilerOptions::default();
    let mut context = ParseContext::new(Kind::KCODE, template::Kind::KHTML, &options, source);
    // change the first token to not be '@'
    token_stream.next_token();
    Block::parse_transition_block(&mut token_stream, &mut context).unwrap();
}

#[test]
#[should_panic]
fn parse_transition_block_single_transition_symbol() {
    let source = r#"@"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens[0..1]);
    let options = CompilerOptions::default();
    let mut context = ParseContext::new(Kind::KCODE, template::Kind::KHTML, &options, source);
    Block::parse_transition_block(&mut token_stream, &mut context).unwrap();
}

#[test]
#[should_panic]
fn parse_transition_block_invalid_token() {
    let source = r#"@!"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let options = CompilerOptions::default();
    let mut context = ParseContext::new(Kind::KCODE, template::Kind::KHTML, &options, source);
    Block::parse_transition_block(&mut token_stream, &mut context).unwrap();
}

#[test]
fn parse_transition_block_render() -> result::Result<()> {
    let source = r#"@render(test)"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let options = CompilerOptions::default();
    let mut context = ParseContext::new(Kind::KCODE, template::Kind::KHTML, &options, source);
    let block = Block::parse_transition_block(&mut token_stream, &mut context)?;
    assert!(matches!(block, Block::KRENDER(_)));
    Ok(())
}

#[test]
#[should_panic]
fn parse_transition_block_render_invalid_context() {
    let source = r#"@render(test)"#;
    let tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let options = CompilerOptions::default();
    let mut context = ParseContext::new(Kind::KCONTENT, template::Kind::KHTML, &options, source);
    Block::parse_transition_block(&mut token_stream, &mut context).unwrap();
}

#[test]
fn parse_view_preserves_rust_arguments_and_following_content() -> result::Result<()> {
    for source in [
        r#"@view("shared/card", context!(title: "a), @b".to_string()))<p>after</p>"#,
        r##"@view(r#"shared/)card"#, context!(title: r#"a"), @b"#.to_string()))<p>after</p>"##,
        "@view(\"shared/card\", { /* ) */ make_context::<(u8, u8)>() },)<p>after</p>",
        "@view(\"shared/card\", { // )\n make_context() })<p>after</p>",
        "@view(\"shared/card\", context!(symbol: ')'))<p>after</p>",
        "@view(\"shared/card\", { /* outer ) /* nested ) */ */ make_context() })<p>after</p>",
        "@view(\"shared/card\", make_context::<'static, &'static str>())<p>after</p>",
        r#"@view("shared/card", context!(symbol: '\u{0029}'))<p>after</p>"#,
        r#"@view("shared/card", context!(text: b"\"), ("))<p>after</p>"#,
        r##"@view("shared/card", context!(text: cr#"a"), (b"#))<p>after</p>"##,
        r##"@view("shared/card", context!(text: br#"a"), (b"#))<p>after</p>"##,
        r#"@view("shared/card", context!(), sbolt::types::template::Kind::KJSON,)<p>after</p>"#,
        r#"@view("shared/card", context!(), { /* ) */ choose_kind::<(u8, u8)>() })<p>after</p>"#,
    ] {
        let tokens = Tokenizer::new(source).into_vec();
        let mut token_stream = TokenSlice::new(&tokens);
        let options = CompilerOptions::default();
        let mut context =
            ParseContext::new(Kind::KINLINEDCODE, template::Kind::KHTML, &options, source);
        let block = Block::parse_transition_block(&mut token_stream, &mut context)?;
        assert!(matches!(block, Block::KVIEW(_)), "{source}");
        assert_eq!(&source[token_stream.peek_token().unwrap().range()], "<");
    }
    Ok(())
}

#[test]
fn parse_view_keeps_token_boundaries_before_strings() -> result::Result<()> {
    for prefix in [
        "r#identifier",
        "r#r",
        "r#br",
        "r#cr",
        "r#\u{e9}r",
        "r#r\u{301}r",
        "1r",
        "1.2r",
        "1e+2r",
        "\"first\"r",
        "'x'r",
        "r#\"first\"#r",
    ] {
        let arguments = format!("(\"shared/card\", ctx!({prefix}\"a\\\"), (b\"))");
        let source = format!("@view{arguments}<p>after</p>");
        let tokens = Tokenizer::new(&source).into_vec();
        let mut token_stream = TokenSlice::new(&tokens);
        let options = CompilerOptions::default();
        let mut context =
            ParseContext::new(Kind::KINLINEDCODE, template::Kind::KHTML, &options, &source);
        let block = Block::parse_transition_block(&mut token_stream, &mut context)?;
        assert_eq!(block.content(), arguments);
        assert_eq!(&source[token_stream.peek_token().unwrap().range()], "<");
    }
    Ok(())
}

#[test]
fn parse_view_line_comments_end_at_lf_not_bare_cr() -> result::Result<()> {
    for comment in [
        "// comment\r ) \"\n",
        "// comment\r ) \"\r more (\n",
        "// ) \"\r\n",
    ] {
        let arguments = format!("(\"shared/card\", {{ {comment} context!() }})");
        let source = format!("@view{arguments}<p>after</p>");
        let tokens = Tokenizer::new(&source).into_vec();
        let mut token_stream = TokenSlice::new(&tokens);
        let options = CompilerOptions::default();
        let mut context =
            ParseContext::new(Kind::KINLINEDCODE, template::Kind::KHTML, &options, &source);
        let block = Block::parse_transition_block(&mut token_stream, &mut context)?;
        assert_eq!(block.content(), arguments);
        assert_eq!(&source[token_stream.peek_token().unwrap().range()], "<");
    }
    Ok(())
}

#[test]
fn parse_large_view_string_preserves_argument_boundary() -> result::Result<()> {
    let payload = ")".repeat(32_000);
    let arguments = format!("(\"shared/card\", context!(text: r#\"{payload}\"#))");
    let source = format!("@view{arguments}<p>after</p>");
    let tokens = Tokenizer::new(&source).into_vec();
    let mut token_stream = TokenSlice::new(&tokens);
    let options = CompilerOptions::default();
    let mut context =
        ParseContext::new(Kind::KINLINEDCODE, template::Kind::KHTML, &options, &source);
    let block = Block::parse_transition_block(&mut token_stream, &mut context)?;
    assert_eq!(block.content(), arguments);
    assert_eq!(&source[token_stream.peek_token().unwrap().range()], "<");
    Ok(())
}

#[test]
fn parse_view_rejects_invalid_arguments_with_source_location() {
    for source in [
        "\n@view",
        "\n@view()",
        "\n@view(\"card\")",
        "\n@view(\"card\", context!(), Kind::KJSON, 3)",
        "\n@view(\"card\", context!(), 1 +)",
        "\n@view(\"card\", context!()",
        "\n@view(\"card\",)",
        "\n@view(\"card\", 1 +)",
        "\n@view(\"card\", context!(text: \"unterminated))",
        "\n@view(\"card\", context!(text: r#\"unterminated))",
        "\n@view(\"card\", { /* outer /* nested */ })",
    ] {
        let tokens = Tokenizer::new(source).into_vec();
        let mut token_stream = TokenSlice::new(&tokens);
        token_stream.next_token();
        let options = CompilerOptions::default();
        let mut context =
            ParseContext::new(Kind::KINLINEDCODE, template::Kind::KHTML, &options, source);
        let error = Block::parse_transition_block(&mut token_stream, &mut context).unwrap_err();
        assert!(
            matches!(error, crate::types::error::CompileError::Parser(Some(location), _, _) if location.line == 1),
            "{source}: {error:?}"
        );
    }
}
