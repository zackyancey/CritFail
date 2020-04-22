use std::fmt;

use crate::{CheckRoll, DamageRoll};

pub struct AttackRoll {
    check: CheckRoll,
    damage: DamageRoll,
}

impl AttackRoll {
    pub fn new(check: CheckRoll, damage: DamageRoll) -> AttackRoll {
        AttackRoll { check, damage }
    }
}

impl fmt::Display for AttackRoll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} ? {}", self.check, self.damage )
    }
}

impl fmt::Debug for AttackRoll {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ? {:?}", self.check, self.damage )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AdvState::*;
    use crate::DamageRollPart::Dice as Dr;
    use crate::DamageRollPart::Modifier as Mr;

    #[test]
    fn no_modifier() {
        let r = AttackRoll::new(
            CheckRoll::new(&Neutral, 10, 16, DamageRoll::new(vec![])),
            DamageRoll::new(vec![Dr(6, vec![5, 4]), Mr(4)]),
        );

        assert_eq!(format!("{}", r), "10 ? 13");
        assert_eq!(format!("{:?}", r), "(10) ? [5+4]+4");
    }

    #[test]
    fn with_modifier() {
        let r = AttackRoll::new(
            CheckRoll::new(&Disadvantage, 5, 12, DamageRoll::new(vec![Mr(3)])),
            DamageRoll::new(vec![Dr(8, vec![2, 6, 8]), Mr(-2)]),
        );

        assert_eq!(format!("{}", r), "8 ? 14");
        assert_eq!(format!("{:?}", r), "(5/12)+3 ? [2+6+8]-2");
    }

    #[test]
    fn critical() {
        let r = AttackRoll::new(
            CheckRoll::new(&Advantage, 20, 4, DamageRoll::new(vec![Mr(3)])),
            DamageRoll::new(vec![Dr(8, vec![2, 6, 8]), Dr(8, vec![1, 5, 2]), Mr(-2)]),
        );

        assert_eq!(format!("{}", r), "Critical ? 22");
        assert_eq!(format!("{:?}", r), "(20/4)+3 ? [2+6+8]+[1+5+2]-2");
    }

    #[test]
    fn critfail() {
        let r = AttackRoll::new(
            CheckRoll::new(&Disadvantage, 15, 1, DamageRoll::new(vec![Mr(3)])),
            DamageRoll::new(vec![Dr(8, vec![3, 1]), Mr(-2)]),
        );

        assert_eq!(format!("{}", r), "Fail ? 2");
        assert_eq!(format!("{:?}", r), "(1/15)+3 ? [3+1]-2");
    }
}
