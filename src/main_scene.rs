use std::f64::consts::PI;

use crate::{
    hud::Hud,
    sprites::{enemy::Enemy, player::Player},
};
use gdnative::{api::{AudioStreamPlayer2D, PathFollow2D, Position2D, RigidBody2D}, prelude::*};
use rand::Rng;

#[derive(NativeClass)]
#[inherit(Node2D)]
pub struct Main {
    #[property]
    mob: Ref<PackedScene>,
    score: i64,
}

#[methods]
impl Main {
    fn new(_owner: &Node2D) -> Self {
        Self {
            mob: PackedScene::new().into_shared(),
            score: 0,
        }
    }

    #[export]
    fn new_game(&mut self, _owner: &Node2D) {
        // music
        let audio_music = unsafe { _owner.get_node_as::<AudioStreamPlayer2D>("Music").unwrap() };
        audio_music.play(0.0);

        // game
        let player = unsafe { _owner.get_node_as_instance::<Player>("Player").unwrap() };
        let hud = unsafe { _owner.get_node_as_instance::<Hud>("HUD").unwrap() };

        let start_position = unsafe { _owner.get_node_as::<Position2D>("StartPosition").unwrap() };
        let start_timer = unsafe { _owner.get_node_as::<Timer>("StartTimer").unwrap() };

        self.score = 0;
        start_timer.start(0.0);

        player
            .map(|item, _player| item.start(&*_player, start_position.position()))
            .ok()
            .unwrap_or_else(|| {});

        hud.map(|item, _hud| {
            item.update_score(&*_hud, self.score.to_string());
            item.show_message(&*_hud, "Get Ready".into());
        })
        .ok()
        .unwrap_or_else(|| {});
    }

    #[export]
    fn game_over(&self, _owner: &Node2D) {
        // music
        let audio_music = unsafe { _owner.get_node_as::<AudioStreamPlayer2D>("Music").unwrap() };
        let audio_dead = unsafe { _owner.get_node_as::<AudioStreamPlayer2D>("DeadSound").unwrap() };
        audio_music.stop();
        audio_dead.play(0.0);

        // clear scene
        let scene_tree = _owner.get_tree().unwrap();
        unsafe {
            scene_tree.assume_safe().call_group("enemy", "queue_free", &[]);
        }

        // game
        let hud = unsafe { _owner.get_node_as_instance::<Hud>("HUD").unwrap() };

        let mob_timer = unsafe { _owner.get_node_as::<Timer>("MobTimer").unwrap() };
        let score_timer = unsafe { _owner.get_node_as::<Timer>("ScoreTimer").unwrap() };

        mob_timer.stop();
        score_timer.stop();

        hud.map(|item, _hud| item.show_game_over(&*_hud))
            .ok()
            .unwrap_or_else(|| {});
    }

    #[export]
    fn _on_start_timer_timeout(&self, _owner: &Node2D) {
        let mob_timer = unsafe { _owner.get_node_as::<Timer>("MobTimer").unwrap() };
        let score_timer = unsafe { _owner.get_node_as::<Timer>("ScoreTimer").unwrap() };

        mob_timer.start(0.0);
        score_timer.start(0.0);
    }

    #[export]
    fn _on_score_timer_timeout(&mut self, _owner: &Node2D) {
        let hud = unsafe { _owner.get_node_as_instance::<Hud>("HUD").unwrap() };

        self.score += 1;

        hud.map(|item, _hud| item.update_score(&*_hud, self.score.to_string()))
            .ok()
            .unwrap_or_else(|| {});
    }

    #[export]
    fn _on_mob_timer_timeout(&mut self, _owner: &Node2D) {
        let mob_spawn_location = unsafe {
            _owner
                .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation")
                .unwrap()
        };
        let mob_entity: Ref<RigidBody2D, _> = instance_scene(&self.mob);

        let mut rng = rand::thread_rng();
        let offset = rng.gen_range(std::u32::MIN..std::u32::MAX);

        mob_spawn_location.set_offset(offset.into());

        let mut direction = mob_spawn_location.rotation() + PI / 2.0;

        direction += rng.gen_range(-PI / 4.0..PI / 4.0);
        
        mob_entity.set_position(mob_spawn_location.position());
        mob_entity.set_rotation(direction);

        let mob_entity = unsafe { mob_entity.into_shared().assume_safe() };
        _owner.add_child(mob_entity, false);

        let mob = mob_entity.cast_instance::<Enemy>().unwrap();

        mob.map(|item, _mob| {
            _mob.set_linear_velocity(Vector2::new(
                rng.gen_range(item.min_speed..item.max_speed),
                0.0,
            ));
            _mob.set_linear_velocity(_mob.linear_velocity().rotated(Angle {
                radians: direction as f32,
            }));

            let hud = unsafe { _owner.get_node_as_instance::<Hud>("HUD").unwrap() };

            hud.map(|_, _hud| {
                _hud.connect(
                    "start",
                    _mob,
                    "on_start_game",
                    VariantArray::new_shared(),
                    0,
                )
                .unwrap();
            })
            .unwrap();
        })
        .unwrap();
    }
}

fn instance_scene<Root>(scene: &Ref<PackedScene, Shared>) -> Ref<Root, Unique>
where
    Root: gdnative::GodotObject<RefKind = ManuallyManaged> + SubClass<Node>,
{
    let scene = unsafe { scene.assume_safe() };

    let instance = scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .expect("should be able to instance scene");

    let instance = unsafe { instance.assume_unique() };

    instance
        .try_cast::<Root>()
        .expect("root node type should be correct")
}
