# godot_rust_dodge

使用 `godot native` 配合 `gdnative` 实现官方 demo: Dodge the Creeps

可能是 `gdnative` 的版本不一样了，所以实现方式跟官方案例有一点点点区别。同时添加了文档提到的但是官方 demo 没实现的内容

## native 体验

由于 godot native 是一套 api，理论上只要适配 godot 提供的 cpp 接口就可以使用任意语言编写脚本。但目前而言 `rust version` 没有比「官方提供方案」CPP 体验好。包括不限于：

- 编辑器无法自动刷新（必须重启项目，菜单 - 项目 - 退出到项目列表 - 重进）

## 依赖
- gdnative 0.9.3

## Links

- DOC: [Your first game](https://docs.godotengine.org/en/stable/getting_started/step_by_step/your_first_game.html)
- [GODOT RUST](https://godot-rust.github.io/book/introduction.html)
- [godot-rust/godot-rust](https://github.com/godot-rust/godot-rust) (GitHub)

## License

Apache 2.0

（或者无视，反正就是一个 clone demo，资源来自官方文档）
