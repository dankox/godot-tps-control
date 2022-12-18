use gdnative::{
    object::bounds::{AssumeSafeLifetime, LifetimeConstraint},
    prelude::*,
};

// Macros need to be defined before the modules that use them.
#[macro_export]
macro_rules! get_node {
    ($base:ident, $path:expr, $typ:ty) => {
        unsafe { $base.get_node_as::<$typ>($path).unwrap().claim() }
    };
}

#[macro_export]
macro_rules! get_node_tref {
    ($base:ident, $path:expr, $typ:ty) => {
        unsafe { $base.get_node_as::<$typ>($path).unwrap() }
    };
}

#[macro_export]
macro_rules! v {
	[$($expr:expr),*] => {
		[$(
			gdnative::prelude::Variant::new($expr)
		),*]
	};
}

pub enum OptRef<T: gdnative::prelude::GodotObject> {
    /// No value.
    None,
    /// Some gdnative Reference value of type `T`.
    Some(Ref<T>),
}

impl<T: gdnative::prelude::GodotObject> OptRef<T> {
    /// Creates `OptRef<T>` value using Godot `node` object
    /// (which in most cases will be base or owner)
    /// and `path` to the referenced node.
    #[inline]
    pub fn from_node<B>(node: &B, path: &str) -> OptRef<T>
    where
        B: SubClass<Node>,
        T: SubClass<Node>,
    {
        unsafe {
            match node.get_node_as::<T>(path) {
                Some(val) => OptRef::Some(val.claim()),
                None => OptRef::None,
            }
        }
    }

    /// Returns unsafe `TRef<T>` value using `Ref<T>.assume_safe()` function from
    /// gdnative if the value is `Some`.
    ///
    /// If the actual value is `None`, it will panic.
    #[inline]
    pub fn tref<'a, 'r>(&'r self) -> TRef<'a, T>
    where
        AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<T::Memory>,
    {
        match self {
            OptRef::Some(val) => unsafe { val.assume_safe() },
            OptRef::None => panic!("called `OptTRef::tref()` on a `None` value"),
        }
    }
}

pub fn clamp<T: std::cmp::PartialOrd>(value: T, min: T, max: T) -> T {
    assert!(min <= max);
    let mut x = value;
    if x < min { x = min; }
    if x > max { x = max; }
    x
}

// mod utils;

mod player;
mod world;

fn register_classes(handle: InitHandle) {
    handle.add_class::<world::World>();
    handle.add_class::<player::Player>();
}

godot_init!(register_classes);
