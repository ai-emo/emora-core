use std::collections::VecDeque;
use crate::behavior::{BehaviorController};
use crate::models::PAD;

#[derive(Debug)]
pub struct Brain {
    pub energy: f32,
    safety: f32,
    pub current_pad: PAD,
    memory: VecDeque<PAD>,
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
        }
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