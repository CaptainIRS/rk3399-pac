#[doc = "Register `GRF_GPIO4D_E` reader"]
pub type R = crate::R<GrfGpio4dESpec>;
#[doc = "Register `GRF_GPIO4D_E` writer"]
pub type W = crate::W<GrfGpio4dESpec>;
#[doc = "GPIO4D0 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4d0E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4d0E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4d0E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4d0E {
    type Ux = u8;
}
#[doc = "Field `GPIO4D0_E` reader - GPIO4D0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d0ER = crate::FieldReader<Gpio4d0E>;
impl Gpio4d0ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4d0E {
        match self.bits {
            0 => Gpio4d0E::B00,
            1 => Gpio4d0E::B01,
            2 => Gpio4d0E::B10,
            3 => Gpio4d0E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4d0E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4d0E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4d0E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4d0E::B11
    }
}
#[doc = "Field `GPIO4D0_E` writer - GPIO4D0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d0EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4d0E>;
impl<'a, REG> Gpio4d0EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d0E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d0E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d0E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d0E::B11)
    }
}
#[doc = "GPIO4D1 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4d1E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4d1E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4d1E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4d1E {
    type Ux = u8;
}
#[doc = "Field `GPIO4D1_E` reader - GPIO4D1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d1ER = crate::FieldReader<Gpio4d1E>;
impl Gpio4d1ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4d1E {
        match self.bits {
            0 => Gpio4d1E::B00,
            1 => Gpio4d1E::B01,
            2 => Gpio4d1E::B10,
            3 => Gpio4d1E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4d1E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4d1E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4d1E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4d1E::B11
    }
}
#[doc = "Field `GPIO4D1_E` writer - GPIO4D1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d1EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4d1E>;
impl<'a, REG> Gpio4d1EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d1E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d1E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d1E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d1E::B11)
    }
}
#[doc = "GPIO4D2 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4d2E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4d2E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4d2E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4d2E {
    type Ux = u8;
}
#[doc = "Field `GPIO4D2_E` reader - GPIO4D2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d2ER = crate::FieldReader<Gpio4d2E>;
impl Gpio4d2ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4d2E {
        match self.bits {
            0 => Gpio4d2E::B00,
            1 => Gpio4d2E::B01,
            2 => Gpio4d2E::B10,
            3 => Gpio4d2E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4d2E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4d2E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4d2E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4d2E::B11
    }
}
#[doc = "Field `GPIO4D2_E` writer - GPIO4D2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d2EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4d2E>;
impl<'a, REG> Gpio4d2EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d2E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d2E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d2E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d2E::B11)
    }
}
#[doc = "GPIO4D3 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4d3E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4d3E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4d3E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4d3E {
    type Ux = u8;
}
#[doc = "Field `GPIO4D3_E` reader - GPIO4D3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d3ER = crate::FieldReader<Gpio4d3E>;
impl Gpio4d3ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4d3E {
        match self.bits {
            0 => Gpio4d3E::B00,
            1 => Gpio4d3E::B01,
            2 => Gpio4d3E::B10,
            3 => Gpio4d3E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4d3E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4d3E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4d3E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4d3E::B11
    }
}
#[doc = "Field `GPIO4D3_E` writer - GPIO4D3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d3EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4d3E>;
impl<'a, REG> Gpio4d3EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d3E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d3E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d3E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d3E::B11)
    }
}
#[doc = "GPIO4D4 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4d4E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4d4E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4d4E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4d4E {
    type Ux = u8;
}
#[doc = "Field `GPIO4D4_E` reader - GPIO4D4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d4ER = crate::FieldReader<Gpio4d4E>;
impl Gpio4d4ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4d4E {
        match self.bits {
            0 => Gpio4d4E::B00,
            1 => Gpio4d4E::B01,
            2 => Gpio4d4E::B10,
            3 => Gpio4d4E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4d4E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4d4E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4d4E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4d4E::B11
    }
}
#[doc = "Field `GPIO4D4_E` writer - GPIO4D4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d4EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4d4E>;
impl<'a, REG> Gpio4d4EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d4E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d4E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d4E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d4E::B11)
    }
}
#[doc = "GPIO4D5 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4d5E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4d5E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4d5E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4d5E {
    type Ux = u8;
}
#[doc = "Field `GPIO4D5_E` reader - GPIO4D5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d5ER = crate::FieldReader<Gpio4d5E>;
impl Gpio4d5ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4d5E {
        match self.bits {
            0 => Gpio4d5E::B00,
            1 => Gpio4d5E::B01,
            2 => Gpio4d5E::B10,
            3 => Gpio4d5E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4d5E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4d5E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4d5E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4d5E::B11
    }
}
#[doc = "Field `GPIO4D5_E` writer - GPIO4D5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d5EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4d5E>;
impl<'a, REG> Gpio4d5EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d5E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d5E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d5E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d5E::B11)
    }
}
#[doc = "GPIO4D6 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4d6E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4d6E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4d6E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4d6E {
    type Ux = u8;
}
#[doc = "Field `GPIO4D6_E` reader - GPIO4D6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d6ER = crate::FieldReader<Gpio4d6E>;
impl Gpio4d6ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4d6E {
        match self.bits {
            0 => Gpio4d6E::B00,
            1 => Gpio4d6E::B01,
            2 => Gpio4d6E::B10,
            3 => Gpio4d6E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4d6E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4d6E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4d6E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4d6E::B11
    }
}
#[doc = "Field `GPIO4D6_E` writer - GPIO4D6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d6EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4d6E>;
impl<'a, REG> Gpio4d6EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d6E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d6E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d6E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d6E::B11)
    }
}
#[doc = "GPIO4D7 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4d7E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4d7E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4d7E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4d7E {
    type Ux = u8;
}
#[doc = "Field `GPIO4D7_E` reader - GPIO4D7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d7ER = crate::FieldReader<Gpio4d7E>;
impl Gpio4d7ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4d7E {
        match self.bits {
            0 => Gpio4d7E::B00,
            1 => Gpio4d7E::B01,
            2 => Gpio4d7E::B10,
            3 => Gpio4d7E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4d7E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4d7E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4d7E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4d7E::B11
    }
}
#[doc = "Field `GPIO4D7_E` writer - GPIO4D7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4d7EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4d7E>;
impl<'a, REG> Gpio4d7EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d7E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d7E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d7E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4d7E::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO4D0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4d0_e(&self) -> Gpio4d0ER {
        Gpio4d0ER::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO4D1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4d1_e(&self) -> Gpio4d1ER {
        Gpio4d1ER::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO4D2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4d2_e(&self) -> Gpio4d2ER {
        Gpio4d2ER::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO4D3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4d3_e(&self) -> Gpio4d3ER {
        Gpio4d3ER::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO4D4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4d4_e(&self) -> Gpio4d4ER {
        Gpio4d4ER::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO4D5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4d5_e(&self) -> Gpio4d5ER {
        Gpio4d5ER::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO4D6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4d6_e(&self) -> Gpio4d6ER {
        Gpio4d6ER::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO4D7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4d7_e(&self) -> Gpio4d7ER {
        Gpio4d7ER::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO4D0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4d0_e(&mut self) -> Gpio4d0EW<GrfGpio4dESpec> {
        Gpio4d0EW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO4D1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4d1_e(&mut self) -> Gpio4d1EW<GrfGpio4dESpec> {
        Gpio4d1EW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO4D2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4d2_e(&mut self) -> Gpio4d2EW<GrfGpio4dESpec> {
        Gpio4d2EW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO4D3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4d3_e(&mut self) -> Gpio4d3EW<GrfGpio4dESpec> {
        Gpio4d3EW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO4D4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4d4_e(&mut self) -> Gpio4d4EW<GrfGpio4dESpec> {
        Gpio4d4EW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO4D5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4d5_e(&mut self) -> Gpio4d5EW<GrfGpio4dESpec> {
        Gpio4d5EW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO4D6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4d6_e(&mut self) -> Gpio4d6EW<GrfGpio4dESpec> {
        Gpio4d6EW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO4D7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4d7_e(&mut self) -> Gpio4d7EW<GrfGpio4dESpec> {
        Gpio4d7EW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio4dESpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO4D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4d_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4d_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio4dESpec;
impl crate::RegisterSpec for GrfGpio4dESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio4d_e::R`](R) reader structure"]
impl crate::Readable for GrfGpio4dESpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio4d_e::W`](W) writer structure"]
impl crate::Writable for GrfGpio4dESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO4D_E to value 0"]
impl crate::Resettable for GrfGpio4dESpec {
    const RESET_VALUE: u32 = 0;
}
