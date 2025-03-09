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
        app.insert_resource(Grid::new(40, 40));
        app.add_systems(Startup, (setup_camera, setup_grid));
        app.add_systems(Update, (render_cells));
    }
}

const CELL_SIZE: f32 = 20.0;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d::default());
}

fn setup_grid(mut commands: Commands, mut grid: ResMut<Grid>, window: Query<&Window>) {
    let window = window.single();
    let window_width = window.width();
    let window_height = window.height();

    let cell_width = window_width / grid.width as f32;
    let cell_height = window_height / grid.height as f32;
    let cell_size = cell_width.min(cell_height);
    let actual_cell_size = cell_size * 0.98;

    let grid_width = cell_size * grid.width as f32;
    let grid_height = cell_size * grid.height as f32;
    let offset_x = (window_width - grid_width) / 2.0 + cell_size / 2.0;
    let offset_y = (window_height - grid_height) / 2.0 + cell_size / 2.0;

    println!("Width: {window_width} height: {window_height}");
    for y in 0..grid.height {
        for x in 0..grid.width {
            let alive = rand::random();
            grid.cells[y][x] = alive;
            let cell = Cell { alive };
            let transform = Transform::from_xyz(
                offset_x + x as f32 * cell_size - grid_width / 2.0,
                offset_y + y as f32 * cell_size - grid_height / 2.0,
                0.0,
            );
            let sprite = Sprite {
                color: if cell.alive {
                    Color::WHITE
                } else {
                    Color::BLACK
                },
                custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                ..Default::default()
            };
            commands.spawn((cell, sprite, transform));
        }
    }
}

fn render_cells(mut query: Query<(&Cell, &mut Sprite)>) {
    for (cell, mut sprite) in query.iter_mut() {
        sprite.color = if cell.alive {
            Color::WHITE
        } else {
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
