# 包 & 模块

几个概念

- packages: 可以被Cargo管理，构建、测试，作为公共库
- crates: 树状的模块，可以作为库引入或者单独执行
- modules(use): 模块
- paths: 结构体、函数或者模块都是可引用路径

## 包
`cargo`用于管理仓库，比如构建、测试、运行、安装包

```bash
cargo new my-project
```

包根目录有一个`Cargo.toml`描述文件

## 模块

```rust
// mod定义模块，可嵌套
pub mod parent {
  pub mod sub1 {
    pub fn add () {}
  }
  mod sub2 {
    fn del () {}
  }
}

// 树形结构
// - crate
//   - parent
//     - sub1
//       - add
//     - sub2
//       - del

```

## 引用路径

Rust内使用`::`表示路径的引用。引用路径可以抵达结构体、模块、包、方法
```rust
fn main () {
  // 前提是对应的模块或者方法是`pub`类型的
  // 顶层的crate::我们可以省略
  parent::sub1::add();
}

```

如果是同一个模块可以用`super`引用相对路径上的模块，如

```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        // 获取根路径的相对路径，使用super
        super::serve_order();
    }

    fn cook_order() {}
}

fn main() {}


```

当然还有结构体的示例

```rust

#![allow(unused_variables)]
fn main() {
  mod back_of_house {
    pub struct Breakfast {
      pub toast: String,
      seasonal_fruit: String,
    }

    impl Breakfast {
      pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
          toast: String::from(toast),
          seasonal_fruit: String::from("peaches"),
        }
      }
    }
  }

  pub fn eat_at_restaurant() {
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
  }
}

```

和枚举

```rust
#![allow(unused_variables)]
fn main() {
  mod back_of_house {
    pub enum Appetizer {
      Soup,
      Salad,
    }
  }

  pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
  }
}


```

## 跨文件引用
用到关键字 `pub` 和 `use`
另外一个关键点是从lib.rs那一级开始，文件夹和文件的名字也会作为一个模块被解析。
```rust
/// 引用标准库
use std::fmt;
use std::io;
/// 引用别名
use std::io::Result as IoResult;
/// 解构引用
use std::fmt::{Result}
/// use引用的数据是私有的，可以使用pub再发布成公共的
pub use std::fmt;
/// 使用外部库(Cargo.toml dependencies)
use rand::Rng;
/// 使用另一个文件内other_module.rs的模块calc 或 文件夹src/other_module/下的calc文件内的`pub fn add`方法
pub use crate::other_module::calc::add;

mod front_of_house {
  pub mod hosting {
    pub fn add_to_waitlist() {}
  }
}

/// 引用自己
use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
  hosting::add_to_waitlist();
}

fn main() {}

```