use bevy::prelude::*;

pub struct PiecesPlugin;

impl Plugin for PiecesPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .add_startup_system(create_pieces)
            .add_system(move_pieces);
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceColor
{
    White,
    Black,
}

#[derive(Clone, Copy, PartialEq)]
pub enum PieceType
{
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Component, Clone, Copy)]
pub struct Piece
{
    pub color: PieceColor,
    pub piece_type: PieceType,
    pub x: u8,
    pub y: u8,
}

fn move_pieces(time: Res<Time>, mut query: Query<(&mut Transform, &Piece)>)
{
    for (mut transform, piece) in query.iter_mut()
    {
        let direction = Vec3::new(piece.x as f32, 0., piece.y as f32) - transform.translation;

        if direction.length() > 0.1
        {
            transform.translation += direction.normalize() * time.delta_seconds();
        }
    }
}

fn create_pieces
(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{
    let king_handle: Handle<Mesh> = asset_server.load("models/chess_kit.glb#Mesh0/Primitive0");
    let king_cross_handle: Handle<Mesh> = asset_server.load("models/chess_kit.glb#Mesh1/Primitive0");
    let pawn_handle: Handle<Mesh> = asset_server.load("models/chess_kit.glb#Mesh2/Primitive0");
    let knight_1_handle: Handle<Mesh> = asset_server.load("models/chess_kit.glb#Mesh3/Primitive0");
    let knight_2_handle: Handle<Mesh> = asset_server.load("models/chess_kit.glb#Mesh4/Primitive0");
    let rook_handle: Handle<Mesh> = asset_server.load("models/chess_kit.glb#Mesh5/Primitive0");
    let bishop_handle: Handle<Mesh> = asset_server.load("models/chess_kit.glb#Mesh6/Primitive0");
    let queen_handle: Handle<Mesh> = asset_server.load("models/chess_kit.glb#Mesh7/Primitive0");

    let white_material = materials.add(Color::rgb(1., 0.8, 0.8).into());
    let black_material = materials.add(Color::rgb(0., 0.2, 0.2).into());

    spawn_rook
        (
            &mut commands,
            white_material.clone(),
            rook_handle.clone(),
            PieceColor::White,
            (0, 0),
        );
    spawn_knight
        (
            &mut commands,
            white_material.clone(),
            knight_1_handle.clone(),
            PieceColor::White,
            knight_2_handle.clone(),
            (0, 1),
        );
    spawn_bishop
        (
            &mut commands,
            white_material.clone(),
            bishop_handle.clone(),
            PieceColor::White,
            (0, 2),
        );
    spawn_queen
        (
            &mut commands,
            white_material.clone(),
            queen_handle.clone(),
            PieceColor::White,
            (0, 3),
        );
    spawn_king
        (
            &mut commands,
            white_material.clone(),
            king_handle.clone(),
            PieceColor::White,
            king_cross_handle.clone(),
            (0, 4),
        );
    spawn_bishop
        (
            &mut commands,
            white_material.clone(),
            bishop_handle.clone(),
            PieceColor::White,
            (0, 5),
        );
    spawn_knight
        (
            &mut commands,
            white_material.clone(),
            knight_1_handle.clone(),
            PieceColor::White,
            knight_2_handle.clone(),
            (0, 6),
        );
    spawn_rook
        (
            &mut commands,
            white_material.clone(),
            rook_handle.clone(),
            PieceColor::White,
            (0, 7),
        );
    for i in 0..8
    {
        spawn_pawn
            (
                &mut commands,
                white_material.clone(),
                pawn_handle.clone(),
                PieceColor::White,
                (1, i as u8),
            );
    }
    spawn_rook
        (
            &mut commands,
            black_material.clone(),
            rook_handle.clone(),
            PieceColor::Black,
            (7, 0),
        );
    spawn_knight
        (
            &mut commands,
            black_material.clone(),
            knight_1_handle.clone(),
            PieceColor::Black,
            knight_2_handle.clone(),
            (7, 1),
        );
    spawn_bishop
        (
            &mut commands,
            black_material.clone(),
            bishop_handle.clone(),
            PieceColor::Black,
            (7, 2),
        );
    spawn_queen
        (
            &mut commands,
            black_material.clone(),
            queen_handle.clone(),
            PieceColor::Black,
            (7, 3),
        );
    spawn_king
        (
            &mut commands,
            black_material.clone(),
            king_handle.clone(),
            PieceColor::Black,
            king_cross_handle.clone(),
            (7, 4),
        );
    spawn_bishop
        (
            &mut commands,
            black_material.clone(),
            bishop_handle.clone(),
            PieceColor::Black,
            (7, 5),
        );
    spawn_knight
        (
            &mut commands,
            black_material.clone(),
            knight_1_handle.clone(),
            PieceColor::Black,
            knight_2_handle.clone(),
            (7, 6),
        );
    spawn_rook
        (
            &mut commands,
            black_material.clone(),
            rook_handle.clone(),
            PieceColor::Black,
            (7, 7),
        );
    for i in 0..8
    {
        spawn_pawn
            (
                &mut commands,
                black_material.clone(),
                pawn_handle.clone(),
                PieceColor::Black,
                (6, i as u8),
            );
    }
}

fn spawn_king
(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    piece_color: PieceColor,
    mesh_cross: Handle<Mesh>,
    position: (u8, u8),
)
{
    commands
        .spawn_bundle(PbrBundle
        {
            transform: Transform::from_translation(Vec3::new
                (
                    position.0 as f32,
                    0.,
                    position.1 as f32,
            )),
            ..Default::default()
        })
        .insert(Piece
        {
            color: piece_color,
            piece_type: PieceType::King,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent|
            {
                parent.spawn_bundle(PbrBundle
                {
                    mesh,
                    material: material.clone(),
                    transform:
                    {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                        transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                        transform
                    },
                    ..Default::default()
                });
                parent.spawn_bundle(PbrBundle
                {
                    mesh: mesh_cross,
                    material,
                    transform:
                    {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -1.9));
                        transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                        transform
                    },
                    ..Default::default()
                });
            });
}

fn spawn_knight
(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh_1: Handle<Mesh>,
    piece_color: PieceColor,
    mesh_2: Handle<Mesh>,
    position: (u8, u8),
)
{
    commands
        .spawn_bundle(PbrBundle
        {
            transform: Transform::from_translation(Vec3::new
                (
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
            ..Default::default()
        })
        .insert(Piece
        {
            color: piece_color,
            piece_type: PieceType::Knight,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent|
            {
                parent.spawn_bundle(PbrBundle
                {
                    mesh: mesh_1,
                    material: material.clone(),
                    transform:
                    {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                        transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                        transform
                    },
                    ..Default::default()
                });
                parent.spawn_bundle(PbrBundle
                {
                    mesh: mesh_2,
                    material,
                    transform:
                    {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 0.9));
                        transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                        transform
                    },
                    ..Default::default()
                });
            });
}

fn spawn_queen
(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    piece_color: PieceColor,
    position: (u8, u8),
)
{
    commands
        .spawn_bundle(PbrBundle
        {
            transform: Transform::from_translation(Vec3::new
                (
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
            ..Default::default()
        })
        .insert(Piece
        {
            color: piece_color,
            piece_type: PieceType::Queen,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent|
            {
                parent.spawn_bundle(PbrBundle
                {
                    mesh,
                    material: material.clone(),
                    transform:
                    {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., -0.95));
                        transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                        transform
                    },
                    ..Default::default()
                });
            });
}

fn spawn_bishop
(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    piece_color: PieceColor,
    position: (u8, u8),
)
{
    commands
        .spawn_bundle(PbrBundle
        {
            transform: Transform::from_translation(Vec3::new
                (
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
            ..Default::default()
        })
        .insert(Piece
        {
            color: piece_color,
            piece_type: PieceType::Bishop,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent|
            {
                parent.spawn_bundle(PbrBundle
                {
                    mesh,
                    material: material.clone(),
                    transform:
                    {
                        let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 0.));
                        transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                        transform
                    },
                    ..Default::default()
                });
            });
}

fn spawn_rook
(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    piece_color: PieceColor,
    position: (u8, u8),
)
{
    commands
        .spawn_bundle(PbrBundle
        {
            transform: Transform::from_translation(Vec3::new
                (
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
            ..Default::default()
        })
        .insert(Piece
        {
            color: piece_color,
            piece_type: PieceType::Rook,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent|
            {
                parent.spawn_bundle(PbrBundle
                {
                    mesh,
                    material: material.clone(),
                    transform:
                    {
                        let mut transform = Transform::from_translation(Vec3::new(-0.1, 0., 1.8));
                        transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                        transform
                    },
                    ..Default::default()
                });
            });
}

fn spawn_pawn
(
    commands: &mut Commands,
    material: Handle<StandardMaterial>,
    mesh: Handle<Mesh>,
    piece_color: PieceColor,
    position: (u8, u8),
)
{
    commands
        .spawn_bundle(PbrBundle
        {
            transform: Transform::from_translation(Vec3::new
                (
                    position.0 as f32,
                    0.,
                    position.1 as f32,
                )),
            ..Default::default()
        })
        .insert(Piece
        {
            color: piece_color,
            piece_type: PieceType::Pawn,
            x: position.0,
            y: position.1,
        })
        .with_children(|parent|
            {
                parent.spawn_bundle(PbrBundle
                {
                    mesh,
                    material: material.clone(),
                    transform:
                    {
                        let mut transform = Transform::from_translation(Vec3::new(-0.2, 0., 2.6));
                        transform.apply_non_uniform_scale(Vec3::new(0.2, 0.2, 0.2));
                        transform
                    },
                    ..Default::default()
                });
            });
}