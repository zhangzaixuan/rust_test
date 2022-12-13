#macos rust 安装文档
```shell
参考网址 https://www.rust-lang.org/zh-CN/tools/install
01 执行rust安装命令
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh\n
02 环境变量生效
source "$HOME/.cargo/env"
03 查看rustc和cargo版本
zhangzhao@zhangzhaodeMacBook-Air ~ % rustc --version
rustc 1.65.0 (897e37553 2022-11-02)
zhangzhao@zhangzhaodeMacBook-Air ~ % cargo --version
cargo 1.65.0 (4bc8f24d3 2022-10-20)
```

#idea 添加rust插件和构建项目
```text
01 plugins 安装 rust和 toml 插件，安装完成后需要对idea restart

02 rustup component add rust-src (rust-src组件的安装)

安装了rust-src,idea才能导入rust的工具引用链；
/Users/zhangzhao/.rustup/toolchains/stable-aarch64-apple-darwin/lib/rustlib/src/rust


出现 No Cargo projects found! 时attach,将项目目录重指一下（后续细究一下）

```
#rust从git导包配置
```
# 放到 `$HOME/.cargo/config` 文件中
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"

# 替换成你偏好的镜像源
replace-with = 'sjtu'
#replace-with = 'ustc'

# 清华大学
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# 中国科学技术大学
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 上海交通大学
[source.sjtu]
registry = "https://mirrors.sjtug.sjtu.edu.cn/git/crates.io-index"

# rustcc社区
[source.rustcc]
registry = "git://crates.rustcc.cn/crates.io-index"

[net]
git-fetch-with-cli=true
```