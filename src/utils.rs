use gdnative::prelude::*;

/// Root here is needs to be the same type (or a parent type) of the node that you put in the child
///   scene as the root. For instance Spatial is used for this example.
pub fn instance_scene<Root>(scene: &Ref<PackedScene>) -> Ref<Root, Unique>
where
    Root: gdnative::object::GodotObject<Memory = ManuallyManaged> + SubClass<Node>,
{
    // let scene: TRef<PackedScene> = unsafe { scene.assume_safe() };
    // let instance = scene
    //     .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
    //     .expect("should be able to instance scene");
    let instance = instance_node_scene(scene);
    let instance = unsafe { instance.assume_unique() };

    instance
        .try_cast::<Root>()
        .expect("root node type should be correct")
}

pub fn instance_node_scene(scene: &Ref<PackedScene>) -> Ref<Node> {
    let scene = unsafe { scene.assume_safe() };
    scene
        .instance(PackedScene::GEN_EDIT_STATE_DISABLED)
        .expect("Failed to instance scene")
    // scene.instance(0).expect("Failed to instance scene")
    // let inst = scene.instance(0).expect("Failed to instance scene");
    // let inst = unsafe {inst.assume_unique()};
    // inst.try_cast::<Spatial>().expect("")
}
