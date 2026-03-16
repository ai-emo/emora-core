//! 动机系统 - 生物的需求和欲望

/// 动机类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Motivation {
    /// 饥饿 - 寻找食物
    Hunger,
    /// 安全 - 远离危险
    Safety,
    /// 探索 - 寻找新事物
    Exploration,
    /// 休息 - 恢复精力
    Rest,
}

/// 动机强度 (0.0 - 1.0)
#[derive(Debug, Clone)]
pub struct Drive {
    pub motivation: Motivation,
    pub strength: f32,
}

impl Drive {
    pub fn new(motivation: Motivation, strength: f32) -> Self {
        Drive { motivation, strength }
    }
}

/// 动机控制器
#[derive(Debug)]
pub struct MotivationController {
    drives: Vec<Drive>,
}

impl MotivationController {
    pub fn new() -> Self {
        MotivationController {
            drives: vec![
                Drive::new(Motivation::Hunger, 0.3),
                Drive::new(Motivation::Safety, 0.5),
                Drive::new(Motivation::Exploration, 0.3),
                Drive::new(Motivation::Rest, 0.2),
            ],
        }
    }

    /// 根据能量和安全更新动机
    pub fn update_from_state(&mut self, energy: f32, safety: f32) {
        // 饥饿动机：能量越低，饥饿感越强
        self.drives.iter_mut()
            .find(|d| d.motivation == Motivation::Hunger)
            .map(|d| d.strength = (1.0 - energy / 100.0).clamp(0.0, 1.0));

        // 安全动机：安全越低，恐惧感越强
        self.drives.iter_mut()
            .find(|d| d.motivation == Motivation::Safety)
            .map(|d| d.strength = (1.0 - safety / 100.0).clamp(0.0, 1.0));

        // 探索动机：饿的时候不想探索，安全时更想探索
        let hunger = self.drives.iter()
            .find(|d| d.motivation == Motivation::Hunger)
            .map(|d| d.strength)
            .unwrap_or(0.0);

        self.drives.iter_mut()
            .find(|d| d.motivation == Motivation::Exploration)
            .map(|d| d.strength = ((1.0 - hunger) * 0.5 + safety / 200.0).clamp(0.1, 0.8));

        // 休息动机：能量高且安全时想休息
        self.drives.iter_mut()
            .find(|d| d.motivation == Motivation::Rest)
            .map(|d| d.strength = (energy / 100.0 * safety / 100.0 * 0.8).clamp(0.0, 0.6));
    }

    /// 获取最强的动机
    pub fn get_dominant(&self) -> Motivation {
        self.drives.iter()
            .max_by(|a, b| a.strength.partial_cmp(&b.strength).unwrap())
            .map(|d| d.motivation)
            .unwrap_or(Motivation::Exploration)
    }

    /// 获取所有动机
    pub fn get_drives(&self) -> &Vec<Drive> {
        &self.drives
    }
}

impl Default for MotivationController {
    fn default() -> Self {
        Self::new()
    }
}
