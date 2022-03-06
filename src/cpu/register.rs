use bitflags::bitflags;

pub type ProgramCounter = u16;
pub type A = u8;
pub type X = u8;
pub type Y = u8;
pub type StackPointer = u8;

bitflags! {
    pub struct ProcFlag: u8 {
        const NEGATIVE = 0b1000_0000;
        const OVERFLOW = 0b0100_0000;
        const DECIMAL = 0b0000_1000;
        const INTERRUPT_DISABLE = 0b0000_0100;
        const ZERO = 0b0000_0010;
        const CARRY = 0b0000_0001;
    }
}

#[derive(Debug)]
pub struct Status {
    status: ProcFlag,
}

impl Status {
    pub fn new() -> Status {
        Status {
            status: ProcFlag::empty(),
        }
    }

    pub fn is_set(&self, flag: ProcFlag) -> bool {
        self.status.contains(flag)
    }

    pub fn is_unset(&self, flag: ProcFlag) -> bool {
        !self.is_set(flag)
    }
}

impl std::cmp::PartialEq<u8> for Status {
    fn eq(&self, o: &u8) -> bool {
        self.status.bits() == *o
    }
}

impl std::cmp::PartialEq<ProcFlag> for Status {
    fn eq(&self, o: &ProcFlag) -> bool {
        self.status == *o
    }
}

impl std::ops::BitOrAssign<ProcFlag> for Status {
    fn bitor_assign(&mut self, flag: ProcFlag) {
        self.status = self.status | flag;
    }
}

impl std::ops::BitAndAssign<ProcFlag> for Status {
    fn bitand_assign(&mut self, flag: ProcFlag) {
        self.status = self.status & flag
    }
}

#[cfg(test)]
mod tests {
    use super::{ProcFlag, Status};

    #[test]
    fn set_a_flag() {
        let mut status = Status::new();
        status |= ProcFlag::DECIMAL;
        assert!(status.is_set(ProcFlag::DECIMAL));
        assert!(!status.is_unset(ProcFlag::DECIMAL));
        assert_eq!(status, 0b0000_1000);
        assert_eq!(status, ProcFlag::DECIMAL);
    }

    #[test]
    fn unset_a_flag() {
        let mut status = Status::new();
        status |= ProcFlag::DECIMAL;
        assert_eq!(status, 0b0000_1000);
        assert_eq!(status, ProcFlag::DECIMAL);
        status &= !ProcFlag::DECIMAL;
        assert!(!status.is_set(ProcFlag::DECIMAL));
        assert!(status.is_unset(ProcFlag::DECIMAL));
        assert_eq!(status, 0b0000_0000);
        assert_eq!(status, ProcFlag::empty());
    }

    #[test]
    fn set_multiple_flags_at_once() {
        let mut status = Status::new();
        status |= ProcFlag::DECIMAL | ProcFlag::CARRY | ProcFlag::NEGATIVE;
        assert!(status.is_set(ProcFlag::DECIMAL));
        assert!(status.is_set(ProcFlag::CARRY));
        assert!(status.is_set(ProcFlag::NEGATIVE));
        assert!(status.is_set(ProcFlag::DECIMAL | ProcFlag::CARRY | ProcFlag::NEGATIVE));
        assert!(!status.is_unset(ProcFlag::DECIMAL));
        assert!(!status.is_unset(ProcFlag::CARRY));
        assert!(!status.is_unset(ProcFlag::NEGATIVE));
        assert!(!status.is_unset(ProcFlag::DECIMAL | ProcFlag::CARRY | ProcFlag::NEGATIVE));
        assert_eq!(status, 0b1000_1001);
        assert_eq!(
            status,
            ProcFlag::DECIMAL | ProcFlag::CARRY | ProcFlag::NEGATIVE
        );
    }

    #[test]
    fn unset_multiple_flags_at_once() {
        let mut status = Status::new();
        status |=
            ProcFlag::OVERFLOW | ProcFlag::INTERRUPT_DISABLE | ProcFlag::ZERO | ProcFlag::DECIMAL;
        assert_eq!(status, 0b0100_1110);
        assert_eq!(
            status,
            ProcFlag::OVERFLOW | ProcFlag::INTERRUPT_DISABLE | ProcFlag::ZERO | ProcFlag::DECIMAL
        );
        status &= !(ProcFlag::INTERRUPT_DISABLE | ProcFlag::ZERO);
        assert!(status.is_set(ProcFlag::OVERFLOW | ProcFlag::DECIMAL));
        assert!(status.is_unset(ProcFlag::INTERRUPT_DISABLE | ProcFlag::ZERO | ProcFlag::NEGATIVE));
        assert_eq!(status, 0b0100_1000);
        assert_eq!(status, ProcFlag::OVERFLOW | ProcFlag::DECIMAL);
    }

    #[test]
    fn test_subset_of_already_set_flags() {
        let mut status = Status::new();
        status |=
            ProcFlag::INTERRUPT_DISABLE | ProcFlag::ZERO | ProcFlag::CARRY | ProcFlag::NEGATIVE;
        assert!(status.is_set(ProcFlag::INTERRUPT_DISABLE | ProcFlag::ZERO));
        assert!(status.is_set(ProcFlag::CARRY | ProcFlag::NEGATIVE));
        assert!(status.is_unset(ProcFlag::DECIMAL | ProcFlag::OVERFLOW));
        assert_eq!(status, 0b1000_0111)
    }
}
