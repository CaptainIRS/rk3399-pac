#[doc = "Register `GPIO2C_P` reader"]
pub type R = crate::R<Gpio2cPSpec>;
#[doc = "Register `GPIO2C_P` writer"]
pub type W = crate::W<Gpio2cPSpec>;
#[doc = "GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c0P {
    #[doc = "0: pervious-state"]
    B00 = 0,
    #[doc = "1: weak 0(pull-down);"]
    B01 = 1,
    #[doc = "2: pervious-state"]
    B10 = 2,
    #[doc = "3: weak 1(pull-up);"]
    B11 = 3,
}
impl From<Gpio2c0P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c0P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c0P {
    type Ux = u8;
}
#[doc = "Field `GPIO2C0_P` reader - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c0PR = crate::FieldReader<Gpio2c0P>;
impl Gpio2c0PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c0P {
        match self.bits {
            0 => Gpio2c0P::B00,
            1 => Gpio2c0P::B01,
            2 => Gpio2c0P::B10,
            3 => Gpio2c0P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c0P::B00
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c0P::B01
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c0P::B10
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c0P::B11
    }
}
#[doc = "Field `GPIO2C0_P` writer - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c0PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c0P>;
impl<'a, REG> Gpio2c0PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c0P::B00)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c0P::B01)
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c0P::B10)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c0P::B11)
    }
}
#[doc = "GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c1P {
    #[doc = "0: pervious-state"]
    B00 = 0,
    #[doc = "1: weak 0(pull-down);"]
    B01 = 1,
    #[doc = "2: pervious-state"]
    B10 = 2,
    #[doc = "3: weak 1(pull-up);"]
    B11 = 3,
}
impl From<Gpio2c1P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c1P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c1P {
    type Ux = u8;
}
#[doc = "Field `GPIO2C1_P` reader - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c1PR = crate::FieldReader<Gpio2c1P>;
impl Gpio2c1PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c1P {
        match self.bits {
            0 => Gpio2c1P::B00,
            1 => Gpio2c1P::B01,
            2 => Gpio2c1P::B10,
            3 => Gpio2c1P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c1P::B00
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c1P::B01
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c1P::B10
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c1P::B11
    }
}
#[doc = "Field `GPIO2C1_P` writer - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c1PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c1P>;
impl<'a, REG> Gpio2c1PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c1P::B00)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c1P::B01)
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c1P::B10)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c1P::B11)
    }
}
#[doc = "GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c2P {
    #[doc = "0: pervious-state"]
    B00 = 0,
    #[doc = "1: weak 0(pull-down);"]
    B01 = 1,
    #[doc = "2: pervious-state"]
    B10 = 2,
    #[doc = "3: weak 1(pull-up);"]
    B11 = 3,
}
impl From<Gpio2c2P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c2P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c2P {
    type Ux = u8;
}
#[doc = "Field `GPIO2C2_P` reader - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c2PR = crate::FieldReader<Gpio2c2P>;
impl Gpio2c2PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c2P {
        match self.bits {
            0 => Gpio2c2P::B00,
            1 => Gpio2c2P::B01,
            2 => Gpio2c2P::B10,
            3 => Gpio2c2P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c2P::B00
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c2P::B01
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c2P::B10
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c2P::B11
    }
}
#[doc = "Field `GPIO2C2_P` writer - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c2PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c2P>;
impl<'a, REG> Gpio2c2PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c2P::B00)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c2P::B01)
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c2P::B10)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c2P::B11)
    }
}
#[doc = "GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c3P {
    #[doc = "0: pervious-state"]
    B00 = 0,
    #[doc = "1: weak 0(pull-down);"]
    B01 = 1,
    #[doc = "2: pervious-state"]
    B10 = 2,
    #[doc = "3: weak 1(pull-up);"]
    B11 = 3,
}
impl From<Gpio2c3P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c3P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c3P {
    type Ux = u8;
}
#[doc = "Field `GPIO2C3_P` reader - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c3PR = crate::FieldReader<Gpio2c3P>;
impl Gpio2c3PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c3P {
        match self.bits {
            0 => Gpio2c3P::B00,
            1 => Gpio2c3P::B01,
            2 => Gpio2c3P::B10,
            3 => Gpio2c3P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c3P::B00
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c3P::B01
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c3P::B10
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c3P::B11
    }
}
#[doc = "Field `GPIO2C3_P` writer - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c3PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c3P>;
impl<'a, REG> Gpio2c3PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c3P::B00)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c3P::B01)
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c3P::B10)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c3P::B11)
    }
}
#[doc = "GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c4P {
    #[doc = "0: pervious-state"]
    B00 = 0,
    #[doc = "1: weak 0(pull-down);"]
    B01 = 1,
    #[doc = "2: pervious-state"]
    B10 = 2,
    #[doc = "3: weak 1(pull-up);"]
    B11 = 3,
}
impl From<Gpio2c4P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c4P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c4P {
    type Ux = u8;
}
#[doc = "Field `GPIO2C4_P` reader - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c4PR = crate::FieldReader<Gpio2c4P>;
impl Gpio2c4PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c4P {
        match self.bits {
            0 => Gpio2c4P::B00,
            1 => Gpio2c4P::B01,
            2 => Gpio2c4P::B10,
            3 => Gpio2c4P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c4P::B00
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c4P::B01
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c4P::B10
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c4P::B11
    }
}
#[doc = "Field `GPIO2C4_P` writer - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c4PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c4P>;
impl<'a, REG> Gpio2c4PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c4P::B00)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c4P::B01)
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c4P::B10)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c4P::B11)
    }
}
#[doc = "GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c5P {
    #[doc = "0: pervious-state"]
    B00 = 0,
    #[doc = "1: weak 0(pull-down);"]
    B01 = 1,
    #[doc = "2: pervious-state"]
    B10 = 2,
    #[doc = "3: weak 1(pull-up);"]
    B11 = 3,
}
impl From<Gpio2c5P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c5P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c5P {
    type Ux = u8;
}
#[doc = "Field `GPIO2C5_P` reader - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c5PR = crate::FieldReader<Gpio2c5P>;
impl Gpio2c5PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c5P {
        match self.bits {
            0 => Gpio2c5P::B00,
            1 => Gpio2c5P::B01,
            2 => Gpio2c5P::B10,
            3 => Gpio2c5P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c5P::B00
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c5P::B01
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c5P::B10
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c5P::B11
    }
}
#[doc = "Field `GPIO2C5_P` writer - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c5PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c5P>;
impl<'a, REG> Gpio2c5PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c5P::B00)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c5P::B01)
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c5P::B10)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c5P::B11)
    }
}
#[doc = "GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c6P {
    #[doc = "0: pervious-state"]
    B00 = 0,
    #[doc = "1: weak 0(pull-down);"]
    B01 = 1,
    #[doc = "2: pervious-state"]
    B10 = 2,
    #[doc = "3: weak 1(pull-up);"]
    B11 = 3,
}
impl From<Gpio2c6P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c6P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c6P {
    type Ux = u8;
}
#[doc = "Field `GPIO2C6_P` reader - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c6PR = crate::FieldReader<Gpio2c6P>;
impl Gpio2c6PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c6P {
        match self.bits {
            0 => Gpio2c6P::B00,
            1 => Gpio2c6P::B01,
            2 => Gpio2c6P::B10,
            3 => Gpio2c6P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c6P::B00
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c6P::B01
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c6P::B10
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c6P::B11
    }
}
#[doc = "Field `GPIO2C6_P` writer - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c6PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c6P>;
impl<'a, REG> Gpio2c6PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c6P::B00)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c6P::B01)
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c6P::B10)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c6P::B11)
    }
}
#[doc = "GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits\n\nValue on reset: 3"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c7P {
    #[doc = "0: pervious-state"]
    B00 = 0,
    #[doc = "1: weak 0(pull-down);"]
    B01 = 1,
    #[doc = "2: pervious-state"]
    B10 = 2,
    #[doc = "3: weak 1(pull-up);"]
    B11 = 3,
}
impl From<Gpio2c7P> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c7P) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c7P {
    type Ux = u8;
}
#[doc = "Field `GPIO2C7_P` reader - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c7PR = crate::FieldReader<Gpio2c7P>;
impl Gpio2c7PR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c7P {
        match self.bits {
            0 => Gpio2c7P::B00,
            1 => Gpio2c7P::B01,
            2 => Gpio2c7P::B10,
            3 => Gpio2c7P::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c7P::B00
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c7P::B01
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c7P::B10
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c7P::B11
    }
}
#[doc = "Field `GPIO2C7_P` writer - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
pub type Gpio2c7PW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c7P>;
impl<'a, REG> Gpio2c7PW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c7P::B00)
    }
    #[doc = "weak 0(pull-down);"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c7P::B01)
    }
    #[doc = "pervious-state"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c7P::B10)
    }
    #[doc = "weak 1(pull-up);"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c7P::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c0_p(&self) -> Gpio2c0PR {
        Gpio2c0PR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c1_p(&self) -> Gpio2c1PR {
        Gpio2c1PR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c2_p(&self) -> Gpio2c2PR {
        Gpio2c2PR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c3_p(&self) -> Gpio2c3PR {
        Gpio2c3PR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c4_p(&self) -> Gpio2c4PR {
        Gpio2c4PR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c5_p(&self) -> Gpio2c5PR {
        Gpio2c5PR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c6_p(&self) -> Gpio2c6PR {
        Gpio2c6PR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c7_p(&self) -> Gpio2c7PR {
        Gpio2c7PR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c0_p(&mut self) -> Gpio2c0PW<Gpio2cPSpec> {
        Gpio2c0PW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c1_p(&mut self) -> Gpio2c1PW<Gpio2cPSpec> {
        Gpio2c1PW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c2_p(&mut self) -> Gpio2c2PW<Gpio2cPSpec> {
        Gpio2c2PW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c3_p(&mut self) -> Gpio2c3PW<Gpio2cPSpec> {
        Gpio2c3PW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c4_p(&mut self) -> Gpio2c4PW<Gpio2cPSpec> {
        Gpio2c4PW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c5_p(&mut self) -> Gpio2c5PW<Gpio2cPSpec> {
        Gpio2c5PW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c6_p(&mut self) -> Gpio2c6PW<Gpio2cPSpec> {
        Gpio2c6PW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO2C PE/PS programmation section, every\n\nGPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c7_p(&mut self) -> Gpio2c7PW<Gpio2cPSpec> {
        Gpio2c7PW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio2cPSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2C PU/PD control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2c_p::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2c_p::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio2cPSpec;
impl crate::RegisterSpec for Gpio2cPSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio2c_p::R`](R) reader structure"]
impl crate::Readable for Gpio2cPSpec {}
#[doc = "`write(|w| ..)` method takes [`gpio2c_p::W`](W) writer structure"]
impl crate::Writable for Gpio2cPSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO2C_P to value 0xffff"]
impl crate::Resettable for Gpio2cPSpec {
    const RESET_VALUE: u32 = 0xffff;
}
