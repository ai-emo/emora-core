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
    // 位置
    pub position: (f32, f32),
    // 移动速度
    pub move_speed: f32,
    // 反射弧系统组件
    proximity_sensor: ProximitySensor,
    internal_sensor: InternalSensor,
    reflex_controller: ReflexController,
    pub world: World,
    // 当前激发的反射
    pub current_reflex: Option<String>,
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
            energy: 70.0,
            safety: 50.0,
            current_pad: PAD { pleasure: 0.0, arousal: 0.5, dominance: 0.0 },
            memory: VecDeque::with_capacity(100),
            sensitivity: PADInertia::default(),
            position: (50.0, 150.0),
            move_speed: 15.0,
            proximity_sensor: ProximitySensor::new(0.0, 100.0),
            internal_sensor: InternalSensor::new(),
            reflex_controller: ReflexController::new(),
            world: World::new(),
            current_reflex: None,
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

        // 记录当前激发的反射
        self.current_reflex = Some(format!("{:?}", reflex.reflex_type));

        // 根据反射类型移动
        self.move_towards_target(reflex.reflex_type);

        // 应用能量消耗
        self.energy = (self.energy - result.energy_cost).clamp(0.0, 100.0);

        // 应用产生的刺激
        if let Some(stimulus) = result.stimulus {
            self.perceive(stimulus);
        }

        println!("[反射弧] {}", result.description);
    }

    /// 根据反射移动
    fn move_towards_target(&mut self, reflex_type: crate::behavior::ReflexType) {
        let (tx, ty) = self.position;
        let mut new_x = tx;
        let mut new_y = ty;

        match reflex_type {
            crate::behavior::ReflexType::Approach => {
                // 寻找最近的食物并靠近
                if let Some((fx, fy, energy)) = self.find_nearest_food() {
                    let dx = fx - tx;
                    let dy = fy - ty;
                    let dist = (dx * dx + dy * dy).sqrt();

                    if dist < 10.0 {
                        // 吃到食物！
                        println!(">>> 吃到食物！能量+{:.0}", energy);
                        self.perceive(Stimulus::Food(energy));
                        // 移除被吃掉的食物
                        self.world.foods.retain(|(x, y, _e)| {
                            (*x - fx).abs() >= 1.0 || (*y - fy).abs() >= 1.0
                        });
                    } else {
                        new_x = tx + (dx / dist) * self.move_speed;
                        new_y = ty + (dy / dist) * self.move_speed;
                    }
                }
            }
            crate::behavior::ReflexType::Flee => {
                // 远离最近的危险
                if let Some((dx, dy, _)) = self.find_nearest_danger() {
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist > 0.0 {
                        new_x = tx - (dx / dist) * self.move_speed * 1.5;
                        new_y = ty - (dy / dist) * self.move_speed * 1.5;
                    }
                    // 靠近危险会受到伤害
                    if dist < 15.0 {
                        println!(">>> 受到危险伤害！安全-20");
                        self.perceive(Stimulus::Threat(20.0));
                    }
                }
            }
            crate::behavior::ReflexType::SeekFood => {
                // 主动寻找最近的食物
                if let Some((fx, fy, _)) = self.find_nearest_food() {
                    let dx = fx - tx;
                    let dy = fy - ty;
                    let dist = (dx * dx + dy * dy).sqrt();
                    new_x = tx + (dx / dist) * self.move_speed;
                    new_y = ty + (dy / dist) * self.move_speed;
                } else {
                    // 没有食物时，随机探索
                    use rand::Rng;
                    let mut rng = rand::rng();
                    new_x = tx + rng.random_range(-self.move_speed..self.move_speed);
                    new_y = ty + rng.random_range(-self.move_speed..self.move_speed);
                }

                // 探索时也可能遇到危险
                if let Some((dx, dy, _)) = self.find_nearest_danger() {
                    let dist = (dx * dx + dy * dy).sqrt();
                    if dist < 15.0 {
                        println!(">>> 探索遇到危险！安全-10");
                        self.perceive(Stimulus::Threat(10.0));
                    }
                }
            }
            crate::behavior::ReflexType::Rest => {
                // 休息不移动，但恢复一点安全感
                println!(">>> 休息中...");
            }
            crate::behavior::ReflexType::Explore => {
                use rand::Rng;
                let mut rng = rand::rng();
                new_x = tx + rng.random_range(-self.move_speed..self.move_speed);
                new_y = ty + rng.random_range(-self.move_speed..self.move_speed);
            }
        }

        // 边界限制
        new_x = new_x.clamp(10.0, 390.0);
        new_y = new_y.clamp(10.0, 390.0);

        self.position = (new_x, new_y);
        self.world.position = (new_x, new_y);
    }

    /// 寻找最近的食物
    fn find_nearest_food(&self) -> Option<(f32, f32, f32)> {
        let mut closest: Option<(f32, f32, f32)> = None;
        let mut closest_dist = f32::MAX;

        for (x, y, e) in &self.world.foods {
            let dx = x - self.position.0;
            let dy = y - self.position.1;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < closest_dist {
                closest_dist = dist;
                closest = Some((*x, *y, *e));
            }
        }
        closest
    }

    /// 寻找最近的危险
    fn find_nearest_danger(&self) -> Option<(f32, f32, f32)> {
        let mut closest: Option<(f32, f32, f32)> = None;
        let mut closest_dist = f32::MAX;

        for (x, y, t) in &self.world.dangers {
            let dx = x - self.position.0;
            let dy = y - self.position.1;
            let dist = (dx * dx + dy * dy).sqrt();
            if dist < closest_dist {
                closest_dist = dist;
                closest = Some((*x, *y, *t));
            }
        }
        closest
    }

    /// 使用反射弧的tick
    pub fn tick_reflex(&mut self) {
        println!("\n=== Tick 开始 ===");

        // 1. 自然能量消耗
        self.energy = (self.energy - 0.3).clamp(0.0, 100.0);

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
        self.energy = (self.energy - 0.3).clamp(0.0, 100.0);
        self.update_emotion();
        let behavior = BehaviorController::decide(self);
        BehaviorController::act(self, &behavior);
        println!(
            "状态: 能量={:.1}, 安全={:.1}, 情绪={:?}, 行为={:?}",
            self.energy, self.safety, self.current_pad, behavior
        );
    }
}