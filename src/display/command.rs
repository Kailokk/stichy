use embassy_time::Duration;

pub struct Command{
    pub byte:u8,
    pub timing:Duration
}

enum DisplayCommand {
    ClearDisplay,
    ReturnHome,
    SetEntryMode {
        cursor_move_rtl: bool,
        can_shift: bool,
    },
    DisplayOnOffControl {
        display_on: bool,
        cursor_on: bool,
        cursor_blinking: bool,
    },
    CursorOrDisplayShift {
        cursor_is_moving: bool,
        display_is_moving: bool,
    },
    FunctionSet {
        display_is_two_lines: bool,
        display_font_is_short: bool,
    },
}

impl DisplayCommand {
    pub const fn to_bytes(self) -> Command {
       let (byte, timing) = match self {
            DisplayCommand::ClearDisplay => (0b0000_0001, Duration::from_millis(2)),
            DisplayCommand::ReturnHome => (0b0000_0010,Duration::from_millis(2)),
            DisplayCommand::SetEntryMode {
                cursor_move_rtl,
                can_shift,
            } => {
                let mut command:u8 = 0b0000_0100;
                Self::set_if_true(cursor_move_rtl, 0b0000_0010, &mut command);
                Self::set_if_true(can_shift, 0b0000_0001, &mut command);
                (command, Duration::from_micros(40))
            }
            DisplayCommand::DisplayOnOffControl {
                display_on,
                cursor_on,
                cursor_blinking,
            } => {
                let mut command:u8 = 0b0000_1000;
                Self::set_if_true(display_on, 0b0000_0100, &mut command);
                Self::set_if_true(cursor_on, 0b0000_0010, &mut command);
                Self::set_if_true(cursor_blinking, 0b0000_0001, &mut command);
                (command, Duration::from_micros(40))
            }
            DisplayCommand::CursorOrDisplayShift {
                cursor_is_moving,
                display_is_moving,
            } =>{
                let mut command:u8 = 0b0001_0000;
                Self::set_if_true(cursor_is_moving, 0b0000_1000, &mut command);
                Self::set_if_true(display_is_moving, 0b0000_0100, &mut command);
                (command, Duration::from_micros(40))
            }
            DisplayCommand::FunctionSet {
                display_is_two_lines,
                display_font_is_short,
            } => {
                let mut command:u8 = 0b0010_0000;
                Self::set_if_true(display_is_two_lines, 0b0000_1000, &mut command);
                Self::set_if_true(display_font_is_short, 0b0000_0100, &mut command);
                (command, Duration::from_micros(40))
            }
        };

        Command { byte, timing }
    }

   const fn set_if_true(value: bool, bit_to_set: u8, byte: &mut u8) {
        if value {
            *byte |= bit_to_set
        }
    }
}

//TODO: Implement tests when debug probe is available using probe-rs and embedded-test libraries
// #[cfg(test)]
// mod test{
//     use crate::display::command::DisplayCommand;

//     #[rstest]
//     #[case(false, false, 0b0000_0100)]
//     #[case(true, false, 0b0000_0110)]
//     #[case(false, true, 0b0000_0101)]
//     #[case(true, true, 0b0000_0111)]
//     pub fn set_entry_mode_renders_correctly(#[case]cursor_move_rtl:bool, #[case]can_shift:bool, #[case]expected:u8){
//         let command = DisplayCommand::SetEntryMode { cursor_move_rtl, can_shift };
//         assert_eq!(expected, command);
//     }

// }
