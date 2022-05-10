<div align="center">

  <h1><code>wasm-demo</code></h1>

  <strong>Using Rust <a href="https://github.com/rustwasm/wasm-pack">wasm-pack</a> as demo.</strong>
</div>

## About

[**📚 Read this template tutorial! 📚**][template-docs]

This template is designed for compiling Rust libraries into WebAssembly and
publishing the resulting package to NPM.

Be sure to check out [other `wasm-pack` tutorials online][tutorials] for other
templates and usages of `wasm-pack`.

[tutorials]: https://rustwasm.github.io/docs/wasm-pack/tutorials/index.html
[template-docs]: https://rustwasm.github.io/docs/wasm-pack/tutorials/npm-browser-packages/index.html

## 🚴 Usage

### 🐑 Use `cargo generate` to Clone this Template

```bash
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

### 🛠️ Build with `wasm-pack build`

```bash
wasm-pack build --release --target web --out-name mod
```

### 🔬 Try to use with `deno`

```bash
~/m/r/hello-wasm ||HEAD⚡?
$ deno
Deno 1.20.3
exit using ctrl+d or close()
> const mod = await import('./pkg/mod.js');
undefined
> await mod.default();
{ memory: WebAssembly.Memory {}, greet: [Function: 1] }
> mod.greet();
Hello, hello-wasm! [Enter]
undefined
>
```
