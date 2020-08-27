# 模板代码

使用了`wasm-pack-template`

## 创建模板

```bash
cargo generate --git https://github.com/rustwasm/wasm-pack-template.git --name my-project
cd my-project
```

## 构建

```bash
wasm-pack build
```

## 打包

```bash
wasm-pack test --headless --firefox
```

## 发布

```bash
wasm-pack publish
```

## 关联包

* [`wasm-bindgen`](https://github.com/rustwasm/wasm-bindgen) `webassembly`和`javascript`交互（通信）
* [`console_error_panic_hook`](https://github.com/rustwasm/console_error_panic_hook) 开发者模式打错误日志
* [`wee_alloc`](https://github.com/rustwasm/wee_alloc) 小型的内存分配器


## 计划

- 语法学习
- 实用功能
