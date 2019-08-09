//////////////////////////////////////////////////////////////////////////
//                                                                      //
// Rusty Yacht - A yatzy game for the terminal               R U S T Y  //
// Copyright (C) 2019  Albin Söderqvist <albin@fripost.org>  U       A  //
//                                                           S       C  //
// This game is free software: you can redistribute it       T       H  //
// and/or modify it under the terms of GNU General Public    Y A C H T  //
// License as published by the Free Software Foundation,                //
// either version 3 of the License, or (at your option)                 //
// any later version.                                                   //
//                                                                      //
// Rusty Yacht is distributed in the hope that it will be fun to play,  //
// but WITHOUT ANY WARRANTY; without even the implied warranty of       //
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the         //
// GNU General Public License for more details.                         //
//                                                                      //
// You should have received a copy of the GNU General Public License    //
// along with the game. If not, see <https://www.gnu.org/licenses/>.    //
//                                                                      //
//////////////////////////////////////////////////////////////////////////

extern crate dirs;
extern crate rand;
extern crate termion;
extern crate rusty_yacht;

use rand::Rng;
use termion::clear;
use rusty_yacht::Dice;
use rusty_yacht::Score;
use rusty_yacht::Scoring;
use rusty_yacht::Highscore;
use rusty_yacht::DiceSelectStatus::{DiceComplete, DiceIncomplete, DiceExit};
use rusty_yacht::print_score_sheet;
use rusty_yacht::is_game_over;
use rusty_yacht::place_points;
use rusty_yacht::welcome;

fn main() {
    let path = Highscore::new_path();
    let mut count: u8 = 0;
    let mut rng = rand::thread_rng();

    // points slot selection
    let mut lines_selected: [u8; 17] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

    let mut dice = Dice::new();
    let validators = Scoring::new();
    let mut scores = Score::new();

    println!("{}", clear::All);

    println!("  Press Enter to roll the dice\n  or Ctrl+c at any time to exit.");
    print_score_sheet(&mut scores, &lines_selected);

    welcome();
    
    let mut lp: bool = true;
    while lp {
        println!("{}", clear::All);

        println!("  Press Enter to roll the dice\n  or Ctrl+c at any time to exit.");
        print_score_sheet(&mut scores, &lines_selected);

        count += 1;
        // Time to place points
        if count > 2 || Dice::keep_all(&dice) {
            count = 1;
            for (i, &item) in dice.to_keep.iter().enumerate() {
                if item == 0 as usize {
                    dice.current[i] = rng.gen_range(1, 7);
                } else {
                    dice.current[i] = item;
                }
            }
            println!("{}", clear::All);
            println!("  Where do you want to place your points?");
            println!("  Use the arrow keys and press Enter to select.");
            place_points(&mut scores, validators, &mut lines_selected,
                         &dice, &mut lp);

            if is_game_over(&scores) {
                println!("{}", clear::All);
                println!("  GAME OVER");
                print_score_sheet(&mut scores, &lines_selected);
                Dice::print(&dice);
                Highscore::log(&path, scores);
                println!("{}", clear::All);
                let highscore = Highscore::new(&path);
                Highscore::print(&highscore);
                break;
            }

            print_score_sheet(&mut scores, &lines_selected);
            Dice::print(&dice);

            for die in &mut dice.current.iter_mut() {
                *die = rand::thread_rng().gen_range(1, 7);
            }
            for die in &mut dice.to_keep {
                *die = 0 as usize;
            }

            println!("{}", clear::All);
            print_score_sheet(&mut scores, &lines_selected);
            Dice::print(&dice);

        } else {
            Dice::print(&dice);

            println!("{}", clear::All);
            print_score_sheet(&mut scores, &lines_selected);

            for (i, &item) in dice.to_keep.iter().enumerate() {
                if item == 0 as usize {
                    dice.current[i] = rand::thread_rng().gen_range(1, 7);
                } else {
                    dice.current[i] = item;
                }
            }
            Dice::print(&dice);
        }
        if count < 3 {
            let mut selected: [usize; 5] = [0, 0, 0, 0, 0];
            let mut left_margin = "".to_string();
            let mut margin_width: usize = 0;
            loop {
                println!("{}", clear::All);
                if count == 1 {
                    println!("  Use the arrow keys and Space to toggle which\n  dice to keep. Then press Enter to reroll.");
                } else {
                    println!("  Use the arrow keys and Space to toggle which\n  dice to keep. Then press Enter to reroll\n  for the last time.");
                }
                print_score_sheet(&mut scores, &lines_selected);
                Dice::print(&dice);
                match Dice::select(&mut dice, &mut left_margin,
                                   &mut margin_width, &mut selected) {
                    DiceExit => { lp = false; break; },
                    DiceComplete => break,
                    DiceIncomplete => continue,
                };
            }
        }
    }
}
