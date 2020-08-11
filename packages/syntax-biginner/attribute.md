# 属性

类似于其他语言中的注解、装饰等，制定一个作用域可以引用的数据

## 基本格式
```rust
/// 测试代码
#[test]

/// Foo注解
#[foo]
struct Foo;

/// 目标平台linux
#[cfg(target_os = "linux")]
mod bar {
    /// 带!表示注解离他最近的项，这里是mod bar
    #![bar]
}
```

## 语法规则

```
Syntax
Attribute :
   InnerAttribute | OuterAttribute

InnerAttribute :
   #![ MetaItem ]

OuterAttribute :
   #[ MetaItem ]

MetaItem :
      IDENTIFIER
   | IDENTIFIER = LITERAL
   | IDENTIFIER ( MetaSeq )

MetaSeq :
      EMPTY
   | MetaItem
   | LITERAL
   | MetaItem , MetaSeq

```

## 常见配置

- `crate_namte` 设置crate名
- `crate_type` 类型
- `test` 测试
- `deprecated` 不推荐
- `derive` 派生Trait
