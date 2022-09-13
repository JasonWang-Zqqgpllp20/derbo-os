use alloc::vec::Vec;

pub struct TerminalController<'a> {
    pub inputline: Vec<char>,
    pub commands: Vec<(&'a str, bool)>,     // if there is a parameter
    // pub text_state: TextState,
}

impl TerminalController<'static> {
    pub fn new() -> TerminalController<'static> {
        TerminalController {
            inputline: Vec::new(),
            commands: Vec::new(),
            // text_state: TextState::UserInput,
        }
    }

    pub fn init(&mut self) {
        self.commands.push( ("hello", false) );
        self.commands.push( ("echo", true) );
        self.commands.push( ("sleep", true) );

        self.commands.push( ("cd", true) );
        self.commands.push( ("ls", false) );
        self.commands.push( ("clear", false) );
        self.commands.push( ("edit", true) );
        self.commands.push( ("mk", true) );
        self.commands.push( ("mkdir", true) );
        self.commands.push( ("rm", true) );

        self.commands.push( ("run", true) );

    }

    // pub fn state_switch(&mut self) {
    //     if self.text_state == TextState::UserInput {
    //         self.text_state = TextState::TextEdit;
    //     } else {
    //         self.text_state = TextState::UserInput;
    //     }
    // }

    pub fn pushchar(&mut self, character: char) {
        self.inputline.push(character);
    }

    pub fn backspace(&mut self) {
        if self.inputline.len() > 0 {
            self.inputline.pop();
        }
    }

    pub fn clear(&mut self) {
        self.inputline = Vec::new();
    }

    pub fn retrieve(&self) -> Result< (&str, Vec<char>), () > {        // maybe can be rewritted by 'alloc::str::RSplit'     
        let mut found: bool = false;
        let mut if_para: bool = false;
        let mut command_input: Vec<char> = Vec::new();
        let mut command_output: &str = "";
        let mut parameter_output: Vec<char> = Vec::new();

        for c in &self.inputline {          // destruct the command from userinput
            if *c == ' ' {
                break;
            } else {
                command_input.push(*c);
            }
        }

        for (comm, para) in &self.commands {
            let mut matched: bool = true;

            for (i, &c) in comm.as_bytes().iter().enumerate() {
                if command_input.len() != comm.len() || command_input.len() == 0 { // invalid input
                    matched = false;
                    break;
                }
                if c != command_input[i] as u8 {           // not match
                    matched = false;
                    break;
                }
            }

            match matched {
                true => {
                    found = true;
                    command_output = comm;
                    if_para = *para;
                    break;
                },
                false => {}
            };
        }
        
        if found == false {
            return Err(());
        } else {
            if if_para == true {
                if self.inputline.len() <= command_output.len() + 1 {   // there is no parameter in userinput
                    return Err(())
                }
                
                for c in &self.inputline[command_output.len()+1..self.inputline.len()] {    // destruct para in userinput
                    parameter_output.push(*c)
                }
                
                return Ok( (&*command_output, parameter_output) );
            } else {
                return Ok( (&*command_output, parameter_output) );
            }
        }
    }
}