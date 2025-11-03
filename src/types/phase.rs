use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Phase {
    Betting,
    Dealing,
    PlayerTurns,
    DealerTurn,
    Payout,
    RoundEnd,
}

impl Phase {
    pub fn next(&self) -> Self {
        match self {
            Phase::Betting => Phase::Dealing,
            Phase::Dealing => Phase::PlayerTurns,
            Phase::PlayerTurns => Phase::DealerTurn,
            Phase::DealerTurn => Phase::Payout,
            Phase::Payout => Phase::RoundEnd,
            Phase::RoundEnd => Phase::Betting,
        }
    }
}

impl Default for Phase {
    fn default() -> Self {
        Phase::Betting
    }
}

#[test]
fn phase_progression() {
    let mut phase = Phase::Betting;

    phase = phase.next();
    assert_eq!(phase, Phase::Dealing);

    phase = phase.next();
    assert_eq!(phase, Phase::PlayerTurns);

    phase = phase.next();
    assert_eq!(phase, Phase::DealerTurn);

    phase = phase.next();
    assert_eq!(phase, Phase::Payout);

    phase = phase.next();
    assert_eq!(phase, Phase::RoundEnd);

    phase = phase.next();
    assert_eq!(phase, Phase::Betting);
}
