use {
  super::*,
  ord::subcommand::list::{Output, Range},
};

#[test]
fn output_found() {
  let core = mockcore::spawn();
  let output = CommandBuilder::new(
    "--index-sats list 4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b:0",
  )
  .core(&core)
  .run_and_deserialize_output::<Output>();

  assert_eq!(
    output,
    Output {
      address: None,
      indexed: true,
      inscriptions: vec![],
      runes: BTreeMap::new(),
      sat_ranges: Some(vec![Range {
        end: 50 * COIN_VALUE,
        name: "nvtdijuwxlp".into(),
        offset: 0,
        rarity: "mythic".parse().unwrap(),
        size: 50 * COIN_VALUE,
        start: 0,
       }]),
      script_pubkey: "OP_PUSHBYTES_65 047cb27a8a1ff2d5c4c00cff87f16c2b5ce61a1aa849b84122a45f06ed6dfae0bd483245a59ffa828bfa442a80b6549f681a72f1406ef53ad60b73e70de5a9fd48 OP_CHECKSIG".to_string(),
      spent: false,
      transaction: "4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b".to_string(),
      value: 5000000000,
    }
  );
}

#[test]
fn output_not_found() {
  let core = mockcore::spawn();
  CommandBuilder::new(
    "--index-sats list 0000000000000000000000000000000000000000000000000000000000000000:0",
  )
  .core(&core)
  .expected_exit_code(1)
  .expected_stderr("error: output not found\n")
  .run_and_extract_stdout();
}

#[test]
fn no_satoshi_index() {
  let core = mockcore::spawn();
  CommandBuilder::new("list 4a5e1e4baab89f3a32518a88c31bc87f618f76673e2cc77ab2127b7afdeda33b:0")
    .core(&core)
    .expected_stderr("error: list requires index created with `--index-sats` flag\n")
    .expected_exit_code(1)
    .run_and_extract_stdout();
}
