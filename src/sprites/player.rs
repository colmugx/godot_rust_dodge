use gdnative::{api::{AnimatedSprite, Area2D, CollisionShape2D, Particles2D, PhysicsBody2D}, prelude::*};

#[derive(NativeClass)]
#[inherit(Area2D)]
#[register_with(Self::register_signals)]
pub struct Player {
    screen_size: Size2,
    #[property(default = 400.0)]
    speed: f32,
}

impl Player {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "hit",
            args: &[],
        });
    }
}

#[methods]
impl Player {
    fn new(_owner: &Area2D) -> Self {
        Self {
            speed: 400.0,
            screen_size: Size2::new(0.0, 0.0),
        }
    }

    #[export]
    fn _ready(&mut self, _owner: &Area2D) {
        self.screen_size = _owner.get_viewport_rect().size;
        _owner.hide();
    }

    #[export]
    fn _physics_process(&mut self, _owner: &Area2D, delta: f32) {
        let input = Input::godot_singleton();
        let mut vel = Vector2::new(0.0, 0.0);

        let ani_sprite = unsafe {
            _owner
                .get_node_as::<AnimatedSprite>("AnimatedSprite")
                .unwrap()
        };
        let particle = unsafe {
            _owner
                .get_node_as::<Particles2D>("Particles")
                .unwrap()
        };

        let mut ani = "";

        if Input::is_action_pressed(&input, "ui_right") {
            vel.x += 1.0;
        }

        if Input::is_action_pressed(&input, "ui_left") {
            vel.x -= 1.0;
        }

        if Input::is_action_pressed(&input, "ui_up") {
            vel.y -= 1.0;
        }

        if Input::is_action_pressed(&input, "ui_down") {
            vel.y += 1.0;
        }

        if vel.x != 0.0 {
            ani = "walk";
            ani_sprite.set_flip_v(false);
            ani_sprite.set_flip_h(vel.x < 0.0);
        } else if vel.y != 0.0 {
            ani = "up";
            ani_sprite.set_flip_v(vel.y > 0.0);
        }

        if vel.length() > 0.0 {
            vel = vel.normalize() * self.speed;
            
            ani_sprite.play(ani, false);
            particle.set_emitting(true);
        } else {
            ani_sprite.stop();
            particle.set_emitting(false);
        }

        let pos = _owner.global_position() + vel * delta;
        let pos = Vector2::new(
            pos.x.clamp(0.0, self.screen_size.width),
            pos.y.clamp(0.0, self.screen_size.height),
        );
        _owner.set_global_position(pos);
    }

    #[export]
    fn on_player_body_entered(&self, _owner: &Area2D, _body: Ref<PhysicsBody2D>) {
        let collision_shape = unsafe {
            _owner
                .get_node_as::<CollisionShape2D>("CollisionShape2D")
                .unwrap()
        };

        _owner.hide();
        _owner.emit_signal("hit", &[]);
        collision_shape.set_deferred("disabled", true);
    }

    #[export]
    pub fn start(&self, _owner: &Area2D, pos: Vector2) {
        let collision_shape = unsafe {
            _owner
                .get_node_as::<CollisionShape2D>("CollisionShape2D")
                .unwrap()
        };

        _owner.set_position(pos);
        _owner.show();
        collision_shape.set_disabled(false);
    }
}
