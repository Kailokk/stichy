use crate::display::set_bit_if_true;

pub enum DisplayCommand {
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
    pub const fn to_byte(self) -> u8 {
        let byte = match self {
            DisplayCommand::ClearDisplay => 0b0000_0001,
            DisplayCommand::ReturnHome => 0b0000_0010,
            DisplayCommand::SetEntryMode {
                cursor_move_rtl,
                can_shift,
            } => {
                let mut command: u8 = 0b0000_0100;
                set_bit_if_true(cursor_move_rtl, 0b0000_0010, &mut command);
                set_bit_if_true(can_shift, 0b0000_0001, &mut command);
                command
            }
            DisplayCommand::DisplayOnOffControl {
                display_on,
                cursor_on,
                cursor_blinking,
            } => {
                let mut command: u8 = 0b0000_1000;
                set_bit_if_true(display_on, 0b0000_0100, &mut command);
                set_bit_if_true(cursor_on, 0b0000_0010, &mut command);
                set_bit_if_true(cursor_blinking, 0b0000_0001, &mut command);
                command
            }
            DisplayCommand::CursorOrDisplayShift {
                cursor_is_moving,
                display_is_moving,
            } => {
                let mut command: u8 = 0b0001_0000;
                set_bit_if_true(cursor_is_moving, 0b0000_1000, &mut command);
                set_bit_if_true(display_is_moving, 0b0000_0100, &mut command);
                command
            }
            DisplayCommand::FunctionSet {
                display_is_two_lines,
                display_font_is_short,
            } => {
                let mut command: u8 = 0b0010_0000;
                set_bit_if_true(display_is_two_lines, 0b0000_1000, &mut command);
                set_bit_if_true(display_font_is_short, 0b0000_0100, &mut command);
                command
            }
        };

        byte
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
