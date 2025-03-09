use bevy::{prelude::*, render};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GoLPlugin)
        .run();
}

pub struct GoLPlugin;

impl Plugin for GoLPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Grid::new(40, 30));
        app.add_systems(Startup, (setup_grid, setup_materials));
        app.add_systems(Update, (render_cells));
    }
}

const CELL_SIZE: f32 = 20.0;

fn setup_grid(
    mut commands: Commands,
    mut grid: ResMut<Grid>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            let alive = rand::random();
            grid.cells[y][x] = alive;
            let cell = Cell { alive };
            let transform = Transform::from_xyz(x as f32 * CELL_SIZE, y as f32 * CELL_SIZE, 0.0);
            let sprite = Sprite::default();
            commands.spawn((cell, transform, sprite));
        }
    }
}

fn setup_materials(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let cm = CellMaterials {
        alive: materials.add(Color::WHITE.into()),
        dead: materials.add(Color::BLACK.into()),
    };
    commands.insert_resource(cm);
}

fn render_cells(mut query: Query<(&Cell, &mut Sprite)>) {
    for (cell, mut sprite) in query.iter_mut() {
        sprite.color = if cell.alive {
            println!("White");
            Color::WHITE
        } else {
            println!("Black");
            Color::BLACK
        };
    }
}

#[derive(Resource)]
struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Vec<bool>>,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        let cells = vec![vec![false; width]; height];
        Self {
            width,
            height,
            cells,
        }
    }
}

#[derive(Component)]
struct Cell {
    alive: bool,
}

#[derive(Resource)]
struct CellMaterials {
    alive: Handle<ColorMaterial>,
    dead: Handle<ColorMaterial>,
}
