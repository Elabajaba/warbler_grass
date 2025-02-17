use bevy::{
    asset::Handle,
    ecs::{bundle::Bundle, component::Component, query::QueryItem},
    math::Vec3,
    prelude::Color,
    render::{
        extract_component::ExtractComponent, mesh::Mesh, prelude::SpatialBundle, primitives::Aabb,
        texture::Image, texture::DEFAULT_IMAGE_HANDLE,
    },
};

use crate::{density_map::DensityMap, height_map::HeightMap, warblers_plugin::GRASS_MESH_HANDLE};

/// This [`Bundle`] spawns a grass chunk in the world.
///
/// This is the recommended way to spawn grass in games.
/// # Note
/// If you only want to input explicit positions of the grass blades you can also use
/// the [`WarblersExplicitBundle`].
#[derive(Bundle)]
pub struct WarblersBundle {
    /// The [`Mesh`] of the grass blades
    ///
    /// Defaults to the mesh seen in the examples.
    /// The mesh may also be changed at runtime.
    /// You might want to take a look at the
    /// `grass_mesh` example for that
    pub grass_mesh: Handle<Mesh>,
    /// An [`HeightMap`] component
    pub height_map: HeightMap,
    /// An [`DensityMap`] component
    pub density_map: DensityMap,
    /// An [`WarblerHeight`] component
    pub height: WarblerHeight,
    /// An [`GrassColor`] component
    pub grass_color: GrassColor,
    /// An [`Aabb`] component
    ///
    /// Note that the Aabb is used to define the world dimensions of the [`DensityMap`] and [`HeightMap`].
    pub aabb: Aabb,
    #[bundle]
    pub spatial: SpatialBundle,
}
impl Default for WarblersBundle {
    fn default() -> Self {
        Self {
            grass_mesh: GRASS_MESH_HANDLE.typed(),
            height_map: DEFAULT_IMAGE_HANDLE.typed().into(),
            density_map: DEFAULT_IMAGE_HANDLE.typed().into(),
            height: WarblerHeight::Uniform(1.),
            grass_color: GrassColor::default(),
            aabb: Default::default(),
            spatial: Default::default(),
        }
    }
}
/// The height of the grass blades
///
/// Can be used in Combination with the [`WarblersBundle`] to spawn grass chunks
#[derive(Component, Clone)]
pub enum WarblerHeight {
    /// Sets the height of the grass blades to a constant value.
    Uniform(f32),
    /// Samples the height from an [`Image`]
    ///
    /// The [`Image`] will be scaled over the plane defined by the [`Aabb`]
    Texture(Handle<Image>),
}
/// Defines the color of the grass blades
#[derive(Component, Clone, ExtractComponent)]
pub struct GrassColor {
    /// The main [Color] of the grass used in your game
    pub main_color: Color,
    /// The bottom [Color] of the grass
    ///
    /// Normally, a darker variant of the main color is choosen to reflect the natural behavior of light
    pub bottom_color: Color,
}
impl Default for GrassColor {
    fn default() -> Self {
        GrassColor {
            main_color: Color::rgb(0.2, 0.5, 0.0),
            bottom_color: Color::rgb(0.1, 0.1, 0.0),
        }
    }
}
impl ExtractComponent for WarblerHeight {
    type Query = &'static Self;

    type Filter = ();

    type Out = Self;

    fn extract_component(item: QueryItem<'_, Self::Query>) -> Option<Self::Out> {
        match item {
            WarblerHeight::Uniform(_) => Some(item.clone()),
            WarblerHeight::Texture(handle) => Some(WarblerHeight::Texture(handle.clone_weak())),
        }
    }
}

/// Used to define the positions of all the grass blades explicitly
///
/// Can be used with the [`WarblersExplicitBundle`]
///
/// # Example
/// ```rust
/// use warbler_grass::prelude::Grass;
/// use bevy::prelude::Vec3;
///
/// let mut positions = Vec::with_capacity(10 * 10);
/// // let's make a simple 10x10 grid
/// for x in 0..10 {
///     for y in 0..10 {
///         positions.push(Vec3::new(x as f32,0., y as f32));
///     }
/// }
/// let height = 2.;
///
/// // One way to create grass
/// let grass1 = Grass::new(positions.clone(), height);
///
/// // Another way
/// let grass2 = Grass::from(&positions[..]).with_height(height);
/// assert_eq!(grass1, grass2);
/// ```
#[derive(Component, Clone, PartialEq, Debug)]
pub struct Grass {
    /// The positions of each grass blade defined
    ///
    /// The positions are always relative to the entity [`Transform`] component.
    pub positions: Vec<Vec3>,
    /// The height of the grass blades
    pub height: f32,
}
impl Default for Grass {
    fn default() -> Self {
        Self {
            positions: Default::default(),
            height: 1.,
        }
    }
}
impl Grass {
    /// Creates a new [`Grass`] instance
    pub fn new(positions: Vec<Vec3>, height: f32) -> Self {
        Grass { positions, height }
    }
    /// sets the [`Grass`] height and returns itself after
    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
}
/// Can be used to create grass from a slice of positions
///
/// The height will be set to the default height
impl From<&[Vec3]> for Grass {
    fn from(value: &[Vec3]) -> Self {
        Self {
            positions: value.into(),
            height: Default::default(),
        }
    }
}
/// A bundle spawning a grass chunk in the world
///
/// It uses explicit positions of all grass blades to generate the them
/// For an example take a look at the `load_explicit` example
#[derive(Bundle)]
pub struct WarblersExplicitBundle {
    /// The [`Mesh`] of the grass blades
    ///
    /// Defaults to the mesh seen in the examples.
    /// The mesh may also be changed at runtime.
    /// You might want to take a look at the
    /// `grass_mesh` example for that
    pub grass_mesh: Handle<Mesh>,
    /// The explicit positions of the grass blades
    pub grass: Grass,
    /// The color of the grass
    pub grass_color: GrassColor,
    #[bundle]
    pub spatial: SpatialBundle,
}

impl Default for WarblersExplicitBundle {
    fn default() -> Self {
        Self {
            grass_mesh: GRASS_MESH_HANDLE.typed(),
            grass_color: GrassColor::default(),
            grass: Grass::default(),
            spatial: Default::default(),
        }
    }
}
