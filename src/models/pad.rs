#[derive(Debug)]
pub struct PAD {
    pub pleasure: f32,
    pub arousal: f32,
    pub dominance: f32
}

#[derive(Debug)]
pub struct PADInertia {
    pleasure: f32,
    arousal: f32,
    dominance: f32,
}

impl Default for PADInertia {
    fn default() -> Self {
        PADInertia {
            pleasure: 0.5,
            arousal: 0.5,
            dominance: 0.5,
        }
    }
}