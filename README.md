# 大作业：Wordle

2023 年夏季学期《程序设计训练》 Rust 课堂大作业（一）。
## 运行方法
本项目建立在Linux环境中,可以通过WSL实现.执行``wsl --set-default-version 2`将WSL版本切换至WSL2.

具体发行版信息为
```bash
cat /etc/issue
Debian GNU/Linux 12 \n \l
```

### 1. WSL图形化界面配置
- 如果正在使用 Windows 11: WSL2 可能已经带有了一个 X Server 实现 (WSLg)。通过以下方法检查：
    - 启动终端进入 WSL2 发行版，执行 echo $DISPLAY，如果显示不为空，说明存在 WSLg。
- 如果没有在使用 Windows 11，或者上述测试输出为空: 需要自行安装 X Server，并手动设置 DISPLAY 环境变量。
    - 安装 VcXsrv。然后运行 XLaunch，会弹出一个配置窗口，一路前进，勾选 Disable access control，最后选择 Finish，然后应该可以在右下角的状态栏中找到它。这时 X Server 已经启动。第一次启动的时候可能会弹出窗口询问是否打开防火墙权限，此时要勾选所有的网络并同意。
    - 接着需要在 WSL 里面配置 DISPLAY 环境变量。这一步在不同的 WSL 版本中要用不同的办法。

**WSL2**

我们在 Windows 上运行了 X Server，那么此时 WSL2 是客户端，Windows 是服务端。我们需要获取 Windows 的 IP 地址，以让 WSL2 通过网络连接运行在 Windows 上的 X Server。

由于默认情况下，Windows 也是 WSL2 的默认网关，因此我们可以通过查询路由表来获取 Windows 的 IP 地址：

```bash
export DISPLAY=$(ip route show default | cut -d' ' -f3):0
```
同理，Windows 也是 WSL2 的默认 DNS 服务器。因此，也可以从 DNS 配置中找到 Windows 的 IP 地址：

```bash
export DISPLAY=$(awk '/nameserver / {print $2; exit}' /etc/resolv.conf 2>/dev/null):0
```
通常情况下，使用上面两种方法得到的 Windows 的 IP 地址是相同的。如果使用了代理等修改网络配置的软件，可能出现结果不一致或者错误的问题，此时需要根据实际情况判断 IP 地址。

**测试图形化界面**
可以在 Linux 中安装 x11-apps 软件包并执行 xclock，如果 X Server 运转正常，可以显示出一个钟表。

```bash
sudo apt update
sudo apt upgrade
sudo apt install -y x11-apps
xclock
```

**问题排查**
如果测试 X Server 时无法正常显示，按照以下的方式排查：

- 检查 DISPLAY 环境变量是否配置了：echo $DISPLAY
- 确认你用的是 WSL1 还是 WSL2：在 命令提示符 里运行 wsl -l -v 可以看到版本
- 确认你在上面运行的命令和你的 WSL 版本是一致的
- 检查 VcXsrv 是否启动：桌面右下角应该可以找到它的图标
- 确认 VcXsrv 启动的时候勾选了 Disable access control：如果不记得了，退出 VcXsrv 再重新开一次
- 在桌面右下角图标里右键，选择 Applications -> xcalc，确认 VcXsrv 本身可以显示窗口
- 修改防火墙规则：设置，找到防火墙，选择高级设置，左侧找到入站规则，在中间找到所有 VcXsrv 相关的条目，如果左边显示的不是绿色的勾，则双击对应的属性，选择允许连接。保证所有 VcXsrv 相关的规则都是绿色的
- 如果还是不行，尝试关闭杀毒软件
- 如果还是不行，尝试用管理员权限启动 VcXsrv
- 如果还是不行，尝试禁用整个防火墙

> 参考: [清华大学计算机系Rust小学期作业文档](https://lab.cs.tsinghua.edu.cn/rust/environment/)

## 2. 安装FLTK
### 2.1 cmake安装
直接`apt`安装
```bash
sudo apt install cmake
```

### 2.2 cmake配置
在Linux下，使用cmake进行项目生成前，务必确保一些基础库的安装：

```bash
# 安装gcc/g++等核心开发构建工具和库（必备）
sudo apt-get install build-essential
# openGL库安装（可选，建议。OpenGL的Library、Utilities以及ToolKit）
sudo apt-get install libgl1-mesa-dev
sudo apt-get install libglu1-mesa-dev
sudo apt-get install freeglut3-dev
# openSSL库（可选）
sudo apt-get install libssl-dev
# x11库（必备）
sudo apt-get install libx11-dev
```

cmake进行项目构建完成后，在我们当前的build目录中，对于macOS/Linux类操作系统，CMake会为我们生成了对应的makefile文件，所以我们直接使用make命令调用本地的clang或则gcc进行编译。

### 2.3 编译静态库文件
首先从官方地址下载FLTK 1.3.8代码：
[Download - Fast Light Toolkit (FLTK)](https://link.zhihu.com/?target=https%3A//www.fltk.org/software.php%3FVERSION%3D1.3.8)

我下载的是fltk-1.3.8-source.tar.bz2版本的压缩包, 下载完成后，将文件内容解压至某个自定义目录，例如在WSL下，我存放在"/mnt/d/program/fltk-1.3.8"目录。

进入该目录后创建一个build目录，并进入build目录，然后使用CMake进行配置。运行``cmake ..``即可

> 参考: [FLTK基于cmake编译以及使用](https://zhuanlan.zhihu.com/p/575224985)
### 2.4 使用fltk库写代码力
在cargo.toml中添加
```bash
fltk = "^1.4"
```
或终端输入
```bash
cargo add flkt
```
> 参考: [官方仓库](https://github.com/fltk-rs/fltk-rs?tab=readme-ov-file)


## 作业要求

具体要求请查看[作业文档](https://lab.cs.tsinghua.edu.cn/rust/projects/wordle/)。

## Honor Code

请在 `HONOR-CODE.md` 中填入你完成作业时参考的内容，包括：

* 开源代码仓库（直接使用 `crate` 除外）
* 查阅的博客、教程、问答网站的网页链接
* 与同学进行的交流

## 自动测试

本作业的基础要求部分使用 Cargo 进行自动化测试，运行 `cargo test [--release] -- --test-threads=1` 即可运行测试。其中 `[--release]` 的意思是可以传 `--release` 参数也可以不传，例如 `cargo test -- --test-threads=1` 表示在 debug 模式下进行单线程测试，而 `cargo test --release -- --test-threads=1` 表示在 release 模式下进行单线程此时。

如果某个测试点运行失败，将会打印 `case [name] incorrect` 的提示（可能会有额外的 `timeout` 提示，可以忽略）。你可以在 `tests/cases` 目录下查看测试用例的内容，还可以使用以下命令手工测试：

```bash
cp tests/cases/[case_name].before.json tests/data/[case_name].run.json # 复制游戏初始状态文件（如果需要）
cargo run [--release] -- [options] < test/cases/[case_name].in > test/cases/[case_name].out # 运行程序
diff tests/cases/[case_name].ans tests/cases/[case_name].out # 比较输出
jq -set tests/data/[case_name].after.json tests/data/[case_name].run.json # 比较游戏状态文件（如果需要）
```

其中 `[options]` 是游戏使用的命令行参数，`[case_name]` 是测试用例的名称。`jq` 工具可以使用各类包管理器（如 `apt` 或 `brew`）安装。

项目配置了持续集成（CI）用于帮助你测试。在推送你的改动后，可以在 GitLab 网页上查看 CI 结果和日志。

## 其他说明

1. `src/builtin_words` 是内嵌于程序中的单词列表，`FINAL` 为所有答案词，`ACCEPTABLE` 为所有候选词。
2. 为了实现更多功能（如 GUI 或求解器），你可以自由地调整本项目的结构（如增加新的 binary 或者划分 crate，或者使用 Cargo workspace 组织多级项目），但需要满足以下条件，并在验收时提前告知助教：
    * 所有的测试命令都能够按现有的方式运行；
    * 不能对 `tests` 目录的内容进行任何修改（但可以整体移动到某个位置）。
