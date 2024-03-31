#[doc = "Register `GPIO4C_P` reader"]
pub type R = crate::R<Gpio4cPSpec>;
#[doc = "Register `GPIO4C_P` writer"]
pub type W = crate::W<Gpio4cPSpec>;
#[doc = "GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c0P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4c0P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c0P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c0P {
    type Ux = u8;
}
#[doc = "Field `GPIO4C0_P` reader - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c0PR = crate::FieldReader<Gpio4c0P>;
impl Gpio4c0PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c0P {
        match self.bits {
            0 => Gpio4c0P::B00,
            1 => Gpio4c0P::B01,
            2 => Gpio4c0P::B10,
            3 => Gpio4c0P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c0P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c0P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c0P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c0P::B11
    }
}
#[doc = "Field `GPIO4C0_P` writer - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c0PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c0P>;
impl<'a, REG> Gpio4c0PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c0P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c0P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c0P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c0P::B11)
    }
}
#[doc = "GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c1P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4c1P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c1P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c1P {
    type Ux = u8;
}
#[doc = "Field `GPIO4C1_P` reader - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c1PR = crate::FieldReader<Gpio4c1P>;
impl Gpio4c1PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c1P {
        match self.bits {
            0 => Gpio4c1P::B00,
            1 => Gpio4c1P::B01,
            2 => Gpio4c1P::B10,
            3 => Gpio4c1P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c1P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c1P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c1P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c1P::B11
    }
}
#[doc = "Field `GPIO4C1_P` writer - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c1PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c1P>;
impl<'a, REG> Gpio4c1PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c1P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c1P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c1P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c1P::B11)
    }
}
#[doc = "GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c2P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4c2P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c2P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c2P {
    type Ux = u8;
}
#[doc = "Field `GPIO4C2_P` reader - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c2PR = crate::FieldReader<Gpio4c2P>;
impl Gpio4c2PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c2P {
        match self.bits {
            0 => Gpio4c2P::B00,
            1 => Gpio4c2P::B01,
            2 => Gpio4c2P::B10,
            3 => Gpio4c2P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c2P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c2P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c2P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c2P::B11
    }
}
#[doc = "Field `GPIO4C2_P` writer - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c2PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c2P>;
impl<'a, REG> Gpio4c2PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c2P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c2P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c2P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c2P::B11)
    }
}
#[doc = "GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c3P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4c3P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c3P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c3P {
    type Ux = u8;
}
#[doc = "Field `GPIO4C3_P` reader - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c3PR = crate::FieldReader<Gpio4c3P>;
impl Gpio4c3PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c3P {
        match self.bits {
            0 => Gpio4c3P::B00,
            1 => Gpio4c3P::B01,
            2 => Gpio4c3P::B10,
            3 => Gpio4c3P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c3P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c3P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c3P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c3P::B11
    }
}
#[doc = "Field `GPIO4C3_P` writer - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c3PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c3P>;
impl<'a, REG> Gpio4c3PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c3P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c3P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c3P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c3P::B11)
    }
}
#[doc = "GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c4P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4c4P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c4P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c4P {
    type Ux = u8;
}
#[doc = "Field `GPIO4C4_P` reader - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c4PR = crate::FieldReader<Gpio4c4P>;
impl Gpio4c4PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c4P {
        match self.bits {
            0 => Gpio4c4P::B00,
            1 => Gpio4c4P::B01,
            2 => Gpio4c4P::B10,
            3 => Gpio4c4P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c4P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c4P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c4P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c4P::B11
    }
}
#[doc = "Field `GPIO4C4_P` writer - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c4PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c4P>;
impl<'a, REG> Gpio4c4PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c4P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c4P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c4P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c4P::B11)
    }
}
#[doc = "GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c5P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4c5P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c5P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c5P {
    type Ux = u8;
}
#[doc = "Field `GPIO4C5_P` reader - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c5PR = crate::FieldReader<Gpio4c5P>;
impl Gpio4c5PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c5P {
        match self.bits {
            0 => Gpio4c5P::B00,
            1 => Gpio4c5P::B01,
            2 => Gpio4c5P::B10,
            3 => Gpio4c5P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c5P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c5P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c5P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c5P::B11
    }
}
#[doc = "Field `GPIO4C5_P` writer - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c5PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c5P>;
impl<'a, REG> Gpio4c5PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c5P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c5P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c5P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c5P::B11)
    }
}
#[doc = "GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c6P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4c6P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c6P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c6P {
    type Ux = u8;
}
#[doc = "Field `GPIO4C6_P` reader - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c6PR = crate::FieldReader<Gpio4c6P>;
impl Gpio4c6PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c6P {
        match self.bits {
            0 => Gpio4c6P::B00,
            1 => Gpio4c6P::B01,
            2 => Gpio4c6P::B10,
            3 => Gpio4c6P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c6P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c6P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c6P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c6P::B11
    }
}
#[doc = "Field `GPIO4C6_P` writer - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c6PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c6P>;
impl<'a, REG> Gpio4c6PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c6P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c6P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c6P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c6P::B11)
    }
}
#[doc = "GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c7P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4c7P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c7P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c7P {
    type Ux = u8;
}
#[doc = "Field `GPIO4C7_P` reader - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c7PR = crate::FieldReader<Gpio4c7P>;
impl Gpio4c7PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c7P {
        match self.bits {
            0 => Gpio4c7P::B00,
            1 => Gpio4c7P::B01,
            2 => Gpio4c7P::B10,
            3 => Gpio4c7P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c7P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c7P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c7P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c7P::B11
    }
}
#[doc = "Field `GPIO4C7_P` writer - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4c7PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c7P>;
impl<'a, REG> Gpio4c7PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c7P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c7P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c7P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c7P::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c0_p(&self) -> Gpio4c0PR {
        Gpio4c0PR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c1_p(&self) -> Gpio4c1PR {
        Gpio4c1PR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c2_p(&self) -> Gpio4c2PR {
        Gpio4c2PR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c3_p(&self) -> Gpio4c3PR {
        Gpio4c3PR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c4_p(&self) -> Gpio4c4PR {
        Gpio4c4PR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c5_p(&self) -> Gpio4c5PR {
        Gpio4c5PR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c6_p(&self) -> Gpio4c6PR {
        Gpio4c6PR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c7_p(&self) -> Gpio4c7PR {
        Gpio4c7PR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c0_p(&mut self) -> Gpio4c0PW<Gpio4cPSpec> {
        Gpio4c0PW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c1_p(&mut self) -> Gpio4c1PW<Gpio4cPSpec> {
        Gpio4c1PW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c2_p(&mut self) -> Gpio4c2PW<Gpio4cPSpec> {
        Gpio4c2PW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c3_p(&mut self) -> Gpio4c3PW<Gpio4cPSpec> {
        Gpio4c3PW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c4_p(&mut self) -> Gpio4c4PW<Gpio4cPSpec> {
        Gpio4c4PW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c5_p(&mut self) -> Gpio4c5PW<Gpio4cPSpec> {
        Gpio4c5PW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c6_p(&mut self) -> Gpio4c6PW<Gpio4cPSpec> {
        Gpio4c6PW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO4C PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c7_p(&mut self) -> Gpio4c7PW<Gpio4cPSpec> {
        Gpio4c7PW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio4cPSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO4C PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4c_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4c_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio4cPSpec;
impl crate::RegisterSpec for Gpio4cPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio4c_p::R`](R) reader structure"]
impl crate::Readable for Gpio4cPSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio4c_p::W`](W) writer structure"]
impl crate::Writable for Gpio4cPSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO4C_P to value 0x6965"]
impl crate::Resettable for Gpio4cPSpec {
    const RESET_VALUE: u32 = 0x6965;
}
