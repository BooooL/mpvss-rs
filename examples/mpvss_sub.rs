// Copyright 2020-2021  MathxH Chen.
//
// Code is licensed under GPLv3.0 License.

use mpvss_rs::Participant;
use num_bigint::{BigUint, ToBigInt};

fn main() {
    let secret_message = String::from("Hello Sub MPVSS Example.");
    let secret = BigUint::from_bytes_be(&secret_message.as_bytes());
    let mut dealer = Participant::new();
    dealer.initialize();
    let mut p1 = Participant::new();
    let mut p2 = Participant::new();
    let mut p3 = Participant::new();
    let mut p4 = Participant::new();
    p1.initialize();
    p2.initialize();
    p3.initialize();
    p4.initialize();

    let distribute_shares_box = dealer.distribute_secret(
        secret.to_bigint().unwrap(),
        vec![
            p1.publickey.clone(),
            p2.publickey.clone(),
            p3.publickey.clone(),
            p4.publickey.clone(),
        ],
        3,
    );

    assert_eq!(p1.verify_distribution_shares(&distribute_shares_box), true);

    assert_eq!(p2.verify_distribution_shares(&distribute_shares_box), true);

    assert_eq!(p3.verify_distribution_shares(&distribute_shares_box), true);

    assert_eq!(p4.verify_distribution_shares(&distribute_shares_box), true);

    // p1 extracts the share. [p2 and p3 do this as well.]
    let s1 = p1
        .extract_secret_share(&distribute_shares_box, &p1.privatekey)
        .unwrap();

    // p1, p2, p3 p4 exchange their descrypted shares.
    // ...
    let s2 = p2
        .extract_secret_share(&distribute_shares_box, &p2.privatekey)
        .unwrap();
    let s3 = p3
        .extract_secret_share(&distribute_shares_box, &p3.privatekey)
        .unwrap();

    let s4 = p4
        .extract_secret_share(&distribute_shares_box, &p4.privatekey)
        .unwrap();

    // p1 verifies the share received from p2. [Actually everybody verifies every received share.]

    assert_eq!(
        p1.verify_share(&s2, &distribute_shares_box, &p2.publickey),
        true
    );

    assert_eq!(
        p2.verify_share(&s3, &distribute_shares_box, &p3.publickey),
        true
    );

    assert_eq!(
        p3.verify_share(&s1, &distribute_shares_box, &s1.publickey),
        true
    );

    assert_eq!(
        p4.verify_share(&s2, &distribute_shares_box, &s2.publickey),
        true
    );

    let share_boxs = [s1.clone(), s2.clone(), s4.clone()];
    let r1 = p1.reconstruct(&share_boxs, &distribute_shares_box).unwrap();
    let r2 = p2.reconstruct(&share_boxs, &distribute_shares_box).unwrap();
    let r3 = p3.reconstruct(&share_boxs, &distribute_shares_box).unwrap();
    let r4 = p4.reconstruct(&share_boxs, &distribute_shares_box).unwrap();

    let r1_str =
        String::from_utf8(r1.to_biguint().unwrap().to_bytes_be()).unwrap();
    assert_eq!(secret_message.clone(), r1_str);
    let r2_str =
        String::from_utf8(r2.to_biguint().unwrap().to_bytes_be()).unwrap();
    assert_eq!(secret_message.clone(), r2_str);
    let r3_str =
        String::from_utf8(r3.to_biguint().unwrap().to_bytes_be()).unwrap();
    assert_eq!(secret_message.clone(), r3_str);
    let r4_str =
        String::from_utf8(r4.to_biguint().unwrap().to_bytes_be()).unwrap();
    assert_eq!(secret_message.clone(), r4_str);

    println!("secret message: {}", secret_message);
    println!("r1 str: {}", r1_str);
    println!("r2 str: {}", r2_str);
    println!("r3 str: {}", r3_str);
    println!("r3 str: {}", r4_str);
}
