//! 记忆系统 - 存储经验用于学习

/// 经验：感知-动作-结果的三元组
#[derive(Debug, Clone)]
pub struct Experience {
    /// 当前感知
    pub state: String,
    /// 执行的动作
    pub action: String,
    /// 获得的奖励
    pub reward: f32,
    /// 下一状态
    pub next_state: String,
    /// 是否结束
    pub done: bool,
}

/// 记忆条目 - 更详细的记录
#[derive(Debug, Clone)]
pub struct MemoryItem {
    /// 感知到的信息描述
    pub perception: String,
    /// 激发的反射类型
    pub reflex: String,
    /// 动作结果
    pub result: String,
    /// 获得的奖励（能量变化）
    pub reward: f32,
    /// 时间步
    pub timestep: usize,
}

impl MemoryItem {
    pub fn new(perception: String, reflex: String, result: String, reward: f32, timestep: usize) -> Self {
        MemoryItem {
            perception,
            reflex,
            result,
            reward,
            timestep,
        }
    }
}

/// 短期记忆 - 类似于工作记忆
#[derive(Debug)]
pub struct ShortTermMemory {
    /// 记忆容量
    capacity: usize,
    /// 记忆条目
    items: Vec<MemoryItem>,
    /// 当前时间步
    timestep: usize,
}

impl ShortTermMemory {
    pub fn new(capacity: usize) -> Self {
        ShortTermMemory {
            capacity,
            items: Vec::with_capacity(capacity),
            timestep: 0,
        }
    }

    /// 添加记忆
    pub fn remember(&mut self, item: MemoryItem) {
        if self.items.len() >= self.capacity {
            // 移除最老的记忆
            self.items.remove(0);
        }
        self.items.push(item);
    }

    /// 获取最近的N条记忆
    pub fn recent(&self, n: usize) -> Vec<&MemoryItem> {
        let start = if self.items.len() > n {
            self.items.len() - n
        } else {
            0
        };
        self.items[start..].iter().collect()
    }

    /// 获取所有记忆
    pub fn all(&self) -> &Vec<MemoryItem> {
        &self.items
    }

    /// 清空记忆
    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// 增加时间步
    pub fn tick(&mut self) {
        self.timestep += 1;
    }

    /// 获取当前时间步
    pub fn get_timestep(&self) -> usize {
        self.timestep
    }
}

impl Default for ShortTermMemory {
    fn default() -> Self {
        Self::new(100)
    }
}

/// 长期记忆 - 存储学习到的值
#[derive(Debug)]
pub struct LongTermMemory {
    /// 状态-动作值函数 Q(s, a)
    q_table: std::collections::HashMap<String, f32>,
    /// 学习率
    learning_rate: f32,
    /// 折扣因子
    discount_factor: f32,
}

impl LongTermMemory {
    pub fn new(learning_rate: f32, discount_factor: f32) -> Self {
        LongTermMemory {
            q_table: std::collections::HashMap::new(),
            learning_rate,
            discount_factor,
        }
    }

    /// 获取Q值
    pub fn get_q(&self, state_action: &str) -> f32 {
        *self.q_table.get(state_action).unwrap_or(&0.0)
    }

    /// 更新Q值 (TD学习)
    pub fn update_q(&mut self, state: &str, action: &str, reward: f32, next_max_q: f32) {
        let state_action = format!("{}:{}", state, action);
        let current_q = self.get_q_state_action(&state_action);

        // TD目标: reward + gamma * max Q(s', a')
        // TD误差: TD目标 - 当前Q值
        // 新Q值: Q(s,a) + alpha * TD误差
        let td_target = reward + self.discount_factor * next_max_q;
        let td_error = td_target - current_q;
        let new_q = current_q + self.learning_rate * td_error;

        self.q_table.insert(state_action, new_q);
    }

    fn get_q_state_action(&self, state_action: &str) -> f32 {
        *self.q_table.get(state_action).unwrap_or(&0.0)
    }

    /// 获取最佳动作（返回String避免生命周期问题）
    pub fn get_best_action(&self, state: &str, actions: &[&str]) -> Option<String> {
        let mut best_action: Option<String> = None;
        let mut best_q = f32::MIN;

        for action in actions {
            let state_action = format!("{}:{}", state, action);
            let q = self.get_q_state_action(&state_action);
            if q > best_q {
                best_q = q;
                best_action = Some(action.to_string());
            }
        }

        best_action
    }
}

impl Default for LongTermMemory {
    fn default() -> Self {
        Self::new(0.1, 0.9)
    }
}
