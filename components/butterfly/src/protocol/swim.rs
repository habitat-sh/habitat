include!("../generated/butterfly.swim.rs");

pub use self::{membership::Health,
               swim::{Payload as SwimPayload,
                      Type as SwimType}};

#[cfg(test)]
mod tests {
    use super::*;

    use serde_json;

    // Theis test assures that we can properly compare Health values
    // along the spectrum of
    //
    //   Alive < Suspect < Confirmed < Departed
    //
    // since that is important in our decision whether or not to
    // propagate membership rumors.
    #[test]
    fn health_is_properly_ordered() {
        assert!(Health::Alive < Health::Suspect);
        assert!(Health::Suspect < Health::Confirmed);
        assert!(Health::Confirmed < Health::Departed);
    }

    #[test]
    fn health_values_are_title_case() {
        assert_eq!(serde_json::to_value(&Health::Alive).unwrap(), "Alive");
        assert_eq!(serde_json::to_value(&Health::Suspect).unwrap(), "Suspect");
        assert_eq!(serde_json::to_value(&Health::Confirmed).unwrap(),
                   "Confirmed");
        assert_eq!(serde_json::to_value(&Health::Departed).unwrap(), "Departed");
    }
}
