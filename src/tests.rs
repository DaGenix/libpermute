use crate::{permute, PermuteKeyData};

fn run_test_with_slice_key(key: &[u8], input: &[u8], buff: &mut [u8], result: &[u8]) {
    buff.copy_from_slice(input);
    permute(key, buff);
    assert_eq!(result, buff);
}

fn run_test_with_incremental_key(key: &[u8], input: &[u8], buff: &mut [u8], result: &[u8]) {
    let mut permute_key_data = PermuteKeyData::new();
    for &x in key {
        permute_key_data.add_bytes(&[x]);
    }
    buff.copy_from_slice(input);
    permute(permute_key_data, buff);
    assert_eq!(result, buff);
}

fn run_test(key: &[u8], input: &[u8], buff: &mut [u8], result: &[u8]) {
    run_test_with_slice_key(key, input, buff, result);
    run_test_with_incremental_key(key, input, buff, result);
}

#[test]
fn test_sequence_1() {
    const KEY: &'static [u8] = &[
        223, 111, 22, 44, 229, 80, 37, 18, 108, 86, 78, 16, 158, 155, 79, 68, 167, 73, 2, 177, 78,
        139, 94, 164, 221, 222, 160, 150, 67, 18, 231, 28,
    ];
    const INPUT: &'static [u8] = b"Hello world";
    const RESULT: &'static [u8] = b"droH lloelw";
    run_test(KEY, INPUT, &mut [0u8; INPUT.len()], RESULT);
}

#[test]
fn test_sequence_2() {
    const KEY: &'static [u8] = &[
        151, 46, 114, 238, 141, 121, 38, 149, 123, 237, 74, 42, 30, 111, 61, 123, 182, 158, 241,
        155, 113, 8, 56, 253, 250, 240, 216, 221, 231, 222, 172, 34,
    ];
    const INPUT: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
    const RESULT: &'static [u8] = b"blwjnveysrmpaodihguctxkqzf";
    run_test(KEY, INPUT, &mut [0u8; INPUT.len()], RESULT);
}
