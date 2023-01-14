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
    ///
    /// If `path` or type of node is incorrect, it will return
    /// `OptRef::None` and will print godot error message with
    /// description what didn't match
    #[inline]
    pub fn from_node<B>(node: &B, path: &str) -> OptRef<T>
    where
        B: SubClass<Node>,
        T: SubClass<Node>,
    {
        unsafe {
            match node.get_node_as::<T>(path) {
                Some(val) => OptRef::Some(val.claim()),
                None => {
                    godot_error!(
                        "called `OptTRef::from_node()` on non existent path '{}' or type '{}'",
                        path,
                        std::any::type_name::<T>()
                    );
                    OptRef::None
                }
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
            OptRef::None => {
                godot_error!("called `OptTRef::tref()` on a `None` value");
                panic!("called `OptTRef::tref()` on a `None` value");
            }
        }
    }

    /// Set `transform` property for `Spatial` node (or for subclass of it) to new
    /// transform.
    ///
    /// If the actual value of `OptTRef` is `None`, it will report a godot error with
    /// details and continue.
    #[inline]
    pub fn set_transform<'a, 'r>(&'r self, transform: Transform)
    where
        AssumeSafeLifetime<'a, 'r>: LifetimeConstraint<T::Memory>,
        T: SubClass<Spatial> + 'a,
    {
        match self {
            OptRef::Some(val) => {
                let tr = unsafe { val.assume_safe() };
                tr.upcast::<Spatial>().set("transform", transform);
            }
            OptRef::None => godot_error!(
                "called `OptTRef::set_transform()` on non `Spatial` node of type: {}",
                std::any::type_name::<T>()
            ),
        }
    }
}

pub fn clamp<T: std::cmp::PartialOrd>(value: T, min: T, max: T) -> T {
    assert!(min <= max);
    let mut x = value;
    if x < min {
        x = min;
    }
    if x > max {
        x = max;
    }
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
