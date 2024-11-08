use bevy::{
  app,
  prelude::*,
  render::{
    render_asset::RenderAssetUsages,
    render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages},
    view::RenderLayers,
  },
};

pub const SPAWN_DUMMY_RENDER_LAYER_0_CAMERA: bool = true;

// Change either of these values to 0 and the button will suddenly appear
pub const UI_RENDER_LAYER: usize = 1;
pub const MAIN_RENDER_LAYER: usize = 2;

pub const MAIN_CAMERA_ORDER: isize = 1;
pub const UI_CAMERA_ORDER: isize = 2;

#[derive(Component)]
pub struct CameraUi;

#[derive(Component)]
pub struct PlayerCamera;

pub fn spawn_ui_camera(mut commands: Commands) {
  let fov: f32 = 10.0;
  let mut x = 0.0;
  let mut y = 1.0;
  let mut z = -0.24;

  let mut transform = Transform::from_translation(Vec3::new(x, y, z)).looking_at(
    Vec3 {
      x: 0.0,
      y: 1.0,
      z: -0.005,
    },
    Vec3::Y,
  );
  y += 0.01;
  z += 0.01;

  transform.translation.y = y;
  transform.translation.z = z;

  commands.spawn((
    Camera3dBundle {
      camera: Camera {
        order: UI_CAMERA_ORDER,
        clear_color: ClearColorConfig::None,
        ..default()
      },
      transform,
      projection: Projection::Perspective(PerspectiveProjection {
        fov: fov.to_radians(),
        ..default()
      }),
      ..default()
    },
    CameraUi,
    RenderLayers::layer(UI_RENDER_LAYER),
  ));
}

pub fn setup_hand_ui(mut commands: Commands, camera: Query<Entity, With<CameraUi>>) {
  for camera in &camera {
    println!("Spawning button");
    commands
      .spawn((
        NodeBundle {
          style: Style {
            width: Val::Percent(90.0),
            height: Val::Percent(90.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::FlexEnd,
            ..default()
          },
          ..default()
        },
        TargetCamera(camera),
      ))
      .with_children(|parent| {
        let name = "Play Card";

        parent
          .spawn((ButtonBundle {
            style: Style {
              width: Val::Px(150.0),
              height: Val::Px(65.0),
              border: UiRect::all(Val::Px(5.0)),
              justify_content: JustifyContent::Center,
              align_items: AlignItems::Center,
              ..default()
            },
            border_color: BorderColor(Color::BLACK),
            border_radius: BorderRadius::MAX,
            background_color: Color::BLACK.into(),
            ..default()
          },))
          .with_children(|parent| {
            parent.spawn((TextBundle::from_section(
              name,
              TextStyle {
                font_size: 20.0,
                color: Color::srgb(0.9, 0.9, 0.9),
                ..default()
              },
            ),));
          });
      });
  }
}

fn main() {
  println!("Hello, world!");

  App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, (setup, spawn_ui_camera, setup_hand_ui).chain())
    .run();
}

/// set up a simple 3D scene
fn setup(
  mut commands: Commands,
  mut meshes: ResMut<Assets<Mesh>>,
  mut materials: ResMut<Assets<StandardMaterial>>,
  mut images: ResMut<Assets<Image>>,
) {
  // circular base
  commands.spawn((
    PbrBundle {
      mesh: meshes.add(Circle::new(4.0)),
      material: materials.add(Color::WHITE),
      transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
      ..default()
    },
    RenderLayers::layer(MAIN_RENDER_LAYER),
  ));
  // cube
  commands.spawn((
    PbrBundle {
      mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
      material: materials.add(Color::srgb_u8(124, 144, 255)),
      transform: Transform::from_xyz(0.0, 0.5, 0.0),
      ..default()
    },
    RenderLayers::layer(MAIN_RENDER_LAYER),
  ));
  // light
  commands.spawn((
    PointLightBundle {
      point_light: PointLight {
        shadows_enabled: true,
        ..default()
      },
      transform: Transform::from_xyz(4.0, 8.0, 4.0),
      ..default()
    },
    RenderLayers::layer(MAIN_RENDER_LAYER),
  ));
  // camera
  commands.spawn((
    Camera3dBundle {
      transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
      projection: Projection::Perspective(PerspectiveProjection {
        fov: 90.0_f32.to_radians(),
        ..default()
      }),
      camera: Camera {
        order: MAIN_CAMERA_ORDER,
        ..default()
      },
      ..default()
    },
    PlayerCamera,
    RenderLayers::layer(MAIN_RENDER_LAYER),
  ));

  if SPAWN_DUMMY_RENDER_LAYER_0_CAMERA {
    let size = Extent3d {
      width: 512,
      height: 512,
      ..default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image::new_fill(
      size,
      TextureDimension::D2,
      &[0, 0, 0, 0],
      TextureFormat::Bgra8UnormSrgb,
      RenderAssetUsages::default(),
    );
    // You need to set these texture usage flags in order to use the image as a render target
    image.texture_descriptor.usage =
      TextureUsages::TEXTURE_BINDING | TextureUsages::COPY_DST | TextureUsages::RENDER_ATTACHMENT;

    let image_handle = images.add(image);
    commands.spawn((
      Camera3dBundle {
        camera: Camera {
          target: image_handle.into(),
          ..default()
        },
        ..default()
      },
      RenderLayers::layer(0),
    ));
  }
}
