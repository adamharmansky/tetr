use crate::game::tetromino;

pub struct ScoreHandler {
    pub combo: u32,
    pub b2b: u32,
}

impl ScoreHandler {
    pub fn new() -> Self {
        Self { combo: 0, b2b: 0 }
    }

    /// Analyze the move taken and return the number of lines to be sent
    ///
    /// This function should be called for EVERY dropped piece, even those which didn't clear a line!
    ///
    /// # Arguments
    ///
    /// * `cleared` - the number of lines cleared
    /// * `piece` - the shape of the piece which cleared the line
    /// * `covered` - whether the piece was obstrued from the top
    pub fn analyze(
        &mut self,
        cleared: u32,
        piece: tetromino::Shape,
        covered: bool,
        effects: &mut crate::game::board::effects::BoardEffects,
    ) -> u32 {
        // a T-spin occurs when a T is placed where it would otherwise be obstructed
        let tspin = if let tetromino::Shape::T = piece {
            covered
        } else {
            false
        };

        // Icrement combo counter when clearing lines, reset it otherwise
        if cleared > 0 {
            self.combo += 1;
            // and modify b2b only when lines were cleared
            if cleared == 4 || tspin {
                self.b2b += 1;
            } else {
                self.b2b = 0;
            }
        } else {
            self.combo = 0;
        }

        let b2b_bonus = if self.b2b > 0 {
            ((self.b2b - 1) as f32).sqrt().floor() as u32
        } else {
            0
        };

        match cleared {
            0 => 0,
            4 => {
                effects.info = Some(crate::game::board::effects::InfoText {
                    text: String::from("TETRIS"),
                    time: std::time::Instant::now(),
                });
                3 + self.combo + b2b_bonus
            }
            x => {
                if tspin {
                    match cleared {
                        1 => {
                            effects.info = Some(crate::game::board::effects::InfoText {
                                text: String::from("T-SPIN SINGLE"),
                                time: std::time::Instant::now(),
                            });
                            2 + self.combo / 2 + b2b_bonus
                        }
                        2 => {
                            effects.info = Some(crate::game::board::effects::InfoText {
                                text: String::from("T-SPIN DOUBLE"),
                                time: std::time::Instant::now(),
                            });
                            4 + self.combo / 2 + b2b_bonus
                        }
                        3 => {
                            effects.info = Some(crate::game::board::effects::InfoText {
                                text: String::from("T-SPIN TRIPLE"),
                                time: std::time::Instant::now(),
                            });
                            6 + self.combo / 2 + b2b_bonus
                        }
                        _ => 0,
                    }
                } else {
                    match cleared {
                        1 => {
                            effects.info = Some(crate::game::board::effects::InfoText {
                                text: String::from("SINGLE"),
                                time: std::time::Instant::now(),
                            });
                        }
                        2 => {
                            effects.info = Some(crate::game::board::effects::InfoText {
                                text: String::from("DOUBLE"),
                                time: std::time::Instant::now(),
                            });
                        }
                        3 => {
                            effects.info = Some(crate::game::board::effects::InfoText {
                                text: String::from("TRIPLE"),
                                time: std::time::Instant::now(),
                            });
                        }
                        _ => (),
                    }
                    x - 1 + self.combo / 2 + b2b_bonus
                }
            }
        }
    }
}
