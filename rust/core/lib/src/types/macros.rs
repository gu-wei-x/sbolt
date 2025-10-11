#[macro_export]
macro_rules! include_views {
    () => {
        include!(env!("TEMPLATES_FILES"));
    };
}

/// A macro to create a `DefaultViewContext` and set multiple data entries.
/// expanded like below:
/// let mut context = DefaultViewContext::new();
/// context.set_data("strvalue", || "Hello, world!".to_string());
/// context.set_data("intvalue", || 123);
///
#[macro_export]
macro_rules! context {
    () => {
        sbolt::types::DefaultViewContext::new()
    };
    (
        $($key:ident $(: $value:expr)?),*
    ) => {{
        use sbolt::types::Context;
        let mut ctx = sbolt::types::DefaultViewContext::new();
        $(
            ctx.set_data(stringify!($key), {
                $(
                    || $value
                )*
            });
        )*
        ctx
    }};
}
