pub trait CheckedAdd: Sized {
    fn checked_add(&self, v: &Self) -> Option<Self>;
}

pub trait CheckedSub: Sized {
    fn checked_sub(&self, v: &Self) -> Option<Self>;
}

pub trait CheckedMul: Sized {
    fn checked_mul(&self, v: &Self) -> Option<Self>;
}

pub trait CheckedDiv: Sized {
    fn checked_div(&self, v: &Self) -> Option<Self>;
}

pub trait CheckedRem: Sized {
    fn checked_rem(&self, v: &Self) -> Option<Self>;
}
