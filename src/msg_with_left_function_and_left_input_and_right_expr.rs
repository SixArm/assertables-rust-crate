#[macro_export]
macro_rules! msg_with_left_function_and_left_input_and_right_expr {
    ($msg:expr, $key:expr, $function_name:expr, $left_input_name:expr, $right_value_name:expr, $left_input:expr, $right_value:expr, $left_output:expr $(,)?) => ({
        format!(
            concat!(
                "{}: `{}(function, left_input, right_expr)`\n",
                "    function name: `{}`,\n",
                "  left input name: `{}`,\n",
                "  right expr name: `{}`,\n",
                "       left input: `{:?}`,\n",
                "       right expr: `{:?}`,\n",
                "      left output: `{:?}`,\n",
                "             left: `{:?}`,\n",
                "            right: `{:?}`",
            ),
            $msg,
            $key,
            $function_name,
            $left_input_name,
            $right_value_name,
            $left_input,
            $right_value,
            $left_output,
            $left_output,
            $right_value
        )
    });
}

#[macro_export]
macro_rules! msg_with_left_function_and_left_input_and_right_expr_and_err {
    ($msg:expr, $key:expr, $function_name:expr, $left_input_name:expr, $right_value_name:expr, $left_input:expr, $right_value:expr, $er:expr $(,)?) => ({
        format!(
            concat!(
                "{}: `{}(function, left_input, right_expr)`\n",
                "    function name: `{:?}`,\n",
                "  left input name: `{:?}`,\n",
                "  right expr name: `{:?}`,\n",
                "       left input: `{:?}`,\n",
                "       right expr: `{:?}`,\n",
                "              err: `{:?}`,\n",
            ),
            $msg,
            $key,
            $function_name,
            $left_input_name,
            $right_value_name,
            $left_input,
            $right_value,
            $err
        )
    });
}
