// Re-export the Bevy prelude so downstream crates can use Bevy types without depending on bevy directly.
pub use bevy::prelude::*;

// Public small API surface: plugin registration helpers, shared component/resource types
pub mod plugins {
    use bevy::prelude::*;
}
// Example shared type
pub mod shared {
    use bevy::prelude::*;
    // #[derive(Component)]
    // pub struct SharedComponent(pub u32);
}
