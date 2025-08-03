pub struct PAD {
    pub(crate) pleasure: f32,
    pub(crate) arousal: f32,
    pub(crate) dominance: f32
}

impl PAD {
    pub fn to_emotion(&self) -> EmotionType {
        match (self.arousal > 0.7, self.dominance < -0.5) {
            (true, true) => EmotionType::Fear,
            (true, false) => EmotionType::Excitement,
            (false, _) => todo!(),
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