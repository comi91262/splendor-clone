
pub struct ActionReward {
    pub action: GameCommand,
    pub reward: f32,
}

impl ActionReward {
    fn new(action: GameCommand, reward: f32) -> ActionReward {
        ActionReward {
            action: action,
            reward: reward,
        }
    }
}

impl fmt::Debug for ActionReward {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Action: {} 報酬点: {}", self.action, self.reward)
    }
}