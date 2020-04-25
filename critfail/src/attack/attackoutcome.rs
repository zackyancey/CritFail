use std::fmt;

use crate::{CheckOutcome, DamageOutcome};

#[derive(Clone)]
pub struct AttackOutcome {
    check: CheckOutcome,
    damage: DamageOutcome,
}

impl AttackOutcome {
    pub fn new(check: CheckOutcome, damage: DamageOutcome) -> AttackOutcome {
        AttackOutcome { check, damage }
    }

    pub fn check(&self) -> &CheckOutcome {
        &self.check
    }

    pub fn damage(&self) -> &DamageOutcome {
        &self.damage
    }
}

impl fmt::Display for AttackOutcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ? {}", self.check, self.damage)
    }
}

impl fmt::Debug for AttackOutcome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ? {:?}", self.check, self.damage)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AdvState::*;
    use crate::OutcomePart::Dice as D;
    use crate::OutcomePart::Modifier as M;

    #[test]
    fn no_modifier() {
        let r = AttackOutcome::new(
            CheckOutcome::new(&Neutral, 10, 16, vec![]),
            DamageOutcome::new(vec![D(6, vec![5, 4]), M(4)]),
        );

        assert_eq!(format!("{}", r), "10 ? 13");
        assert_eq!(format!("{:?}", r), "(10) ? [5+4]+4");
    }

    #[test]
    fn with_modifier() {
        let r = AttackOutcome::new(
            CheckOutcome::new(&Disadvantage, 5, 12, vec![M(3)]),
            DamageOutcome::new(vec![D(8, vec![2, 6, 8]), M(-2)]),
        );

        assert_eq!(format!("{}", r), "8 ? 14");
        assert_eq!(format!("{:?}", r), "(5/12)+3 ? [2+6+8]-2");
    }

    #[test]
    fn critical() {
        let r = AttackOutcome::new(
            CheckOutcome::new(&Advantage, 20, 4, vec![M(3)]),
            DamageOutcome::new(vec![D(8, vec![2, 6, 8]), D(8, vec![1, 5, 2]), M(-2)]),
        );

        assert_eq!(format!("{}", r), "Critical ? 22");
        assert_eq!(format!("{:?}", r), "(20/4)+3 ? [2+6+8]+[1+5+2]-2");
    }

    #[test]
    fn critfail() {
        let r = AttackOutcome::new(
            CheckOutcome::new(&Disadvantage, 15, 1, vec![M(3)]),
            DamageOutcome::new(vec![D(8, vec![3, 1]), M(-2)]),
        );

        assert_eq!(format!("{}", r), "Fail ? 2");
        assert_eq!(format!("{:?}", r), "(1/15)+3 ? [3+1]-2");
    }
}
