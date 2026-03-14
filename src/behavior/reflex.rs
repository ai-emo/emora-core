//! 反射弧系统 - 感知到动作的直接映射

use crate::models::{Percept, PerceptType};
use crate::Stimulus;

/// 动作类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Action {
    /// 移动向目标
    MoveTowards,
    /// 逃离
    Flee,
    /// 探索移动
    Explore,
    /// 等待
    Wait,
    /// 休息
    Rest,
    /// 进食
    Eat,
}

/// 动作结果
#[derive(Debug)]
pub struct ActionResult {
    /// 产生的刺激
    pub stimulus: Option<Stimulus>,
    /// 消耗的能量
    pub energy_cost: f32,
    /// 动作描述
    pub description: String,
}

/// 动作执行器
pub struct ActionExecutor;

impl ActionExecutor {
    /// 执行反射弧产生的动作
    pub fn execute(reflex_type: &ReflexType) -> ActionResult {
        match reflex_type {
            ReflexType::Flee => ActionResult {
                stimulus: Some(Stimulus::Comfort(2.0)),
                energy_cost: 3.0,
                description: "逃跑：快速离开危险区域".to_string(),
            },
            ReflexType::SeekFood | ReflexType::Approach => ActionResult {
                stimulus: Some(Stimulus::Food(1.0)),
                energy_cost: 2.0,
                description: "移动：向目标位置移动".to_string(),
            },
            ReflexType::Rest => ActionResult {
                stimulus: Some(Stimulus::Comfort(3.0)),
                energy_cost: -1.0, // 休息恢复能量
                description: "休息：放松恢复".to_string(),
            },
            ReflexType::Explore => ActionResult {
                stimulus: Some(Stimulus::Threat(1.0)),
                energy_cost: 2.0,
                description: "探索：搜索周围环境".to_string(),
            },
        }
    }
}

/// 反射弧类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReflexType {
    /// 趋近反射 - 靠近食物
    Approach,
    /// 逃离反射 - 远离危险
    Flee,
    /// 探索反射 - 四处探索
    Explore,
    /// 觅食反射 - 寻找食物
    SeekFood,
    /// 休息反射 - 感到安全时休息
    Rest,
}

/// 反射弧 - 刺激到动作的映射
#[derive(Debug, Clone)]
pub struct ReflexArc {
    /// 触发这种反射的感知类型
    pub trigger: PerceptType,
    /// 触发阈值
    pub threshold: f32,
    /// 反射弧类型
    pub reflex_type: ReflexType,
    /// 优先级（数值越大优先级越高）
    pub priority: i32,
}

impl ReflexArc {
    pub fn new(trigger: PerceptType, threshold: f32, reflex_type: ReflexType, priority: i32) -> Self {
        ReflexArc {
            trigger,
            threshold,
            reflex_type,
            priority,
        }
    }

    /// 检查这个反射是否应该被触发
    pub fn is_triggered(&self, percepts: &[Percept]) -> bool {
        for percept in percepts {
            if percept.ptype == self.trigger && percept.intensity >= self.threshold {
                return true;
            }
        }
        false
    }
}

/// 默认的无条件反射弧
pub fn get_default_reflexes() -> Vec<ReflexArc> {
    vec![
        // 危险感知 → 逃离
        ReflexArc::new(PerceptType::DangerDetected, 0.1, ReflexType::Flee, 100),
        // 感知到食物 → 趋近
        ReflexArc::new(PerceptType::FoodDetected, 0.1, ReflexType::Approach, 80),
        // 饥饿 → 觅食
        ReflexArc::new(PerceptType::Hungry, 0.3, ReflexType::SeekFood, 70),
        // 感到安全且不饿 → 休息
        ReflexArc::new(PerceptType::Safe, 0.5, ReflexType::Rest, 10),
    ]
}

/// 反射弧控制器
#[derive(Debug)]
pub struct ReflexController {
    reflexes: Vec<ReflexArc>,
}

impl ReflexController {
    pub fn new() -> Self {
        ReflexController {
            reflexes: get_default_reflexes(),
        }
    }

    /// 添加自定义反射弧
    pub fn add_reflex(&mut self, reflex: ReflexArc) {
        self.reflexes.push(reflex);
        // 按优先级排序
        self.reflexes.sort_by(|a, b| b.priority.cmp(&a.priority));
    }

    /// 处理感知，返回激发的反射弧
    pub fn process(&self, percepts: &[Percept]) -> Option<ReflexArc> {
        for reflex in &self.reflexes {
            if reflex.is_triggered(percepts) {
                return Some(reflex.clone());
            }
        }
        None
    }
}

impl Default for ReflexController {
    fn default() -> Self {
        Self::new()
    }
}

/// 将反射弧转换为动作产生的刺激
pub fn reflex_to_stimulus(reflex: &ReflexType) -> Option<Stimulus> {
    match reflex {
        ReflexType::Flee => {
            // 逃离会产生一点舒适感（远离危险）
            Some(Stimulus::Comfort(2.0))
        }
        ReflexType::SeekFood | ReflexType::Approach => {
            // 寻找食物可能带来食物
            Some(Stimulus::Food(1.0))
        }
        ReflexType::Rest => {
            // 休息产生舒适感
            Some(Stimulus::Comfort(3.0))
        }
        ReflexType::Explore => {
            // 探索可能遇到危险
            Some(Stimulus::Threat(1.0))
        }
    }
}
