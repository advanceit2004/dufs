# Custom WebUI

This fork keeps the customized WebUI in `webui/custom` instead of editing the upstream `assets` directory directly.

The Rust binary embeds `webui/custom/index.html`, `webui/custom/index.css`, `webui/custom/index.js`, and `webui/custom/favicon.ico` at compile time, so the existing Cargo, Docker, and GitHub Release flows continue to produce a self-contained `dufs` binary.

The same directory can also be used as an external asset override when testing against an upstream binary:

```sh
dufs --assets webui/custom
```

Compatibility requirements:

- `index.html` must keep the `__INDEX_DATA__` placeholder.
- `index.html` must keep the `__ASSETS_PREFIX__` placeholder for linked assets.
- The output directory must contain `index.html`; `index.css`, `index.js`, and `favicon.ico` are served through the same asset prefix.

When syncing from upstream, prefer keeping backend changes minimal and preserving this directory as the fork-owned WebUI source.
