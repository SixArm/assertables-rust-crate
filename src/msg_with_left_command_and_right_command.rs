#[macro_export]
macro_rules! msg_with_left_command_and_right_command {
    ($msg:expr, $key:expr, $left_command_name:expr, $right_command_name:expr, $left_command:expr, $right_command:expr, $left:expr, $right:expr $(,)?) => ({
        format!(
            concat!(
                "{}: `{}(left_command, right_command)`\n",
                "  left command name: `{}`,\n",
                " right command name: `{}`,\n",
                "       left command: `{:?}`,\n",
                "      right command: `{:?}`,\n",
                "               left: `{:?}`,\n",
                "              right: `{:?}`"
            ),
            $msg,
            $key,
            $left_command_name,
            $right_command_name,
            $left_command,
            $right_command,
            $left,
            $right
        )
    });
}
