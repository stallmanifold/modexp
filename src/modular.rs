use modinv::ModInv;
use modmult::ModMult;
use std::ops::{Add, Mul, Sub, Neg};
use std::fmt::Debug;
use num::{Zero, One, Integer, Bounded};


#[derive(Clone, Debug)]
pub struct Mod<I> where I: Clone + Debug {
    value: I,
    modulus: I,
}

impl<I> Mod<I> where I: Debug + Clone + Eq + ModInv<I> + Integer {
    fn new(value: &I, modulus: &I) -> Mod<I> {
        Mod {
            value: value.mod_floor(modulus),
            modulus: modulus.clone(),
        }
    }

    fn un_mod(&self) -> I {
        self.value.clone()
    }

    fn inv(&self) -> Option<Mod<I>> {
        self.value.mod_inv(&self.modulus).map(|val| { 
            Mod::new(&val, &self.modulus) 
        })
    }

    fn has_inv(&self) -> bool {
        self.inv().is_some()
    }

    fn zero(modulus: &I) -> Mod<I> {
        Mod::new(&Zero::zero(), modulus)
    }

    fn one(modulus: &I) -> Mod<I> {
        Mod::new(&One::one(), modulus)
    }
}

impl<I> Add<Mod<I>> for Mod<I> 
    where I: Clone + Eq + Debug + Add<I, Output=I> + ModInv<I> + Integer 
{
    type Output = Mod<I>;

    fn add(self, rhs: Mod<I>) -> Mod<I> {
        assert_eq!(self.value, rhs.value);

        let new_value = (self.value + rhs.value).mod_floor(&self.modulus);
        
        Mod::new(&new_value, &self.modulus)
    }
}

impl<'a, I> Add<&'a Mod<I>> for Mod<I>
    where I: Clone + Eq + Debug + Add<&'a I, Output=I> + ModInv<I> + Integer
{
    type Output = Mod<I>;

    fn add(self, rhs: &'a Mod<I>) -> Mod<I> {
        assert_eq!(self.value, rhs.value);

        let new_value = (self.value + &rhs.value).mod_floor(&self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

impl<'a, I> Add<Mod<I>> for &'a Mod<I>
    where I: Clone + Eq + Debug + Add<&'a I, Output=I> + ModInv<I> + Integer
{
    type Output = Mod<I>;

    fn add(self, rhs: Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = (rhs.value + &self.value).mod_floor(&self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

impl<'a, 'b, I> Add<&'a Mod<I>> for &'b Mod<I>
    where I: Clone + Eq + Debug + Add<&'a I, Output=I> + ModInv<I> + Integer,
          &'b I: Add<&'a I, Output=I>
{
    type Output = Mod<I>;

    fn add(self, rhs: &'a Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = (&self.value + &rhs.value).mod_floor(&self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

impl<I> Mul<Mod<I>> for Mod<I>
    where I: Clone + Eq + Debug + Mul<I, Output=I> + ModInv<I> + ModMult + Integer
{
    type Output = Mod<I>;

    fn mul(self, rhs: Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = self.value.mod_mult(&rhs.value, &self.modulus);
        
        Mod::new(&new_value, &self.modulus)
    }
}

impl<'a, I> Mul<&'a Mod<I>> for Mod<I>
    where I: Clone + Eq + Debug + Mul<&'a I, Output=I> + ModInv<I> + ModMult + Integer
{
    type Output = Mod<I>;

    fn mul(self, rhs: &'a Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = self.value.mod_mult(&rhs.value, &self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

impl<'a, I> Mul<Mod<I>> for &'a Mod<I>
    where I: Clone + Eq + Debug + Mul<&'a I, Output=I> + ModInv<I> + ModMult + Integer
{
    type Output = Mod<I>;

    fn mul(self, rhs: Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = self.value.mod_mult(&rhs.value, &self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

impl<'a, 'b, I> Mul<&'a Mod<I>> for &'b Mod<I>
    where I: Clone + Eq + Debug + Mul<&'a I, Output=I> + ModInv<I> + ModMult + Integer,
          &'b I: ModMult
{
    type Output = Mod<I>;

    fn mul(self, rhs: &'a Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = self.value.mod_mult(&rhs.value, &self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

impl<I> Sub<Mod<I>> for Mod<I>
    where I: Clone + Eq + Debug + Sub<I, Output=I> + ModInv<I> + Integer
{
    type Output = Mod<I>;

    fn sub(self, rhs: Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = (self.value + rhs.value).mod_floor(&self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

impl<'a, I> Sub<&'a Mod<I>> for Mod<I>
    where I: Clone + Eq + Debug + Sub<&'a I, Output=I> + ModInv<I> + Integer
{
    type Output = Mod<I>;

    fn sub(self, rhs: &'a Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = (self.value - &rhs.value).mod_floor(&self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

impl<'a, I> Sub<Mod<I>> for &'a Mod<I>
    where I: Clone + Eq + Debug + Sub<&'a I, Output=I> + ModInv<I> + Integer,
          &'a I: Sub<I, Output=I>
{
    type Output = Mod<I>;

    fn sub(self, rhs: Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = (&self.value - rhs.value).mod_floor(&self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

impl<'a, 'b, I> Sub<&'a Mod<I>> for &'b Mod<I>
    where I: Clone + Eq + Debug + Sub<&'a I, Output=I> + ModInv<I> + Integer,
          &'b I: Sub<&'a I, Output=I>
{
    type Output = Mod<I>;

    fn sub(self, rhs: &'a Mod<I>) -> Mod<I> {
        assert_eq!(self.modulus, rhs.modulus);

        let new_value = (&self.value - &rhs.value).mod_floor(&self.modulus);

        Mod::new(&new_value, &self.modulus)
    }
}

impl<I> Neg for Mod<I> 
    where I: Eq + Clone + Debug + ModInv<I> + Neg<Output=I> + Integer
{    
    type Output = Mod<I>;

    fn neg(self) -> Mod<I> {
        Mod::new(&(-self.value), &self.modulus)
    }
} 

impl<'a, I> Neg for &'a Mod<I> 
    where I: Eq + Clone + Debug + ModInv<I> + Integer,
          &'a I: Neg<Output=I>
{
    type Output = Mod<I>;

    fn neg(self) -> Mod<I> {
        let neg_val = - &self.value;

        Mod::new(&neg_val, &self.modulus)
    }
}


#[cfg(test)]
mod tests {

} 