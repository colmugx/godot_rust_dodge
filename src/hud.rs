use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(CanvasLayer)]
#[register_with(Self::register_signals)]
pub struct Hud;

impl Hud {
    fn register_signals(builder: &ClassBuilder<Self>) {
        builder.add_signal(Signal {
            name: "start",
            args: &[],
        })
    }
}

#[methods]
impl Hud {
    fn new(_owner: &CanvasLayer) -> Self {
        Self
    }

    #[export]
    pub fn show_message(&self, _owner: &CanvasLayer, text: String) {
        let message = unsafe { _owner.get_node_as::<Label>("Message").unwrap() };
        let message_timer = unsafe { _owner.get_node_as::<Timer>("MessageTimer").unwrap() };

        message.set_text(text);
        message.show();
        message_timer.start(0.0);
    }

    #[export]
    pub fn update_score(&self, _owner: &CanvasLayer, text: String) {
        let score = unsafe { _owner.get_node_as::<Label>("ScoreLabel").unwrap() };
        score.set_text(text);
    }

    #[export]
    pub fn show_game_over(&self, _owner: &CanvasLayer) {
        self.show_message(_owner, "Game Over".into());

        let message = unsafe { _owner.get_node_as::<Label>("Message").unwrap() };
        message.set_text("Dodge the\nCreeps!");
        message.show();

        let button = unsafe { _owner.get_node_as::<Button>("StartButton").unwrap() };
        button.show();
    }

    #[export]
    fn _on_start_button_pressed(&self, _owner: &CanvasLayer) {
        let button = unsafe { _owner.get_node_as::<Button>("StartButton").unwrap() };
        button.hide();

        _owner.emit_signal("start", &[]);
    }

    #[export]
    fn _on_message_timer_timeout(&self, _owner: &CanvasLayer) {
        let message = unsafe { _owner.get_node_as::<Label>("Message").unwrap() };
        message.hide();
    }
}
