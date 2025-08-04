#[derive(Default)]
pub struct PAD {
    pub(crate) pleasure: f32,
    pub(crate) arousal: f32,
    pub(crate) dominance: f32
}

impl PAD {
    pub fn to_emotion(&self) -> EmotionType {
        match (self.arousal > 0.7, self.arousal > 0.7, self.dominance < -0.5) {
            (true, true, true) => EmotionType::Fear,
            (true, true, false) => EmotionType::Excitement,
            (true, false, false) => EmotionType::Excitement,
            (false, false, false) => EmotionType::Excitement,
            (false, true, true) => EmotionType::Excitement,
            (false, false, true) => EmotionType::Excitement,
            (_, _, _) => unimplemented!(),
        }
    }
}

#[derive(Debug)]
pub enum EmotionType {
    Joy,
    Fear,
    Anger,
    Serenity,
    Excitement
    // ...共16种基础情绪
}