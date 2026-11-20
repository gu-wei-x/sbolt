# Keywords

- **layout** : spefcify a [layout](./layout.md) for a template.
- **model** : declare a model with `@model key: crate::Teams;`, or
  `@model crate::Teams;` for the default context key `"model"`.
  See [typed models](./syntax.md#typed-models).
- **render** : render a [section](./layout.md) in [layout](./layout.md) template.
- **section** : define a named [section](./layout.md) used in layout.
- **use** : import `rust` module in a template.
- **view** : render a separate template inline using
  `@view("shared/card", context!(...))`. See
  [custom views](./syntax.md#custom-views) for arguments and rendering behavior.
