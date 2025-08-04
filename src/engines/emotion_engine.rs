use std::collections::VecDeque;
use crate::models::PAD;

#[derive(Default)]
pub struct EmotionEngine {
    pub current_pad: PAD,
    memory: VecDeque<PAD>,
}

impl EmotionEngine {
    pub fn new() -> Self {
        EmotionEngine {
            current_pad: PAD { pleasure: 0.2, arousal: 0.4, dominance: -0.1 },
            memory: VecDeque::with_capacity(100),
        }
    }
    
    pub fn update(&mut self) {
        self.current_pad.pleasure += 1.0;
    }
}