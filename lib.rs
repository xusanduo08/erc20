#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {
  use ink::storage::Mapping;
  use trait_erc20::{ TERC20, Error, Result };
    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
      total_supply: Balance,
      balances: Mapping<AccountId, Balance>,
      allowances: Mapping<(AccountId, AccountId), Balance>
    }



    #[ink(event)]
    pub struct Transfer {
      from: Option<AccountId>,
      to: Option<AccountId>,
      value: Balance
    }

    #[ink(event)]
    pub struct Approve {
      from: AccountId,
      to: AccountId,
      value: Balance,
    }

    impl Erc20 {
      /// Constructor that initializes the `bool` value to the given `init_value`.
      #[ink(constructor)]
      pub fn new(total_supply: Balance) -> Self {
        let mut balances = Mapping::new();
        balances.insert(Self::env().caller(), &total_supply);
        Self::env().emit_event(Transfer{
          from: None,
          to: Some(Self::env().caller()),
          value: total_supply,
        });
        Self { total_supply, balances, ..Default::default() }
      }

      pub fn transfer_helper(&mut self, from: &AccountId, to: &AccountId, value: Balance) -> Result<()> {
        let balance_from = self.balance_of(*from);
        let balance_to = self.balance_of(*to);

        if value > balance_from {
          return Err(Error::BalanceTooLow);
        }

        self.balances.insert(from, &(balance_from - value));
        self.balances.insert(to, &(balance_to + value));

        self.env().emit_event(Transfer {
          from: Some(*from),
          to: Some(*to),
          value,
        });

        Ok(())
      }
    }

    impl TERC20 for Erc20 {
      #[ink(message)]
      fn total_supply(&self) -> Balance {
        self.total_supply
      }

      #[ink(message)]
      fn balance_of(&self, who: AccountId) -> Balance {
        self.balances.get(&who).unwrap_or_default()
      }

      #[ink(message)]
      fn approve(&mut self, to: AccountId, value: Balance) -> Result<()> { // 允许谁动用多少资金
        let sender = self.env().caller();
        self.allowances.insert(&(sender, to), &value); // 允许to调用sender的value数字的金额
        
        self.env().emit_event(Approve {
          from: sender,
          to,
          value
        });

        Ok(())
      }

      #[ink(message)]
      fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
        let sender = self.env().caller();
        
        return self.transfer_helper(&sender, &to, value);
      }

      #[ink(message)]
      fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
        let sender = self.env().caller();
        let mut allowances = self.allowances.get(&(from, sender)).unwrap_or_default(); // 获取允许sender调用from的金额

        if allowances < value {
          return Err(Error::AllowancesTooLow);
        }

        self.allowances.insert(&(from, sender), &(allowances - value));

        return self.transfer_helper(&from, &to, value);
      }

      
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
      /// Imports all the definitions from the outer scope so we can use them here.
      use super::*;
      type Event = <Erc20 as ::ink::reflect::ContractEventBase>::Type;
        /// We test if the default constructor does its job.
      #[ink::test]
      fn constructor_works() {
        let erc20 = Erc20::new(10000);
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        assert_eq!(erc20.total_supply(), 10000);
        assert_eq!(erc20.balance_of(accounts.alice), 10000);

        let emitted_events = ink::env::test::recorded_events().collect::<Vec<_>>();

        let event = &emitted_events[0];

        let decode = <Event as scale::Decode>::decode(&mut &event.data[..]).expect("decode error");

        match decode {
          Event::Transfer(Transfer{ from, to, value }) => {
            assert!(from.is_none(), "mint from error");
            assert_eq!(to, Some(accounts.alice), "mint to error");
            assert_eq!(value, 10000, "mint value error");
          }
          _ => panic!("match error"),
        }
      }

      #[ink::test]
      fn transfer_works() {
        let mut erc20 = Erc20::new(10000);
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();

        let res = erc20.transfer(accounts.bob, 12);
        assert!(res.is_ok());
        assert_eq!(erc20.balance_of(accounts.alice), 10000 - 12);
        assert_eq!(erc20.balance_of(accounts.bob), 12);
      }

      #[ink::test]
      fn invalid_transfer_should_fail() {
        let mut erc20 = Erc20::new(10000);
        let accounts = ink::env::test::default_accounts::<ink::env::DefaultEnvironment>();
        ink::env::test::set_caller::<ink::env::DefaultEnvironment>(accounts.bob);

        let res = erc20.transfer(accounts.charlie, 12);
        assert!(res.is_err());
        assert_eq!(res, Err(Error::BalanceTooLow));
      }
    }


    /// This is how you'd write end-to-end (E2E) or integration tests for ink! contracts.
    ///
    /// When running these you need to make sure that you:
    /// - Compile the tests with the `e2e-tests` feature flag enabled (`--features e2e-tests`)
    /// - Are running a Substrate node which contains `pallet-contracts` in the background
    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// A helper function used for calling contract messages.
        use ink_e2e::{build_message, subxt::client};

        /// The End-to-End test `Result` type.
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        /// We test that we can upload and instantiate the contract using its default constructor.
        #[ink_e2e::test]
        async fn default_works(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
          let total_supply = 123;
          let constructor = Erc20Ref::new(total_supply);

          // When
          let contract_account_id = client
              .instantiate("erc20", &ink_e2e::alice(), constructor, 0, None)
              .await
              .expect("instantiate failed")
              .account_id;
          let alice_acc_id = ink_e2e::account_id(ink_e2e::AccountKeyring::Alice);
          let bob_acc_id = ink_e2e::account_id(ink_e2e::AccountKeyring::Bob);

          let transfer_message = build_message::<Erc20Ref>(contract_account_id.clone()).call(|erc20| erc20.transfer(bob_acc_id, 2));

          let res = client.call(&ink_e2e::alice(), transfer_message, 0, None).await;

          assert!(res.is_ok());

          let balance_of_message = build_message::<Erc20Ref>(contract_account_id.clone()).call(|erc20| erc20.balance_of(alice_acc_id));
          let balance_of_alice = client.call_dry_run(&ink_e2e::alice(), &balance_of_message, 0, None).await;
            
          assert_eq!(balance_of_alice.return_value(), 121);

          Ok(())
          // 报错，提示：erc20::e2e_tests::default_works' panicked at 'We should find a port before the reader ends'
        }
        
    }
}
