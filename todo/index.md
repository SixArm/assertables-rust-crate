# TODO


## Assert process

assert_process_success & assert_process_failure, via these platform constants:

```
::std::process::ExitCode::SUCCESS
::std::process::ExitCode::FAILURE
```

## Command status code

assert_command_status_code_eq


Code something like this:

```
let cmd = ::std::process::Command::new(program)
let code = cmd.status().unwrap().code();
code == x;
```
