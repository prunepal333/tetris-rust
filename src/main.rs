use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, Texture, TextureCreator};
use sdl2::video::{Window, WindowContext};

use std::fs::File;
use std::io::{self, Read, Write};
use std::thread;
use std::time::{Duration, SystemTime};
use std::error::Error;

const TETRIS_HEIGHT : usize = 32;
const HIGHSCORE_FILE: &'static str = "scores.txt";
const LEVEL_LINES: [u32; 10] = [20, 40, 60, 80, 100, 120, 140, 160, 180, 200];

const LEVEL_TIMES: [u32;10] = [1000, 850, 750, 650, 600, 550, 500, 400, 350, 300];

const NUM_HIGHSCORES: usize = 5;

type Piece = Vec<Vec<u8>>;
type States = Vec<Piece>;

trait TetriminoGenerator{
    fn new() -> Tetrimino;
}

#[derive(Debug,)]
struct Tetrimino {
    states: States,
    current_state: usize,
    x: isize,
    y: usize,
}

struct TetriminoI;
impl TetriminoGenerator for TetriminoI {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![1, 1, 1, 1],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
                        ],
                        vec![vec![1, 0, 0, 0],
                             vec![1, 0, 0, 0],
                             vec![1, 0, 0, 0],
                             vec![1, 0, 0, 0],
                        ],
                    ],
            current_state: 0,
            x: 4,
            y: 0,
        }
    }
}

struct TetriminoZ;
impl TetriminoGenerator for TetriminoZ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![5, 5, 0, 0],
                              vec![0, 5, 5, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
                        ],
                        vec![vec![0, 5, 0, 0],
                             vec![5, 5, 0, 0],
                             vec![5, 0, 0, 0],
                             vec![0, 0, 0, 0]
                        ],
                    ],
            current_state: 0,
            x: 4,
            y: 0,
        }
    }
}

struct TetriminoO;
impl TetriminoGenerator for TetriminoO {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![2, 2, 0, 0],
                              vec![2, 2, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
                        ],
                    ],
            current_state: 0,
            x: 5,
            y: 0,
        }
    }
}

struct TetriminoS;
impl TetriminoGenerator for TetriminoS {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![0, 4, 4, 0],
                              vec![4, 4, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0],
                        ],
                        vec![vec![4, 0, 0, 0],
                             vec![4, 4, 0, 0],
                             vec![0, 4, 0, 0],
                             vec![0, 0, 0, 0],
                        ],
                    ],
            current_state: 0,
            x: 4,
            y: 0,
        }
    }
}

struct TetriminoL;
impl TetriminoGenerator for TetriminoL{
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![7, 0, 0, 0],
                              vec![7, 0, 0, 0],
                              vec![7, 0, 0, 0],
                              vec![7, 7, 0, 0],
                        ],
                        vec![vec![0, 0, 0, 7],
                             vec![7, 7, 7, 7],
                             vec![0, 0, 0, 0],
                             vec![0, 0, 0, 0],
                        ],
                        vec![vec![7, 7, 0, 0],
                             vec![0, 7, 0, 0],
                             vec![0, 7, 0, 0],
                             vec![0, 7, 0, 0],
                        ],
                        vec![vec![7, 7, 7, 7],
                             vec![7, 0, 0, 0],
                             vec![0, 0, 0, 0],
                             vec![0, 0, 0, 0],
                        ],
                    ],
            current_state: 0,
            x: 4, 
            y: 0,
        }
    }
}

struct TetriminoJ;
impl TetriminoGenerator for TetriminoJ {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![0, 6, 0, 0],
                              vec![0, 6, 0, 0],
                              vec![0, 6, 0, 0],
                              vec![6, 6, 0, 0],
                        ],
                        vec![vec![6, 6, 6, 6],
                             vec![0, 0, 0, 6],
                             vec![0, 0, 0, 0],
                             vec![0, 0, 0, 0],
                        ],
                        vec![vec![6, 6, 0, 0],
                             vec![6, 0, 0, 0],
                             vec![6, 0, 0, 0],
                             vec![6, 0, 0, 0],
                        ],
                        vec![vec![6, 0, 0, 0],
                             vec![6, 6, 6, 6],
                             vec![0, 0, 0, 0],
                             vec![0, 0, 0, 0],
                        ],
                    ],
            current_state: 0,
            x: 4,
            y: 0,
        }
    }
}

struct TetriminoT;
impl TetriminoGenerator for TetriminoT {
    fn new() -> Tetrimino {
        Tetrimino {
            states: vec![vec![vec![3, 3, 3, 0],
                              vec![0, 3, 0, 0],
                              vec![0, 0, 0, 0],
                              vec![0, 0, 0, 0]
                        ],
                        vec![vec![3, 0, 0, 0],
                             vec![3, 3, 0, 0],
                             vec![3, 0, 0, 0],
                             vec![0, 0, 0, 0]
                        ],
                    ],
            current_state: 0,
            x: 4,
            y: 0,
        }
    }
}

impl Tetrimino {
    fn create_at_random() -> Tetrimino{
        static mut PREV: u8 = 7;
        let mut rand_num = rand::random::<u8>() % 7;
        if unsafe{ PREV } == rand_num {
            rand_num = rand::random::<u8>() % 7;
        }
        unsafe {PREV = rand_num};
        match rand_num {
            0 => TetriminoI::new(),
            1 => TetriminoO::new(),
            2 => TetriminoT::new(),
            3 => TetriminoS::new(),
            4 => TetriminoZ::new(),
            5 => TetriminoJ::new(),
            6 => TetriminoL::new(),
            _ => unreachable!(),
        }
    }
    fn test_position(&self, game_map: &[Vec<u8>], tmp_state: usize, x: isize, y: usize) -> bool{
        
        //test for whether a current state is viable to use.
        for y_ in 0..4 {
            for x_ in 0..4 {
                let x = x + x_;
                if self.states[tmp_state][y_][x_ as usize] != 0          //if there exists block in (y_, x_)
                    &&                                          // and
                    (
                     x < 0 ||
                     (y + y_) >= game_map.len() ||
                     x as usize >= game_map[y+y_].len() ||   //either x position of the block exceeds too far right
                     game_map[y+y_][x as usize] != 0                  // or game map already holds a block at the position
                    ) {
                        return false;
                    }
            }
        }
        return true;
    }
    fn rotate(&mut self, game_map: &[Vec<u8>]) {
        let mut tmp_state = self.current_state + 1;
        if tmp_state >= self.states.len() {
            tmp_state = 0;
        }
        let adjustments = [0, -1, 1, -2, 2];
        'adjustment_for: for adjustment in adjustments.into_iter(){
            if self.test_position(game_map, tmp_state, self.x + adjustment, self.y) {
                self.current_state = tmp_state;
                break 'adjustment_for;
            }
        }
    }
    fn test_current_position(&self, game_map: &[Vec<u8>]) -> bool {
        self.test_position(game_map, self.current_state as usize, self.x, self.y)
    }

    fn change_position(&mut self, game_map: &[Vec<u8>], new_x: isize, new_y: usize) -> bool{
        if self.test_position(game_map, self.current_state as usize, new_x, new_y) {
            self.x = new_x;
            self.y = new_y;
            true
        }
        else{
            false
        }
    }
}

type Grid = Vec<Vec<u8>>;
#[derive(Debug)]
struct Tetris {
    game_map: Grid,
    current_level: u32,
    score: u32,
    num_lines: u32,
    current_piece: Option<Tetrimino>,
}

impl Tetris {
    fn new() -> Tetris{
        let mut game_map = Vec::new();
        for _ in 0..16 {
            game_map.push(vec![0u8;10]);
        }
        Tetris{
            game_map,
            current_level: 1,
            score: 0,
            num_lines: 0,
            current_piece: None,
        }
    }
    fn update_score(&mut self, to_add: u32) {
        self.score += to_add;
    }
    fn inc_level(&mut self) {
        self.current_level += 1;
    }
    
    fn inc_line(&mut self) {
        self.num_lines += 1;
        if self.num_lines > LEVEL_LINES[self.current_level as usize - 1] {
            self.inc_level();
        }
    }
    fn check_lines(&mut self) {
        let mut current_row = 0;
        let mut score_to_add = 0u32;
        while current_row < self.game_map.len() {
            let mut complete = true;
            for x in self.game_map[current_row].iter() {
                if *x == 0 {
                    complete = false;
                    break;
                }
            }
            if complete {
                score_to_add += self.current_level;
                self.game_map.remove(current_row);
                current_row -= 1;
            }
            current_row += 1;
        }
        if self.game_map.len() == 0 {
            score_to_add += 1000;
        }
        while self.game_map.len() < 16 {
            self.inc_line();
            self.game_map.insert(0, vec![0u8; 10]);
        }
        self.update_score(score_to_add);
        
    }
    fn create_new_tetrimino(&self) -> Tetrimino{
        Tetrimino::create_at_random()
    }
    fn make_permanent(&mut self) {
        let mut to_add = 0;

        let mut shift_y = 0;
        if let Some(piece) = self.current_piece.take() {
            //➔ Iterate over y-direction of tetrimino grid as long as it fits into the game map
            while shift_y < piece.states[piece.current_state as usize].len()
                && shift_y + piece.y < self.game_map.len() {
                    
                let mut shift_x = 0;

                //➔ Iterate over x-direction of tetrimino grid as long as it fits into the game map
                while shift_x < piece.states[piece.current_state as usize][shift_y].len() 
                    && (piece.x + shift_x as isize) < self.game_map[shift_y + piece.y].len() as isize {
                    
                    //➔ If the current tetrimino item is a block
                    if piece.states[piece.current_state as usize][shift_y][shift_x] != 0 {

                        let x = shift_x as isize + piece.x;
                        let y = shift_y + piece.y;
                        //➔ Update the game map to hold the block
                        self.game_map[y as usize][x as usize] = piece.states[piece.current_state as usize][shift_y][shift_x];
                    }
                    shift_x += 1;
                }
                shift_y += 1;
            }
            to_add += self.current_level;
        }
        self.update_score(to_add);
        self.check_lines();
    }
}

fn handle_events(tetris: &mut Tetris, quit: &mut bool, timer: &mut SystemTime, event_pump: &mut sdl2::EventPump) -> bool {
    let mut make_permanent = false;

    if let Some(ref mut piece) = tetris.current_piece {

        let mut tmp_x = piece.x;
        let mut tmp_y = piece.y;
        for event in event_pump.poll_iter() {
            match event  {
                Event::Quit {..} |
                Event::KeyDown {keycode: Some(Keycode::Escape), ..} => {
                    *quit = true;
                    break
                },
                Event::KeyDown {keycode: Some(Keycode::Left), ..} => {
                    tmp_x -= 1;
                },
                Event::KeyDown {keycode: Some(Keycode::Right), ..} => {
                    tmp_x += 1;
                },
                Event::KeyDown {keycode: Some(Keycode::Up), ..} => {
                    piece.rotate(&tetris.game_map);
                },
                Event::KeyDown {keycode: Some(Keycode::Down), ..} => {
                    tmp_y += 1;
                },
                Event::KeyDown {keycode: Some(Keycode::Space), ..} => {
                    let x = piece.x;
                    let mut y = piece.y;
                    
                    while piece.change_position(&tetris.game_map, x, y + 1) == true {
                        y += 1;
                    }
                    make_permanent = true;
                },
                _ => {}
            }
        } // for 
        if !make_permanent {
            //If piece can not change position to (tmp_x, tmp_y) and current y position
            //is not the previous y position
            if piece.change_position(&tetris.game_map, tmp_x, tmp_y) == false 
                && tmp_y != piece.y
            {
                make_permanent = true;
            }
        }
    } // if let
    if make_permanent {
        tetris.make_permanent();
        *timer = SystemTime::now();
    }
    make_permanent
}

fn write_into_file(content: &str, filename: &str) -> io::Result<()> {
    let mut f = File::open(filename)?;
    f.write_all(content.as_bytes())
}

fn read_from_file(filename: &str) -> io::Result<String>{
    let mut f = File::open(filename)?;
    let mut content = String::new();
    f.read_to_string(&mut content)?;
    Ok(content)
}

fn slice_to_string(slice: &[u32]) -> String {
    slice.iter().map( |item| {
        item.to_string()
    }).collect::<Vec<String>>().join(" ")
}

fn line_to_slice(line: &str) -> Vec<u32> {
    // line.split(" ").filter( |item| {
    //     item.parse::<u32>().ok()
    // }).map ( |item| {
    //     item.parse::<u32>()
    // }).collect::<Vec<u32>>()

    line.split(" ").filter_map( |item| {
        item.parse::<u32>().ok()
    }).collect::<Vec<u32>>()
}

fn save_highscores_and_lines(highscores: &[u32], num_lines_sent: &[u32]) -> bool {
    let highscores = slice_to_string(highscores);
    let num_lines_sent = slice_to_string(num_lines_sent);
    let content = format!("{}\n{}\n", highscores, num_lines_sent);
    write_into_file(&content, HIGHSCORE_FILE).is_ok()
}

fn load_highscores_and_lines() -> Option<(Vec<u32>, Vec<u32>)> {
    if let Ok(content) = read_from_file(HIGHSCORE_FILE) {
        let mut lines = content.splitn(2, "\n")
            .map( |line| line_to_slice(line))
            .collect::<Vec<_>>();
        if lines.len() == 2 {
            let output: (Vec<u32>, Vec<u32>) = (lines.pop().unwrap(), lines.pop().unwrap());
            return Some((output.1, output.0)); 
        }
    }
    None
}
fn update_vec(v: &mut Vec<u32>, value: u32) -> bool{
    if v.len() < NUM_HIGHSCORES {
        v.push(value);
        v.sort();
        return true;
    }else {
        for entry in v.iter_mut(){ 
            if value > *entry { 
                *entry = value;
                return true;
            }
        }
    }
    return false;
}
fn print_game_information(tetris: &Tetris) {
    let mut highest_score = true;
    let mut highest_lines = true;

    if let Some((mut highscores, mut num_lines)) = load_highscores_and_lines() {
        if !update_vec(&mut highscores, tetris.score){ 
            highest_score = false;
        }
        if !update_vec(&mut num_lines, tetris.num_lines) {
            highest_lines = false;
        }
        if highest_score || highest_lines  {
            save_highscores_and_lines(&highscores, &num_lines);
        }
        println!("-------------------GAME OVER-----------------------");
        println!("Score: {}{}", tetris.score, if highest_score {"[New Record]"}else{""});
        println!("Num lines: {}{}", tetris.num_lines, if highest_score{"[New Record]"}else{""});
        println!("Current Level: {}", tetris.current_level);
    }
}

fn is_time_over(tetris: &Tetris, timer: &SystemTime)->bool{
    let millis = match timer.elapsed() {
        Ok(elapsed) => elapsed.as_secs() as u32 * 1000 + elapsed.subsec_nanos() / 1_000_000,
        Err(_) => return false
    };
    if millis > LEVEL_TIMES[tetris.current_level as usize]{
        true
    }else{
        false
    }
}

fn create_texture_rect<'a>(canvas: &mut Canvas<Window>, 
    texture_creator: &'a TextureCreator<WindowContext>,
     r: u8, g: u8, b: u8, 
     width: u32, height: u32) -> Option<Texture<'a>> {
    
        if let Ok(mut texture) = texture_creator.create_texture_target(None, width, height) {
            canvas.with_texture_canvas(&mut texture, |texture| {
                texture.set_draw_color(Color::RGB(r, g, b));
                texture.clear();
            }).expect("Failed to color a texture!");
            Some(texture)
        }else {
            None
        }
}

fn main() -> Result<(), Box<dyn Error>>{
    
    let sdl_context = sdl2::init()?;
    let mut tetris = Tetris::new();
    let mut timer = SystemTime::now();
    let mut event_pump = sdl_context.event_pump()?;
    let width = 1360u32;
    let height = 768u32;
    let grid_x = (width - TETRIS_HEIGHT as u32 * 10) as i32 / 2;
    let grid_y = (height - TETRIS_HEIGHT as u32 * 16) as i32 / 2;

    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Tetris", width, height)
        .position_centered()
        .build()?;
    
    let mut canvas = window.into_canvas()
        .target_texture()
        .present_vsync()    //to enable v-sync
        .build()?;
    
    let texture_creator: TextureCreator<_> = canvas.texture_creator();

    let grid = create_texture_rect(&mut canvas, 
        &texture_creator, 255, 255, 255, 
        width - 2 * grid_x as u32,
        height - 2 * grid_y as u32).expect("failed to create the grid texture");
    
    let border = create_texture_rect(&mut canvas,
         &texture_creator, 0, 0, 0, 
         width - 2 * grid_x as u32 + 10,
         height - 2 * grid_y as u32 + 10).expect("failed to create the border texture");

    macro_rules! texture {
        ($r: expr, $g: expr, $b: expr) => (
            create_texture_rect(&mut canvas,
                &texture_creator, 
                $r, $g, $b, 
                TETRIS_HEIGHT as u32, 
                TETRIS_HEIGHT as u32).unwrap()
        )
    }

    //fixed-length array
    let textures = [texture!(255, 69, 69), texture!(255, 220, 69), texture!(237, 150, 37), texture!(171, 99, 237),
                        texture!(77, 149, 239), texture!(39, 218, 225), texture!(45, 216, 47)];
    
    
    
    loop {
        if is_time_over(&tetris, &timer) {
            #[allow(unused_assignments)]
            let mut make_permanent = false;
            if let Some(ref mut piece) = tetris.current_piece {
                let x = piece.x;
                let y = piece.y + 1;
                make_permanent = !piece.change_position(&tetris.game_map, x, y);

                if make_permanent {
                    tetris.make_permanent();
                }
                timer = SystemTime::now();
            }
            //➔ set the background of the canvas
            canvas.set_draw_color(Color::RGB(255, 0, 0));
            canvas.clear();

            //➔ draw the border
            canvas.copy(&border, None, Rect::new(
                grid_x - 10,
                grid_y - 10,
                TETRIS_HEIGHT as u32 * 10 + 20, 
                TETRIS_HEIGHT as u32 * 16 + 20)).expect("failed to render the border");

            //➔ draw the grid
            canvas.copy(&grid, None, Rect::new(
                grid_x, 
                grid_y, 
                TETRIS_HEIGHT as u32 * 10,
                TETRIS_HEIGHT as u32 * 16)).expect("failed to render the grid");

            //if no tetrimino is available
            if tetris.current_piece.is_none() {
                let current_piece = tetris.create_new_tetrimino();
                if !current_piece.test_current_position(&tetris.game_map) {
                    print_game_information(&tetris);
                    return Ok(());
                }
                tetris.current_piece = Some(current_piece);
            }

            let mut quit = false;
            if !handle_events(&mut tetris, &mut quit, &mut timer, &mut event_pump) {
                if let Some(ref mut piece) = tetris.current_piece {
                    
                    //➔ Draw the current tetrimino 
                    for (row_num, _row_val) in piece.states[piece.current_state].iter().enumerate() {
                        for (col_num, &col_val) in piece.states[piece.current_state][row_num].iter().enumerate() {
                            if col_val != 0 {
                                canvas.copy(&textures[col_val as usize - 1], None, 
                                    Rect::new(
                                        grid_x + (piece.x + col_num as isize) as i32 * TETRIS_HEIGHT as i32, 
                                        grid_y + (piece.y + row_num as usize) as i32 * TETRIS_HEIGHT as i32,    
                                        TETRIS_HEIGHT as u32,
                                        TETRIS_HEIGHT as u32
                                    )
                                ).expect("failed to render tetrimino");
                            }
                        }
                    }
                }
            }
            if quit {
                print_game_information(&tetris);
                return Ok(());
            }
            //We need to draw the game map in here.

            //➔ Draw the game map
            for (row_num, _row_val) in tetris.game_map.iter().enumerate() {
                for (col_num, col_val) in tetris.game_map[row_num].iter().enumerate() {
                    if *col_val == 0 {
                        continue;
                    }
                    canvas.copy(&textures[*col_val as usize - 1], None,
                        Rect::new(
                            grid_x + (col_num as i32 * TETRIS_HEIGHT as i32),
                            grid_y + (row_num as i32 * TETRIS_HEIGHT as i32),
                            TETRIS_HEIGHT as u32, 
                            TETRIS_HEIGHT as u32
                        )
                    ).expect("failed to render game map into window");
                }
            }
            //➔ present the window
            canvas.present();

            //sleep enough to get the 60fps frame rate
            thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        }
    }
}