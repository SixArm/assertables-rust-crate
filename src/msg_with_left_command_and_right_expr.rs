#[macro_export]
macro_rules! msg_with_left_command_and_right_expr {
    ($msg:expr, $key:expr, $left_command_name:expr, $right_expr_name:expr, $left_command:expr, $right_expr:expr, $left:expr, $right:expr $(,)?) => ({
        format!(
            concat!(
                "{}: `{}(left_command, right_expr)`\n",
                "  left command name: `{}`,\n",
                "    right expr name: `{}`,\n",
                "       left command: `{:?}`,\n",
                "         right expr: `{:?}`,\n",
                "               left: `{:?}`,\n",
                "              right: `{:?}`",
            ),
            $msg,
            $key,
            $left_command_name,
            $right_expr_name,
            $left_command,
            $right_expr,
            $left,
            $right
        )
    });
}
