# WASM Build Path

Use the repository script:

```bash
./scripts/build_wasm.sh
```

This command:
- runs `wasm-pack build --target web --out-dir pkg`
- refreshes `dist/index.html`

If `wasm-pack` is missing, install instructions:
- https://rustwasm.github.io/wasm-pack/installer/
