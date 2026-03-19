use std::collections::HashSet;

use crate::doc_parse::{get_help_page, get_man_page, parse_command_long_options};

#[test]
fn get_man_page_ls() {
    let result = get_man_page("ls", "sh")
        .expect("failed to get man page")
        .expect("man page is none");
    assert!(!result.is_empty());
}

#[test]
fn get_man_page_cargo() {
    let result = get_man_page("cargo", "sh").expect("failed to get man page");
    assert!(result.is_none());
}

#[test]
fn get_help_page_cargo() {
    let result = get_help_page("cargo", "sh")
        .expect("failed to get help page")
        .expect("help page is none");
    assert!(!result.is_empty());
}

#[test]
fn get_help_page_gibberish() {
    let result = get_help_page("alkdfja", "sh").expect("failed to get help page");
    assert!(result.is_none());
}

#[test]
fn parse_ls_long_options() {
    let result = HashSet::from_iter(parse_command_long_options("ls", "sh").expect("failed parsing options"));
    let expected = HashSet::from([
        String::from("--all"),
        String::from("--almost-all"),
        String::from("--author"),
        String::from("--block-size"),
        String::from("--classify"),
        String::from("--color"),
        String::from("--context"),
        String::from("--dereference"),
        String::from("--dereference-command-line"),
        String::from("--dereference-command-line-symlink-to-dir"),
        String::from("--directory"),
        String::from("--dired"),
        String::from("--escape"),
        String::from("--file-type"),
        String::from("--format"),
        String::from("--full-time"),
        String::from("--group-directories-first"),
        String::from("--help"),
        String::from("--hide"),
        String::from("--hide-control-chars"),
        String::from("--human-readable"),
        String::from("--hyperlink"),
        String::from("--ignore"),
        String::from("--ignore-backups"),
        String::from("--indicator-style"),
        String::from("--inode"),
        String::from("--kibibytes"),
        String::from("--literal"),
        String::from("--no-group"),
        String::from("--numeric-uid-gid"),
        String::from("--quote-name"),
        String::from("--quoting-style"),
        String::from("--recursive"),
        String::from("--reverse"),
        String::from("--show-control-chars"),
        String::from("--si"),
        String::from("--size"),
        String::from("--sort"),
        String::from("--tabsize"),
        String::from("--time"),
        String::from("--time-style"),
        String::from("--version"),
        String::from("--width"),
        String::from("--zero"),
    ]);
    assert_eq!(result, expected);
}
