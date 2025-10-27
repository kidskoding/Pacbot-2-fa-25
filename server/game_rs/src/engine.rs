use pyo3::prelude::*;
use serde::{Deserialize, Serialize};
use tokio::time::{self, Duration, Interval};


pub mod state;



#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct GameEngine {
    pub state: GameState,
    pub active_engines: i32,
    pub ticker: Interval,

}

impl GameEngine {
    pub fn new(clock_rate: i32) -> self {
        state: GameState::new(),
        active_engines: 0,
        ticker : time::interval(Duration::from_secs(1)),
    }

    pub fn quit() {
        println!("Game is quitting.");
        return;
    }


    pub fn updateLoop() {

        active_engines += 1
        if active_engines >= 1 {
            println!("Too many game engines running");
            return;
        }

        let serial_length = 0;
        let just_ticked = true;

        loop {

            if just_ticked { //also add update ready function 
                //game state functions to be executed go here
            
            }




        }
        
    //if game isn't paused
    ticker.tick().await;

}