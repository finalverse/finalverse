use finalverse_protocol::{BehaviorAction, ReasoningContext};

#[derive(Clone, Default)]
pub struct Planner;

impl Planner {
    pub fn plan(&self, ctx: &ReasoningContext) -> BehaviorAction {
        if ctx.tension > 0.7 {
            BehaviorAction::Flee("danger".into())
        } else if ctx.harmony_level < 0.3 {
            BehaviorAction::Wander
        } else {
            BehaviorAction::Rest
        }
    }
}
