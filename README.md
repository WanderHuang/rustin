# rustin

[中文](./README_ZH.md)

Let's build web pages by `rust` & `js`!

## 📎Plan

- [x] Test the efficency of wasm, using Rust in browser.
- [ ] Create a scaffold, using Rust and Web Worker, for future web development.

## 🚀 [Webassembly](https://webassembly.github.io/spec/core/)
A type of machine code for browser to handle time waste jobs, which can assembly from multiple programming language.

### Test
A non-block calculation in browser implement with `web worker`, mean while, we can use `wasm` to handle time waste job.
All for a better experience of web site.
![design](./img/framework.png)

### Result
![benchmark](./img/benchmark-1.jpeg)

## 🧵 [Worker](https://developer.mozilla.org/en-US/docs/Web/API/Web_Workers_API/Using_web_workers)
`Web Workers` is a simulated multi-thread environment in browser.

- [x] No block to main thread.
- [x] Asynchronous communication.
- [x] More efficient.
- [x] ...

## 🐂 [Rxjs](https://github.com/ReactiveX/RxJS)

A paradigm of programming, functional and reactive. It's a perspective of time, we handle data and event during all the time.
A different perspective for dealing with data flow and event.

- [x] Functional
- [x] Reactive
- [x] operators
- [x] handle complex events
- [x] Synchronous or Asynchronous.
- [x] Broadcast
- [x] ...

## 😋 [Rust](https://www.rust-lang.org/)

Rust is a language for future with many advanced programming tech.

### slogan

From offical site:

A language empowering everyone to build reliable and efficient software.

- Performance
  Rust is blazingly fast and memory-efficient: with no runtime or garbage collector, it can power performance-critical services, run on embedded devices, and easily integrate with other languages.

- reliable
  Rust’s rich type system and ownership model guarantee memory-safety and thread-safety — enabling you to eliminate many classes of bugs at compile-time.

- Productivity
  Rust has great documentation, a friendly compiler with useful error messages, and top-notch tooling — an integrated package manager and build tool, smart multi-editor support with auto-completion and type inspections, an auto-formatter, and more.

My opinion：
  It's kind of wierd to use rust for frontend developer who using javascript all the time.
  - struct | trait | type
  - ownership
  - multi-thread
  - so on

  But we can take nourishment from comprehensive documentation by offical. Those advanced techs.

  Not a bad trial for full-stack engineer to use Rust(`server / wasm`)。
  
## 📖Resources

### documentation
- [Official](https://www.rust-lang.org/zh-CN/)
- [emscripten](https://emscripten.org/)
  ![](./img/EmscriptenToolchain.png)

### Library & Tool
- [`wasm-bindgen`](https://crates.io/crates/wasm-bindgen)
- [`wasm-bindgen-futures`](https://crates.io/crates/wasm-bindgen-futures)
- [`js-sys`](https://crates.io/crates/js-sys)
- [`web-sys`](https://crates.io/crates/web-sys)
- [`console_error_panic_hook`](https://crates.io/crates/console_error_panic_hook)
- [`console_log`](https://crates.io/crates/console_log)
- [`wee_alloc`](https://crates.io/crates/wee_alloc)
- [`parity-wasm`](https://crates.io/crates/parity-wasm)
- [`wasmparser`](https://crates.io/crates/wasmparser)
- [`wasmi`](https://crates.io/crates/wasmi)
- [`cranelift-wasm`](https://crates.io/crates/cranelift-wasm)
- [`wasm-pack`](https://github.com/rustwasm/wasm-pack)
- [`wasm-opt`](https://github.com/WebAssembly/binaryen)
- [`wasm2js`](https://github.com/WebAssembly/binaryen)
- [`wasm-gc`](https://github.com/alexcrichton/wasm-gc)
- [`wasm-snip`](https://github.com/rustwasm/wasm-snip)
- [`twiggy`](https://github.com/rustwasm/twiggy)
- [`wasm-objdump`](https://github.com/WebAssembly/wabt)
- [`wasm-nm`](https://github.com/fitzgen/wasm-nm)

## 🌚Not working now

- import `c` module(`.h`)
- File system | IO
- multi-thread

## 🌝Usually for

- Algorithm
- Image processing
- Basic data structure.
- Text processing
- Game application