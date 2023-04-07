//GraphicsUsingOOP - Sliding Boxes
//Max Edward | 4/6/23

use sdl2::event::Event; //The event module imports input such as key presses.
use sdl2::keyboard::Keycode; //The keyboard module allows key presses to be registered, 
                             //such as the escape key.
use sdl2::pixels::Color; //The pixels module represents RGB color data. 
                         //HEX colors for example.
use sdl2::rect::Rect; //The rect module allows the creation of rectangles and their 
                      //attributes like size and position.
use std::time::{Duration, Instant}; //The time module allows for measuring time, 
                                    //such as measuring time elapsed.

const WINDOW_WIDTH: u32 = 640; //Width of the generated window (pixels)
const WINDOW_HEIGHT: u32 = 480; //Height of the generated window (pixels)
const BOX_VELOCITY: i32 = 5; //Speed at which the box is moving (pixels/frame)
const BOX_DELAY: u64 = 100; //Delay in milliseconds before proceeding box moves 
                            //(allows for staggered affect)
const BOX_INSTANCE_DELAY: i32 = 1;

struct Box { //Construct for Box
    x: i32, //x position in window
    y: i32, //y position in window
    box_width: u32, //Width of box
    box_height: u32, //height of box
    box_color: Color, //color of box
    direction: i32, //direction of box (positive or negative)
    delay: u64, //delay of box (used with BOX_DELAY)
}

impl Box { //Implementation of Box (all Box related functions)
    fn new( //Create new box object
        x: i32,
        y: i32,
        box_width: u32,
        box_height: u32,    //Redefine variables of box
        box_color: Color,
        direction: i32,
        delay: u64,
    ) -> Self { //Return new instance of box object with said variables
        Self {
            x,
            y,
            box_width,
            box_height,
            box_color,
            direction,
            delay,
        }
    }

    //Draws filled rectangle with specified position, width, height, and color
    fn draw(&self, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) -> Result<()
        , String> {
        let rect = Rect::new(self.x, self.y, self.box_width, self.box_height);
        canvas.set_draw_color(self.box_color);
        canvas.fill_rect(rect)
    }

    //Updates box direction and position after specified amount of time
    fn update(&mut self, elapsed_time: u64) {
        if elapsed_time >= self.delay {
            self.y += self.direction * BOX_VELOCITY;
            if self.y <= 0 {
                self.direction = 1;
            } 
            else if self.y + self.box_height as i32 >= WINDOW_HEIGHT as i32 {
                self.direction = -1;
            }
        }
    }
}

//Main Function
fn main() -> Result<(), String> { //Returns OK(()) or error message in string form
    let sdl_context = sdl2::init()?; //Initializes the SDL2 Library for usage. "?" is 
                                     //for error propagation
    let video_subsystem = sdl_context.video()?; //Gets video subsystem from SDL2 and 
                                                //returns an object. "?" is for 
                                                //error propagation
    let window = video_subsystem //Creates a window object using variables
        .window("Max Edward | Sliding Boxes", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    //Generates a mutable canvas object
    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    //Setting parameters for canvas
    canvas.set_draw_color(Color::RGB(255, 140, 0)); //Background Color
    canvas.clear(); //Clearing canvas from previous activity
    canvas.present(); //Updates canvas to show recent activity

    let mut event_pump = sdl_context.event_pump()?; //Responsible for collecting and 
                                                    //dispatching events
    let mut boxes = Vec::new(); //Creates vector for all boxes generated
    for i in 0..10 { //Loop 10 times (number of boxes)
        let x = (i as i32) * 50 + 50; //Box x position for initial and 
                                           //proceeding boxes
        let y = 200; //Box initial y position
        let box_width = 30; //Width of box (pixels)
        let box_height = 30; //Height of box (pixels)
        let box_color = Color::RGB(0, 100, 0); //Box color
        
        //Setting box initial direction
        let direction = if i % BOX_INSTANCE_DELAY == 0 { -1 } else { 1 }; 
        let delay = i as u64 * BOX_DELAY; //Delay of proceeding box before moving
        
        //Creates box object with properties
        boxes.push(Box::new(x, y, box_width, box_height, box_color, direction, delay));
    }

    let start_time = Instant::now(); //Initializes start time as current time
    'running: loop { //Creates loop with "running" label to break out of later
        for event in event_pump.poll_iter() {
            match event { //Matches given pattern or keypress to event
                Event::Quit { .. } //Quit event for closing window
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape), //If escape key is pressed...
                    ..
                } => break 'running, //...Break out of running loop
                _ => {}
            }
        }

        //Clearing canvas after one full movement (erases previous moves and objects)
        canvas.set_draw_color(Color::RGB(255, 140, 0));
        canvas.clear();

        //Calculate elapsed time since program start
        let elapsed_time = start_time.elapsed().as_millis() as u64; 
        for box_obj in &mut boxes { //Iterates over each box, allows for 
                                              //modification
            box_obj.update(elapsed_time); 
            box_obj.draw(&mut canvas)?; //Draws new box with changes in 
                                        //direction/velocity
        }

        canvas.present(); //Updates canvas

        //Controls frame rate and runtime speed of program. Slight delay for processes
        std::thread::sleep(Duration::from_millis(10)); 
                                                           
    }

    Ok(()) //Return OK / End of program
}
