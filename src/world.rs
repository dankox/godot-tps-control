use gdnative::prelude::*;

#[derive(NativeClass)]
#[inherit(Node)]
pub struct World;

#[methods]
impl World {
    pub fn new(_owner: &Node) -> Self {
        World {}
    }

    #[method]
    fn _ready(&mut self, #[base] _owner: &Node) {
        godot_print!("world loaded");
    }
}
