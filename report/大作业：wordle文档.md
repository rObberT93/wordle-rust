# 程序结构

```
src
├── main.rs				// 主函数模式控制
├── game.rs				// 游戏相关函数
├── arg.rs				// 命令行参数解析
├── stats.rs			// 游戏状态存储
├── tui_mode.rs			// TUI
└── builtin_words.rs	// 词典
```

## 程序说明

- 在主函数中判断模式：
  - `TUI`
  - 测试模式
  - 交互模式
- `game.rs`中，结构体`Game`存储本局游戏状态，主要功能如下
  - 根据猜测的单词更新字母表状态，并获取当前猜测单词颜色状态
  - 打印单词和字母表
  - 对答案（`-w` 模式下）和猜测判断是否合法，在困难模式下判断是否严格使用提示
  - 获取伪随机单词
  - 判断游戏是否结束
  - （附加功能）筛选符合条件的剩余单词列表
  - （附加功能）给单词评分并返回分最高的5个候选词
  - （附加功能）全局下计算尝试次数
- `args.rs`中，命令行读取游戏初始状态，遇到冲突触发错误
- `stats.rs`中，存储和多局游戏相关的信息
  - 其中结构体`Stats`中的`Vec` `games`存储所有单局游戏的答案和猜测，以便写入`json`文件
- （附加功能）`tui_mode.rs`中主要功能为绘制界面和处理按键事件，可以完成交互模式中的基本所有功能，包括但不限于
  - 进入随机模式并指定day和seed
  - 进入指定答案模式，包括命令行指定答案和每局开局输入答案两种模式
  - 开启困难模式
  - 输出统计数据
  - 指定候选词和可用词列表
  - 读取json文件并将游戏状态存入json
  - 读取config文件中的配置

# 游戏主要功能说明和展示

|        参数         |          可选参数          |             功能             |                            备注                             |
| :-----------------: | :------------------------: | :--------------------------: | :---------------------------------------------------------: |
|      -w/--word      |       单词，如crane        |         指定答案模式         |                 参数为空默认为指定答案模式                  |
|     -r/--random     |             /              |           随机模式           |              随机模式和指定参数模式只能二选一               |
|   -D/--difficult    |             /              |           困难模式           |                必须使用上一步状态给出的提示                 |
|     -t/--stats      |             /              |       统计输出游戏信息       |                              /                              |
|      -d/--day       |         局数，如5          |        指定开始时局数        | 默认值为1，大小不能超过答案词库的大小；只能在随机模式下使用 |
|      -s/--seed      |        种子，如101         |     决定答案词库打乱顺序     |              默认值固定；只能再随机模式下使用               |
|   -f/--final-set    |       候选词库文件名       |   使用指定词库作为答案词库   |                     默认取内置候选词库                      |
| -a/--acceptable-set |       可选词库文件名       |   使用指定词库作为可用词库   |                     默认取内置可用词库                      |
|     -S/--state      | 保存有游戏状态的json文件名 | 保存和加载随机模式的游戏状态 |                     不合法错误异常退出                      |
|     -c/--config     |       配置文件文件名       |         指定启动配置         |        命令行参数优先级高于配置文件，可能会发生覆盖         |

以下为提高功能部分

|                             参数                             |                  功能                   |                             备注                             |
| :----------------------------------------------------------: | :-------------------------------------: | :----------------------------------------------------------: |
|                            --tui                             |               开启TUI模式               | TUI模式下仍可指定参数进行初始化，TUI界面中也可输入并指定答案 |
|                            --hint                            |              启用提示模式               |       将在每局输入猜测后筛选出符合状态的候选词，并输出       |
| 在指定提示模式下，将会对用户进行询问，输入y/n，即是否需要提示 | 根据算法，选出得分最高的5个词作为推荐词 |        只选出得分最高的至多5个词，只在提示模式下存在         |
|                            --test                            |        根据算法计算平均尝试次数         |                              /                               |
|                            --gui                             |               开启GUI模式               |                      可选择是否困难模式                      |

## 游戏模式展示

### 基础功能

`cargo run` 进入指定答案模式，将在每局开头对用户进行答案询问，并在游戏结束询问是否开启下一局游戏

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707025832059.png" alt="image-20230707025832059" style="zoom: 50%;" />

`cargo run -- -w hello` 进入指定答案模式，且指定答案为hello

*输入猜测单词后会对单词长度和是否在可用词列表中进行检查，不合法将输入报错，并提示重新输入*

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707030133478.png" alt="image-20230707030133478" style="zoom:50%;" />

`cargo run -- -r -s 100 -d 10`进入随机模式

*输入n退出游戏，y继续游戏*

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707030758049.png" alt="image-20230707030758049" style="zoom:50%;" />

`cargo run -- -r -D`开启困难模式，每一步必须利用上一步提示

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707031102947.png" alt="image-20230707031102947" style="zoom:50%;" />

### 冲突实例

比如在指定答案模式下不能使用种子和天数参数

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707041829204.png" alt="image-20230707041829204" style="zoom:33%;" />

随机模式和指定答案模式不能同时启用

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707042001733.png" alt="image-20230707042001733" style="zoom:33%;" />

天数指定过大或小于等于0

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707042055787.png" alt="image-20230707042055787" style="zoom: 50%;" />

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230708045534255.png" alt="image-20230708045534255" style="zoom: 50%;" />

等......

### 提高功能——求解

`cargo run --  -r --hint`

每局猜测后将输出筛选后的候选词，并询问是否需要提示

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707031450124.png" alt="image-20230707031450124" style="zoom:50%;" />

`cargo run -- --test`

遍历候选词列表和可用词列表，计算平均尝试次数，由于计算量过大，在使用并行计算后也未显著加快计算效率，因此使用随机选择的规模为官方候选词库一半大小的词库和包含答案词库前提下随机选择的规模为官方可用词库一半大小的可用词进行计算，进行多次随机选择，结果如下：

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707032336413.png" alt="image-20230707032336413" style="zoom:50%;" />

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707032416177.png" alt="image-20230707032416177" style="zoom:50%;" />

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707032511925.png" alt="image-20230707032511925" style="zoom:50%;" />

可见此算法还是有一定准确性的，只是计算复杂度过大，有待优化

### 提高功能——TUI

TUI模式下通过键盘输入进行交互，交互模式的信息在TUI模式下可以展示

天数、种子、候选词列表、可用词列表、游戏状态保存路径、游戏配置文件路径等参数均可正常选用

`cargo run -- -r -d 1 -s 101 --tui`进入随机模式：

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707032707727.png" alt="image-20230707032707727" style="zoom:50%;" />

可逐次展示统计信息：

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707032746872.png" alt="image-20230707032746872" style="zoom:50%;" />

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707032756893.png" alt="image-20230707032756893" style="zoom:50%;" />

并询问是否需要开启下一轮游戏：

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707032826273.png" alt="image-20230707032826273" style="zoom:50%;" />

`cargo run -- --tui -D`

开启困难模式，在下一步操作不符合上一步状态提示时与交互模式相同，会显示警告：

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230708045907278.png" alt="image-20230708045907278" style="zoom:50%;" />

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230708045955526.png" alt="image-20230708045955526" style="zoom:50%;" />

`cargo run -- --w --tui`

进入指定答案模式，输入自定义答案开始游戏：

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707033023636.png" alt="image-20230707033023636" style="zoom:50%;" />

输入后自定义答案及时消失，开始游戏猜测：

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707033210189.png" alt="image-20230707033210189" style="zoom:50%;" />

并在末尾询问是否开启下一轮游戏，与命令行交互模式的游戏逻辑一致。

游戏结束可正常退出程序：

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20230707042323325.png" alt="image-20230707042323325" style="zoom: 33%;" />

### 提高功能——GUI

- 开始界面：输入名字点击Start后即可开始，可以选择是否困难模式，若不选择，即普通随机模式

<img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20240310121258537.png" alt="image-20240310121258537" style="zoom: 33%;" />

​	底部的一句话可以根据时间调整：P

- 进入游戏界面：

  <img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20240310121534366.png" alt="image-20240310121534366" style="zoom: 33%;" />

  键盘略带有金属光泽（），页面组织结构和配色与正版游戏基本相似

  <img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20240310121956130.png" alt="image-20240310121956130" style="zoom: 33%;" />

  若一个不存在在词库里的单词被点击之后会弹出提示框提醒“不在单词表中”

  <img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20240310122050195.png" alt="image-20240310122050195" style="zoom: 33%;" />

  - 若选择困难模式

    尝试这样的输入：

    <img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20240310122854293.png" alt="image-20240310122854293" style="zoom:33%;" />

    嗯，不出所料

    <img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20240310122931771.png" alt="image-20240310122931771" style="zoom:33%;" />

- 点击页面上的exit随时退出游戏

- 若答案正确，页面的标题变成“You win！”，若尝试6词仍然答案错误，页面标题显示“You lose！”可以选择重新开始进行一轮游戏或者退出游戏

  <img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20240310122208655.png" alt="image-20240310122208655" style="zoom:33%;" />

  <img src="C:\Users\yuton\AppData\Roaming\Typora\typora-user-images\image-20240310122636925.png" alt="image-20240310122636925" style="zoom: 33%;" />



# 提高要求的实现方式

## 求解器

在求解器部分，对候选词筛选的思路如下：

- 一轮筛选判断位置：判断单词中绿色字母位置不变，单词中包含黄色字母且不在原位置，单词中红色字母不在原位置，并给红色字母标记状态
- 二轮筛选判断数量：遍历字母的5个位置统计各个字母出现的次数，如果某字母被标记了状态，说明被筛选出的单词的该字母的数量只能严格等于已有状态中该字母的黄色+绿色字母的数量；如果未被标记状态，说明被筛选出的单词的该字母的数量需要大于等于已有状态中该字母黄色+绿色字母的数量，例如最后猜测单词为abcxx，给出状态为gyrgr，说明被筛选出的单词必须满足a(?)(!c)x(!x)的形式，且被筛选出的单词中，b至少出现一次，c不能出现，x只能出现1次

对单词评分并给出推荐词的思路如下

- 受二分法启发，分别计算筛选出来的所有候选词0、1、2、3、4号位置的A-Z字母数量总数，给总数进行归一化得到字母权重，得到0-1之间的值
- 对每个单词将5个值相加得到分数，分数在0-5之间
- 使用rayon库中的`par_iter`和`par_sort`，对分数进行排序并选出分最高的5个

计算平均尝试次数的思路如下

- 遍历答案词库和候选词库，后续每局选用推荐词中的第一个
- 当推荐词的第一个和答案相等时返回尝试次数
- 该算法大致稳定在4-5局得到正确答案

## TUI

开始选用了cursive包，尝试了一天半后，却发现cursive库的按钮事件非常复杂，且颜色设置遇到了困难，故换用TUI包重新进行尝试，在刚入手的时候进度非常缓慢，遇到了很多未知的bug，比如出了缓冲区文本框会消失等等奇怪的问题。并且网上没有成型的教程，只能参照官方文档和开源代码缓慢学习。

在完成TUI模式时，尽量复用了已有代码，通过参数传入到TUI逻辑中，虽然成品比较简易，但还是很好用的，功能也比较完善！

## GUI

使用Rust原生的GUI库——fltk，按照官方游戏进行了一定程度的模仿 : P

# 完成此作业的感想

总体来说，完成本次作业的时间还是非常紧张的，但在这一周左右的时间中我也学到了很多东西。在最开始写代码的时候，由于对Rust语言的不熟悉，简单的判断单词状态都卡了我很久。逐渐对Rust语言熟悉起来，大作业的进度也推进了很多。我使用了Rust的一些库，比如`atty`、`rand`、`serde_json`、`console`、`colored`、`tui`、`crossterm`、`rayon`等等，但在使用这些包的时候我也遇到了很多问题，我不断参阅文档、参考GitHub上的开源代码和访问StackOverflow等社区，我逐渐自己解决了这些问题。在完成作业的过程中，我不少出现编译器报错很多的情况，也因此我认识到了Rust语言的安全性，Rust编译器的强大使得很多bug在编译期就可以被检查出来，比运行再遇到数百行的报错要效率高很多。

我第一次使用了Rust的测试框架进行单元测试和集成测试，我深感这会对开发检查提供很大的便利，而且Cargo作为包管理工具也让开发方便了很多。

在完成提高功能时，我也遇到了很多问题，比如开始设计的算法复杂度过高或者错误、TUI不断摸索成型......

Rust语言对于我来说是一种全新的语言，也是一种完全陌生的体验。通过完成这次作业，我加深了对Rust语言性能和安全性的认识，此外，通过完成这次作业，我不断的修改我已经写过的接口、存入新东西等等，不断的对已有的代码进行修正和改进，才能完成后面的功能和提高功能。期待在接下来的Rust语言学习中学习更多的新东西，也希望我可以继续提高我的代码能力！