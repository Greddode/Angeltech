use bevy::prelude::*;
use avian3d::prelude::*;

pub struct LevelPlugin;
impl Plugin for LevelPlugin
{
    fn build(&self, app: &mut App)
    {
     app.add_systems(Startup, init_level);
    }
}

fn  init_level(
    mut commands : Commands,
    mut meshes : ResMut<Assets<Mesh>>,
    mut materials : ResMut<Assets<StandardMaterial>>, )
{
    let level_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(1000. ,0. ,1000.),
        Mesh3d(meshes.add(Plane3d::new(Vec3::Y,  Vec2::splat(1000.)))),
        MeshMaterial3d(level_material.clone()),
        Transform::IDENTITY,
        ));

    commands.spawn((
        RigidBody::Static,
        Collider::cuboid(30.,30.,30.),
        Mesh3d(meshes.add(Cuboid::from_length(60.))),
        MeshMaterial3d(level_material.clone()),
        Transform::from_xyz(0.,0.,-100.),
        ));

    commands.spawn((
        DirectionalLight {
            color: Color::srgb(0.98, 0.95, 0.82),
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(0.0, 20.0, 0.0)
            .looking_at(Vec3::new(-0.15, -0.1, -0.15), Vec3::Y),
    )
    );
    commands.insert_resource(AmbientLight {
        color: Color::srgb(0.98, 0.95, 0.82),
        brightness: 100.0,
        affects_lightmapped_meshes: true,
    });
}

