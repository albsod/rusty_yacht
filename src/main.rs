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
/////////////////////////////////////////////////////////////////////////

extern crate chrono;
extern crate dirs;
extern crate rand;
extern crate termion;

use std::io::{Write, stdout, stdin, BufRead, BufReader, ErrorKind};
use std::fs::*;
use std::path::PathBuf;
use chrono::prelude::*;
use rand::Rng;
use termion::clear;
use termion::style;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use SlotSelectStatus::*;
use DiceSelectStatus::*;

struct Score<'a> {
    name: &'a str,
    value: String,
    selected: bool,
}

enum DiceSelectStatus {
    DiceComplete,
    DiceExit,
    DiceIncomplete,
}

enum SlotSelectStatus {
    SlotAlreadySelected,
    SlotExit,
    SlotInvalid,
    SlotComplete,
    SlotIncomplete,
}

fn main() {



    // Create highscore path if non-existing
    let home = dirs::home_dir().expect("No home directory!");
    let mut path = PathBuf::new();
    path.push(home);
    path.push(".config");
    path.push("rusty-yacht");
    create_dir_all(&path).expect("Could not create ~/.config/rusty-yacht");
    path.push("highscore.txt");

    let mut bonus: u8 = 0;
    let mut subtotal: u8 = 0;
    let mut total: u16 = 0;
    let mut count: u8 = 0;
    let mut dice: [usize; 5] = [0,0,0,0,0]; // dice values
    let mut rng = rand::thread_rng();
    let mut to_keep: [usize; 5] = [0,0,0,0,0]; // values of dice to keep
    // points slot selection
    let mut lines_selected: [u8; 17] = [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0];
    let mut i = 0 as usize;

    // init of score values
    let score_ones = Score {
        name: "Ones", value: "  ".to_string(), selected: false,
    };
    let score_twos = Score {
        name: "Twos", value: "  ".to_string(), selected: false,
    };
    let score_threes = Score {
        name: "Threes", value: "  ".to_string(), selected: false,
    };
    let score_fours = Score {
        name: "Fours", value: "  ".to_string(), selected: false,
    };
    let score_fives = Score {
        name: "Fives", value: "  ".to_string(), selected: false,
    };
    let score_sixes = Score {
        name: "Sixes", value: "  ".to_string(), selected: false,
    };
    let score_bonus = Score {
        name: "Bonus", value: "  ".to_string(), selected: false,
    };
    let score_one_pair = Score {
        name: "One Pair", value: "  ".to_string(), selected: false,
    };
    let score_two_pairs = Score {
        name: "Two Pairs", value: "  ".to_string(), selected: false,
    };
    let score_three_kind = Score {
        name: "Three of a Kind", value: "  ".to_string(), selected: false,
    };
    let score_four_kind = Score {
        name: "Four of a Kind", value: "  ".to_string(), selected: false,
    };
    let score_small_str = Score {
        name: "Small Straight", value: "  ".to_string(), selected: false,
    };
    let score_large_str = Score {
        name: "Large Straight", value: "  ".to_string(), selected: false,
    };
    let score_full_house = Score {
        name: "Full House", value: "  ".to_string(), selected: false,
    };
    let score_chance = Score {
        name: "Chance", value: "  ".to_string(), selected: false,
    };
    let score_yatzy = Score {
        name: "Yatzy", value: "  ".to_string(), selected: false,
    };
    let score_subtotal = Score {
        name: "Sum", value: "   ".to_string(), selected: false,
    };
    let score_total = Score {
        name: "Total", value: "   ".to_string(), selected: false,
    };

    // Move the score values to this array
    let mut scores = [score_ones, score_twos, score_threes, score_fours,
                      score_fives, score_sixes, score_one_pair,
                      score_two_pairs, score_three_kind, score_four_kind,
                      score_small_str, score_large_str, score_full_house,
                      score_chance, score_yatzy, score_bonus, score_subtotal,
                      score_total];

    // Move the value-check functions to this array
    let value = [ones, twos, threes, fours, fives, sixes, one_pair,
                 two_pairs, three_kind, four_kind, small_str,
                 large_str, full_house, chance, yatzy];

    // First roll?
    let mut first_run = 1;

    'outer: loop {
        println!("{}", clear::All);

        println!("  Press Enter to roll the dice\n  or Ctrl+c at any time to exit.");
        print_score_sheet(&mut scores, &mut subtotal, &mut bonus, &mut total, &lines_selected);

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
            loop {
                lines_selected[i] = 1;
                print_score_sheet(&mut scores, &mut subtotal, &mut bonus, &mut total, &lines_selected);
                print_dice(&dice, &to_keep);

                match select_slot(&dice, value, &mut scores, &mut lines_selected, &mut i) {
                    SlotExit            =>   break 'outer,
                    SlotAlreadySelected => { println!("{}", clear::All);
                                             println!("  Sorry, you can't use this slot again.");
                                             println!("  Press Enter to continue.");
                                             print_score_sheet(&mut scores, &mut subtotal, &mut bonus, &mut total, &lines_selected);
                                             print_dice(&dice, &to_keep);
                                             let stdin = stdin();
                                             let mut stdout = stdout().into_raw_mode().unwrap();
                                             write!(stdout, "{}", termion::cursor::Hide).unwrap();
                                             for c in stdin.keys() {
                                                 match c.unwrap() {
                                                     Key::Ctrl(c) => if c == 'c' {
                                                         break 'outer;
                                                     },
                                                     Key::Char('\n') => {
                                                         println!("{}", clear::All);
                                                         break;
                                                     },
                                                     Key::Up => {
                                                         if i != 0 {
                                                             i -= 1;
                                                             lines_selected[i+1] = 0;
                                                         } else {
                                                             i = 14;
                                                             lines_selected[0] = 0;
                                                         }
                                                         lines_selected[i] = 1;
                                                         println!("{}", clear::All);
                                                         break;
                                                     },
                                                     Key::Down => {
                                                         if i != 14 {
                                                             i += 1;
                                                             lines_selected[i-1] = 0;
                                                         } else {
                                                             i = 0;
                                                             lines_selected[14] = 0;
                                                         }
                                                         lines_selected[i] = 1;
                                                         println!("{}", clear::All);
                                                         break;
                                                     },
                                                     _ => { println!("{}", clear::All);
                                                            continue;
                                                     }
                                                 }
                                             }
                                             stdout.flush().unwrap();
                    },

                    SlotComplete      => { println!("{}", clear::All);
                                           println!("  Selection complete. Press Enter to continue.");
                                           print_score_sheet(&mut scores, &mut subtotal, &mut bonus, &mut total, &lines_selected);
                                           print_dice(&dice, &to_keep);
                                           let stdin = stdin();
                                           let mut stdout = stdout().into_raw_mode().unwrap();
                                           write!(stdout, "{}", termion::cursor::Hide).unwrap();
                                           for c in stdin.keys() {
                                               match c.unwrap() {
                                                   Key::Ctrl(c) => if c == 'c' {
                                                       break 'outer;
                                                   },
                                                   Key::Char('\n') => break,
                                                   _ => continue,
                                               }
                                           }
                                           stdout.flush().unwrap();
                                           break; },
                    SlotIncomplete    => { println!("{}", clear::All); },
                    SlotInvalid       => {
                        println!("{}", clear::All);
                        println!("  Invalid selection. Press - to strike it out");
                        println!("  or an arrow key to cancel.");
                    },
                }
            }
            lines_selected[i] = 0;
            println!("{}", clear::All);

            if is_game_over(&scores) == true {
                let mut name = String::new();
                loop {
                    println!("{}", clear::All);
                    println!("  GAME OVER");
                    print_score_sheet(&mut scores, &mut subtotal, &mut bonus, &mut total, &lines_selected);
                    print_dice(&dice, &to_keep);
                    println!("Input a name to log your score:");
                    stdin().read_line(&mut name)
                        .expect("Failed to read line");
                    name.pop(); // remove trailing newline
                    let namelen = name.len();
                    if namelen > 25 {
                        println!("  Too long! Max length is 25 characters.");
                        name = String::from("");
                    } else {
                        let name_padding = 25 - namelen;
                        for _ in 0..name_padding {
                            name.push(' ');
                        }
                        break;
                    }
                }
                let mut highscore_date = Local::now().date().to_string();
                highscore_date.truncate(10);

                let mut file = OpenOptions::new()
                    .append(true)
                    .open(&path)
                    .unwrap_or_else(|error| {
                        if error.kind() == ErrorKind::NotFound {
                            File::create(&path).unwrap_or_else(|error| {
                                panic!("Tried to create ~/.config/rusty-yacht/highscore.txt but there was a problem: {:?}", error);
                            })
                        } else {
                            panic!("There was a problem opening the highscore file: {:?}", error);
                        }
                    });

                match total {
                    0...9 => { if let Err(e) = writeln!(file, "{}| {} |  {:?}", name,highscore_date,total) {
                        eprintln!("Couldn't write to file: {}", e); }
                    },
                    10...99 => { if let Err(e) = writeln!(file, "{}| {} | {:?}", name,highscore_date,total) {
                        eprintln!("Couldn't write to file: {}", e); }
                    },
                    _ => { if let Err(e) = writeln!(file, "{}| {} |{:?}", name,highscore_date,total) {
                        eprintln!("Couldn't write to file: {}", e); }
                    },
                }
                println!("{}", clear::All);
                print_highscore(&read_highscore(&path));
                break;

            }

            print_score_sheet(&mut scores, &mut subtotal, &mut bonus, &mut total, &lines_selected);
            print_dice(&dice, &to_keep);

            //if is_game_over(&scores) == true {
            //}

            for die in &mut dice.iter_mut() {
                *die = rand::thread_rng().gen_range(1, 7);
            }
            for die in &mut to_keep {
                *die = 0 as usize;
            }

            println!("{}", clear::All);
            print_score_sheet(&mut scores, &mut subtotal, &mut bonus, &mut total, &lines_selected);
            print_dice(&dice, &to_keep);

        } else {
            if first_run == 1 {
                println!("R U S T Y R U S T Y R U S T Y R U S T Y R U S T Y");
                println!("U       A U       A U       A U       A U       A");
                println!("S       C S       C S       S T       C S       C");
                println!("T       H T       H T       H S       H T       H");
                println!("Y A C H T Y A C H T Y A C H T Y A C H T Y A C H T\n");

                let stdin = stdin();
                let mut stdout = stdout().into_raw_mode().unwrap();
                write!(stdout, "{}", termion::cursor::Hide).unwrap();
                for c in stdin.keys() {
                    match c.unwrap() {
                        Key::Char('\n') => break,
                        Key::Ctrl(c) => { if c == 'c' { break 'outer; } else { continue;} },
                        _ => continue,
                    }
                }
                stdout.flush().unwrap();
                first_run = 0;
                // This isn't currently in use:
            } else if first_run == 42 {
                println!("╔═══════╗ ╔═══════╗ ╔═══════╗ ╔═══════╗ ╔═══════╗");
                println!("║ ┌───┐ ║ ║ ┌───┐ ║ ║ ┌───┐ ║ ║ ┌───┐ ║ ║ ┌───┐ ║");
                println!("║ │ Y │ ║ ║ │ A │ ║ ║ │ T │ ║ ║ │ Z │ ║ ║ │ Y │ ║");
                println!("║ └───┘ ║ ║ └───┘ ║ ║ └───┘ ║ ║ └───┘ ║ ║ └───┘ ║");
                println!("╚═══════╝ ╚═══════╝ ╚═══════╝ ╚═══════╝ ╚═══════╝\n");
                first_run = 0;
            } else {
                print_dice(&dice, &to_keep);
            }

            println!("{}", clear::All);
            print_score_sheet(&mut scores, &mut subtotal, &mut bonus, &mut total, &lines_selected);

            for (i, &item) in to_keep.iter().enumerate() {
                if item == 0 as usize {
                    dice[i] = rand::thread_rng().gen_range(1, 7);
                } else {
                    dice[i] = item;
                }
            }
            print_dice(&dice, &to_keep);

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
                print_score_sheet(&mut scores, &mut subtotal, &mut bonus, &mut total, &lines_selected);
                print_dice(&dice, &to_keep);
                match select_dice(&mut dice, &mut to_keep, &mut left_margin, &mut margin_width, &mut selected) {
                    DiceExit => break 'outer,
                    DiceComplete => break,
                    DiceIncomplete => continue,
                };
            }
        }
    }
}

fn is_game_over(scores: &[Score; 18]) -> bool {
    for item in scores.iter() {
        if item.name == "Bonus" {
            continue;
        }
        if item.value == "  " {
            return false;
        }
    }
    true
}

fn print_score_sheet(scores: &mut [Score; 18], subtotal: &mut u8, bonus: &mut u8, total: &mut u16, lines_selected: &[u8]) {
    if subtotal != &mut 0 {
        *subtotal = 0;
    }
    if bonus != &mut 0 {
        *bonus = 0;
    }
    if total != &mut 0 {
        *total = 0;
    }

    for elm in scores.iter() {
        if elm.name == "One Pair" {
            break;
        } else if elm.value != "  " && elm.value != "   " && elm.value != " –" {
            *subtotal += elm.value.trim().parse::<u8>().unwrap();
        }
    }

    if *subtotal > 99 {
        scores[16].value.replace_range(.., &subtotal.to_string());
    } else if *subtotal > 9 {
        scores[16].value.replace_range(1.., &subtotal.to_string());
    } else if *subtotal > 0 {
        scores[16].value.replace_range(2.., &subtotal.to_string());
    }

    if *subtotal > 62 {
        scores[15].value.replace_range(.., "50");
    } else if
        scores[0].value != "  " &&
        scores[1].value != "  " &&
        scores[2].value != "  " &&
        scores[3].value != "  " &&
        scores[4].value != "  " &&
        scores[5].value != "  "
    {
        scores[15].value.replace_range(1.., "–");
    }

    for elm in scores.iter() {
        if elm.value != "  " &&
            elm.value != "   " &&
            elm.value != " –" &&
            elm.name != "Sum" {
                *total += elm.value.trim().parse::<u16>().unwrap();
            }
    }

    if *total > 99 {
        scores[17].value.replace_range(.., &total.to_string());
    } else if *total > 9 {
        scores[17].value.replace_range(1.., &total.to_string());
    } else if *total > 0 {
        scores[17].value.replace_range(2.., &total.to_string());
    }

    println!("╔═══════════════════════════════════════════════╗");
    println!("║ RUSTY YACHT                                   ║");
    println!("╠═══════════════════════════╦═══════════════════╣");
    println!("║                       Max ║             Score ║");
    println!("╟───────────────────────────╫───────────────────╢");
    if lines_selected[0] == 1 {
        println!("║{} Ones                    5 {}║{}                {} {}║",
                 style::Invert, style::Reset, style::Invert,
                 scores[0].value, style::Reset);
    } else { println!("║ Ones                    5 ║                {} ║",
                      scores[0].value); }
    if lines_selected[1] == 1 {
        println!("║{} Twos                   10 {}║{}                {} {}║",
                 style::Invert, style::Reset, style::Invert, scores[1].value, style::Reset);
    } else {
        println!("║ Twos                   10 ║                {} ║",
                 scores[1].value); }
    if lines_selected[2] == 1 {
        println!("║{} Threes                 15 {}║{}                {} {}║",
                 style::Invert, style::Reset, style::Invert, scores[2].value, style::Reset);
    } else {
        println!("║ Threes                 15 ║                {} ║",
                 scores[2].value); }
    if lines_selected[3] == 1 {
        println!("║{} Fours                  20 {}║{}                {} {}║",
                 style::Invert, style::Reset, style::Invert, scores[3].value, style::Reset);
    } else {println!("║ Fours                  20 ║                {} ║",
                     scores[3].value); }
    if lines_selected[4] == 1 {
        println!("║{} Fives                  25 {}║{}                {} {}║",
                 style::Invert, style::Reset, style::Invert, scores[4].value, style::Reset);
    } else {
        println!("║ Fives                  25 ║                {} ║",
                 scores[4].value); }
    if lines_selected[5] == 1 {
        println!("║{} Sixes                  30 {}║{}                {} {}║",
                 style::Invert, style::Reset, style::Invert, scores[5].value, style::Reset);
    } else { println!("║ Sixes                  30 ║                {} ║",
                      scores[5].value); }
    println!("╟───────────────────────────╫───────────────────╢");
    if lines_selected[16] == 1 {
        println!("║{} Sum                105 {}║{}                    {} {}║",
                 style::Invert, style::Reset, style::Invert, scores[16].value, style::Reset);
    } else { println!("║ Sum                   105 ║               {} ║",
                      scores[16].value); }
    println!("║ Bonus                  50 ║                {} ║", scores[15].value);
  //println!("╟───────────────────────────╫───────────────────╢");
    if lines_selected[6] == 1 {
        println!("║{} One Pair               12 {}║{}                {} {}║",
             style::Invert, style::Reset, style::Invert, scores[6].value, style::Reset);
    } else { println!("║ One Pair               12 ║                {} ║",
                      scores[6].value); }
    if lines_selected[7] == 1 {
        println!("║{} Two Pairs              22 {}║{}                {} {}║",
                 style::Invert, style::Reset, style::Invert, scores[7].value, style::Reset);
    } else {
        println!("║ Two Pairs              22 ║                {} ║",
                 scores[7].value); }
    if lines_selected[8] == 1 {
        println!("║{} Three of a Kind        18 {}║{}                {} {}║",
                 style::Invert, style::Reset, style::Invert, scores[8].value, style::Reset);
    } else {
        println!("║ Three of a Kind        18 ║                {} ║",
                 scores[8].value);
    }
    if lines_selected[9] == 1 {
        println!("║{} Four of a Kind         24 {}║{}                {} {}║",
                 style::Invert, style::Reset, style::Invert, scores[9].value, style::Reset);
    } else {
        println!("║ Four of a Kind         24 ║                {} ║",
                 scores[9].value);
    }
    if lines_selected[10] == 1 {
        println!("║{} Small Straight         15 {}║{}                {} {}║",
                 style::Invert, style::Reset, style::Invert, scores[10].value, style::Reset);
    } else {
        println!("║ Small Straight         15 ║                {} ║",
                 scores[10].value);
    }
    if lines_selected[11] == 1 {
        println!("║{} Large Straight         20 {}║{}                {} {}║",
                 style::Invert, style::Reset, style::Invert, scores[11].value, style::Reset);
    } else {
        println!("║ Large Straight         20 ║                {} ║",
                 scores[11].value);
    }
    if lines_selected[12] == 1 {
        println!("║{} Full House             28 {}║{}                {} {}║",
                 style::Invert, style::Reset, style::Invert, scores[12].value, style::Reset);
    } else {
        println!("║ Full House             28 ║                {} ║",
                 scores[12].value);
    }
    if lines_selected[13] == 1 {
        println!("║{} Chance                 30 {}║{}                {} {}║",
                 style::Invert, style::Reset, style::Invert, scores[13].value, style::Reset);
    } else {
        println!("║ Chance                 30 ║                {} ║",
                 scores[13].value);
    }
    if lines_selected[14] == 1 {
        println!("║{} Yatzy                  50 {}║{}                {} {}║",
                 style::Invert, style::Reset, style::Invert, scores[14].value, style::Reset);
    } else {
        println!("║ Yatzy                  50 ║                {} ║",
                 scores[14].value);
    }
    println!("╟───────────────────────────╫───────────────────╢");
    println!("║ Total                 374 ║               {} ║", scores[17].value);
    println!("╚═══════════════════════════╩═══════════════════╝");

    scores[17].value.replace_range(.., "   ");
}

fn read_highscore(path: &std::path::PathBuf) -> Vec<(u32, String, String)> {
    let file = File::open(&path).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&path).unwrap_or_else(|error| {
                panic!("Tried to create ~/.config/rusty-yacht/highscore.txt but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the highscore file: {:?}", error);
        }
    });
    let file = BufReader::new(file);
    let mut highscore = Vec::new();
    for line in file.lines() {
        let mut l = line.unwrap();
        let mut name = String::new();
        for c in l.chars() {
            if c == '#' {
                break;
            }
            if c == '|' {
                break;
            }
            name.push(c);
        }
        let date_begin = l.len() - 15;
        let sc_begin = l.len() - 3;
        if name.len() > 0 {
            let date = String::from(&l[date_begin..sc_begin-1]);
            let sc = String::from(&l[sc_begin..]);
            let sc: u32 = sc.trim().parse()
                .expect("Not a number!");
            highscore.push((sc, date, name));
            highscore.sort();
            highscore.reverse();
        }
    }
    highscore
}

fn print_highscore(highscore: &Vec<(u32, String, String)>) {
    println!("╔═══════════════════════════════════════════════╗");
    println!("║ HIGH SCORE                                    ║");
    println!("╠══════════════════════════╦════════════╦═══════╣");
    println!("║ Name                     ║ Date       ║ Score ║");
    println!("╟──────────────────────────╫────────────╫───────╢");

    for elem in highscore {
        match elem.0 {
            0...9   => { println!("║ {}║ {}║     {} ║", elem.2, elem.1, elem.0); },
            10...99 => { println!("║ {}║ {}║    {} ║", elem.2, elem.1, elem.0); },
            _       => { println!("║ {}║ {}║   {} ║", elem.2, elem.1, elem.0); },
        }
    }

    println!("╚══════════════════════════╩════════════╩═══════╝");
}

fn print_dice(dice: &[usize; 5], to_keep: &[usize; 5]) {
    let mut dot: [[char; 15]; 3] = [[' '; 15]; 3];

    for (i, item) in dice.iter().enumerate() {
        match item {
            1 => {
                if to_keep[i] > 0 {
                    match i {
                        0 => dot[1][1] = '●',
                        1 => dot[1][4] = '●',
                        2 => dot[1][7] = '●',
                        3 => dot[1][10] = '●',
                        _ => dot[1][13] = '●',
                    }
                } else {
                    match i {
                        0 => dot[1][1] = '○',
                        1 => dot[1][4] = '○',
                        2 => dot[1][7] = '○',
                        3 => dot[1][10] = '○',
                        _ => dot[1][13] = '○',
                    }
                }
            },
            2 => {
                if to_keep[i] > 0 {
                    match i {
                        0 => { dot[0][1] = '●';
                               dot[2][0] = '●'; },
                        1 => { dot[0][3] = '●';
                               dot[2][2] = '●'; },
                        2 => { dot[0][5] = '●';
                               dot[2][4] = '●'; },
                        3 => { dot[0][7] = '●';
                               dot[2][6] = '●'; },
                        _ => { dot[0][9] = '●';
                               dot[2][8] = '●'; },
                    }
                } else {
                    match i {
                        0 => { dot[0][1] = '○';
                               dot[2][0] = '○'; },
                        1 => { dot[0][3] = '○';
                               dot[2][2] = '○'; },
                        2 => { dot[0][5] = '○';
                               dot[2][4] = '○'; },
                        3 => { dot[0][7] = '○';
                               dot[2][6] = '○'; },
                        _ => { dot[0][9] = '○';
                               dot[2][8] = '○'; },
                    }
                }
            },
            3 => {
                if to_keep[i] > 0 {
                    match i {
                        0 => { dot[0][1] = '●';
                               dot[1][1] = '●';
                               dot[2][0] = '●'; },
                        1 => { dot[0][3] = '●';
                               dot[1][4] = '●';
                               dot[2][2] = '●'; },
                        2 => { dot[0][5] = '●';
                               dot[1][7] = '●';
                               dot[2][4] = '●'; },
                        3 => { dot[0][7] = '●';
                               dot[1][10] = '●';
                               dot[2][6] = '●'; },
                        _ => { dot[0][9] = '●';
                               dot[1][13] = '●';
                               dot[2][8] = '●'; },
                    }
                } else {
                    match i {
                        0 => { dot[0][1] = '○';
                               dot[1][1] = '○';
                               dot[2][0] = '○'; },
                        1 => { dot[0][3] = '○';
                               dot[1][4] = '○';
                               dot[2][2] = '○'; },
                        2 => { dot[0][5] = '○';
                               dot[1][7] = '○';
                               dot[2][4] = '○'; },
                        3 => { dot[0][7] = '○';
                               dot[1][10] = '○';
                               dot[2][6] = '○'; },
                        _ => { dot[0][9] = '○';
                               dot[1][13] = '○';
                               dot[2][8] = '○'; },
                    }
                }
            },
            4 => {
                if to_keep[i] > 0 {
                    match i {
                        0 => { dot[0][0] = '●';
                               dot[0][1] = '●';
                               dot[2][0] = '●';
                               dot[2][1] = '●'; },
                        1 => { dot[0][2] = '●';
                               dot[0][3] = '●';
                               dot[2][2] = '●';
                               dot[2][3] = '●'; },
                        2 => { dot[0][4] = '●';
                               dot[0][5] = '●';
                               dot[2][4] = '●';
                               dot[2][5] = '●'; },
                        3 => { dot[0][6] = '●';
                               dot[0][7] = '●';
                               dot[2][6] = '●';
                               dot[2][7] = '●'; },
                        _ => { dot[0][8] = '●';
                               dot[0][9] = '●';
                               dot[2][8] = '●';
                               dot[2][9] = '●'; },
                    }
                } else {
                    match i {
                        0 => { dot[0][0] = '○';
                               dot[0][1] = '○';
                               dot[2][0] = '○';
                               dot[2][1] = '○'; },
                        1 => { dot[0][2] = '○';
                               dot[0][3] = '○';
                               dot[2][2] = '○';
                               dot[2][3] = '○'; },
                        2 => { dot[0][4] = '○';
                               dot[0][5] = '○';
                               dot[2][4] = '○';
                               dot[2][5] = '○'; },
                        3 => { dot[0][6] = '○';
                               dot[0][7] = '○';
                               dot[2][6] = '○';
                               dot[2][7] = '○'; },
                        _ => { dot[0][8] = '○';
                               dot[0][9] = '○';
                               dot[2][8] = '○';
                               dot[2][9] = '○'; },
                    }
                }
            },
            5 => {
                if to_keep[i] > 0 {
                    match i {
                        0 => { dot[0][0] = '●';
                               dot[0][1] = '●';
                               dot[1][1] = '●';
                               dot[2][0] = '●';
                               dot[2][1] = '●';},
                        1 => { dot[0][2] = '●';
                               dot[0][3] = '●';
                               dot[1][4] = '●';
                               dot[2][2] = '●';
                               dot[2][3] = '●'},
                        2 => { dot[0][4] = '●';
                               dot[0][5] = '●';
                               dot[1][7] = '●';
                               dot[2][4] = '●';
                               dot[2][5] = '●';},
                        3 => { dot[0][6] = '●';
                               dot[0][7] = '●';
                               dot[1][10] = '●';
                               dot[2][6] = '●';
                               dot[2][7] = '●';},
                        _ => { dot[0][8] = '●';
                               dot[0][9] = '●';
                               dot[1][13] = '●';
                               dot[2][8] = '●';
                               dot[2][9] = '●';},
                    }
                } else {
                    match i {
                        0 => { dot[0][0] = '○';
                               dot[0][1] = '○';
                               dot[1][1] = '○';
                               dot[2][0] = '○';
                               dot[2][1] = '○';},
                        1 => { dot[0][2] = '○';
                               dot[0][3] = '○';
                               dot[1][4] = '○';
                               dot[2][2] = '○';
                               dot[2][3] = '○'},
                        2 => { dot[0][4] = '○';
                               dot[0][5] = '○';
                               dot[1][7] = '○';
                               dot[2][4] = '○';
                               dot[2][5] = '○';},
                        3 => { dot[0][6] = '○';
                               dot[0][7] = '○';
                               dot[1][10] = '○';
                               dot[2][6] = '○';
                               dot[2][7] = '○';},
                        _ => { dot[0][8] = '○';
                               dot[0][9] = '○';
                               dot[1][13] = '○';
                               dot[2][8] = '○';
                               dot[2][9] = '○';},
                    }
                }
            },
            _ => {
                if to_keep[i] > 0 {
                    match i {
                        0 => { dot[0][0] = '●';
                               dot[0][1] = '●';
                               dot[1][0] = '●';
                               dot[1][2] = '●';
                               dot[2][0] = '●';
                               dot[2][1] = '●'; },
                        1 => { dot[0][2] = '●';
                               dot[0][3] = '●';
                               dot[1][3] = '●';
                               dot[1][5] = '●';
                               dot[2][2] = '●';
                               dot[2][3] = '●'; },
                        2 => { dot[0][4] = '●';
                               dot[0][5] = '●';
                               dot[1][6] = '●';
                               dot[1][8] = '●';
                               dot[2][4] = '●';
                               dot[2][5] = '●'; },
                        3 => { dot[0][6] = '●';
                               dot[0][7] = '●';
                               dot[1][9] = '●';
                               dot[1][11] = '●';
                               dot[2][6] = '●';
                               dot[2][7] = '●'; },
                        _ => { dot[0][8] = '●';
                               dot[0][9] = '●';
                               dot[1][12] = '●';
                               dot[1][14] = '●';
                               dot[2][8] = '●';
                               dot[2][9] = '●'; },
                    }
                } else {
                    match i {
                        0 => { dot[0][0] = '○';
                               dot[0][1] = '○';
                               dot[1][0] = '○';
                               dot[1][2] = '○';
                               dot[2][0] = '○';
                               dot[2][1] = '○'; },
                        1 => { dot[0][2] = '○';
                               dot[0][3] = '○';
                               dot[1][3] = '○';
                               dot[1][5] = '○';
                               dot[2][2] = '○';
                               dot[2][3] = '○'; },
                        2 => { dot[0][4] = '○';
                               dot[0][5] = '○';
                               dot[1][6] = '○';
                               dot[1][8] = '○';
                               dot[2][4] = '○';
                               dot[2][5] = '○'; },
                        3 => { dot[0][6] = '○';
                               dot[0][7] = '○';
                               dot[1][9] = '○';
                               dot[1][11] = '○';
                               dot[2][6] = '○';
                               dot[2][7] = '○'; },
                        _ => { dot[0][8] = '○';
                               dot[0][9] = '○';
                               dot[1][12] = '○';
                               dot[1][14] = '○';
                               dot[2][8] = '○';
                               dot[2][9] = '○'; },
                    }
                }
            }
        }
    }

    println!("╔═══════╗ ╔═══════╗ ╔═══════╗ ╔═══════╗ ╔═══════╗");
    println!("║ {}   {} ║ ║ {}   {} ║ ║ {}   {} ║ ║ {}   {} ║ ║ {}   {} ║",
             dot[0][0], dot[0][1], dot[0][2], dot[0][3], dot[0][4],
             dot[0][5], dot[0][6], dot[0][7], dot[0][8], dot[0][9]);
    println!("║ {} {} {} ║ ║ {} {} {} ║ ║ {} {} {} ║ ║ {} {} {} ║ ║ {} {} {} ║",
             dot[1][0], dot[1][1], dot[1][2], dot[1][3], dot[1][4],
             dot[1][5], dot[1][6], dot[1][7], dot[1][8], dot[1][9],
             dot[1][10], dot[1][11], dot[1][12], dot[1][13], dot[1][14]);
    println!("║ {}   {} ║ ║ {}   {} ║ ║ {}   {} ║ ║ {}   {} ║ ║ {}   {} ║",
             dot[2][0], dot[2][1], dot[2][2], dot[2][3], dot[2][4],
             dot[2][5], dot[2][6], dot[2][7], dot[2][8], dot[2][9]);
    println!("╚═══════╝ ╚═══════╝ ╚═══════╝ ╚═══════╝ ╚═══════╝");
}

fn ones(dice: &[usize; 5]) -> Option<usize> {
    let mut value: usize = 0;
    for i in dice {
        if i == &1 {
            value += i;
        }
    }
    if value > 0 {
        return Some(value);
    }
    None
}

fn twos(dice: &[usize; 5]) -> Option<usize> {
    let mut value: usize = 0;
    for i in dice {
        if i == &2 {
            value += i;
        }
    }

    if value > 1 {
        return Some(value);
    }
    None
}

fn threes(dice: &[usize; 5]) -> Option<usize> {
    let mut value: usize = 0;
    for i in dice {
        if i == &3 {
            value += i;
        }
    }

    if value > 2 {
        return Some(value);
    }
    None
}

fn fours(dice: &[usize; 5]) -> Option<usize> {
    let mut value: usize = 0;
    for i in dice {
        if i == &4 {
            value += i;
        }
    }

    if value > 3 {
        return Some(value);
    }
    None
}

fn fives(dice: &[usize; 5]) -> Option<usize> {
    let mut value: usize = 0;
    for i in dice {
        if i == &5 {
            value += i;
        }
    }
    if value > 4 {
        return Some(value);
    }
    None
}

fn sixes(dice: &[usize; 5]) -> Option<usize> {
    let mut value: usize = 0;
    for i in dice {
        if i == &6 {
            value += i;
        }
    }
    if value > 5 {
        return Some(value);
    }
    None
}

fn one_pair(dice: &[usize; 5]) -> Option<usize> {
    let mut dice_str: [usize; 5] = dice.clone();
    dice_str.sort();
    let mut value: usize = 0;
    if dice_str[0] == dice_str[1] {
        value += dice_str[0] + dice_str[1];
    }
    if dice_str[1] == dice_str[2] {
        if dice_str[1] + dice_str[2] > value {
            value = dice_str[1] + dice_str[2]
        }
    }
    if dice_str[2] == dice_str[3] {
        if dice_str[2] + dice_str[3] > value {
            value = dice_str[2] + dice_str[3]
        }
    }
    if dice_str[3] == dice_str[4] {
        if dice_str[3] + dice_str[4] > value {
            value = dice_str[3] + dice_str[4]
        }
    }

    if value > 1 {
        return Some(value);
    }

    None
}

fn two_pairs(dice: &[usize; 5]) -> Option<usize> {
    let mut dice_str: [usize; 5] = dice.clone();
    dice_str.sort();
    let mut value_fst: usize = 0;
    let mut value_snd: usize = 0;
    if dice_str[0] == dice_str[1] {
        value_fst = dice_str[0] + dice_str[1];
    } else if dice_str[1] == dice_str[2] {
        value_fst = dice_str[1] + dice_str[2];
    }

    if dice_str[2] == dice_str[3] {
        value_snd = dice_str[2] + dice_str[3];
    } else if dice_str[3] == dice_str[4] {
        value_snd = dice_str[3] + dice_str[4];
    }

    if value_fst > 1 && value_snd > 1 &&
        value_fst != value_snd {
            return Some(value_fst + value_snd);
        }
    None
}

fn three_kind(dice: &[usize; 5]) -> Option<usize> {
    let mut dice_str: [usize; 5] = dice.clone();
    dice_str.sort();
    let mut value: usize = 0;
    if dice_str[0] == dice_str[1] && dice_str[1] == dice_str[2] {
        for (i, die) in dice_str.iter().enumerate() {
            if i < 3 {
                value += die;
            }
        }
        return Some(value);

    } else if dice_str[1] == dice_str[2] && dice_str[2] == dice_str[3] {
        for (i, die) in dice_str.iter().enumerate() {
            if i > 0 && i < 4 {
                value += die;
            }
        }
        return Some(value);

    } else if dice_str[2] == dice_str[3] && dice_str[3] == dice_str[4] {
        for (i, die) in dice_str.iter().enumerate() {
            if i > 1 {
                value += die;
            }
        }
        return Some(value);
    }
    None
}

fn four_kind(dice: &[usize; 5]) -> Option<usize> {
    let mut dice_str: [usize; 5] = dice.clone();
    dice_str.sort();
    let mut value: usize = 0;
    if dice_str[0] == dice_str[1] &&
        dice_str[1] == dice_str[2] &&
        dice_str[2] == dice_str[3] {
            for (i, die) in dice_str.iter().enumerate() {
                if i < 4 {
                    value += die;
                }
            }
            return Some(value);
        } else if dice_str[1] == dice_str[2] &&
        dice_str[2] == dice_str[3] &&
        dice_str[3] == dice_str[4] {
            for (i, die) in dice_str.iter().enumerate() {
                if i > 0 {
                    value += die;
                }
            }
            return Some(value);
        }
    None
}

fn small_str(dice: &[usize; 5]) -> Option<usize> {
    let mut dice_str: [usize; 5] = dice.clone();
    dice_str.sort();
    if dice_str[0] == 1 &&
        dice_str[1] == 2 &&
        dice_str[2] == 3 &&
        dice_str[3] == 4 &&
        dice_str[4] == 5 {
            return Some(15);
        }
    None
}

fn large_str(dice: &[usize; 5]) -> Option<usize> {
    let mut dice_str: [usize; 5] = dice.clone();
    dice_str.sort();
    if dice_str[0] == 2 &&
        dice_str[1] == 3 &&
        dice_str[2] == 4 &&
        dice_str[3] == 5 &&
        dice_str[4] == 6 {
            return Some(20);
        }
    None
}

fn full_house(dice: &[usize; 5]) -> Option<usize> {
    let mut dice_str: [usize; 5] = dice.clone();
    dice_str.sort();
    if ((dice_str[0] == dice_str[1] && dice_str[1] == dice_str[2]) &&
        (dice_str[3] == dice_str[4])) ||
        ((dice_str[0] == dice_str[1]) &&
         (dice_str[2] == dice_str[3] && dice_str[3] == dice_str[4])) &&
        ((dice_str[0] == dice_str[1]
         && dice_str[1] == dice_str[2]
         && dice_str[2] == dice_str[3]) == false) {
            let value = dice.iter().fold(0,|a, &b| a + b);
            return Some(value);
        }
    None
}

fn chance(dice: &[usize; 5]) -> Option<usize> {
    let value = dice.iter().fold(0,|a, &b| a + b);
    Some(value)
}

fn yatzy(dice: &[usize; 5]) -> Option<usize> {
    for (i, &_item) in dice.iter().enumerate() {
        if dice[i] != dice[0] {
            return None;
        }
    }
    Some(50)
}

fn select_dice(dice: &[usize; 5],
               to_keep: &mut [usize; 5],
               left_margin: &mut String,
               margin_width: &mut usize,
               selected: &mut [usize; 5]) -> DiceSelectStatus {
    // Enter raw mode
    // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();

    let term_size = match termion::terminal_size() {
        Ok(num) => num,
        Err(_) => (151, 38), // In case of error, set this value
    };

    let bottom_line = term_size.1;

    // Flush stdout (i.e. make the output appear).
    stdout.flush().unwrap();
    println!("{}●━━━━━━━●{}", *left_margin, termion::cursor::Goto(1, bottom_line -4));

    for c in stdin.keys() {
        // Clear the current line.
        write!(stdout, "{}{}", termion::cursor::Goto(1, bottom_line), termion::clear::CurrentLine).unwrap();

        match c.unwrap() {
            Key::Ctrl(c) => { if c == 'c' { return DiceExit; } },
            Key::Char('\n') => { selected[*margin_width] = 1;
                                 return DiceComplete;},
            Key::Char(' ')  => { println!("{}●━━━━━━━●{}", *left_margin, termion::cursor::Goto(1, bottom_line -4));
                                 if selected[*margin_width] == 0 && to_keep[*margin_width] == 0 {
                                     to_keep[*margin_width] = dice[*margin_width];
                                     selected[*margin_width] = 1;
                                     return DiceIncomplete;
                                 } else {
                                     to_keep[*margin_width] = 0;
                                     selected[*margin_width] = 0;
                                     return DiceIncomplete;
                                 } },
            Key::Left      => { if *margin_width > 0 { *margin_width = *margin_width -1};
                                if *margin_width == 0 {
                                    *left_margin = "".to_string();
                                } else if *margin_width == 1 {
                                    *left_margin = "          ".to_string();
                                } else if *margin_width == 2 {
                                    *left_margin = "                    ".to_string();
                                } else if *margin_width == 3 {
                                    *left_margin = "                              ".to_string();
                                } else if *margin_width == 4 {
                                    *left_margin = "                                        ".to_string();
                                }
                                println!("{}●━━━━━━━●{}", *left_margin, termion::cursor::Goto(1, bottom_line -4)); },
            Key::Down      => { if *margin_width > 0 { *margin_width = *margin_width -1};
                                if *margin_width == 0 {
                                    *left_margin = "".to_string();
                                } else if *margin_width == 1 {
                                    *left_margin = "          ".to_string();
                                } else if *margin_width == 2 {
                                    *left_margin = "                    ".to_string();
                                } else if *margin_width == 3 {
                                    *left_margin = "                              ".to_string();
                                } else if *margin_width == 4 {
                                    *left_margin = "                                        ".to_string();
                                }
                                println!("{}●━━━━━━━●{}", *left_margin, termion::cursor::Goto(1, bottom_line -4)); },
            Key::Right     => { if *margin_width < 4 { *margin_width += 1};
                                if *margin_width == 0 {
                                    *left_margin = "".to_string();
                                } else if *margin_width == 1 {
                                    *left_margin = "          ".to_string();
                                } else if *margin_width == 2 {
                                    *left_margin = "                    ".to_string();
                                } else if *margin_width == 3 {
                                    *left_margin = "                              ".to_string();
                                } else if *margin_width == 4 {
                                    *left_margin = "                                        ".to_string();
                                }
                                println!("{}●━━━━━━━●{}", *left_margin, termion::cursor::Goto(1, bottom_line -4)); },
            Key::Up     => { if *margin_width < 4 { *margin_width += 1};
                             if *margin_width == 0 {
                                 *left_margin = "".to_string();
                             } else if *margin_width == 1 {
                                 *left_margin = "          ".to_string();
                             } else if *margin_width == 2 {
                                 *left_margin = "                    ".to_string();
                             } else if *margin_width == 3 {
                                 *left_margin = "                              ".to_string();
                             } else if *margin_width == 4 {
                                 *left_margin = "                                        ".to_string();
                             }
                             println!("{}●━━━━━━━●{}", *left_margin, termion::cursor::Goto(1, bottom_line -4)); },
            _              => continue,
        }

        // Flush again.
        stdout.flush().unwrap();
    }

    // Show the cursor again before we exit.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    DiceIncomplete
}

fn select_slot(dice: &[usize; 5],
               value: [fn(&[usize; 5]) -> Option<usize>; 15],
               scores: &mut [Score],
               lines_selected: &mut [u8],
               i: &mut usize) -> SlotSelectStatus {

    // Enter raw mode
    // Get the standard input stream.
    let stdin = stdin();
    // Get the standard output stream and go to raw mode.
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    // Flush stdout (i.e. make the output appear).
    stdout.flush().unwrap();

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl(c) => { if c == 'c' { return SlotExit; } },
            Key::Char('\n') => { if scores[*i].value != "  ".to_string()
                                 { return SlotAlreadySelected;
                                 } else if value[*i](dice) != None {
                                     println!("value[*i](dice) != None");
                                     if value[*i](dice).unwrap() > 9 {
                                         scores[*i].value.replace_range(.., &value[*i](&dice).unwrap().to_string());
                                     } else {
                                         scores[*i].value.replace_range(1.., &value[*i](&dice).unwrap().to_string());
                                     }
                                     scores[*i].selected = true;
                                     stdout.flush().unwrap();
                                     return SlotComplete;
                                 } else {
                                     scores[*i].selected = true;
                                     stdout.flush().unwrap();
                                     return SlotInvalid;
                                 }
            },
            Key::Char('-') => { if scores[*i].value == "  ".to_string() {
                scores[*i].selected = true;
                scores[*i].value.replace_range(1.., "–");
                stdout.flush().unwrap();
                return SlotComplete;
            } },
            Key::Up      => { if i > &mut 0
                              {
                                  *i -= 1;
                                  lines_selected[*i] = 1;
                                  lines_selected[*i+1] = 0;
                                  return SlotIncomplete;
                              } else {
                                  *i = 14;
                                  lines_selected[*i] = 1;
                                  lines_selected[0] = 0;
                                  return SlotIncomplete;
                              } },
            Key::Left      => { if *i > 0
                                {
                                    *i -= 1;
                                    lines_selected[*i] = 1;
                                    lines_selected[*i+1] = 0;
                                    return SlotIncomplete;
                                } else {
                                    *i = 14;
                                    lines_selected[*i] = 1;
                                    lines_selected[0] = 0;
                                    return SlotIncomplete;
                                } },
            Key::Down      => { if *i < 14
                                {
                                    *i += 1;
                                    lines_selected[*i] = 1;
                                    lines_selected[*i-1] = 0;
                                    return SlotIncomplete;
                                } else {
                                    *i = 0;
                                    lines_selected[*i] = 1;
                                    lines_selected[14] = 0;
                                    return SlotIncomplete;
                                } },
            Key::Right      => { if *i < 14
                                 {
                                     *i += 1;
                                     lines_selected[*i] = 1;
                                     lines_selected[*i-1] = 0;
                                     return SlotIncomplete;
                                 } else {
                                     *i = 0;
                                     lines_selected[*i] = 1;
                                     lines_selected[14] = 0;
                                     return SlotIncomplete;
                                 } },
            _              => continue,
        }
    }

    // Flush again.
    stdout.flush().unwrap();

    // Show the cursor again before we exit.
    write!(stdout, "{}", termion::cursor::Show).unwrap();
    SlotIncomplete
}
