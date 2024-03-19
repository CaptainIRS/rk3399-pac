#[doc = "Register `GRF_GPIO2A_P` reader"]
pub type R = crate::R<GrfGpio2aPSpec>;
#[doc = "Register `GRF_GPIO2A_P` writer"]
pub type W = crate::W<GrfGpio2aPSpec>;
#[doc = "GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a0P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio2a0P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a0P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a0P {
    type Ux = u8;
}
#[doc = "Field `GPIO2A0_P` reader - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a0PR = crate::FieldReader<Gpio2a0P>;
impl Gpio2a0PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a0P {
        match self.bits {
            0 => Gpio2a0P::B00,
            1 => Gpio2a0P::B01,
            2 => Gpio2a0P::B10,
            3 => Gpio2a0P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a0P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a0P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a0P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a0P::B11
    }
}
#[doc = "Field `GPIO2A0_P` writer - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a0PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a0P>;
impl<'a, REG> Gpio2a0PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a0P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a0P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a0P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a0P::B11)
    }
}
#[doc = "GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a1P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio2a1P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a1P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a1P {
    type Ux = u8;
}
#[doc = "Field `GPIO2A1_P` reader - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a1PR = crate::FieldReader<Gpio2a1P>;
impl Gpio2a1PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a1P {
        match self.bits {
            0 => Gpio2a1P::B00,
            1 => Gpio2a1P::B01,
            2 => Gpio2a1P::B10,
            3 => Gpio2a1P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a1P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a1P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a1P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a1P::B11
    }
}
#[doc = "Field `GPIO2A1_P` writer - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a1PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a1P>;
impl<'a, REG> Gpio2a1PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a1P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a1P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a1P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a1P::B11)
    }
}
#[doc = "GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a2P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio2a2P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a2P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a2P {
    type Ux = u8;
}
#[doc = "Field `GPIO2A2_P` reader - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a2PR = crate::FieldReader<Gpio2a2P>;
impl Gpio2a2PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a2P {
        match self.bits {
            0 => Gpio2a2P::B00,
            1 => Gpio2a2P::B01,
            2 => Gpio2a2P::B10,
            3 => Gpio2a2P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a2P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a2P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a2P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a2P::B11
    }
}
#[doc = "Field `GPIO2A2_P` writer - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a2PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a2P>;
impl<'a, REG> Gpio2a2PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a2P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a2P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a2P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a2P::B11)
    }
}
#[doc = "GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a3P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio2a3P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a3P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a3P {
    type Ux = u8;
}
#[doc = "Field `GPIO2A3_P` reader - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a3PR = crate::FieldReader<Gpio2a3P>;
impl Gpio2a3PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a3P {
        match self.bits {
            0 => Gpio2a3P::B00,
            1 => Gpio2a3P::B01,
            2 => Gpio2a3P::B10,
            3 => Gpio2a3P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a3P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a3P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a3P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a3P::B11
    }
}
#[doc = "Field `GPIO2A3_P` writer - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a3PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a3P>;
impl<'a, REG> Gpio2a3PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a3P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a3P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a3P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a3P::B11)
    }
}
#[doc = "GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a4P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio2a4P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a4P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a4P {
    type Ux = u8;
}
#[doc = "Field `GPIO2A4_P` reader - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a4PR = crate::FieldReader<Gpio2a4P>;
impl Gpio2a4PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a4P {
        match self.bits {
            0 => Gpio2a4P::B00,
            1 => Gpio2a4P::B01,
            2 => Gpio2a4P::B10,
            3 => Gpio2a4P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a4P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a4P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a4P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a4P::B11
    }
}
#[doc = "Field `GPIO2A4_P` writer - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a4PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a4P>;
impl<'a, REG> Gpio2a4PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a4P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a4P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a4P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a4P::B11)
    }
}
#[doc = "GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a5P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio2a5P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a5P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a5P {
    type Ux = u8;
}
#[doc = "Field `GPIO2A5_P` reader - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a5PR = crate::FieldReader<Gpio2a5P>;
impl Gpio2a5PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a5P {
        match self.bits {
            0 => Gpio2a5P::B00,
            1 => Gpio2a5P::B01,
            2 => Gpio2a5P::B10,
            3 => Gpio2a5P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a5P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a5P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a5P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a5P::B11
    }
}
#[doc = "Field `GPIO2A5_P` writer - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a5PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a5P>;
impl<'a, REG> Gpio2a5PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a5P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a5P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a5P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a5P::B11)
    }
}
#[doc = "GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a6P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio2a6P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a6P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a6P {
    type Ux = u8;
}
#[doc = "Field `GPIO2A6_P` reader - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a6PR = crate::FieldReader<Gpio2a6P>;
impl Gpio2a6PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a6P {
        match self.bits {
            0 => Gpio2a6P::B00,
            1 => Gpio2a6P::B01,
            2 => Gpio2a6P::B10,
            3 => Gpio2a6P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a6P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a6P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a6P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a6P::B11
    }
}
#[doc = "Field `GPIO2A6_P` writer - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a6PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a6P>;
impl<'a, REG> Gpio2a6PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a6P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a6P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a6P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a6P::B11)
    }
}
#[doc = "GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 1"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a7P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio2a7P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a7P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a7P {
    type Ux = u8;
}
#[doc = "Field `GPIO2A7_P` reader - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a7PR = crate::FieldReader<Gpio2a7P>;
impl Gpio2a7PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a7P {
        match self.bits {
            0 => Gpio2a7P::B00,
            1 => Gpio2a7P::B01,
            2 => Gpio2a7P::B10,
            3 => Gpio2a7P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a7P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a7P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a7P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a7P::B11
    }
}
#[doc = "Field `GPIO2A7_P` writer - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2a7PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a7P>;
impl<'a, REG> Gpio2a7PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a7P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a7P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a7P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a7P::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a0_p(&self) -> Gpio2a0PR {
        Gpio2a0PR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a1_p(&self) -> Gpio2a1PR {
        Gpio2a1PR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a2_p(&self) -> Gpio2a2PR {
        Gpio2a2PR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a3_p(&self) -> Gpio2a3PR {
        Gpio2a3PR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a4_p(&self) -> Gpio2a4PR {
        Gpio2a4PR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a5_p(&self) -> Gpio2a5PR {
        Gpio2a5PR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a6_p(&self) -> Gpio2a6PR {
        Gpio2a6PR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a7_p(&self) -> Gpio2a7PR {
        Gpio2a7PR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a0_p(&mut self) -> Gpio2a0PW<GrfGpio2aPSpec> {
        Gpio2a0PW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a1_p(&mut self) -> Gpio2a1PW<GrfGpio2aPSpec> {
        Gpio2a1PW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a2_p(&mut self) -> Gpio2a2PW<GrfGpio2aPSpec> {
        Gpio2a2PW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a3_p(&mut self) -> Gpio2a3PW<GrfGpio2aPSpec> {
        Gpio2a3PW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a4_p(&mut self) -> Gpio2a4PW<GrfGpio2aPSpec> {
        Gpio2a4PW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a5_p(&mut self) -> Gpio2a5PW<GrfGpio2aPSpec> {
        Gpio2a5PW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a6_p(&mut self) -> Gpio2a6PW<GrfGpio2aPSpec> {
        Gpio2a6PW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO2A PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a7_p(&mut self) -> Gpio2a7PW<GrfGpio2aPSpec> {
        Gpio2a7PW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio2aPSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2A PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2a_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2a_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio2aPSpec;
impl crate::RegisterSpec for GrfGpio2aPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio2a_p::R`](R) reader structure"]
impl crate::Readable for GrfGpio2aPSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio2a_p::W`](W) writer structure"]
impl crate::Writable for GrfGpio2aPSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO2A_P to value 0x6aa5"]
impl crate::Resettable for GrfGpio2aPSpec {
    const RESET_VALUE: u32 = 0x6aa5;
}
