use core::fmt::Debug;

pub enum GameStatus {
    Idle,
    Entering,
    Exiting,
    Running,
}

pub struct GameState<TScene: Clone + Copy + Debug> {
    pub status: GameStatus,
    pub current: Option<TScene>,
    pub next: Option<TScene>,
}

impl<TScene: Clone + Copy + Debug> GameState<TScene> {
    pub fn set_transition(&mut self, next: TScene) {
        self.next = Some(next);
    }
    
    pub fn update(&mut self) {
        match &self.status {
            GameStatus::Idle => {
                match self.next {
                    Some(next_state) => {
                        log::trace!("[Transition] IDLE to ENTERED::{:?}", next_state);
                        self.status = GameStatus::Entering;
                        self.current = Some(next_state);
                    }
                    None => {
                        log::trace!("[Transition] IDLE to no state");
                    }
                }
            }
            GameStatus::Entering => {
                log::trace!("[Transition] ENTERED::{:?} to RUNNING::{:?}",
                    self.current, self.current
                );
                self.status = GameStatus::Running;
            }
            GameStatus::Exiting => match self.next {
                Some(_) => {
                    match self.current {
                        None => log::trace!(
                            "[Transition] EXITING::NoState to ENTERED::{:?}",
                            self.next.unwrap()
                        ),
                        Some(_) => println!(
                            "[Transition] EXITING::{:?} to ENTERED::{:?}",
                            self.current.unwrap(),
                            self.next.unwrap() 
                        ),
                    }

                    self.status = GameStatus::Entering;
                    self.current = self.next.clone();
                    self.next = None;
                }
                _ => {
                    log::trace!("[Transition] Can't move from EXITING::{:?} to ENTERING::None", self.current.unwrap());
                }
            }
            GameStatus::Running => match self.next {
                None => {}
                Some(_) => {
                    match self.current {
                        None => log::trace!(
                            "[Transition] RUNNING::NoState, EXITING::{:?}",
                            self.next.unwrap()
                        ),
                        Some(_) => log::trace!(
                            "[Transition] RUNNING::{:?} to EXITING::{:?}, next state is {:?}",
                            self.current.unwrap(),
                            self.current.unwrap(),
                            self.next.unwrap(),
                        )
                    }

                    self.status = GameStatus::Exiting;
                }
            }
        }
    }
}
