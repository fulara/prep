use std::io;
use std::io::Read;

extern crate getch;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum InteractionResult {
    Accept,
    Reject,
}

pub fn ask_user(about: &str) -> InteractionResult {
    ask_user_impl(about, &mut RealInteractor {});
    InteractionResult::Accept
}

fn ask_user_impl(about: &str, interactor: &mut Interactor) -> InteractionResult {
    match interactor.get_key() as char {
        'y' => InteractionResult::Accept,
        _ => InteractionResult::Reject,
    }
}

trait Interactor {
    fn get_key(&mut self) -> u8;
}

struct RealInteractor;
impl Interactor for RealInteractor {
    fn get_key(&mut self) -> u8 {
        if let Ok(getch) = getch::Getch::new() {
            if let Ok(char) = getch.getch() {
                return char;
            };
        };

        let mut buf = [0u8];
        io::stdin().read_exact(&mut buf).expect(
            "Failed to read from terminal",
        );

        return buf[0];
    }
}

#[cfg(test)]
mod interactor_test {
    struct FakeInteractor {
        fake_inputs: Vec<u8>,
        current_index: usize,
    }

    impl Interactor for FakeInteractor {
        fn get_key(&mut self) -> u8 {
            if self.current_index >= self.fake_inputs.len() {
                panic!("not enough inputs in fake interactor");
            }

            let key = self.fake_inputs[self.current_index];

            self.current_index += 1;

            key
        }
    }

    impl FakeInteractor {
        fn new(inputs: Vec<u8>) -> FakeInteractor {
            FakeInteractor {
                fake_inputs: inputs,
                current_index: 0,
            }
        }
    }

    use super::*;
    #[test]
    fn testing_interaction() {
        assert_eq!(
            InteractionResult::Reject,
            ask_user_impl(
                "ask",
                &mut FakeInteractor::new(String::from("a").into_bytes()),
            )
        );

        assert_eq!(
            InteractionResult::Accept,
            ask_user_impl(
                "ask",
                &mut FakeInteractor::new(String::from("y").into_bytes()),
            )
        );

    }
}
