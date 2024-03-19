#[doc = "Register `GRF_GPIO4A_P` reader"]
pub type R = crate::R<GrfGpio4aPSpec>;
#[doc = "Register `GRF_GPIO4A_P` writer"]
pub type W = crate::W<GrfGpio4aPSpec>;
#[doc = "GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a0P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4a0P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a0P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a0P {
    type Ux = u8;
}
#[doc = "Field `GPIO4A0_P` reader - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a0PR = crate::FieldReader<Gpio4a0P>;
impl Gpio4a0PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a0P {
        match self.bits {
            0 => Gpio4a0P::B00,
            1 => Gpio4a0P::B01,
            2 => Gpio4a0P::B10,
            3 => Gpio4a0P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a0P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a0P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a0P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a0P::B11
    }
}
#[doc = "Field `GPIO4A0_P` writer - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a0PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a0P>;
impl<'a, REG> Gpio4a0PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a0P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a0P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a0P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a0P::B11)
    }
}
#[doc = "GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a1P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4a1P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a1P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a1P {
    type Ux = u8;
}
#[doc = "Field `GPIO4A1_P` reader - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a1PR = crate::FieldReader<Gpio4a1P>;
impl Gpio4a1PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a1P {
        match self.bits {
            0 => Gpio4a1P::B00,
            1 => Gpio4a1P::B01,
            2 => Gpio4a1P::B10,
            3 => Gpio4a1P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a1P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a1P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a1P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a1P::B11
    }
}
#[doc = "Field `GPIO4A1_P` writer - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a1PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a1P>;
impl<'a, REG> Gpio4a1PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a1P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a1P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a1P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a1P::B11)
    }
}
#[doc = "GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a2P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4a2P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a2P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a2P {
    type Ux = u8;
}
#[doc = "Field `GPIO4A2_P` reader - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a2PR = crate::FieldReader<Gpio4a2P>;
impl Gpio4a2PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a2P {
        match self.bits {
            0 => Gpio4a2P::B00,
            1 => Gpio4a2P::B01,
            2 => Gpio4a2P::B10,
            3 => Gpio4a2P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a2P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a2P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a2P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a2P::B11
    }
}
#[doc = "Field `GPIO4A2_P` writer - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a2PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a2P>;
impl<'a, REG> Gpio4a2PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a2P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a2P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a2P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a2P::B11)
    }
}
#[doc = "GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a3P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4a3P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a3P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a3P {
    type Ux = u8;
}
#[doc = "Field `GPIO4A3_P` reader - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a3PR = crate::FieldReader<Gpio4a3P>;
impl Gpio4a3PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a3P {
        match self.bits {
            0 => Gpio4a3P::B00,
            1 => Gpio4a3P::B01,
            2 => Gpio4a3P::B10,
            3 => Gpio4a3P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a3P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a3P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a3P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a3P::B11
    }
}
#[doc = "Field `GPIO4A3_P` writer - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a3PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a3P>;
impl<'a, REG> Gpio4a3PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a3P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a3P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a3P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a3P::B11)
    }
}
#[doc = "GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a4P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4a4P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a4P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a4P {
    type Ux = u8;
}
#[doc = "Field `GPIO4A4_P` reader - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a4PR = crate::FieldReader<Gpio4a4P>;
impl Gpio4a4PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a4P {
        match self.bits {
            0 => Gpio4a4P::B00,
            1 => Gpio4a4P::B01,
            2 => Gpio4a4P::B10,
            3 => Gpio4a4P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a4P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a4P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a4P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a4P::B11
    }
}
#[doc = "Field `GPIO4A4_P` writer - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a4PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a4P>;
impl<'a, REG> Gpio4a4PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a4P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a4P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a4P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a4P::B11)
    }
}
#[doc = "GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a5P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4a5P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a5P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a5P {
    type Ux = u8;
}
#[doc = "Field `GPIO4A5_P` reader - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a5PR = crate::FieldReader<Gpio4a5P>;
impl Gpio4a5PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a5P {
        match self.bits {
            0 => Gpio4a5P::B00,
            1 => Gpio4a5P::B01,
            2 => Gpio4a5P::B10,
            3 => Gpio4a5P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a5P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a5P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a5P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a5P::B11
    }
}
#[doc = "Field `GPIO4A5_P` writer - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a5PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a5P>;
impl<'a, REG> Gpio4a5PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a5P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a5P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a5P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a5P::B11)
    }
}
#[doc = "GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a6P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4a6P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a6P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a6P {
    type Ux = u8;
}
#[doc = "Field `GPIO4A6_P` reader - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a6PR = crate::FieldReader<Gpio4a6P>;
impl Gpio4a6PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a6P {
        match self.bits {
            0 => Gpio4a6P::B00,
            1 => Gpio4a6P::B01,
            2 => Gpio4a6P::B10,
            3 => Gpio4a6P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a6P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a6P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a6P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a6P::B11
    }
}
#[doc = "Field `GPIO4A6_P` writer - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a6PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a6P>;
impl<'a, REG> Gpio4a6PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a6P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a6P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a6P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a6P::B11)
    }
}
#[doc = "GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a7P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio4a7P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a7P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a7P {
    type Ux = u8;
}
#[doc = "Field `GPIO4A7_P` reader - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a7PR = crate::FieldReader<Gpio4a7P>;
impl Gpio4a7PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a7P {
        match self.bits {
            0 => Gpio4a7P::B00,
            1 => Gpio4a7P::B01,
            2 => Gpio4a7P::B10,
            3 => Gpio4a7P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a7P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a7P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a7P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a7P::B11
    }
}
#[doc = "Field `GPIO4A7_P` writer - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio4a7PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a7P>;
impl<'a, REG> Gpio4a7PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a7P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a7P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a7P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a7P::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a0_p(&self) -> Gpio4a0PR {
        Gpio4a0PR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a1_p(&self) -> Gpio4a1PR {
        Gpio4a1PR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a2_p(&self) -> Gpio4a2PR {
        Gpio4a2PR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a3_p(&self) -> Gpio4a3PR {
        Gpio4a3PR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a4_p(&self) -> Gpio4a4PR {
        Gpio4a4PR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a5_p(&self) -> Gpio4a5PR {
        Gpio4a5PR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a6_p(&self) -> Gpio4a6PR {
        Gpio4a6PR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a7_p(&self) -> Gpio4a7PR {
        Gpio4a7PR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a0_p(&mut self) -> Gpio4a0PW<GrfGpio4aPSpec> {
        Gpio4a0PW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a1_p(&mut self) -> Gpio4a1PW<GrfGpio4aPSpec> {
        Gpio4a1PW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a2_p(&mut self) -> Gpio4a2PW<GrfGpio4aPSpec> {
        Gpio4a2PW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a3_p(&mut self) -> Gpio4a3PW<GrfGpio4aPSpec> {
        Gpio4a3PW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a4_p(&mut self) -> Gpio4a4PW<GrfGpio4aPSpec> {
        Gpio4a4PW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a5_p(&mut self) -> Gpio4a5PW<GrfGpio4aPSpec> {
        Gpio4a5PW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a6_p(&mut self) -> Gpio4a6PW<GrfGpio4aPSpec> {
        Gpio4a6PW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO4A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a7_p(&mut self) -> Gpio4a7PW<GrfGpio4aPSpec> {
        Gpio4a7PW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio4aPSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO4A PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4a_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4a_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio4aPSpec;
impl crate::RegisterSpec for GrfGpio4aPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio4a_p::R`](R) reader structure"]
impl crate::Readable for GrfGpio4aPSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio4a_p::W`](W) writer structure"]
impl crate::Writable for GrfGpio4aPSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO4A_P to value 0xaa96"]
impl crate::Resettable for GrfGpio4aPSpec {
    const RESET_VALUE: u32 = 0xaa96;
}
