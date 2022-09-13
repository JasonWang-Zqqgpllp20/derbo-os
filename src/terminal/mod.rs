pub mod command;
pub mod controller;
pub mod terminal1;
pub mod terminal2;
pub mod task1;
pub mod task2;

#[derive(PartialEq, Copy, Clone)]
pub enum TextState {
    UserInput,
    TextEdit,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum SwitchState {
    Terminal1,
    Terminal2,
}

#[derive(Debug)]
pub struct TerminalSwitchState {
    state: SwitchState,
}

impl TerminalSwitchState {
    pub fn new() -> TerminalSwitchState {
        TerminalSwitchState {
            state: SwitchState::Terminal1,
        }
    }

    pub fn switch_state(&mut self) -> Result<SwitchState, SwitchState> {
        if self.state == SwitchState::Terminal1 {
            self.state = SwitchState::Terminal2;
        } else {
            self.state = SwitchState::Terminal1;
        }
        return Ok(self.state)
    }

    pub fn get_state(&self) -> SwitchState {
        self.state
    }
}