use std::collections::VecDeque;
use crate::behavior::{BehaviorController, ReflexController, ReflexArc, ActionExecutor};
use crate::models::{PADInertia, PAD, ProximitySensor, InternalSensor, World, Percept, SensorData, Sensor};

#[derive(Debug)]
pub struct Brain {
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
}

#[derive(Debug)]
pub enum Stimulus {
    Food(f32),
    Threat(f32),
    Comfort(f32),
}

impl Brain {
    pub fn new() -> Self {
        Brain {
            energy: 50.0,
            safety: 50.0,
            current_pad: PAD { pleasure: 0.0, arousal: 0.5, dominance: 0.0 },
            memory: VecDeque::with_capacity(100),
            sensitivity: PADInertia::default(),
            proximity_sensor: ProximitySensor::new(0.0, 100.0),
            internal_sensor: InternalSensor::new(),
            reflex_controller: ReflexController::new(),
            world: World::new(),
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

    /// 感知阶段：传感器获取信息
    fn sense(&self) -> Vec<SensorData> {
        let mut data = Vec::new();

        // 外部传感器感知
        let proximity_data = self.proximity_sensor.sense(self, &self.world);
        data.push(proximity_data);

        // 内部传感器感知
        let internal_data = self.internal_sensor.sense(self, &self.world);
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

    /// 反射弧阶段：根据感知激发反射
    fn reflex(&self, percepts: &[Percept]) -> Option<ReflexArc> {
        self.reflex_controller.process(percepts)
    }

    /// 动作阶段：执行反射动作
    fn act(&mut self, reflex: &ReflexArc) {
        let result = ActionExecutor::execute(&reflex.reflex_type);

        // 应用能量消耗
        self.energy = (self.energy - result.energy_cost).clamp(0.0, 100.0);

        // 应用产生的刺激
        if let Some(stimulus) = result.stimulus {
            self.perceive(stimulus);
        }

        println!("[反射弧] {}", result.description);
    }

    /// 使用反射弧的tick
    pub fn tick_reflex(&mut self) {
        println!("\n=== Tick 开始 ===");

        // 1. 自然能量消耗
        self.energy = (self.energy - 1.0).clamp(0.0, 100.0);

        // 2. 感知阶段
        let sensor_data = self.sense();

        // 3. 感知处理阶段
        let percepts = self.perceive_signals(&sensor_data);

        println!("感知: {:?}", percepts);

        // 4. 反射弧阶段
        if let Some(reflex) = self.reflex(&percepts) {
            println!("激发反射: {:?}", reflex.reflex_type);
            self.act(&reflex);
        } else {
            println!("无反射激发，使用默认行为");
            // 没有反射时使用原有行为系统
            self.update_emotion();
            let behavior = BehaviorController::decide(self);
            BehaviorController::act(self, &behavior);
        }

        // 5. 更新情绪
        self.update_emotion();

        println!(
            "状态: 能量={:.1}, 安全={:.1}, 情绪={:?}",
            self.energy, self.safety, self.current_pad
        );
        println!("=== Tick 结束 ===\n");
    }

    pub fn tick(&mut self) {
        self.energy = (self.energy - 1.0).clamp(0.0, 100.0);
        self.update_emotion();
        let behavior = BehaviorController::decide(self);
        BehaviorController::act(self, &behavior);
        println!(
            "状态: 能量={:.1}, 安全={:.1}, 情绪={:?}, 行为={:?}",
            self.energy, self.safety, self.current_pad, behavior
        );
    }
}