use rand::Rng;
use crate::{Brain, Stimulus};

#[derive(Debug)]
pub enum Behavior {
    SeekFood,
    Explore,
    Rest,
    Idle,
}

pub struct BehaviorController;

impl BehaviorController {
    pub(crate) fn decide(emo: &mut Brain) -> Behavior {
        if emo.current_pad.pleasure < -0.3 {
            Behavior::SeekFood
        } else if emo.current_pad.arousal > 0.7 {
            Behavior::Explore
        } else if emo.current_pad.pleasure > 0.5 && emo.current_pad.arousal < 0.3 {
            Behavior::Rest
        } else {
            Behavior::Idle
        }
    }

    pub(crate) fn act(emo: &mut Brain, behavior: &Behavior) {
        match behavior {
            Behavior::SeekFood => {
                println!("行动: 寻找食物");
                let mut rng = rand::rng();
                if rng.random_bool(0.3) {
                    println!("找到食物，能量+4");
                    // 模拟找到少量食物
                    emo.perceive(Stimulus::Food(5.0));
                } else {
                    println!("没找到食物，什么也没得到");
                }
            }
            Behavior::Explore => {
                println!("行动: 探索环境");
                // 探索可能带来威胁
                emo.perceive(Stimulus::Threat(2.0));
            }
            Behavior::Rest => {
                println!("行动: 休息恢复");
                // 休息稍微提升安全感
                emo.perceive(Stimulus::Comfort(1.0));
            }
            Behavior::Idle => {
                println!("行动: 无事可做，等待...");
            }
        }
    }
}