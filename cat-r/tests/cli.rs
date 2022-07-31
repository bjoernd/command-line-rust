use std::fs;
use assert_cmd::Command;
use paste::paste;

type TestResult = Result<(), Box<dyn std::error::Error>>;

#[test]
fn ok_no_args() -> TestResult {
    let mut cmd = Command::cargo_bin("cat-r")?;
    cmd.assert()
        .success();
    Ok(())
}

fn run(args: &[&str], expected_file: &str, expected_err: &str) -> TestResult {
    let expected = fs::read_to_string(expected_file)?;
    Command::cargo_bin("cat-r")?
        .args(args)
        .assert()
        .success()
        .stdout(expected)
        .stderr(String::from(expected_err));
    Ok(())
}

fn run_and_fail(args: &[&str], expected_out: &str, expected_err: &str) -> TestResult {
    Command::cargo_bin("cat-r")?
        .args(args)
        .assert()
        .failure()
        .stdout(String::from(expected_out))
        .stderr(String::from(expected_err));
    Ok(())
}

macro_rules! ok_file {
    ($fname:ident) => {
        paste::item!(
            #[test]
            fn [< ok_ $fname >] () -> TestResult {
                run(&[&format!("tests/inputs/{}.txt", stringify!($fname)).as_str()],
                    format!("tests/inputs/{}.txt", stringify!($fname)).as_str(),
                    "")
            }
        );
    };
}

macro_rules! ok_files {
    ($fname:ident, $params:expr, $exp_name:expr) => {
        paste::item!(
            #[test]
            fn [< ok_ $fname >] () -> TestResult {
                run($params,
                    format!("tests/expected/{}.txt", $exp_name).as_str(),
                    "")
            }
        );
    };
}

const SPIDERS : &str = "tests/inputs/spiders.txt";
const BUSTLE : &str  = "tests/inputs/the_bustle.txt";
const EMPTY : &str   = "tests/input/empty.txt";

ok_file!(empty);
ok_file!(fox);
ok_file!(spiders);
ok_file!(the_bustle);

ok_files!(two_files, &[SPIDERS, BUSTLE], "spiders+bustle");
ok_files!(two_files_num, &[SPIDERS, BUSTLE, "-n"], "spiders+bustle.num");
ok_files!(one_file_num, &[BUSTLE, "-n"], "bustle.num");

ok_files!(one_file_num_noblank, &[BUSTLE, "-b"], "bustle.num-b");


#[test]
fn fail_both_num_args() -> TestResult {
    run_and_fail(&[EMPTY, "-b", "-n"], "",
    "error: The argument '-n' cannot be used with '-b'

USAGE:
    cat-r.exe -n -b

For more information try --help\n")
}