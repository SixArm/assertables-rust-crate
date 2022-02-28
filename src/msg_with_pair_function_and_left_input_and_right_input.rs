#[macro_export]
macro_rules! msg_with_pair_function_and_left_input_and_right_input {
    ($msg:expr, $key:expr, $function_name:expr, $left_input_name:expr, $right_input_name:expr, $left_input:expr, $right_input:expr, $left_output:expr, $right_output:expr $(,)?) => ({
        format!(
            concat!(
                "{}: `{}(function, left_input, right_input)`\n",
                "    function name: `{}`,\n",
                "  left input name: `{}`,\n",
                " right input name: `{}`,\n",
                "       left input: `{:?}`,\n",
                "      right input: `{:?}`,\n",
                "      left output: `{:?}`,\n",
                "     right output: `{:?}`"
            ),
            $msg,
            $key,
            $function_name,
            $left_input_name,
            $right_input_name,
            $left_input,
            $right_input,
            $left_output,
            $right_output
        )
    });
}

#[macro_export]
macro_rules! msg_with_pair_function_and_left_input_and_right_input_and_err {
    ($msg:expr, $key:expr, $function_name:expr, $left_input_name:expr, $right_input_name:expr, $left_input:expr, $right_input:expr, $er:expr $(,)?) => ({
        format!(
            concat!(
                "{}: `{}(function, left_input, right_input)`\n",
                "    function name: `{:?}`,\n",
                "  left input name: `{:?}`,\n",
                " right input name: `{:?}`,\n",
                "       left input: `{:?}`,\n",
                "      right input: `{:?}`,\n",
                "              err: `{:?}`,\n",
            ),
            $msg,
            $key,
            $function_name,
            $left_input_name,
            $right_input_name,
            $left_input,
            $right_input,
            $err
        )
    });
}
