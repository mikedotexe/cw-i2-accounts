use crate::tests::{alice, contracts, default_app};
use cw_i2_creator_pkg::msgs::instantiate_msg::InstantiateMsg;
use cw_multi_test::Executor;

#[test]
fn the_usual() {
    let mut app = default_app();
    let code_id = app.store_code(contracts::me());
    assert!(
        app.instantiate_contract(
            code_id,
            alice(), // from
            &InstantiateMsg { code_id, checksum: "028d2726311d36a899868afc4162752fdd098019fa1577e1c29181a28ba5f733".to_string() },
            &[], // funds
            "regifted the labelmaker",
            None, // No admin, so it'll be immutable
        )
        .is_ok(),
        "Should instantiate fine"
    );

    println!("To see this, you'll have to run the command:");
    println!("cargo test -- --nocapture");
    println!("or 'cargo t' which won't show warnings");
}
