use crate::shell_query::get_command_path;

#[test]
fn get_command_path_valid() {
    let result = get_command_path("bash").expect("failed to get command path");
    assert!(result.is_some());
} 

#[test]
fn get_command_path_invalid() {
    let result = get_command_path("asdfargawra").expect("failed to get command path");
    assert!(result.is_none());
}
