use crate::correct_command;

#[test]
fn correct_command_name_only() {
    let result = correct_command("chmad").expect("unable to correct command");
    let expected = Some(String::from("chmod"));
    assert_eq!(result, expected);
}

#[test]
fn correct_long_option_only() {
    let result = correct_command("ls --allmost-all").expect("unable to correct command");
    let expected = Some(String::from("ls --almost-all"));
    assert_eq!(result, expected);
}

#[test]
fn correct_command_name_and_long_option() {
    let result = correct_command("mcdir --partens").expect("unable to correct command");
    let expected = Some(String::from("mkdir --parents"));
    assert_eq!(result, expected);
}

#[test]
fn correct_multiple_commands() {
    let result = correct_command("ls --allmost-all | mcdir --partens")
        .expect("unable to correct command");
    let expected = Some(String::from("ls --almost-all | mkdir --parents"));
    assert_eq!(result, expected);
}

#[test]
fn correct_correct_command_with_correct_long_options() {
    let result = correct_command("touch --no-create").expect("unable to correct command");
    let expected = None;
    assert_eq!(result, expected);
}
