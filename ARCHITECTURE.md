# NoctisRoll 架构设计

## 概述

NoctisRoll 是一个现代化的、模块化的 TRPG 掷骰系统，完整实现了 [OneDice](https://github.com/OlivOS-Team/onedice) 标准。本库采用 Rust 语言编写，强调类型安全、模块化设计和可扩展性。

## 架构设计

### 核心模块

```
src/
├── lib.rs              # 库入口和预导入
├── core.rs             # 核心类型和 trait (Dice, RollResult, DiceContext)
├── error.rs            # 错误类型
├── utils.rs            # 工具函数
├── dice/               # 骰子类型实现
│   ├── mod.rs
│   ├── standard.rs     # 标准多面骰 (d)
│   ├── fate.rs         # FATE/Fudge 骰 (f/df)
│   ├── exploding.rs    # 爆炸骰 (!)
│   ├── pool.rs         # 骰池 (a, c)
│   └── composite.rs    # 复合表达式
└── parser/             # 解析器 (可选功能)
    ├── mod.rs
    ├── pest_parser.rs
    └── dice.pest       # Pest 语法文件
```

### 核心抽象

#### `Dice` Trait
所有骰子类型的基础 trait，定义了统一的接口：
- `roll()` - 执行掷骰并返回结果
- `describe()` - 返回骰子描述
- `expected_value()` - 计算期望值
- `min_value()` - 最小可能值
- `max_value()` - 最大可能值

#### `ModifiableDice` Trait
支持操作的骰子扩展 trait：
- `keep_highest()` / `keep_lowest()` - 保留最高/最低骰子
- `drop_highest()` / `drop_lowest()` - 丢弃最高/最低骰子
- `reroll_below()` / `reroll_above()` - 重掷低于/高于阈值的骰子
- `explode_above()` / `explode_below()` - 爆炸骰

#### `RollResult`
掷骰结果的完整表示：
- `rolls: Vec<DieRoll>` - 单个骰子结果
- `total: i64` - 总和
- `description: String` - 描述字符串
- `success: Option<bool>` - 是否成功（成功型骰池）
- `success_count: Option<u32>` - 成功数
- `failure_count: Option<u32>` - 失败数

### 支持的骰子类型

#### 1. 标准多面骰 (`StandardDice`)
- **语法**: `AdB(kq)C(pb)DaE`
- **示例**: `2d20`, `4d6k3`, `d100p2`
- **特性**: 保留最高/最低、奖惩骰、成功阈值

#### 2. FATE/Fudge 骰 (`FateDice`)
- **语法**: `AfB`
- **示例**: `4dF`, `8df`
- **特性**: 每面值为 -1, 0, +1

#### 3. 爆炸骰 (`ExplodingDice`)
- **语法**: `AdB!≥C`
- **示例**: `2d6!6`, `4d10!≥8`
- **特性**: 达到阈值时额外掷骰

#### 4. 无限加骰池 (`InfiniteAddingPool`)
- **语法**: `AaBkCqDmE`
- **示例**: `5a8k6m10`
- **特性**: World of Darkness 风格，成功时增加骰子

#### 5. 双重十字骰池 (`DoubleCrossPool`)
- **语法**: `AcBmC`
- **示例**: `5c8m10`
- **特性**: Double Cross 规则，特殊计分方式

### 设计原则

#### 1. 类型安全
- 所有操作都有编译时检查
- 错误处理通过 `DiceResult<T>` 类型
- 避免运行时 panic

#### 2. 模块化
- 每种骰子类型独立实现
- 易于添加新骰子类型
- 解析器为可选功能

#### 3. 性能优化
- 零分配简单掷骰
- 惰性求值
- 批量操作支持

#### 4. 可扩展性
- 通过 trait 定义接口
- 支持自定义骰子类型
- 可配置的上下文和随机源

### OneDice 兼容性

#### 已实现的功能
- [x] 四则运算 `+-*/`
- [x] 优先级运算 `()`
- [x] 普通多面掷骰 `d`
- [x] 惩罚骰/奖励骰 `p/b`
- [x] 无限加骰池 `a`
- [x] 双重十字加骰池 `c`
- [x] FATE掷骰池 `f/df`
- [x] 三目运算 `?`
- [x] 优势劣势/最大最小 `kh/kl/dh/dl`
- [x] 限制最大/最小 `max/min`
- [x] 按位与/或 `&/|`
- [x] 判断大于/小于 `>/<`

#### 计划中的功能
- [ ] 爆骰 `xo`
- [ ] 阶乘 `!`
- [ ] 按位非 `!`
- [ ] 循环 `lp`
- [ ] 裁切 `sp`
- [ ] 弹射 `tp`

### 使用示例

#### 基本使用
```rust
use noctisroll::prelude::*;

// 创建骰子
let dice = StandardDice::new(2, 20).keep_highest(1);
let result = dice.roll();
println!("{} = {}", dice.describe(), result.total);
```

#### 骰池系统
```rust
use noctisroll::dice::{InfiniteAddingPool, DoubleCrossPool};

// World of Darkness 风格
let pool = InfiniteAddingPool::new(5, 8)
    .with_success_threshold(6);
let result = pool.roll();

// Double Cross 风格
let pool = DoubleCrossPool::new(5, 8);
let result = pool.roll();
```

#### 批量操作
```rust
use noctisroll::utils::{batch_roll, RollStatistics};

let dice = StandardDice::new(100, 6);
let results = batch_roll(&dice, 1000);
let stats = RollStatistics::from_rolls(
    &results.iter().flat_map(|r| &r.rolls).cloned().collect::<Vec<_>>()
);
```

### 性能考虑

1. **内存分配**: 简单掷骰避免堆分配
2. **随机数生成**: 使用线程本地 RNG
3. **批量操作**: 支持并行化
4. **缓存友好**: 数据布局优化

### 测试策略

1. **单元测试**: 每个骰子类型独立测试
2. **属性测试**: 验证数学属性（如范围、期望值）
3. **集成测试**: 完整表达式解析和求值
4. **性能测试**: 批量操作基准测试

### 扩展指南

#### 添加新骰子类型
1. 在 `src/dice/` 下创建新模块
2. 实现 `Dice` trait
3. 可选实现 `ModifiableDice` trait
4. 在 `src/dice/mod.rs` 中导出
5. 添加测试用例

#### 添加新操作
1. 扩展 `ModifiableDice` trait
2. 在相关骰子类型中实现
3. 更新解析器支持新语法
4. 添加测试用例

### 依赖管理

#### 必需依赖
- `rand = "0.8"` - 随机数生成
- `thiserror = "1.0"` - 错误处理
- `serde = "1.0"` - 序列化支持

#### 可选依赖
- `pest = "2.7"` - 解析器支持（启用 `parser` feature）
- `pest_derive = "2.7"` - Pest 宏支持

### 发布计划

#### v0.2.0 (当前)
- 核心骰子类型实现
- 基本 OneDice 支持
- 模块化架构

#### v0.3.0 (计划中)
- 完整解析器实现
- 更多 OneDice 功能
- 性能优化

#### v1.0.0 (目标)
- 稳定 API
- 完整文档
- 性能基准
- 生产就绪