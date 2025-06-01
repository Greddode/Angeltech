use bevy::{prelude::*, window::PrimaryWindow};
use avian3d::prelude::*;
use crate::game::player::camera_controller::CameraController;
use super::camera_controller;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, init_player);
    }
}
#[derive(Component)]
pub struct Player {}
fn init_player(mut commands: Commands)
{
    let fov = 90.0_f32.to_radians();
    commands.spawn((
        Player{},
        Camera3d::default(),
        Camera {
            order: 1,
            ..default()
        },
        Transform::from_xyz(0.,10.,0.).look_at(Vec3::ZERO, Vec3::Y),
        Projection::from(PerspectiveProjection
        {
            fov: fov,
            ..default()
        }),
        camera_controller::CameraController 
        { 
            sensitivity: 0.035,
            rotation: Vec2::ZERO,
            rotation_lock: 88.0, },
        ));
}