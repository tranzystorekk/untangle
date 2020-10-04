use assert_cmd::{crate_name, Command};

#[test]
fn untangle_succeeds_on_valid_input() {
    let input = "4 4
    p r p p
    g g p g
    r * r r
    r r p g";

    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    let assert = cmd.write_stdin(input).assert();

    assert.success();
}

#[test]
fn untangle_fails_on_empty_file() {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    let assert = cmd.assert();

    assert.failure();
}

#[test]
fn untangle_fails_on_shape_parse_error() {
    let input = "three three
    p p g
    g * p
    p g g";

    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    let assert = cmd.write_stdin(input).assert();

    assert.failure();
}

#[test]
fn untangle_fails_on_field_parse_error() {
    let input = "4 4
    p r p 5
    g g p g
    r * r r
    r r p g";

    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    let assert = cmd.write_stdin(input).assert();

    assert.failure();
}

#[test]
fn untangle_fails_on_fields_number_not_matching_shape() {
    let input = "4 4
    p r p p
    g g p g
    r * r r
    r r p g
    g g g g";

    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    let assert = cmd.write_stdin(input).assert();

    assert.failure();
}

#[test]
fn untangle_fails_on_nonexistent_flag() {
    let mut cmd = Command::cargo_bin(crate_name!()).unwrap();
    let assert = cmd.arg("--nonsense").assert();

    assert.failure();
}
