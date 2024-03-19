#[doc = "Register `GRF_GPIO3D_P` reader"]
pub type R = crate::R<GrfGpio3dPSpec>;
#[doc = "Register `GRF_GPIO3D_P` writer"]
pub type W = crate::W<GrfGpio3dPSpec>;
#[doc = "GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d0P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3d0P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d0P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d0P {
    type Ux = u8;
}
#[doc = "Field `GPIO3D0_P` reader - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d0PR = crate::FieldReader<Gpio3d0P>;
impl Gpio3d0PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d0P {
        match self.bits {
            0 => Gpio3d0P::B00,
            1 => Gpio3d0P::B01,
            2 => Gpio3d0P::B10,
            3 => Gpio3d0P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d0P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d0P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d0P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d0P::B11
    }
}
#[doc = "Field `GPIO3D0_P` writer - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d0PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d0P>;
impl<'a, REG> Gpio3d0PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d0P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d0P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d0P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d0P::B11)
    }
}
#[doc = "GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d1P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3d1P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d1P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d1P {
    type Ux = u8;
}
#[doc = "Field `GPIO3D1_P` reader - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d1PR = crate::FieldReader<Gpio3d1P>;
impl Gpio3d1PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d1P {
        match self.bits {
            0 => Gpio3d1P::B00,
            1 => Gpio3d1P::B01,
            2 => Gpio3d1P::B10,
            3 => Gpio3d1P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d1P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d1P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d1P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d1P::B11
    }
}
#[doc = "Field `GPIO3D1_P` writer - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d1PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d1P>;
impl<'a, REG> Gpio3d1PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d1P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d1P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d1P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d1P::B11)
    }
}
#[doc = "GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d2P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3d2P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d2P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d2P {
    type Ux = u8;
}
#[doc = "Field `GPIO3D2_P` reader - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d2PR = crate::FieldReader<Gpio3d2P>;
impl Gpio3d2PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d2P {
        match self.bits {
            0 => Gpio3d2P::B00,
            1 => Gpio3d2P::B01,
            2 => Gpio3d2P::B10,
            3 => Gpio3d2P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d2P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d2P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d2P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d2P::B11
    }
}
#[doc = "Field `GPIO3D2_P` writer - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d2PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d2P>;
impl<'a, REG> Gpio3d2PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d2P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d2P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d2P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d2P::B11)
    }
}
#[doc = "GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d3P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3d3P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d3P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d3P {
    type Ux = u8;
}
#[doc = "Field `GPIO3D3_P` reader - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d3PR = crate::FieldReader<Gpio3d3P>;
impl Gpio3d3PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d3P {
        match self.bits {
            0 => Gpio3d3P::B00,
            1 => Gpio3d3P::B01,
            2 => Gpio3d3P::B10,
            3 => Gpio3d3P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d3P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d3P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d3P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d3P::B11
    }
}
#[doc = "Field `GPIO3D3_P` writer - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d3PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d3P>;
impl<'a, REG> Gpio3d3PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d3P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d3P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d3P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d3P::B11)
    }
}
#[doc = "GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d4P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3d4P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d4P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d4P {
    type Ux = u8;
}
#[doc = "Field `GPIO3D4_P` reader - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d4PR = crate::FieldReader<Gpio3d4P>;
impl Gpio3d4PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d4P {
        match self.bits {
            0 => Gpio3d4P::B00,
            1 => Gpio3d4P::B01,
            2 => Gpio3d4P::B10,
            3 => Gpio3d4P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d4P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d4P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d4P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d4P::B11
    }
}
#[doc = "Field `GPIO3D4_P` writer - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d4PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d4P>;
impl<'a, REG> Gpio3d4PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d4P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d4P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d4P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d4P::B11)
    }
}
#[doc = "GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d5P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3d5P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d5P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d5P {
    type Ux = u8;
}
#[doc = "Field `GPIO3D5_P` reader - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d5PR = crate::FieldReader<Gpio3d5P>;
impl Gpio3d5PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d5P {
        match self.bits {
            0 => Gpio3d5P::B00,
            1 => Gpio3d5P::B01,
            2 => Gpio3d5P::B10,
            3 => Gpio3d5P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d5P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d5P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d5P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d5P::B11
    }
}
#[doc = "Field `GPIO3D5_P` writer - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d5PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d5P>;
impl<'a, REG> Gpio3d5PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d5P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d5P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d5P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d5P::B11)
    }
}
#[doc = "GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d6P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3d6P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d6P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d6P {
    type Ux = u8;
}
#[doc = "Field `GPIO3D6_P` reader - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d6PR = crate::FieldReader<Gpio3d6P>;
impl Gpio3d6PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d6P {
        match self.bits {
            0 => Gpio3d6P::B00,
            1 => Gpio3d6P::B01,
            2 => Gpio3d6P::B10,
            3 => Gpio3d6P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d6P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d6P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d6P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d6P::B11
    }
}
#[doc = "Field `GPIO3D6_P` writer - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d6PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d6P>;
impl<'a, REG> Gpio3d6PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d6P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d6P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d6P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d6P::B11)
    }
}
#[doc = "GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 2"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d7P {
    #[doc = "0: Z(Noraml operaton);"]
    B00 = 0,
    #[doc = "1: weak 1(pull-up);"]
    B01 = 1,
    #[doc = "2: weak 0(pull-down);"]
    B10 = 2,
    #[doc = "3: Repeater(Bus keeper)"]
    B11 = 3,
}
impl From<Gpio3d7P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d7P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d7P {
    type Ux = u8;
}
#[doc = "Field `GPIO3D7_P` reader - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d7PR = crate::FieldReader<Gpio3d7P>;
impl Gpio3d7PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d7P {
        match self.bits {
            0 => Gpio3d7P::B00,
            1 => Gpio3d7P::B01,
            2 => Gpio3d7P::B10,
            3 => Gpio3d7P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d7P::B00
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d7P::B01
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d7P::B10
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d7P::B11
    }
}
#[doc = "Field `GPIO3D7_P` writer - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio3d7PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d7P>;
impl<'a, REG> Gpio3d7PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Z(Noraml operaton);"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d7P::B00)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d7P::B01)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d7P::B10)
    }
    #[doc = "Repeater(Bus keeper)"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d7P::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d0_p(&self) -> Gpio3d0PR {
        Gpio3d0PR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d1_p(&self) -> Gpio3d1PR {
        Gpio3d1PR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d2_p(&self) -> Gpio3d2PR {
        Gpio3d2PR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d3_p(&self) -> Gpio3d3PR {
        Gpio3d3PR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d4_p(&self) -> Gpio3d4PR {
        Gpio3d4PR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d5_p(&self) -> Gpio3d5PR {
        Gpio3d5PR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d6_p(&self) -> Gpio3d6PR {
        Gpio3d6PR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d7_p(&self) -> Gpio3d7PR {
        Gpio3d7PR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d0_p(&mut self) -> Gpio3d0PW<GrfGpio3dPSpec> {
        Gpio3d0PW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d1_p(&mut self) -> Gpio3d1PW<GrfGpio3dPSpec> {
        Gpio3d1PW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d2_p(&mut self) -> Gpio3d2PW<GrfGpio3dPSpec> {
        Gpio3d2PW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d3_p(&mut self) -> Gpio3d3PW<GrfGpio3dPSpec> {
        Gpio3d3PW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d4_p(&mut self) -> Gpio3d4PW<GrfGpio3dPSpec> {
        Gpio3d4PW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d5_p(&mut self) -> Gpio3d5PW<GrfGpio3dPSpec> {
        Gpio3d5PW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d6_p(&mut self) -> Gpio3d6PW<GrfGpio3dPSpec> {
        Gpio3d6PW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO3D PU/PD programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d7_p(&mut self) -> Gpio3d7PW<GrfGpio3dPSpec> {
        Gpio3d7PW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio3dPSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO3D PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3d_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3d_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio3dPSpec;
impl crate::RegisterSpec for GrfGpio3dPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio3d_p::R`](R) reader structure"]
impl crate::Readable for GrfGpio3dPSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio3d_p::W`](W) writer structure"]
impl crate::Writable for GrfGpio3dPSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO3D_P to value 0xaaaa"]
impl crate::Resettable for GrfGpio3dPSpec {
    const RESET_VALUE: u32 = 0xaaaa;
}
