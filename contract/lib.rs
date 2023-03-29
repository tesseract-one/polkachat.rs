#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod texter {
    use ink::prelude::string::*;
    use ink::prelude::vec::*;
    use ink::storage::Mapping;

    #[ink(storage)]
    pub struct Texter {
        texts: Mapping<u32, (AccountId, String)>,
        length: u32,
    }

    #[ink(event)]
    pub struct MessageAdded {
        id: u32,
        sender: AccountId,
        text: String,
    }

    impl Texter {
        #[ink(constructor)]
        pub fn default() -> Self {
            Self {
                texts: Default::default(),
                length: 0,
            }
        }

        #[ink(message)]
        pub fn add(&mut self, text: String) {
            self.texts
                .insert(self.length, &(self.env().caller(), text.clone()));
            self.env().emit_event(MessageAdded {
                id: self.length,
                sender: self.env().caller(),
                text,
            });
            self.length += 1;
        }

        #[ink(message)]
        pub fn get(&self, from: u32, to: u32) -> Vec<(u32, AccountId, String)> {
            (from..to)
                .into_iter()
                .map(|id| {
                    let (acc, text) = self.texts.get(id).unwrap();
                    (id, acc, text)
                })
                .collect()
        }

        #[ink(message)]
        pub fn len(&self) -> u32 {
            self.length
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        type Event = <Texter as ::ink::reflect::ContractEventBase>::Type;

        fn assert_message_event(
            event: &ink::env::test::EmittedEvent,
            expected_id: u32,
            expected_sender: AccountId,
            expected_text: String,
        ) {
            let decoded_event = <Event as scale::Decode>::decode(&mut &event.data[..])
                .expect("encountered invalid contract event data buffer");
            let Event::MessageAdded(MessageAdded { id, sender, text }) = decoded_event;
            assert_eq!(id, expected_id, "encountered invalid MessageAdded.id");
            assert_eq!(
                sender, expected_sender,
                "encountered invalid MessageAdded.sender"
            );
            assert_eq!(text, expected_text, "encountered invalid MessageAdded.text");
        }

        #[ink::test]
        fn default_works() {
            let texter = Texter::default();
            assert_eq!(texter.len(), 0);
            assert_eq!(texter.get(0, 0), vec![]);
        }

        #[ink::test]
        fn add_and_get_works() {
            let mut texter = Texter::default();
            let texts = vec![
                String::from("0"),
                String::from("1"),
                String::from("2"),
                String::from("3"),
                String::from("4"),
            ];
            texts.iter().for_each(|text| texter.add(text.clone()));

            let callee = ink::env::test::callee::<Environment>();
            let events: Vec<_> = ink::env::test::recorded_events().collect();
            assert_eq!(events.len(), texts.len());
            events
                .iter()
                .zip(texts.iter())
                .enumerate()
                .for_each(|(id, (event, text))| {
                    assert_message_event(event, id as u32, callee, text.clone());
                });

            assert_eq!(
                texter.get(1, 3),
                vec![
                    (1, callee, String::from("1")),
                    (2, callee, String::from("2"))
                ]
            );
        }

        #[ink::test]
        fn len_works() {
            let mut texter = Texter::default();
            let texts = vec![
                String::from("0"),
                String::from("1"),
                String::from("2"),
                String::from("3"),
                String::from("4"),
            ];
            texts.iter().for_each(|text| texter.add(text.clone()));
            assert_eq!(texter.len(), 5);
        }
    }
    // TODO: Add e2e-tests
}
