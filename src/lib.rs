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
            if dice_str == [1, 2, 3, 4, 5] {
                return Some(15);
            }
            None
        }

        fn large_str(current: &[usize; 5]) -> Option<usize> {
            let mut dice_str: [usize; 5] = current.clone();
            dice_str.sort();
            if dice_str == [2, 3, 4, 5, 6] {
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

pub struct ScoreCategory {
    name: &'static str,
    value: String,
    selected: bool,
    highlighted: bool,
}

pub struct Score([ScoreCategory; 18]);

impl<'a> IntoIterator for &'a Score {
    type Item = &'a ScoreCategory;
    type IntoIter = std::slice::Iter<'a, ScoreCategory>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl std::ops::Deref for Score {
    type Target = [ScoreCategory];

    fn deref(&self) -> &[ScoreCategory] {
        &self.0
    }
}

impl std::ops::DerefMut for Score {
    fn deref_mut(&mut self) -> &mut [ScoreCategory] {
        &mut self.0
    }
}

impl Score {
    pub fn new() -> Score {
        // initialize score values
        let score_ones = ScoreCategory {
            name: "Ones", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_twos = ScoreCategory {
            name: "Twos", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_threes = ScoreCategory {
            name: "Threes", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_fours = ScoreCategory {
            name: "Fours", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_fives = ScoreCategory {
            name: "Fives", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_sixes = ScoreCategory {
            name: "Sixes", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_bonus = ScoreCategory {
            name: "Bonus", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_one_pair = ScoreCategory {
            name: "One Pair", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_two_pairs = ScoreCategory {
            name: "Two Pairs", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_three_kind = ScoreCategory {
            name: "Three of a Kind", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_four_kind = ScoreCategory {
            name: "Four of a Kind", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_small_str = ScoreCategory {
            name: "Small Straight", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_large_str = ScoreCategory {
            name: "Large Straight", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_full_house = ScoreCategory {
            name: "Full House", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_chance = ScoreCategory {
            name: "Chance", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_yatzy = ScoreCategory {
            name: "Yatzy", value: "  ".to_string(), selected: false, highlighted: false
        };
        let score_subtotal = ScoreCategory {
            name: "Sum", value: "   ".to_string(), selected: false, highlighted: false
        };
        let score_total = ScoreCategory {
            name: "Total", value: "   ".to_string(), selected: false, highlighted: false
        };

        // Return the score values as a Score struct
        Score ([score_ones, score_twos, score_threes, score_fours,
                 score_fives, score_sixes, score_one_pair, score_two_pairs,
                 score_three_kind, score_four_kind, score_small_str,
                 score_large_str, score_full_house, score_chance,
                 score_yatzy, score_bonus, score_subtotal, score_total])
    }

    pub fn print(&mut self) {

        let mut subtotal: u8 = 0;
        let mut total: u16 = 0;

        for elm in self[0..6].iter() {
            if elm.value != "  " && elm.value != "   " && elm.value != " –" {
                subtotal += elm.value.trim().parse::<u8>().unwrap();
            }
        }
        if subtotal > 0 {
            self[16].value = format!("{:>3}", subtotal);
        }
        
        if total > 0 {
            self[17].value = format!("{:>3}", total);
        }

        if subtotal > 62 {
            self[15].value = format!("50");
        } else if
            self[0].value != "  " &&
            self[1].value != "  " &&
            self[2].value != "  " &&
            self[3].value != "  " &&
            self[4].value != "  " &&
            self[5].value != "  "
        {
            self[15].value = format!(" –");
        }

        for elm in self.iter() {
            if elm.value != "  " &&
                elm.value != "   " &&
                elm.value != " –" &&
                elm.name != "Sum" {
                    total += elm.value.trim().parse::<u16>().unwrap();
                }
        }

        if total > 0 {
            self[17].value = format!("{:>3}", total);
        }

        println!("╔═══════════════════════════════════════════════╗");
        println!("║ RUSTY YACHT                                   ║");
        println!("╠═══════════════════════════╦═══════════════════╣");
        println!("║                       Max ║             Score ║");
        println!("╟───────────────────────────╫───────────────────╢");
        if self[0].highlighted == true {
            println!("║{} Ones                    5 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert,
                     self[0].value, style::Reset);
        } else { println!("║ Ones                    5 ║                {} ║",
                          self[0].value); }
        if self[1].highlighted == true {
            println!("║{} Twos                   10 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self[1].value, style::Reset);
        } else {
            println!("║ Twos                   10 ║                {} ║",
                     self[1].value); }
        if self[2].highlighted == true {
            println!("║{} Threes                 15 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self[2].value, style::Reset);
        } else {
            println!("║ Threes                 15 ║                {} ║",
                     self[2].value); }
        if self[3].highlighted == true {
            println!("║{} Fours                  20 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self[3].value, style::Reset);
        } else {println!("║ Fours                  20 ║                {} ║",
                         self[3].value); }
        if self[4].highlighted == true {
            println!("║{} Fives                  25 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self[4].value, style::Reset);
        } else {
            println!("║ Fives                  25 ║                {} ║",
                     self[4].value); }
        if self[5].highlighted == true {
            println!("║{} Sixes                  30 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self[5].value, style::Reset);
        } else { println!("║ Sixes                  30 ║                {} ║",
                          self[5].value); }
        println!("╟───────────────────────────╫───────────────────╢");
        // if self[16].highlighted == true {
        //     println!("║{} Sum                105 {}║{}                    {} {}║",
        //              style::Invert, style::Reset, style::Invert, self[16].value, style::Reset); } else {
        println!("║ Sum                   105 ║               {} ║", self[16].value);
        println!("║ Bonus                  50 ║                {} ║", self[15].value);
        //println!("╟───────────────────────────╫───────────────────╢");
        if self[6].highlighted == true {
            println!("║{} One Pair               12 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self[6].value, style::Reset);
        } else { println!("║ One Pair               12 ║                {} ║",
                          self[6].value); }
        if self[7].highlighted == true {
            println!("║{} Two Pairs              22 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self[7].value, style::Reset);
        } else {
            println!("║ Two Pairs              22 ║                {} ║",
                     self[7].value); }
        if self[8].highlighted == true {
            println!("║{} Three of a Kind        18 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self[8].value, style::Reset);
        } else {
            println!("║ Three of a Kind        18 ║                {} ║",
                     self[8].value);
        }
        if self[9].highlighted == true {
            println!("║{} Four of a Kind         24 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self[9].value, style::Reset);
        } else {
            println!("║ Four of a Kind         24 ║                {} ║",
                     self[9].value);
        }
        if self[10].highlighted == true {
            println!("║{} Small Straight         15 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self[10].value, style::Reset);
        } else {
            println!("║ Small Straight         15 ║                {} ║",
                     self[10].value);
        }
        if self[11].highlighted == true {
            println!("║{} Large Straight         20 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self[11].value, style::Reset);
        } else {
            println!("║ Large Straight         20 ║                {} ║",
                     self[11].value);
        }
        if self[12].highlighted == true {
            println!("║{} Full House             28 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self[12].value, style::Reset);
        } else {
            println!("║ Full House             28 ║                {} ║",
                     self[12].value);
        }
        if self[13].highlighted == true {
            println!("║{} Chance                 30 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self[13].value, style::Reset);
        } else {
            println!("║ Chance                 30 ║                {} ║",
                     self[13].value);
        }
        if self[14].highlighted == true {
            println!("║{} Yatzy                  50 {}║{}                {} {}║",
                     style::Invert, style::Reset, style::Invert, self[14].value, style::Reset);
        } else {
            println!("║ Yatzy                  50 ║                {} ║",
                     self[14].value);
        }
        println!("╟───────────────────────────╫───────────────────╢");
        println!("║ Total                 374 ║               {:>3} ║", self[17].value);
        println!("╚═══════════════════════════╩═══════════════════╝");

        self[17].value = format!("   ");

    }

    pub fn place_points(&mut self, validators: [fn(&[usize; 5]) -> Option<usize>; 15],
                        dice: &Dice) {
        let mut i: usize = 0;
        loop {
            self[i].highlighted = true;
            self.print();
            dice.print();

            match self.select_slot(&dice, validators, &mut i) {
                SlotSelectStatus::Exit => std::process::exit(0),
                SlotSelectStatus::AlreadySelected => { println!("{}", clear::All);
                                                       println!("  Sorry, you can't use this slot again.");
                                                       println!("  Press Enter to continue.");
                                                       self.print();
                                                       dice.print();
                                                       let stdin = stdin();
                                                       let mut stdout = stdout().into_raw_mode().unwrap();
                                                       write!(stdout, "{}", termion::cursor::Hide).unwrap();
                                                       for c in stdin.keys() {
                                                           match c.unwrap() {
                                                               Key::Ctrl(c) => if c == 'c' {
                                                                   stdout.suspend_raw_mode().unwrap();
                                                                   std::process::exit(0);
                                                               },
                                                               Key::Char('\n') => {
                                                                   println!("{}", clear::All);
                                                                   break;
                                                               },
                                                               Key::Up => {
                                                                   if i != 0 {
                                                                       i -= 1;
                                                                       self[i+1].highlighted = false;
                                                                   } else {
                                                                       i = 14;
                                                                       self[0].highlighted = false;
                                                                   }
                                                                   self[i].highlighted = true;
                                                                   println!("{}", clear::All);
                                                                   break;
                                                               },
                                                               Key::Down => {
                                                                   if i != 14 {
                                                                       i += 1;
                                                                       self[i-1].highlighted = false;
                                                                   } else {
                                                                       i = 0;
                                                                       self[14].highlighted = false;
                                                                   }
                                                                   self[i].highlighted = true;
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
                                                 dice.print();
                                                 let stdin = stdin();
                                                 let mut stdout = stdout().into_raw_mode().unwrap();
                                                 write!(stdout, "{}", termion::cursor::Hide).unwrap();
                                                 for c in stdin.keys() {
                                                     match c.unwrap() {
                                                         Key::Ctrl(c) => if c == 'c' {
                                                             stdout.suspend_raw_mode().unwrap();
                                                             std::process::exit(0);
                                                         },
                                                         Key::Char('\n') => break,
                                                         _ => continue,
                                                     }
                                                 }
                                                 stdout.flush().unwrap();
                                                 break;
                },
                SlotSelectStatus::Incomplete => {
                    println!("{}", clear::All);
                },
                SlotSelectStatus::Invalid => {
                    println!("{}", clear::All);
                    println!("  Invalid selection. Press - to strike it out");
                    println!("  or an arrow key to cancel.");
                },
            }
        }
        self[i].highlighted = false;
        println!("{}", clear::All);
    }
    pub fn is_final(&self) -> bool {
        for item in self.iter() {
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
                    if self[*i].value != "  ".to_string() {
                        return SlotSelectStatus::AlreadySelected;
                    } else if validators[*i](&dice.current) != None {
                        self[*i].value = format!("{:>2}", &validators[*i](&dice.current).unwrap());
                        self[*i].selected = true;
                        stdout.flush().unwrap();
                        return SlotSelectStatus::Complete;
                    } else {
                        self[*i].selected = true;
                        stdout.flush().unwrap();
                        return SlotSelectStatus::Invalid;
                    }
                },
                Key::Char('-') => {
                    if self[*i].value == "  ".to_string() {
                        self[*i].selected = true;
                        self[*i].value = format!(" –");
                        stdout.flush().unwrap();
                        return SlotSelectStatus::Complete;
                    }
                },
                Key::Up => {
                    if i > &mut 0 {
                        *i -= 1;
                        self[*i].highlighted = true;
                        self[*i+1].highlighted = false;
                        return SlotSelectStatus::Incomplete;
                    } else {
                        *i = 14;
                        self[*i].highlighted = true;
                        self[0].highlighted = false;
                        return SlotSelectStatus::Incomplete;
                    }
                },
                Key::Left => {
                    if *i > 0 {
                        *i -= 1;
                        self[*i].highlighted = true;
                        self[*i+1].highlighted = false;
                        return SlotSelectStatus::Incomplete;
                    } else {
                        *i = 14;
                        self[*i].highlighted = true;
                        self[0].highlighted = false;
                        return SlotSelectStatus::Incomplete;
                    }
                },
                Key::Down => {
                    if *i < 14 {
                        *i += 1;
                        self[*i].highlighted = true;
                        self[*i-1].highlighted = false;
                        return SlotSelectStatus::Incomplete;
                    } else {
                        *i = 0;
                        self[*i].highlighted = true;
                        self[14].highlighted = false;
                        return SlotSelectStatus::Incomplete;
                    }
                },
                Key::Right => {
                    if *i < 14 {
                        *i += 1;
                        self[*i].highlighted = true;
                        self[*i-1].highlighted = false;
                        return SlotSelectStatus::Incomplete;
                    } else {
                        *i = 0;
                        self[*i].highlighted = true;
                        self[14].highlighted = false;
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

    pub fn log(self, path: &std::path::PathBuf) {
        Highscore::log(&path, self);
        clear_screen();
        let highscore = Highscore::new(&path);
        Highscore::print(&highscore);
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

    pub fn log(path: &std::path::PathBuf, score: Score) {
        let mut name = String::new();
        loop {
            println!("Input a name to log your score:");
            stdin().read_line(&mut name)
                .expect("Failed to read line");
            name.pop(); // remove trailing newline
            let namelen = name.chars().count();
            if namelen < 1 {
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

        for elm in score.iter() {
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

    pub fn select(&mut self, score: &mut Score, count: &i32) {
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
                DiceSelectStatus::Exit => std::process::exit(0),
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
        println!("{}●━━━━━━━●{}", *left_margin, termion::cursor::Goto(1, bottom_line -4));

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
            Key::Ctrl(c) => if c == 'c' {
                stdout.suspend_raw_mode().unwrap();
                std::process::exit(0);
            },
            _ => continue,
        }
    }

    stdout.flush().unwrap();
}

pub fn clear_screen() {
    println!("{}", clear::All);
}
