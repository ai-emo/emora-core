mod easy_creatures;
mod reflex;

pub use easy_creatures::{BehaviorController, Behavior};
pub use reflex::{ReflexArc, ReflexController, ReflexType, reflex_to_stimulus, Action, ActionResult, ActionExecutor};
