//! 传感器模块 - 感知外部和内部环境

use crate::Brain;

// ============ 传感器定义 ============

/// 传感器类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SensorType {
    /// 接近传感器 - 检测食物或危险
    Proximity,
    /// 内部传感器 - 感知自身状态
    Internal,
}

/// 传感器读数
#[derive(Debug, Clone)]
pub struct SensorData {
    /// 传感器类型
    pub sensor_type: SensorType,
    /// 感知到的值
    pub value: f32,
    /// 感知到的实体类型（食物、危险、无）
    pub entity: EntityType,
    /// 距离（如果是距离传感器）
    pub distance: f32,
}

/// 实体类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EntityType {
    /// 食物
    Food,
    /// 危险
    Danger,
    /// 什么也没有
    Nothing,
}

/// 传感器 trait - 所有传感器的通用接口
pub trait Sensor {
    /// 感知环境，返回传感器数据
    fn sense(&self, brain: &Brain, world: &World) -> SensorData;
}

/// 世界环境信息
#[derive(Debug, Clone, Default)]
pub struct World {
    /// 食物列表 (x, y, 能量值)
    pub foods: Vec<(f32, f32, f32)>,
    /// 危险列表 (x, y, 威胁值)
    pub dangers: Vec<(f32, f32, f32)>,
    /// 当前位置
    pub position: (f32, f32),
    /// 视野范围
    pub view_distance: f32,
}

impl World {
    pub fn new() -> Self {
        World {
            foods: Vec::new(),
            dangers: Vec::new(),
            position: (0.0, 0.0),
            view_distance: 100.0,
        }
    }

    /// 添加食物
    pub fn add_food(&mut self, x: f32, y: f32, energy: f32) {
        self.foods.push((x, y, energy));
    }

    /// 添加危险
    pub fn add_danger(&mut self, x: f32, y: f32, threat: f32) {
        self.dangers.push((x, y, threat));
    }
}

/// 接近传感器 - 检测前方物体
#[derive(Debug)]
pub struct ProximitySensor {
    /// 感知角度（度）
    pub angle: f32,
    /// 感知距离
    pub distance: f32,
}

impl ProximitySensor {
    pub fn new(angle: f32, distance: f32) -> Self {
        ProximitySensor { angle, distance }
    }

    /// 直接感知，不需要Brain引用
    pub fn sense_inner(&self, position: (f32, f32), world: &World) -> SensorData {
        // 寻找最近的食物
        let mut closest_food: Option<(f32, f32, f32)> = None;
        let mut closest_food_dist = f32::MAX;

        for (x, y, energy) in &world.foods {
            let dx = x - position.0;
            let dy = y - position.1;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < self.distance && dist < closest_food_dist {
                closest_food_dist = dist;
                closest_food = Some((*x, *y, *energy));
            }
        }

        // 寻找最近的危险
        let mut closest_danger: Option<(f32, f32, f32)> = None;
        let mut closest_danger_dist = f32::MAX;

        for (x, y, threat) in &world.dangers {
            let dx = x - position.0;
            let dy = y - position.1;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < self.distance && dist < closest_danger_dist {
                closest_danger_dist = dist;
                closest_danger = Some((*x, *y, *threat));
            }
        }

        // 返回最近的感知
        if closest_danger_dist < closest_food_dist {
            if let Some((_, _, threat)) = closest_danger {
                SensorData {
                    sensor_type: SensorType::Proximity,
                    value: threat,
                    entity: EntityType::Danger,
                    distance: closest_danger_dist,
                }
            } else {
                SensorData {
                    sensor_type: SensorType::Proximity,
                    value: 0.0,
                    entity: EntityType::Nothing,
                    distance: self.distance,
                }
            }
        } else if let Some((_, _, energy)) = closest_food {
            SensorData {
                sensor_type: SensorType::Proximity,
                value: energy,
                entity: EntityType::Food,
                distance: closest_food_dist,
            }
        } else {
            SensorData {
                sensor_type: SensorType::Proximity,
                value: 0.0,
                entity: EntityType::Nothing,
                distance: self.distance,
            }
        }
    }
}

impl Sensor for ProximitySensor {
    fn sense(&self, brain: &Brain, world: &World) -> SensorData {
        // 寻找最近的食物
        let mut closest_food: Option<(f32, f32, f32)> = None;
        let mut closest_food_dist = f32::MAX;

        for (x, y, energy) in &world.foods {
            let dx = x - world.position.0;
            let dy = y - world.position.1;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < self.distance && dist < closest_food_dist {
                closest_food_dist = dist;
                closest_food = Some((*x, *y, *energy));
            }
        }

        // 寻找最近的危险
        let mut closest_danger: Option<(f32, f32, f32)> = None;
        let mut closest_danger_dist = f32::MAX;

        for (x, y, threat) in &world.dangers {
            let dx = x - world.position.0;
            let dy = y - world.position.1;
            let dist = (dx * dx + dy * dy).sqrt();

            if dist < self.distance && dist < closest_danger_dist {
                closest_danger_dist = dist;
                closest_danger = Some((*x, *y, *threat));
            }
        }

        // 返回最近的感知
        if closest_danger_dist < closest_food_dist {
            if let Some((_, _, threat)) = closest_danger {
                SensorData {
                    sensor_type: SensorType::Proximity,
                    value: threat,
                    entity: EntityType::Danger,
                    distance: closest_danger_dist,
                }
            } else {
                SensorData {
                    sensor_type: SensorType::Proximity,
                    value: 0.0,
                    entity: EntityType::Nothing,
                    distance: self.distance,
                }
            }
        } else if let Some((_, _, energy)) = closest_food {
            SensorData {
                sensor_type: SensorType::Proximity,
                value: energy,
                entity: EntityType::Food,
                distance: closest_food_dist,
            }
        } else {
            SensorData {
                sensor_type: SensorType::Proximity,
                value: 0.0,
                entity: EntityType::Nothing,
                distance: self.distance,
            }
        }
    }
}

/// 内部传感器 - 感知自身状态
#[derive(Debug)]
pub struct InternalSensor;

impl InternalSensor {
    pub fn new() -> Self {
        InternalSensor
    }
}

impl Sensor for InternalSensor {
    fn sense(&self, brain: &Brain, _world: &World) -> SensorData {
        // 能量感知
        SensorData {
            sensor_type: SensorType::Internal,
            value: brain.energy,
            entity: EntityType::Nothing,
            distance: 0.0,
        }
    }
}

// ============ 感知定义 ============

/// 感知类型
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PerceptType {
    /// 感知到食物
    FoodDetected,
    /// 感知到危险
    DangerDetected,
    /// 什么都没感知到
    NothingDetected,
    /// 饥饿感
    Hungry,
    /// 安全
    Safe,
    /// 危险感
    Threatened,
}

/// 感知 - 处理后的传感器信息
#[derive(Debug, Clone)]
pub struct Percept {
    /// 感知类型
    pub ptype: PerceptType,
    /// 强度 0.0 - 1.0
    pub intensity: f32,
    /// 来源距离
    pub distance: f32,
}

impl Percept {
    /// 从传感器数据生成感知
    pub fn from_sensor_data(data: &SensorData, energy: f32, safety: f32) -> Vec<Percept> {
        let mut percepts = Vec::new();

        // 处理外部感知
        match data.entity {
            EntityType::Food => {
                // 距离越近，感知越强
                let intensity = (1.0 - data.distance / 100.0).clamp(0.0, 1.0);
                percepts.push(Percept {
                    ptype: PerceptType::FoodDetected,
                    intensity,
                    distance: data.distance,
                });
            }
            EntityType::Danger => {
                let intensity = (1.0 - data.distance / 100.0).clamp(0.0, 1.0);
                percepts.push(Percept {
                    ptype: PerceptType::DangerDetected,
                    intensity,
                    distance: data.distance,
                });
            }
            EntityType::Nothing => {
                percepts.push(Percept {
                    ptype: PerceptType::NothingDetected,
                    intensity: 0.0,
                    distance: data.distance,
                });
            }
        }

        // 处理内部状态感知 - 饥饿感
        // 能量低于70就会开始感到饿，想要找食物
        if energy < 70.0 {
            let intensity = (1.0 - energy / 70.0).clamp(0.0, 1.0);
            percepts.push(Percept {
                ptype: PerceptType::Hungry,
                intensity,
                distance: 0.0,
            });
        }

        if safety < 30.0 {
            let intensity = (1.0 - safety / 30.0).clamp(0.0, 1.0);
            percepts.push(Percept {
                ptype: PerceptType::Threatened,
                intensity,
                distance: 0.0,
            });
        }

        percepts
    }
}
