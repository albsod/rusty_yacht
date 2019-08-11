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

extern crate chrono;
extern crate rand;
extern crate termion;

use rand::Rng;
use std::io::{Write, stdout, stdin, BufRead, BufReader, ErrorKind};
use std::fs::*;
use std::fs::create_dir_all;
use std::path::PathBuf;
use chrono::prelude::*;
use termion::clear;
use termion::event::Key;
use termion::raw::IntoRawMode;
use termion::style;
use termion::input::TermRead;

pub enum DiceSelectStatus {
    Complete,
    Exit,
    Incomplete,
}

pub enum SlotSelectStatus {
    AlreadySelected,
    Exit,
    Invalid,
    Complete,
    Incomplete,
}

pub struct ScoreValidator;

impl ScoreValidator {
    pub fn new() -> [fn(&[usize; 5]) -> Option<usize>; 15] {
        fn ones(current: &[usize; 5]) -> Option<usize> {
            let mut value: usize = 0;
            for i in current {
                if i == &1 {
                    value += i;
                }
            }
            if value > 0 {
                return Some(value);
            }
            None
        }

        fn twos(current: &[usize; 5]) -> Option<usize> {
            let mut value: usize = 0;
            for i in current {
                if i == &2 {
                    value += i;
                }
            }

            if value > 1 {
                return Some(value);
            }
            None
        }

        fn threes(current: &[usize; 5]) -> Option<usize> {
            let mut value: usize = 0;
            for i in current {
                if i == &3 {
                    value += i;
                }
            }

            if value > 2 {
                return Some(value);
            }
            None
        }

        fn fours(current: &[usize; 5]) -> Option<usize> {
            let mut value: usize = 0;
            for i in current {
                if i == &4 {
                    value += i;
                }
            }

            if value > 3 {
                return Some(value);
            }
            None
        }

        fn fives(current: &[usize; 5]) -> Option<usize> {
            let mut value: usize = 0;
            for i in current {
                if i == &5 {
                    value += i;
                }
            }
            if value > 4 {
                return Some(value);
            }
            None
        }

        fn sixes(current: &[usize; 5]) -> Option<usize> {
            let mut value: usize = 0;
            for i in current {
                if i == &6 {
                    value += i;
                }
            }
            if value > 5 {
                return Some(value);
            }
            None
        }

        fn one_pair(current: &[usize; 5]) -> Option<usize> {
            let mut dice_str: [usize; 5] = current.clone();
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

        fn two_pairs(current: &[usize; 5]) -> Option<usize> {
            let mut dice_str: [usize; 5] = current.clone();
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

        fn three_kind(current: &[usize; 5]) -> Option<usize> {
            let mut dice_str: [usize; 5] = current.clone();
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

        fn four_kind(current: &[usize; 5]) -> Option<usize> {
            let mut dice_str: [usize; 5] = current.clone();
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

        fn small_str(current: &[usize; 5]) -> Option<usize> {
            let mut dice_str: [usize; 5] = current.clone();
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

        fn large_str(current: &[usize; 5]) -> Option<usize> {
            let mut dice_str: [usize; 5] = current.clone();
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

        fn full_house(current: &[usize; 5]) -> Option<usize> {
            let mut dice_str: [usize; 5] = current.clone();
            dice_str.sort();
            if ((dice_str[0] == dice_str[1] && dice_str[1] == dice_str[2]) &&
                (dice_str[3] == dice_str[4])) ||
                ((dice_str[0] == dice_str[1]) &&
                 (dice_str[2] == dice_str[3] && dice_str[3] == dice_str[4])) &&
                ((dice_str[0] == dice_str[1]
                  && dice_str[1] == dice_str[2]
                  && dice_str[2] == dice_str[3]) == false) {
                    let value = current.iter().fold(0,|a, &b| a + b);
                    return Some(value);
                }
            None
        }

        fn chance(current: &[usize; 5]) -> Option<usize> {
            let value = current.iter().fold(0,|a, &b| a + b);
            Some(value)
        }

        fn yatzy(current: &[usize; 5]) -> Option<usize> {
            for (i, &_item) in current.iter().enumerate() {
                if current[i] != current[0] {
                    return None;
                }
            }
            Some(50)
        }

        [ones, twos, threes, fours, fives, sixes, one_pair,
         two_pairs, three_kind, four_kind, small_str,
         large_str, full_house, chance, yatzy]
    }
}

struct Score {
    name: &'static str,
    value: String,
    selected: bool,
    highlighted: u8,
}

pub struct Scores {
    scores: [Score; 18],
}

impl Scores {
    pub fn new() -> Scores {
        // initialize score values
        let score_ones = Score {
            name: "Ones", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_twos = Score {
            name: "Twos", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_threes = Score {
            name: "Threes", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_fours = Score {
            name: "Fours", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_fives = Score {
            name: "Fives", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_sixes = Score {
            name: "Sixes", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_bonus = Score {
            name: "Bonus", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_one_pair = Score {
            name: "One Pair", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_two_pairs = Score {
            name: "Two Pairs", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_three_kind = Score {
            name: "Three of a Kind", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_four_kind = Score {
            name: "Four of a Kind", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_small_str = Score {
            name: "Small Straight", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_large_str = Score {
            name: "Large Straight", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_full_house = Score {
            name: "Full House", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_chance = Score {
            name: "Chance", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_yatzy = Score {
            name: "Yatzy", value: "  ".to_string(), selected: false, highlighted: 0
        };
        let score_subtotal = Score {
            name: "Sum", value: "   ".to_string(), selected: false, highlighted: 0
        };
        let score_total = Score {
            name: "Total", value: "   ".to_string(), selected: false, highlighted: 0
        };

        // Return the score values as a Scores struct
        Scores {scores: [score_ones, score_twos, score_threes, score_fours,
                         score_fives, score_sixes, score_one_pair, score_two_pairs,
                         score_three_kind, score_four_kind, score_small_str,
                         score_large_str, score_full_house, score_chance,
                         score_yatzy, score_bonus, score_subtotal, score_total]}
    }

    pub fn print(&mut self) {

        let mut subtotal: u8 = 0;
        let mut total: u16 = 0;

        for elm in self.scores.iter() {
            if elm.name == "One Pair" {
                break;
            } else if elm.value != "  " && elm.value != "   " && elm.value != " –" {
                subtotal += elm.value.trim().parse::<u8>().unwrap();
            }
        }

        if subtotal > 99 {
            self.scores[16].value.replace_range(.., &subtotal.to_string());
        } else if subtotal > 9 {
            self.scores[16].value.replace_range(1.., &subtotal.to_string());
        } else if subtotal > 0 {
            self.scores[16].value.replace_range(2.., &subtotal.to_string());
        }

        if subtotal > 62 {
            self.scores[15].value.replace_range(.., "50");
        } else if
            self.scores[0].value != "  " &&
            self.scores[1].value != "  " &&
            self.scores[2].value != "  " &&
            self.scores[3].value != "  " &&
            self.scores[4].value != "  " &&
            self.scores[5].value != "  "
        {
            self.scores[15].value.replace_range(1.., "–");
        }

        for elm in self.scores.iter() {
            if elm.value != "  " &&
                elm.value != "   " &&
                elm.value != " –" &&
                elm.name != "Sum" {
                    total += elm.value.trim().parse::<u16>().unwrap();
                }
        }

        if total > 99 {
            self.scores[17].value.replace_range(.., &total.to_string());
        } else if total > 9 {
            self.scores[17].value.replace_range(1.., &total.to_string());
        } else if total > 0 {
            self.scores[17].value.replace_range(2.., &total.to_string());
        }

        println!("╔═══════════════════════════════════════════════╗");
        println!("║ RUSTY YACHT                                   ║");
        println!("╠═══════════════════════════╦═══════════════════╣");
        println!("║                       Max ║             Score ║");
        println!("╟───────────────────────────╫───────────────────╢");
        if self.scores[0].highlighted == 1 {
            println!("║{} Ones                    5 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert,
                     self.scores[0].value, style::Reset);
        } else { println!("║ Ones                    5 ║                {} ║",
                          self.scores[0].value); }
        if self.scores[1].highlighted == 1 {
            println!("║{} Twos                   10 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self.scores[1].value, style::Reset);
        } else {
            println!("║ Twos                   10 ║                {} ║",
                     self.scores[1].value); }
        if self.scores[2].highlighted == 1 {
            println!("║{} Threes                 15 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self.scores[2].value, style::Reset);
        } else {
            println!("║ Threes                 15 ║                {} ║",
                     self.scores[2].value); }
        if self.scores[3].highlighted == 1 {
            println!("║{} Fours                  20 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self.scores[3].value, style::Reset);
        } else {println!("║ Fours                  20 ║                {} ║",
                         self.scores[3].value); }
        if self.scores[4].highlighted == 1 {
            println!("║{} Fives                  25 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self.scores[4].value, style::Reset);
        } else {
            println!("║ Fives                  25 ║                {} ║",
                     self.scores[4].value); }
        if self.scores[5].highlighted == 1 {
            println!("║{} Sixes                  30 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self.scores[5].value, style::Reset);
        } else { println!("║ Sixes                  30 ║                {} ║",
                          self.scores[5].value); }
        println!("╟───────────────────────────╫───────────────────╢");
        if self.scores[16].highlighted == 1 {
            println!("║{} Sum                105 {}║{}                    {} {}║",
                     style::Invert, style::Reset, style::Invert, self.scores[16].value, style::Reset);
        } else { println!("║ Sum                   105 ║               {} ║",
                          self.scores[16].value); }
        println!("║ Bonus                  50 ║                {} ║", self.scores[15].value);
        //println!("╟───────────────────────────╫───────────────────╢");
        if self.scores[6].highlighted == 1 {
            println!("║{} One Pair               12 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self.scores[6].value, style::Reset);
        } else { println!("║ One Pair               12 ║                {} ║",
                          self.scores[6].value); }
        if self.scores[7].highlighted == 1 {
            println!("║{} Two Pairs              22 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self.scores[7].value, style::Reset);
        } else {
            println!("║ Two Pairs              22 ║                {} ║",
                     self.scores[7].value); }
        if self.scores[8].highlighted == 1 {
            println!("║{} Three of a Kind        18 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self.scores[8].value, style::Reset);
        } else {
            println!("║ Three of a Kind        18 ║                {} ║",
                     self.scores[8].value);
        }
        if self.scores[9].highlighted == 1 {
            println!("║{} Four of a Kind         24 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self.scores[9].value, style::Reset);
        } else {
            println!("║ Four of a Kind         24 ║                {} ║",
                     self.scores[9].value);
        }
        if self.scores[10].highlighted == 1 {
            println!("║{} Small Straight         15 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self.scores[10].value, style::Reset);
        } else {
            println!("║ Small Straight         15 ║                {} ║",
                     self.scores[10].value);
        }
        if self.scores[11].highlighted == 1 {
            println!("║{} Large Straight         20 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self.scores[11].value, style::Reset);
        } else {
            println!("║ Large Straight         20 ║                {} ║",
                     self.scores[11].value);
        }
        if self.scores[12].highlighted == 1 {
            println!("║{} Full House             28 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self.scores[12].value, style::Reset);
        } else {
            println!("║ Full House             28 ║                {} ║",
                     self.scores[12].value);
        }
        if self.scores[13].highlighted == 1 {
            println!("║{} Chance                 30 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self.scores[13].value, style::Reset);
        } else {
            println!("║ Chance                 30 ║                {} ║",
                     self.scores[13].value);
        }
        if self.scores[14].highlighted == 1 {
            println!("║{} Yatzy                  50 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self.scores[14].value, style::Reset);
        } else {
            println!("║ Yatzy                  50 ║                {} ║",
                     self.scores[14].value);
        }
        println!("╟───────────────────────────╫───────────────────╢");
        println!("║ Total                 374 ║               {} ║", self.scores[17].value);
        println!("╚═══════════════════════════╩═══════════════════╝");

        self.scores[17].value.replace_range(.., "   ");
    }

    pub fn place_points(&mut self, validators: [fn(&[usize; 5]) -> Option<usize>; 15],
                        dice: &Dice, lp: &mut bool) {
        let mut i = 0 as usize;
        loop {
            self.scores[i].highlighted = 1;
            self.print();
            Dice::print(&dice);

            match self.select_slot(&dice, validators, &mut i) {
                SlotSelectStatus::Exit => { *lp = false;
                                             break;},
                SlotSelectStatus::AlreadySelected => { println!("{}", clear::All);
                                                       println!("  Sorry, you can't use this slot again.");
                                                       println!("  Press Enter to continue.");
                                                       self.print();
                                                       Dice::print(&dice);
                                                       let stdin = stdin();
                                                       let mut stdout = stdout().into_raw_mode().unwrap();
                                                       write!(stdout, "{}", termion::cursor::Hide).unwrap();
                                                       for c in stdin.keys() {
                                                           match c.unwrap() {
                                                               Key::Ctrl(c) => if c == 'c' {
                                                                   *lp = false;
                                                                   break;
                                                               },
                                                               Key::Char('\n') => {
                                                                   println!("{}", clear::All);
                                                                   break;
                                                               },
                                                               Key::Up => {
                                                                   if i != 0 {
                                                                       i -= 1;
                                                                       self.scores[i+1].highlighted = 0;
                                                                   } else {
                                                                       i = 14;
                                                                       self.scores[0].highlighted = 0;
                                                                   }
                                                                   self.scores[i].highlighted = 1;
                                                                   println!("{}", clear::All);
                                                                   break;
                                                               },
                                                               Key::Down => {
                                                                   if i != 14 {
                                                                       i += 1;
                                                                       self.scores[i-1].highlighted = 0;
                                                                   } else {
                                                                       i = 0;
                                                                       self.scores[14].highlighted = 0;
                                                                   }
                                                                   self.scores[i].highlighted = 1;
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

                SlotSelectStatus::Complete  => { println!("{}", clear::All);
                                                 println!("  Selection complete. Press Enter to continue.");
                                                 self.print();
                                                 Dice::print(&dice);
                                                 let stdin = stdin();
                                                 let mut stdout = stdout().into_raw_mode().unwrap();
                                                 write!(stdout, "{}", termion::cursor::Hide).unwrap();
                                                 for c in stdin.keys() {
                                                     match c.unwrap() {
                                                         Key::Ctrl(c) => if c == 'c' {
                                                             *lp = false;
                                                             break;
                                                         },
                                                         Key::Char('\n') => break,
                                                         _ => continue,
                                                     }
                                                 }
                                                 stdout.flush().unwrap();
                                                 break; },
                SlotSelectStatus::Incomplete => { println!("{}", clear::All); },
                SlotSelectStatus::Invalid => {
                    println!("{}", clear::All);
                    println!("  Invalid selection. Press - to strike it out");
                    println!("  or an arrow key to cancel.");
                },
            }
        }
        self.scores[i].highlighted = 0;
        println!("{}", clear::All);
    }
    pub fn is_final(&self) -> bool {
        for item in self.scores.iter() {
            if item.name == "Bonus" {
                continue;
            }
            if item.value == "  " {
                return false;
            }
        }
        true
    }
    fn select_slot(&mut self, dice: &Dice,
                   validators: [fn(&[usize; 5]) -> Option<usize>; 15],
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
                Key::Ctrl(c) => {
                    if c == 'c' {
                        return SlotSelectStatus::Exit;
                    }
                },
                Key::Char('\n') => {
                    if self.scores[*i].value != "  ".to_string() {
                        return SlotSelectStatus::AlreadySelected;
                    } else if validators[*i](&dice.current) != None {
                        //println!("validators[*i](dice) != None"); // ???
                        if validators[*i](&dice.current).unwrap() > 9 {
                            self.scores[*i].value.replace_range(.., &validators[*i](&dice.current).unwrap().to_string());
                        } else {
                            self.scores[*i].value.replace_range(1.., &validators[*i](&dice.current).unwrap().to_string());
                        }
                        self.scores[*i].selected = true;
                        stdout.flush().unwrap();
                        return SlotSelectStatus::Complete;
                    } else {
                        self.scores[*i].selected = true;
                        stdout.flush().unwrap();
                        return SlotSelectStatus::Invalid;
                    }
                },
                Key::Char('-') => {
                    if self.scores[*i].value == "  ".to_string() {
                        self.scores[*i].selected = true;
                        self.scores[*i].value.replace_range(1.., "–");
                        stdout.flush().unwrap();
                        return SlotSelectStatus::Complete;
                    }
                },
                Key::Up => {
                    if i > &mut 0 {
                        *i -= 1;
                        self.scores[*i].highlighted = 1;
                        self.scores[*i+1].highlighted = 0;
                        return SlotSelectStatus::Incomplete;
                    } else {
                        *i = 14;
                        self.scores[*i].highlighted = 1;
                        self.scores[0].highlighted = 0;
                        return SlotSelectStatus::Incomplete;
                    }
                },
                Key::Left => {
                    if *i > 0 {
                        *i -= 1;
                        self.scores[*i].highlighted = 1;
                        self.scores[*i+1].highlighted = 0;
                        return SlotSelectStatus::Incomplete;
                    } else {
                        *i = 14;
                        self.scores[*i].highlighted = 1;
                        self.scores[0].highlighted = 0;
                        return SlotSelectStatus::Incomplete;
                    }
                },
                Key::Down => {
                    if *i < 14 {
                        *i += 1;
                        self.scores[*i].highlighted = 1;
                        self.scores[*i-1].highlighted = 0;
                        return SlotSelectStatus::Incomplete;
                    } else {
                        *i = 0;
                        self.scores[*i].highlighted = 1;
                        self.scores[14].highlighted = 0;
                        return SlotSelectStatus::Incomplete;
                    }
                },
                Key::Right => {
                    if *i < 14 {
                        *i += 1;
                        self.scores[*i].highlighted = 1;
                        self.scores[*i-1].highlighted = 0;
                        return SlotSelectStatus::Incomplete;
                    } else {
                        *i = 0;
                        self.scores[*i].highlighted = 1;
                        self.scores[14].highlighted = 0;
                        return SlotSelectStatus::Incomplete;
                    }
                },
                _ => continue,
            }
        }

        // Flush again.
        stdout.flush().unwrap();

        // Show the cursor again before we exit.
        write!(stdout, "{}", termion::cursor::Show).unwrap();
        SlotSelectStatus::Incomplete
    }


}


pub struct Highscore;

impl Highscore {

    pub fn new_path() -> PathBuf {
        let home = dirs::home_dir().expect("No home directory!");
        let mut path = PathBuf::new();
        path.push(home);
        path.push(".config");
        path.push("rusty-yacht");
        create_dir_all(&path).expect("Could not create ~/.config/rusty-yacht");
        path.push("highscore");
        path
    }

    pub fn new(path: &std::path::PathBuf) -> Vec<(u32, String, String)> {
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
            let l = line.unwrap();
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
            let score_begin = l.len() - 3;
            if name.len() > 0 {
                let date = String::from(&l[date_begin..score_begin-2]);
                let score = String::from(&l[score_begin..]);
                let score: u32 = score.trim().parse()
                    .expect("Not a number!");
                highscore.push((score, date, name));
                highscore.sort();
                highscore.reverse();
            }
        }
        highscore
    }

    pub fn print(highscore: &Vec<(u32, String, String)>) {
        println!("╔═══════════════════════════════════════════════╗");
        println!("║ HIGH-SCORE TABLE                              ║");
        println!("╠══════════════════════════╦════════════╦═══════╣");
        println!("║ Name                     ║ Date       ║ Score ║");
        println!("╟──────────────────────────╫────────────╫───────╢");

        for elem in highscore {
            match elem.0 {
                0..=9   => { println!("║ {} ║ {} ║     {} ║", elem.2, elem.1, elem.0); },
                10..=99 => { println!("║ {} ║ {} ║    {} ║", elem.2, elem.1, elem.0); },
                _       => { println!("║ {} ║ {} ║   {} ║", elem.2, elem.1, elem.0); },
            }
        }

        println!("╚══════════════════════════╩════════════╩═══════╝");
    }

    pub fn log(path: &std::path::PathBuf, scores: Scores) {
        let mut name = String::new();
        loop {
            println!("Input a name to log your score:");
            stdin().read_line(&mut name)
                .expect("Failed to read line");
            name.pop(); // remove trailing newline
            let namelen = name.chars().count();
            if namelen < 1 {
                ;
            } else if namelen > 24 {
                println!("Too long! Max length is 24 characters.\n");
                name = String::from("");
            } else {
                let name_padding = 24 - namelen;
                for _ in 0..name_padding {
                    name.push(' ');
                }
                break;
            }
        }
        let mut date = Local::now().date().to_string();
        date.truncate(10);

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
        let mut total: u16 = 0;

        for elm in scores.scores.iter() {
            if elm.value != "  " &&
                elm.value != "   " &&
                elm.value != " –" &&
                elm.name != "Sum" {
                    total += elm.value.trim().parse::<u16>().unwrap();
                }
        }

        match total {
            0..=9 => { if let Err(e) = writeln!(file, "{}| {} |  {:?}", name, date, total) {
                eprintln!("Couldn't write to file: {}", e); }
            },
            10..=99 => { if let Err(e) = writeln!(file, "{}| {} | {:?}", name, date, total) {
                eprintln!("Couldn't write to file: {}", e); }
            },
            _ => { if let Err(e) = writeln!(file, "{}| {} |{:?}", name, date, total) {
                eprintln!("Couldn't write to file: {}", e); }
            },
        }
    }
}

pub struct Dice {
    pub current: [usize; 5],
    pub to_keep: [usize; 5],
}

impl Dice {
    pub fn new() -> Dice {
        Dice {
            current: [0,0,0,0,0],
            to_keep: [0,0,0,0,0],
        }
    }

    pub fn keep_all(&self) -> bool {
        for &i in self.to_keep.iter() {
            if i == 0 {
                return false;
            }
        }
        true
    }

    pub fn roll(&mut self) {
        for (i, &item) in self.to_keep.iter().enumerate() {
            if item == 0 as usize {
                self.current[i] = rand::thread_rng().gen_range(1, 7);
            } else {
                self.current[i] = item;
            }
        }
    }

    pub fn reroll_all(&mut self) {
        for die in &mut self.current.iter_mut() {
            *die = rand::thread_rng().gen_range(1, 7);
        }
        for die in &mut self.to_keep {
            *die = 0 as usize;
        }
    }

    pub fn print(&self) {
        let mut dot: [[char; 15]; 3] = [[' '; 15]; 3];

        for (i, item) in self.current.iter().enumerate() {
            match item {
                1 => {
                    if self.to_keep[i] > 0 {
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
                    if self.to_keep[i] > 0 {
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
                    if self.to_keep[i] > 0 {
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
                    if self.to_keep[i] > 0 {
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
                    if self.to_keep[i] > 0 {
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
                    if self.to_keep[i] > 0 {
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

    pub fn select(&mut self, score: &mut Scores, count: &i32, lp: &mut bool) {
        let mut selected: [usize; 5] = [0, 0, 0, 0, 0];
        let mut left_margin = "".to_string();
        let mut margin_width: usize = 0;
        loop {
            clear_screen();
            if *count == 2 {
                println!("  Use the arrow keys and Space to toggle which\n  dice to keep. Then press Enter to reroll\n  for the last time.");
            } else {
                println!("  Use the arrow keys and Space to toggle which\n  dice to keep. Then press Enter to reroll.");

            }
            score.print();
            self.print();
            match self.select_checker(&mut left_margin, &mut margin_width,
                                      &mut selected) {
                DiceSelectStatus::Exit => {
                    *lp = false;
                    break;
                },
                DiceSelectStatus::Complete => break,
                DiceSelectStatus::Incomplete => continue,
            };
        }
    }
    fn select_checker(&mut self, left_margin: &mut String,
                      margin_width: &mut usize, selected: &mut [usize; 5])
                      -> DiceSelectStatus {

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
        println!("{}●━━━━━━━●{}", left_margin, termion::cursor::Goto(1, bottom_line -4));

        for c in stdin.keys() {
            // Clear the current line.
            write!(stdout, "{}{}", termion::cursor::Goto(1, bottom_line), termion::clear::CurrentLine).unwrap();

            match c.unwrap() {
                Key::Ctrl(c) => { if c == 'c' { return DiceSelectStatus::Exit; } },
                Key::Char('\n') => { selected[*margin_width] = 1;
                                     return DiceSelectStatus::Complete;},
                Key::Char(' ')  => { println!("{}●━━━━━━━●{}", *left_margin, termion::cursor::Goto(1, bottom_line -4));
                                     if selected[*margin_width] == 0 && self.to_keep[*margin_width] == 0 {
                                         self.to_keep[*margin_width] = self.current[*margin_width];
                                         selected[*margin_width] = 1;
                                         return DiceSelectStatus::Incomplete;
                                     } else {
                                         self.to_keep[*margin_width] = 0;
                                         selected[*margin_width] = 0;
                                         return DiceSelectStatus::Incomplete;
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
        DiceSelectStatus::Incomplete
    }
}

pub fn welcome() {
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
            Key::Ctrl(c) => if c == 'c' { break; } else { continue; },
            _ => continue,
        }
    }

    stdout.flush().unwrap();
}

pub fn clear_screen() {
    println!("{}", clear::All);
}
