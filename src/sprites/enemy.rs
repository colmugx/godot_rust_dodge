use gdnative::{
    api::{AnimatedSprite, RigidBody2D},
    prelude::*,
};
use rand::seq::SliceRandom;

#[derive(Copy, Clone)]
enum MobType {
    Walk,
    Swim,
    Fly,
}

impl MobType {
    fn to_str(self) -> String {
        match self {
            MobType::Walk => "walk".to_string(),
            MobType::Swim => "swim".to_string(),
            MobType::Fly => "fly".to_string(),
        }
    }
}

const MOB_TYPES: [MobType; 3] = [MobType::Walk, MobType::Swim, MobType::Fly];

#[derive(NativeClass)]
#[inherit(RigidBody2D)]
pub struct Enemy {
    #[property(default = 150.0)]
    pub min_speed: f32,
    #[property(default = 250.0)]
    pub max_speed: f32,
}

#[methods]
impl Enemy {
    pub fn new(_owner: &RigidBody2D) -> Self {
        Self {
            min_speed: 150.0,
            max_speed: 250.0,
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &RigidBody2D) {
        let mut rng = rand::thread_rng();
        let ani_sprite = unsafe {
            _owner
                .get_node_as::<AnimatedSprite>("AnimatedSprite")
                .unwrap()
        };

        ani_sprite.set_animation(MOB_TYPES.choose(&mut rng).unwrap().to_str());
    }

    #[export]
    fn on_viewport_exited(&self, _owner: &RigidBody2D) {
        _owner.queue_free();
    }

    #[export]
    fn on_start_game(&self, _owner: &RigidBody2D) {
        _owner.queue_free();
    }
}
