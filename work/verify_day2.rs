#[path = "../src/day02.rs"]
mod day02;

fn main() {
    let mut expected_1 = 0u128;
    let mut expected_2 = 0u128;
    for n in 1u64..=999_999 {
        let text = n.to_string();
        let bytes = text.as_bytes();
        if bytes.len() % 2 == 0 && bytes[..bytes.len() / 2] == bytes[bytes.len() / 2..] {
            expected_1 += n as u128;
        }
        let repeated = (1..bytes.len()).any(|block_len| {
            bytes.len() % block_len == 0
                && (block_len..bytes.len()).all(|i| bytes[i] == bytes[i % block_len])
        });
        if repeated {
            expected_2 += n as u128;
        }
    }

    let actual = day02::both("1-999999").unwrap();
    assert_eq!(actual, (expected_1, expected_2));
    println!("verified {:?}", actual);
}
