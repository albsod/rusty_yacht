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
    let mut bonus: u8 = 0;
    let mut subtotal: u8 = 0;
    let mut total: u16 = 0;
    let mut count: u8 = 0;
    let mut dice = Dice::new();
    let mut rng = rand::thread_rng();
    let mut to_keep: [usize; 5] = [0,0,0,0,0]; // values of dice to keep

    // points slot selection
    let mut lines_selected: [u8; 17] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];

    let mut scores = Score::new();
    let validators = Scoring::new();

    println!("{}", clear::All);

    println!("  Press Enter to roll the dice\n  or Ctrl+c at any time to exit.");
    print_score_sheet(&mut scores, &mut subtotal, &mut bonus, &mut total,
                      &lines_selected);

    welcome();

    let mut lp: bool = true;
    while lp {
        println!("{}", clear::All);

        println!("  Press Enter to roll the dice\n  or Ctrl+c at any time to exit.");
        print_score_sheet(&mut scores, &mut subtotal, &mut bonus, &mut total,
                          &lines_selected);

        let mut all_dice_are_kept = true;

        for &i in to_keep.iter() {
            if i == 0 {
                all_dice_are_kept = false;
            }
        }

        count += 1;
        // Time to place points
        if count > 2 || all_dice_are_kept {
            count = 1;
            for (i, &item) in to_keep.iter().enumerate() {
                if item == 0 as usize {
                    dice[i] = rng.gen_range(1, 7);
                } else {
                     dice[i] = item;
                }
            }
            println!("{}", clear::All);
            println!("  Where do you want to place your points?");
            println!("  Use the arrow keys and press Enter to select.");
            place_points(&mut scores, validators, &mut subtotal,
                         &mut bonus, &mut total, &mut lines_selected,
                         &dice, &to_keep, &mut lp);

            if is_game_over(&scores) {
                println!("{}", clear::All);
                println!("  GAME OVER");
                print_score_sheet(&mut scores, &mut subtotal, &mut bonus,
                                  &mut total, &lines_selected);
                Dice::print(&dice, &to_keep);
                Highscore::log(&path, &total);
                println!("{}", clear::All);
                let highscore = Highscore::new(&path);
                Highscore::print(&highscore);
                break;
            }

            print_score_sheet(&mut scores, &mut subtotal, &mut bonus,
                              &mut total, &lines_selected);
            Dice::print(&dice, &to_keep);

            for die in &mut dice.iter_mut() {
                *die = rand::thread_rng().gen_range(1, 7);
            }
            for die in &mut to_keep {
                *die = 0 as usize;
            }

            println!("{}", clear::All);
            print_score_sheet(&mut scores, &mut subtotal, &mut bonus,
                              &mut total, &lines_selected);
            Dice::print(&dice, &to_keep);

        } else {
            Dice::print(&dice, &to_keep);

            println!("{}", clear::All);
            print_score_sheet(&mut scores, &mut subtotal, &mut bonus,
                              &mut total, &lines_selected);

            for (i, &item) in to_keep.iter().enumerate() {
                if item == 0 as usize {
                    dice[i] = rand::thread_rng().gen_range(1, 7);
                } else {
                    dice[i] = item;
                }
            }
            Dice::print(&dice, &to_keep);
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
                print_score_sheet(&mut scores, &mut subtotal, &mut bonus,
                                  &mut total, &lines_selected);
                Dice::print(&dice, &to_keep);
                match Dice::select(&mut dice, &mut to_keep, &mut left_margin,
                                  &mut margin_width, &mut selected) {
                    DiceExit => { lp = false; break; },
                    DiceComplete => break,
                    DiceIncomplete => continue,
                };
            }
        }
    }
}
