// #![cfg(feature = "test-bpf")]

// use {
//     assert_matches::*,
//     solana_program::{
//         instruction::{AccountMeta, Instruction},
//         pubkey::Pubkey,
//     },
//     solana_sdk::{signature::Signer, transaction::Transaction},
//     solana_validator::test_validator::*,
// };

// #[test]
// fn test_validator_transaction() {
//     solana_logger::setup_with_default("solana_program_runtime=debug");
//     let program_id = Pubkey::new_unique();

//     let (test_validator, payer) = TestValidatorGenesis::default()
//         .add_program("bpf_program_template", program_id)
//         .start();
//     let rpc_client = test_validator.get_rpc_client();

//     let blockhash = rpc_client.get_latest_blockhash().unwrap();

//     let mut transaction = Transaction::new_with_payer(
//         &[Instruction {
//             program_id,
//             accounts: vec![AccountMeta::new(payer.pubkey(), false)],
//             data: vec![1, 2, 3],
//         }],
//         Some(&payer.pubkey()),
//     );
//     transaction.sign(&[&payer], blockhash);

//     assert_matches!(rpc_client.send_and_confirm_transaction(&transaction), Ok(_));
// }


// Sanity tests
#[cfg(test)]
mod test {
  use super::*;
  use solana_program::{borsh::get_instance_packed_len, pubkey::Pubkey};

  #[test]
  fn test_mail() {
    let mail = Mail {
      id: String::from("00000000-0000-0000-0000-000000000000"),
      from_address: Pubkey::default().to_string(),
      to_address: Pubkey::default().to_string(),
      subject: String::from("Hey Mike"),
      body: String::from("Body text with some characters"),
      sent_date: String::from("9/29/2021, 3:58:02 PM"),
    };

    let mut temp_slice = [0; 500];

    mail.serialize(&mut &mut temp_slice[..]).unwrap();

    let mail =
      Mail::try_from_slice(&temp_slice[..get_instance_packed_len(&mail).unwrap()]).unwrap();

    assert_eq!(mail.subject, "Hey Mike");
  }
  #[test]
  fn test_mail_account() {
    let mail = Mail {
      id: String::from("00000000-0000-0000-0000-000000000000"),
      from_address: Pubkey::default().to_string(),
      to_address: Pubkey::default().to_string(),
      subject: String::from("Hey Mike"),
      body: String::from("Body text with some characters"),
      sent_date: String::from("9/29/2021, 3:58:02 PM"),
    };

    let mail_account = MailAccount {
      inbox: vec![mail],
      sent: Vec::new(),
    };

    let mut temp_slice = [0; 500];

    mail_account.serialize(&mut &mut temp_slice[..]).unwrap();

    let mail_account =
      MailAccount::try_from_slice(&temp_slice[..get_instance_packed_len(&mail_account).unwrap()])
        .unwrap();

    assert_eq!(mail_account.inbox[0].subject, "Hey Mike");
  }

  #[test]
  fn test_data_length() {
    let data_length = DataLength { length: 5 };

    let mut temp_slice = [0; 4];

    data_length.serialize(&mut &mut temp_slice[..]).unwrap();

    assert_eq!(temp_slice, [5, 0, 0, 0]);

    let data_length = DataLength::try_from_slice(&temp_slice[..4]).unwrap();

    assert_eq!(data_length.length, 5);
  }
}