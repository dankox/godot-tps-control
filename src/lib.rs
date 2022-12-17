use gdnative::prelude::*;

// Macros need to be defined before the modules that use them.
#[macro_export]
macro_rules! get_node {
	($base:ident, $path:expr, $typ:ty) => {
		unsafe {
			$base
				.get_node($path)
				.unwrap()
				.cast::<$typ>()
				.assume_shared()
		}
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

mod world;

fn register_classes(handle: InitHandle) {
	handle.add_class::<world::World>();
}

godot_init!(register_classes);