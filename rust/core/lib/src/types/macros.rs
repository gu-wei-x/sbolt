#[macro_export]
macro_rules! include_views {
    () => {
        include!(env!("TEMPLATES_FILES"));
    };
}

/// Creates a `DefaultViewContext` with named entries or a single model value.
///
/// `context!(value)` is shorthand for `context!(model: std::sync::Arc::new(value))`.
/// Explicit named entries store values as supplied.
#[macro_export]
macro_rules! context {
    () => {
        $crate::types::DefaultViewContext::new()
    };
    ($model:expr $(,)?) => {
        $crate::context!(model: ::std::sync::Arc::new($model))
    };
    (
        $($key:ident $(: $value:expr)?),*
    ) => {{
        use $crate::types::Context;
        let mut ctx = $crate::types::DefaultViewContext::new();
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
