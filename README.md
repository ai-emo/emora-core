# 电子生物 - 习惯生物系统

## 概述

本项目实现了一个基于PAD情感模型的电子生物，逐步从反射生物进化到智慧生物。

当前阶段：**阶段2 - 习惯生物**

## 运行方式

### 命令行版本
```bash
cargo run --bin emora
```

### GUI可视化版本
```bash
cargo run --bin emora-gui
```

## 演进路线

- [x] **阶段1**: 反射生物 - 传感器→反射→动作
- [x] **阶段2**: 习惯生物 - 短期记忆 + TD学习 (当前)
- [ ] **阶段3**: 情感生物 - 情感记忆 + 联想学习
- [ ] **阶段4**: 智慧生物 - 完整强化学习 + 长期规划

---

## 阶段2：习惯生物

### 新增功能

1. **短期记忆 (ShortTermMemory)**: 存储最近的感知-动作-结果
2. **长期记忆 (LongTermMemory)**: Q值表，存储状态-动作对的价值
3. **TD学习 (TD-Learning)**: 时间差分学习算法

### 架构

```
传感器 (Sensor) → 感知 (Percept) → 反射弧/学习 → 动作 (Action)
                      ↓
                 短期记忆 (记录)
                      ↓
                 TD学习器 (更新Q值)
                      ↓
                 长期记忆 (Q表)
```

### 核心组件

#### 1. 记忆系统 (`models/memory.rs`)

```rust
/// 记忆条目
pub struct MemoryItem {
    pub perception: String,  // 感知描述
    pub reflex: String,       // 激发的反射
    pub result: String,       // 动作结果
    pub reward: f32,         // 获得的奖励
    pub timestep: usize,      // 时间步
}

/// 短期记忆 - 工作记忆
pub struct ShortTermMemory {
    capacity: usize,
    items: Vec<MemoryItem>,
}

/// 长期记忆 - Q值表
pub struct LongTermMemory {
    q_table: HashMap<String, f32>,
    learning_rate: f32,
    discount_factor: f32,
}
```

#### 2. TD学习器 (`models/learner.rs`)

```rust
pub struct TDLearner {
    short_term: ShortTermMemory,
    long_term: LongTermMemory,
}

impl TDLearner {
    /// 记录经验
    pub fn record(&mut self, perception, reflex, result, reward);
    /// TD学习更新
    pub fn learn(&mut self);
    /// 决定最佳动作
    pub fn decide(&self, perception, available_actions) -> Option<String>;
}
```

### 学习算法

TD学习核心公式：

```
Q(s,a) ← Q(s,a) + α * [r + γ * max(Q(s',a')) - Q(s,a)]
```

- α (alpha): 学习率
- γ (gamma): 折扣因子
- r: 奖励
- s: 当前状态
- a: 动作

### 使用示例

```rust
use emora_core::{HabitBrain, Stimulus, World};

fn main() {
    let mut agent = HabitBrain::new();

    let mut world = World::new();
    world.add_food(10.0, 5.0, 20.0);
    world.add_danger(-20.0, 0.0, 30.0);

    agent.set_world(world);

    // 降低能量触发饥饿
    agent.perceive(Stimulus::Food(-40.0));

    // 运行多个时间步
    for _ in 0..10 {
        agent.tick_habit();
    }

    // 查看学习结果
    let memories = agent.get_learner().get_short_term().all();
    println!("记忆条目数: {}", memories.len());
}
```

### 决策模式

HabitBrain有三种决策模式：

1. **Reflex (反射模式)**: 记忆少于10条时，仅使用先天反射
2. **Hybrid (混合模式)**: 记忆大于10条时，优先使用反射
3. **Learned (学习模式)**: 未来将支持使用学到的Q值

---

## 阶段1回顾：反射弧系统

### 反射弧架构

```
传感器 (Sensor)  →  感知 (Percept)  →  反射弧 (Reflex)  →  动作 (Action)
     ↓                    ↓                   ↓                  ↓
 ProximitySensor    FoodDetected        Approach          MoveTowards
 InternalSensor    DangerDetected       Flee              Flee
                   Hungry               SeekFood          SeekFood
                   Threatened           Rest              Rest
```

### 默认反射弧

| 触发条件 | 阈值 | 反射类型 | 优先级 |
|---------|------|---------|-------|
| DangerDetected | 0.1 | Flee | 100 |
| FoodDetected | 0.1 | Approach | 80 |
| Hungry | 0.3 | SeekFood | 70 |
| Safe | 0.5 | Rest | 10 |

---

## GUI可视化

使用egui实现的实时2D地图界面：

```bash
cargo run --bin emora-gui
```

### 功能特性

- **2D地图视图**
  - 绿色背景表示草地
  - 网格线便于观察位置
  - 红色圆点表示食物（大小表示能量值）
  - 黑色圆点表示危险区域
  - 彩色光晕表示电子生物
  - 轨迹线显示移动历史

- **情绪可视化**
  - 生物颜色随情绪变化：
    - 黄色：兴奋 (高愉悦+高唤醒)
    - 红色：惊恐 (低愉悦+高唤醒)
    - 蓝色：悲伤 (低愉悦+低唤醒)
    - 绿色：愉悦
    - 灰色：中性
  - 生物上方显示能量条

- **控制面板**
  - 切换脑类型（Reflex/Habit）
  - Start/Pause/Step/Reset按钮
  - 速度调节滑块
  - 运行计数显示

- **状态显示**
  - 能量值
  - PAD三维度数值
  - 情绪状态文字

---

## 状态说明

- **能量 (Energy)**: 0-100，影响愉悦度 (Pleasure)
- **安全 (Safety)**: 0-100，影响支配度 (Dominance)
- **PAD模型**:
  - Pleasure = Energy/100 * 2 - 1
  - Dominance = Safety/100 * 2 - 1
  - Arousal = (Hunger + Fear) / 2
