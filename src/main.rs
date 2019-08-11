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
extern crate rusty_yacht;

use rusty_yacht::Dice;
use rusty_yacht::Scores;
use rusty_yacht::ScoreValidator;
use rusty_yacht::Highscore;
use rusty_yacht::DiceSelectStatus;
use rusty_yacht::welcome;
use rusty_yacht::clear_screen;

fn main() {
    let file = Highscore::new_path();
    let mut dice = Dice::new();
    let validators = ScoreValidator::new();
    let mut score = Scores::new();

    clear_screen();
    println!("  Press Enter to roll the dice\n  or Ctrl+c at any time to exit.");
    score.print();
    welcome();

    let mut count: u8 = 0;
    let mut lp: bool = true;
    while lp {
        clear_screen();
        println!("  Press Enter to roll the dice\n  or Ctrl+c at any time to exit.");
        score.print();

        if count < 2 && dice.keep_all() == false {
            clear_screen();
            score.print();
            dice.roll();
            count += 1;
            dice.print();

            let mut selected: [usize; 5] = [0, 0, 0, 0, 0];
            let mut left_margin = "".to_string();
            let mut margin_width: usize = 0;
            loop {
                clear_screen();
                if count == 2 {
                    println!("  Use the arrow keys and Space to toggle which\n  dice to keep. Then press Enter to reroll\n  for the last time.");
                } else {
                    println!("  Use the arrow keys and Space to toggle which\n  dice to keep. Then press Enter to reroll.");

                }
                score.print();
                dice.print();
                match dice.select(&mut left_margin, &mut margin_width,
                                  &mut selected) {
                    DiceSelectStatus::Exit => {
                        lp = false;
                        break;
                    },
                    DiceSelectStatus::Complete => break,
                    DiceSelectStatus::Incomplete => continue,
                };
            }

        // Time to place points
        } else if count > 1 || dice.keep_all() {
            count = 0;
            dice.roll();
            clear_screen();
            println!("  Where do you want to place your points?");
            println!("  Use the arrow keys and press Enter to select.");
            score.place_points(validators, &dice, &mut lp);

            if score.is_final() {
                clear_screen();
                println!("  GAME OVER");
                score.print();
                dice.print();
                Highscore::log(&file, score);
                clear_screen();
                let highscore = Highscore::new(&file);
                Highscore::print(&highscore);
                break;
            }

            score.print();
            dice.print();
            dice.reroll_all();
            clear_screen();
            score.print();
            dice.print();
        }
    }
}
