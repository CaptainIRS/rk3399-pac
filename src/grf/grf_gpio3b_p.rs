#[doc = "Register `GRF_GPIO3B_P` reader"]
pub type R = crate::R<GrfGpio3bPSpec>;
#[doc = "Register `GRF_GPIO3B_P` writer"]
pub type W = crate::W<GrfGpio3bPSpec>;
#[doc = "GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b0P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3b0P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b0P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b0P {
    type Ux = u8;
}
#[doc = "Field `GPIO3B0_P` reader - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b0PR = crate::FieldReader<Gpio3b0P>;
impl Gpio3b0PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3b0P {
        match self.bits {
            0 => Gpio3b0P::B00,
            1 => Gpio3b0P::B01,
            2 => Gpio3b0P::B10,
            3 => Gpio3b0P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b0P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b0P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3b0P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3b0P::B11
    }
}
#[doc = "Field `GPIO3B0_P` writer - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b0PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3b0P>;
impl<'a, REG> Gpio3b0PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b0P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b0P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b0P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b0P::B11)
    }
}
#[doc = "GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b1P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3b1P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b1P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b1P {
    type Ux = u8;
}
#[doc = "Field `GPIO3B1_P` reader - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b1PR = crate::FieldReader<Gpio3b1P>;
impl Gpio3b1PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3b1P {
        match self.bits {
            0 => Gpio3b1P::B00,
            1 => Gpio3b1P::B01,
            2 => Gpio3b1P::B10,
            3 => Gpio3b1P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b1P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b1P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3b1P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3b1P::B11
    }
}
#[doc = "Field `GPIO3B1_P` writer - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b1PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3b1P>;
impl<'a, REG> Gpio3b1PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b1P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b1P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b1P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b1P::B11)
    }
}
#[doc = "GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b2P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3b2P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b2P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b2P {
    type Ux = u8;
}
#[doc = "Field `GPIO3B2_P` reader - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b2PR = crate::FieldReader<Gpio3b2P>;
impl Gpio3b2PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3b2P {
        match self.bits {
            0 => Gpio3b2P::B00,
            1 => Gpio3b2P::B01,
            2 => Gpio3b2P::B10,
            3 => Gpio3b2P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b2P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b2P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3b2P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3b2P::B11
    }
}
#[doc = "Field `GPIO3B2_P` writer - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b2PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3b2P>;
impl<'a, REG> Gpio3b2PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b2P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b2P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b2P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b2P::B11)
    }
}
#[doc = "GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b3P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3b3P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b3P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b3P {
    type Ux = u8;
}
#[doc = "Field `GPIO3B3_P` reader - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b3PR = crate::FieldReader<Gpio3b3P>;
impl Gpio3b3PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3b3P {
        match self.bits {
            0 => Gpio3b3P::B00,
            1 => Gpio3b3P::B01,
            2 => Gpio3b3P::B10,
            3 => Gpio3b3P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b3P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b3P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3b3P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3b3P::B11
    }
}
#[doc = "Field `GPIO3B3_P` writer - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b3PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3b3P>;
impl<'a, REG> Gpio3b3PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b3P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b3P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b3P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b3P::B11)
    }
}
#[doc = "GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b4P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3b4P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b4P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b4P {
    type Ux = u8;
}
#[doc = "Field `GPIO3B4_P` reader - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b4PR = crate::FieldReader<Gpio3b4P>;
impl Gpio3b4PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3b4P {
        match self.bits {
            0 => Gpio3b4P::B00,
            1 => Gpio3b4P::B01,
            2 => Gpio3b4P::B10,
            3 => Gpio3b4P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b4P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b4P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3b4P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3b4P::B11
    }
}
#[doc = "Field `GPIO3B4_P` writer - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b4PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3b4P>;
impl<'a, REG> Gpio3b4PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b4P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b4P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b4P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b4P::B11)
    }
}
#[doc = "GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b5P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3b5P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b5P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b5P {
    type Ux = u8;
}
#[doc = "Field `GPIO3B5_P` reader - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b5PR = crate::FieldReader<Gpio3b5P>;
impl Gpio3b5PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3b5P {
        match self.bits {
            0 => Gpio3b5P::B00,
            1 => Gpio3b5P::B01,
            2 => Gpio3b5P::B10,
            3 => Gpio3b5P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b5P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b5P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3b5P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3b5P::B11
    }
}
#[doc = "Field `GPIO3B5_P` writer - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b5PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3b5P>;
impl<'a, REG> Gpio3b5PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b5P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b5P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b5P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b5P::B11)
    }
}
#[doc = "GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b6P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3b6P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b6P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b6P {
    type Ux = u8;
}
#[doc = "Field `GPIO3B6_P` reader - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b6PR = crate::FieldReader<Gpio3b6P>;
impl Gpio3b6PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3b6P {
        match self.bits {
            0 => Gpio3b6P::B00,
            1 => Gpio3b6P::B01,
            2 => Gpio3b6P::B10,
            3 => Gpio3b6P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b6P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b6P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3b6P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3b6P::B11
    }
}
#[doc = "Field `GPIO3B6_P` writer - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b6PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3b6P>;
impl<'a, REG> Gpio3b6PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b6P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b6P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b6P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b6P::B11)
    }
}
#[doc = "GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3b7P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3b7P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3b7P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3b7P {
    type Ux = u8;
}
#[doc = "Field `GPIO3B7_P` reader - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b7PR = crate::FieldReader<Gpio3b7P>;
impl Gpio3b7PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3b7P {
        match self.bits {
            0 => Gpio3b7P::B00,
            1 => Gpio3b7P::B01,
            2 => Gpio3b7P::B10,
            3 => Gpio3b7P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3b7P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3b7P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3b7P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3b7P::B11
    }
}
#[doc = "Field `GPIO3B7_P` writer - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3b7PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3b7P>;
impl<'a, REG> Gpio3b7PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b7P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b7P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b7P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3b7P::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3b0_p(&self) -> Gpio3b0PR {
        Gpio3b0PR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3b1_p(&self) -> Gpio3b1PR {
        Gpio3b1PR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3b2_p(&self) -> Gpio3b2PR {
        Gpio3b2PR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3b3_p(&self) -> Gpio3b3PR {
        Gpio3b3PR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3b4_p(&self) -> Gpio3b4PR {
        Gpio3b4PR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3b5_p(&self) -> Gpio3b5PR {
        Gpio3b5PR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3b6_p(&self) -> Gpio3b6PR {
        Gpio3b6PR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3b7_p(&self) -> Gpio3b7PR {
        Gpio3b7PR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b0_p(&mut self) -> Gpio3b0PW<GrfGpio3bPSpec> {
        Gpio3b0PW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b1_p(&mut self) -> Gpio3b1PW<GrfGpio3bPSpec> {
        Gpio3b1PW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b2_p(&mut self) -> Gpio3b2PW<GrfGpio3bPSpec> {
        Gpio3b2PW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b3_p(&mut self) -> Gpio3b3PW<GrfGpio3bPSpec> {
        Gpio3b3PW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b4_p(&mut self) -> Gpio3b4PW<GrfGpio3bPSpec> {
        Gpio3b4PW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b5_p(&mut self) -> Gpio3b5PW<GrfGpio3bPSpec> {
        Gpio3b5PW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b6_p(&mut self) -> Gpio3b6PW<GrfGpio3bPSpec> {
        Gpio3b6PW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO3B PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3b7_p(&mut self) -> Gpio3b7PW<GrfGpio3bPSpec> {
        Gpio3b7PW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio3bPSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO3B PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3b_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3b_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio3bPSpec;
impl crate::RegisterSpec for GrfGpio3bPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio3b_p::R`](R) reader structure"]
impl crate::Readable for GrfGpio3bPSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio3b_p::W`](W) writer structure"]
impl crate::Writable for GrfGpio3bPSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO3B_P to value 0x5559"]
impl crate::Resettable for GrfGpio3bPSpec {
    const RESET_VALUE: u32 = 0x5559;
}
