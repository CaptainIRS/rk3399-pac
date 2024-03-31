#[doc = "Register `GPIO4A_E` reader"]
pub type R = crate::R<Gpio4aESpec>;
#[doc = "Register `GPIO4A_E` writer"]
pub type W = crate::W<Gpio4aESpec>;
#[doc = "GPIO4A0 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a0E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4a0E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a0E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a0E {
    type Ux = u8;
}
#[doc = "Field `GPIO4A0_E` reader - GPIO4A0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a0ER = crate::FieldReader<Gpio4a0E>;
impl Gpio4a0ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a0E {
        match self.bits {
            0 => Gpio4a0E::B00,
            1 => Gpio4a0E::B01,
            2 => Gpio4a0E::B10,
            3 => Gpio4a0E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a0E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a0E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a0E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a0E::B11
    }
}
#[doc = "Field `GPIO4A0_E` writer - GPIO4A0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a0EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a0E>;
impl<'a, REG> Gpio4a0EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a0E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a0E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a0E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a0E::B11)
    }
}
#[doc = "GPIO4A1 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a1E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4a1E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a1E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a1E {
    type Ux = u8;
}
#[doc = "Field `GPIO4A1_E` reader - GPIO4A1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a1ER = crate::FieldReader<Gpio4a1E>;
impl Gpio4a1ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a1E {
        match self.bits {
            0 => Gpio4a1E::B00,
            1 => Gpio4a1E::B01,
            2 => Gpio4a1E::B10,
            3 => Gpio4a1E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a1E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a1E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a1E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a1E::B11
    }
}
#[doc = "Field `GPIO4A1_E` writer - GPIO4A1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a1EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a1E>;
impl<'a, REG> Gpio4a1EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a1E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a1E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a1E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a1E::B11)
    }
}
#[doc = "GPIO4A2 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a2E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4a2E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a2E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a2E {
    type Ux = u8;
}
#[doc = "Field `GPIO4A2_E` reader - GPIO4A2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a2ER = crate::FieldReader<Gpio4a2E>;
impl Gpio4a2ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a2E {
        match self.bits {
            0 => Gpio4a2E::B00,
            1 => Gpio4a2E::B01,
            2 => Gpio4a2E::B10,
            3 => Gpio4a2E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a2E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a2E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a2E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a2E::B11
    }
}
#[doc = "Field `GPIO4A2_E` writer - GPIO4A2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a2EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a2E>;
impl<'a, REG> Gpio4a2EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a2E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a2E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a2E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a2E::B11)
    }
}
#[doc = "GPIO4A3 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a3E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4a3E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a3E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a3E {
    type Ux = u8;
}
#[doc = "Field `GPIO4A3_E` reader - GPIO4A3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a3ER = crate::FieldReader<Gpio4a3E>;
impl Gpio4a3ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a3E {
        match self.bits {
            0 => Gpio4a3E::B00,
            1 => Gpio4a3E::B01,
            2 => Gpio4a3E::B10,
            3 => Gpio4a3E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a3E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a3E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a3E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a3E::B11
    }
}
#[doc = "Field `GPIO4A3_E` writer - GPIO4A3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a3EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a3E>;
impl<'a, REG> Gpio4a3EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a3E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a3E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a3E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a3E::B11)
    }
}
#[doc = "GPIO4A4 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a4E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4a4E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a4E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a4E {
    type Ux = u8;
}
#[doc = "Field `GPIO4A4_E` reader - GPIO4A4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a4ER = crate::FieldReader<Gpio4a4E>;
impl Gpio4a4ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a4E {
        match self.bits {
            0 => Gpio4a4E::B00,
            1 => Gpio4a4E::B01,
            2 => Gpio4a4E::B10,
            3 => Gpio4a4E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a4E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a4E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a4E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a4E::B11
    }
}
#[doc = "Field `GPIO4A4_E` writer - GPIO4A4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a4EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a4E>;
impl<'a, REG> Gpio4a4EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a4E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a4E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a4E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a4E::B11)
    }
}
#[doc = "GPIO4A5 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a5E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4a5E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a5E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a5E {
    type Ux = u8;
}
#[doc = "Field `GPIO4A5_E` reader - GPIO4A5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a5ER = crate::FieldReader<Gpio4a5E>;
impl Gpio4a5ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a5E {
        match self.bits {
            0 => Gpio4a5E::B00,
            1 => Gpio4a5E::B01,
            2 => Gpio4a5E::B10,
            3 => Gpio4a5E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a5E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a5E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a5E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a5E::B11
    }
}
#[doc = "Field `GPIO4A5_E` writer - GPIO4A5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a5EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a5E>;
impl<'a, REG> Gpio4a5EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a5E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a5E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a5E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a5E::B11)
    }
}
#[doc = "GPIO4A6 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a6E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4a6E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a6E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a6E {
    type Ux = u8;
}
#[doc = "Field `GPIO4A6_E` reader - GPIO4A6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a6ER = crate::FieldReader<Gpio4a6E>;
impl Gpio4a6ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a6E {
        match self.bits {
            0 => Gpio4a6E::B00,
            1 => Gpio4a6E::B01,
            2 => Gpio4a6E::B10,
            3 => Gpio4a6E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a6E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a6E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a6E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a6E::B11
    }
}
#[doc = "Field `GPIO4A6_E` writer - GPIO4A6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a6EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a6E>;
impl<'a, REG> Gpio4a6EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a6E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a6E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a6E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a6E::B11)
    }
}
#[doc = "GPIO4A7 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a7E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4a7E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a7E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a7E {
    type Ux = u8;
}
#[doc = "Field `GPIO4A7_E` reader - GPIO4A7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a7ER = crate::FieldReader<Gpio4a7E>;
impl Gpio4a7ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a7E {
        match self.bits {
            0 => Gpio4a7E::B00,
            1 => Gpio4a7E::B01,
            2 => Gpio4a7E::B10,
            3 => Gpio4a7E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a7E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a7E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a7E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a7E::B11
    }
}
#[doc = "Field `GPIO4A7_E` writer - GPIO4A7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4a7EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a7E>;
impl<'a, REG> Gpio4a7EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a7E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a7E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a7E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a7E::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO4A0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a0_e(&self) -> Gpio4a0ER {
        Gpio4a0ER::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO4A1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a1_e(&self) -> Gpio4a1ER {
        Gpio4a1ER::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO4A2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a2_e(&self) -> Gpio4a2ER {
        Gpio4a2ER::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO4A3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a3_e(&self) -> Gpio4a3ER {
        Gpio4a3ER::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO4A4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a4_e(&self) -> Gpio4a4ER {
        Gpio4a4ER::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO4A5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a5_e(&self) -> Gpio4a5ER {
        Gpio4a5ER::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO4A6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a6_e(&self) -> Gpio4a6ER {
        Gpio4a6ER::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO4A7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4a7_e(&self) -> Gpio4a7ER {
        Gpio4a7ER::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO4A0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a0_e(&mut self) -> Gpio4a0EW<Gpio4aESpec> {
        Gpio4a0EW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO4A1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a1_e(&mut self) -> Gpio4a1EW<Gpio4aESpec> {
        Gpio4a1EW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO4A2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a2_e(&mut self) -> Gpio4a2EW<Gpio4aESpec> {
        Gpio4a2EW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO4A3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a3_e(&mut self) -> Gpio4a3EW<Gpio4aESpec> {
        Gpio4a3EW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO4A4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a4_e(&mut self) -> Gpio4a4EW<Gpio4aESpec> {
        Gpio4a4EW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO4A5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a5_e(&mut self) -> Gpio4a5EW<Gpio4aESpec> {
        Gpio4a5EW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO4A6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a6_e(&mut self) -> Gpio4a6EW<Gpio4aESpec> {
        Gpio4a6EW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO4A7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a7_e(&mut self) -> Gpio4a7EW<Gpio4aESpec> {
        Gpio4a7EW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio4aESpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO4A drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio4a_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio4a_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio4aESpec;
impl crate::RegisterSpec for Gpio4aESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio4a_e::R`](R) reader structure"]
impl crate::Readable for Gpio4aESpec {}
#[doc = "`write(|w| ..)` method takes [`gpio4a_e::W`](W) writer structure"]
impl crate::Writable for Gpio4aESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO4A_E to value 0"]
impl crate::Resettable for Gpio4aESpec {
    const RESET_VALUE: u32 = 0;
}
