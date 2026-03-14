//! 学习系统 - TD学习实现

use crate::models::memory::{ShortTermMemory, LongTermMemory, MemoryItem};

/// TD学习器 - 实现时间差学习
#[derive(Debug)]
pub struct TDLearner {
    short_term: ShortTermMemory,
    long_term: LongTermMemory,
}

impl TDLearner {
    pub fn new(memory_capacity: usize, learning_rate: f32, discount_factor: f32) -> Self {
        TDLearner {
            short_term: ShortTermMemory::new(memory_capacity),
            long_term: LongTermMemory::new(learning_rate, discount_factor),
        }
    }

    /// 记录一次经验
    pub fn record(&mut self, perception: String, reflex: String, result: String, reward: f32) {
        let item = MemoryItem::new(
            perception,
            reflex,
            result,
            reward,
            self.short_term.get_timestep(),
        );
        self.short_term.remember(item);
    }

    /// 学习 - 从记忆中更新Q值
    pub fn learn(&mut self) {
        let memories = self.short_term.recent(2);

        if memories.len() < 2 {
            return;
        }

        // 获取当前经验和下一经验
        let current = &memories[memories.len() - 2];
        let next = &memories[memories.len() - 1];

        // 计算奖励（能量变化）
        let reward = next.reward;

        // TD学习更新
        // 下一状态的最大Q值（简化版本，使用固定的候选动作）
        let actions = vec!["Approach", "Flee", "SeekFood", "Rest"];
        let next_max_q = actions
            .iter()
            .map(|a| self.long_term.get_q(&format!("{}:{}", next.perception, a)))
            .fold(f32::MIN, f32::max);

        self.long_term.update_q(&current.perception, &current.reflex, reward, next_max_q);
    }

    /// 决定最佳动作（使用学习到的知识）
    pub fn decide(&self, perception: &str, available_actions: &[&str]) -> Option<String> {
        self.long_term.get_best_action(perception, available_actions)
    }

    /// 获取短期记忆
    pub fn get_short_term(&self) -> &ShortTermMemory {
        &self.short_term
    }

    /// 获取长期记忆
    pub fn get_long_term(&self) -> &LongTermMemory {
        &self.long_term
    }

    /// 时间步前进
    pub fn tick(&mut self) {
        self.short_term.tick();
    }
}

impl Default for TDLearner {
    fn default() -> Self {
        Self::new(100, 0.1, 0.9)
    }
}
