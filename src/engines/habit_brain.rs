//! 习惯脑 - 支持短期记忆和TD学习的脑

use std::collections::VecDeque;
use crate::behavior::{BehaviorController, ReflexController, ReflexArc, ActionExecutor};
use crate::models::{
    PADInertia, PAD, ProximitySensor, InternalSensor, World, Percept, SensorData, Sensor,
    TDLearner
};
use crate::engines::Stimulus;

#[derive(Debug)]
pub struct HabitBrain {
    pub energy: f32,
    safety: f32,
    pub current_pad: PAD,
    sensitivity: PADInertia,
    memory: VecDeque<PAD>,
    // 反射弧系统组件
    proximity_sensor: ProximitySensor,
    internal_sensor: InternalSensor,
    reflex_controller: ReflexController,
    world: World,
    // 学习系统
    learner: TDLearner,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ActionMode {
    /// 反射模式 - 先天反射
    Reflex,
    /// 学习模式 - 使用学到的知识
    Learned,
    /// 混合模式 - 两者结合
    Hybrid,
}

impl HabitBrain {
    pub fn new() -> Self {
        HabitBrain {
            energy: 50.0,
            safety: 50.0,
            current_pad: PAD { pleasure: 0.0, arousal: 0.5, dominance: 0.0 },
            memory: VecDeque::with_capacity(100),
            sensitivity: PADInertia::default(),
            proximity_sensor: ProximitySensor::new(0.0, 100.0),
            internal_sensor: InternalSensor::new(),
            reflex_controller: ReflexController::new(),
            world: World::new(),
            learner: TDLearner::default(),
        }
    }

    /// 设置世界环境
    pub fn set_world(&mut self, world: World) {
        self.world = world;
    }

    /// 接受刺激，更新能量和安全
    pub fn perceive(&mut self, stimulus: Stimulus) {
        match stimulus {
            Stimulus::Food(v) => self.energy = (self.energy + v).clamp(0.0, 100.0),
            Stimulus::Threat(v) => self.safety = (self.safety - v).clamp(0.0, 100.0),
            Stimulus::Comfort(v) => self.safety = (self.safety + v).clamp(0.0, 100.0),
        }
        self.update_emotion();
    }

    fn update_emotion(&mut self) {
        self.current_pad.pleasure = (self.energy / 100.0) * 2.0 - 1.0;
        self.current_pad.dominance = (self.safety / 100.0) * 2.0 - 1.0;

        let hunger = 1.0 - (self.energy / 100.0);
        let fear = 1.0 - (self.safety / 100.0);
        self.current_pad.arousal = ((hunger + fear) / 2.0).clamp(0.0, 1.0);
    }

    /// 获取当前感知描述
    fn get_perception_string(&self, percepts: &[Percept]) -> String {
        let mut parts = Vec::new();

        for p in percepts {
            parts.push(format!("{:?}:{:.2}", p.ptype, p.intensity));
        }

        if parts.is_empty() {
            "Nothing".to_string()
        } else {
            parts.join("+")
        }
    }

    /// 感知阶段：传感器获取信息
    fn sense(&self) -> Vec<SensorData> {
        let mut data = Vec::new();

        // 获取当前位置用于距离计算
        let position = self.world.position;

        // 接近传感器
        let proximity_data = self.proximity_sensor.sense_inner(position, &self.world);
        data.push(proximity_data);

        // 内部传感器
        let internal_data = SensorData {
            sensor_type: crate::models::SensorType::Internal,
            value: self.energy,
            entity: crate::models::EntityType::Nothing,
            distance: 0.0,
        };
        data.push(internal_data);

        data
    }

    /// 感知处理阶段：将传感器数据转换为感知
    fn perceive_signals(&self, sensor_data: &[SensorData]) -> Vec<Percept> {
        let mut percepts = Vec::new();
        for data in sensor_data {
            let mut p = Percept::from_sensor_data(data, self.energy, self.safety);
            percepts.append(&mut p);
        }
        percepts
    }

    /// 决定使用反射还是学习
    fn decide_mode(&self, _percepts: &[Percept]) -> ActionMode {
        let memory_count = self.learner.get_short_term().all().len();
        if memory_count > 10 {
            ActionMode::Hybrid
        } else {
            ActionMode::Reflex
        }
    }

    /// 反射弧阶段：根据感知激发反射
    fn reflex(&self, percepts: &[Percept]) -> Option<ReflexArc> {
        self.reflex_controller.process(percepts)
    }

    /// 动作阶段：执行反射动作
    fn act(&mut self, reflex: &ReflexArc) -> (String, f32) {
        let result = ActionExecutor::execute(&reflex.reflex_type);

        // 应用能量消耗
        let energy_before = self.energy;
        self.energy = (self.energy - result.energy_cost).clamp(0.0, 100.0);
        let energy_delta = self.energy - energy_before;

        // 应用产生的刺激
        if let Some(stimulus) = result.stimulus {
            self.perceive(stimulus);
        }

        (result.description, energy_delta)
    }

    /// 习惯生物的tick
    pub fn tick_habit(&mut self) {
        println!("\n=== 习惯脑 Tick 开始 ===");

        // 1. 自然能量消耗
        let energy_before = self.energy;
        self.energy = (self.energy - 1.0).clamp(0.0, 100.0);

        // 2. 感知阶段
        let sensor_data = self.sense();

        // 3. 感知处理阶段
        let percepts = self.perceive_signals(&sensor_data);
        let perception_str = self.get_perception_string(&percepts);

        println!("感知: {:?}", percepts);

        // 4. 决定模式
        let mode = self.decide_mode(&percepts);
        println!("决策模式: {:?}", mode);

        // 5. 动作选择
        let (reflex, action_result, reward) = match mode {
            ActionMode::Reflex | ActionMode::Hybrid => {
                if let Some(r) = self.reflex(&percepts) {
                    let (desc, delta) = self.act(&r);
                    (Some(r), desc, delta)
                } else {
                    self.update_emotion();
                    let behavior = BehaviorController::decide_habit(self);
                    BehaviorController::act_habit(self, &behavior);
                    (None, format!("{:?}", behavior), 0.0)
                }
            }
            ActionMode::Learned => {
                self.update_emotion();
                let behavior = BehaviorController::decide_habit(self);
                BehaviorController::act_habit(self, &behavior);
                (None, format!("{:?}", behavior), 0.0)
            }
        };

        // 6. 记录经验并学习
        let reflex_name = reflex.map(|r| format!("{:?}", r.reflex_type)).unwrap_or_else(|| "None".to_string());
        self.learner.record(perception_str, reflex_name, action_result, reward);

        // 学习
        self.learner.learn();

        // 7. 更新情绪
        self.update_emotion();

        // 8. 时间步前进
        self.learner.tick();

        println!("状态: 能量={:.1}, 安全={:.1}, 情绪={:?}", self.energy, self.safety, self.current_pad);

        let memories = self.learner.get_short_term().all();
        println!("记忆条目数: {}", memories.len());

        println!("=== Tick 结束 ===\n");
    }

    pub fn get_learner(&self) -> &TDLearner {
        &self.learner
    }
}
