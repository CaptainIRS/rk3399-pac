#[doc = "Register `GRF_GPIO2A_E` reader"]
pub type R = crate::R<GrfGpio2aESpec>;
#[doc = "Register `GRF_GPIO2A_E` writer"]
pub type W = crate::W<GrfGpio2aESpec>;
#[doc = "GPIO2A0 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a0E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2a0E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a0E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a0E {
    type Ux = u8;
}
#[doc = "Field `GPIO2A0_E` reader - GPIO2A0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a0ER = crate::FieldReader<Gpio2a0E>;
impl Gpio2a0ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a0E {
        match self.bits {
            0 => Gpio2a0E::B00,
            1 => Gpio2a0E::B01,
            2 => Gpio2a0E::B10,
            3 => Gpio2a0E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a0E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a0E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a0E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a0E::B11
    }
}
#[doc = "Field `GPIO2A0_E` writer - GPIO2A0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a0EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a0E>;
impl<'a, REG> Gpio2a0EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a0E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a0E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a0E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a0E::B11)
    }
}
#[doc = "GPIO2A1 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a1E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2a1E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a1E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a1E {
    type Ux = u8;
}
#[doc = "Field `GPIO2A1_E` reader - GPIO2A1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a1ER = crate::FieldReader<Gpio2a1E>;
impl Gpio2a1ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a1E {
        match self.bits {
            0 => Gpio2a1E::B00,
            1 => Gpio2a1E::B01,
            2 => Gpio2a1E::B10,
            3 => Gpio2a1E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a1E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a1E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a1E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a1E::B11
    }
}
#[doc = "Field `GPIO2A1_E` writer - GPIO2A1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a1EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a1E>;
impl<'a, REG> Gpio2a1EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a1E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a1E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a1E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a1E::B11)
    }
}
#[doc = "GPIO2A2 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a2E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2a2E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a2E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a2E {
    type Ux = u8;
}
#[doc = "Field `GPIO2A2_E` reader - GPIO2A2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a2ER = crate::FieldReader<Gpio2a2E>;
impl Gpio2a2ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a2E {
        match self.bits {
            0 => Gpio2a2E::B00,
            1 => Gpio2a2E::B01,
            2 => Gpio2a2E::B10,
            3 => Gpio2a2E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a2E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a2E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a2E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a2E::B11
    }
}
#[doc = "Field `GPIO2A2_E` writer - GPIO2A2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a2EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a2E>;
impl<'a, REG> Gpio2a2EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a2E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a2E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a2E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a2E::B11)
    }
}
#[doc = "GPIO2A3 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a3E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2a3E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a3E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a3E {
    type Ux = u8;
}
#[doc = "Field `GPIO2A3_E` reader - GPIO2A3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a3ER = crate::FieldReader<Gpio2a3E>;
impl Gpio2a3ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a3E {
        match self.bits {
            0 => Gpio2a3E::B00,
            1 => Gpio2a3E::B01,
            2 => Gpio2a3E::B10,
            3 => Gpio2a3E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a3E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a3E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a3E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a3E::B11
    }
}
#[doc = "Field `GPIO2A3_E` writer - GPIO2A3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a3EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a3E>;
impl<'a, REG> Gpio2a3EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a3E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a3E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a3E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a3E::B11)
    }
}
#[doc = "GPIO2A4 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a4E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2a4E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a4E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a4E {
    type Ux = u8;
}
#[doc = "Field `GPIO2A4_E` reader - GPIO2A4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a4ER = crate::FieldReader<Gpio2a4E>;
impl Gpio2a4ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a4E {
        match self.bits {
            0 => Gpio2a4E::B00,
            1 => Gpio2a4E::B01,
            2 => Gpio2a4E::B10,
            3 => Gpio2a4E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a4E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a4E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a4E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a4E::B11
    }
}
#[doc = "Field `GPIO2A4_E` writer - GPIO2A4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a4EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a4E>;
impl<'a, REG> Gpio2a4EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a4E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a4E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a4E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a4E::B11)
    }
}
#[doc = "GPIO2A5 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a5E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2a5E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a5E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a5E {
    type Ux = u8;
}
#[doc = "Field `GPIO2A5_E` reader - GPIO2A5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a5ER = crate::FieldReader<Gpio2a5E>;
impl Gpio2a5ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a5E {
        match self.bits {
            0 => Gpio2a5E::B00,
            1 => Gpio2a5E::B01,
            2 => Gpio2a5E::B10,
            3 => Gpio2a5E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a5E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a5E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a5E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a5E::B11
    }
}
#[doc = "Field `GPIO2A5_E` writer - GPIO2A5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a5EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a5E>;
impl<'a, REG> Gpio2a5EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a5E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a5E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a5E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a5E::B11)
    }
}
#[doc = "GPIO2A6 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a6E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2a6E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a6E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a6E {
    type Ux = u8;
}
#[doc = "Field `GPIO2A6_E` reader - GPIO2A6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a6ER = crate::FieldReader<Gpio2a6E>;
impl Gpio2a6ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a6E {
        match self.bits {
            0 => Gpio2a6E::B00,
            1 => Gpio2a6E::B01,
            2 => Gpio2a6E::B10,
            3 => Gpio2a6E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a6E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a6E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a6E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a6E::B11
    }
}
#[doc = "Field `GPIO2A6_E` writer - GPIO2A6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a6EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a6E>;
impl<'a, REG> Gpio2a6EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a6E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a6E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a6E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a6E::B11)
    }
}
#[doc = "GPIO2A7 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2a7E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2a7E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2a7E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2a7E {
    type Ux = u8;
}
#[doc = "Field `GPIO2A7_E` reader - GPIO2A7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a7ER = crate::FieldReader<Gpio2a7E>;
impl Gpio2a7ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2a7E {
        match self.bits {
            0 => Gpio2a7E::B00,
            1 => Gpio2a7E::B01,
            2 => Gpio2a7E::B10,
            3 => Gpio2a7E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2a7E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2a7E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2a7E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2a7E::B11
    }
}
#[doc = "Field `GPIO2A7_E` writer - GPIO2A7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2a7EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2a7E>;
impl<'a, REG> Gpio2a7EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a7E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a7E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a7E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2a7E::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO2A0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a0_e(&self) -> Gpio2a0ER {
        Gpio2a0ER::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO2A1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a1_e(&self) -> Gpio2a1ER {
        Gpio2a1ER::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO2A2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a2_e(&self) -> Gpio2a2ER {
        Gpio2a2ER::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO2A3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a3_e(&self) -> Gpio2a3ER {
        Gpio2a3ER::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO2A4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a4_e(&self) -> Gpio2a4ER {
        Gpio2a4ER::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO2A5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a5_e(&self) -> Gpio2a5ER {
        Gpio2a5ER::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO2A6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a6_e(&self) -> Gpio2a6ER {
        Gpio2a6ER::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO2A7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2a7_e(&self) -> Gpio2a7ER {
        Gpio2a7ER::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO2A0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a0_e(&mut self) -> Gpio2a0EW<GrfGpio2aESpec> {
        Gpio2a0EW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO2A1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a1_e(&mut self) -> Gpio2a1EW<GrfGpio2aESpec> {
        Gpio2a1EW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO2A2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a2_e(&mut self) -> Gpio2a2EW<GrfGpio2aESpec> {
        Gpio2a2EW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO2A3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a3_e(&mut self) -> Gpio2a3EW<GrfGpio2aESpec> {
        Gpio2a3EW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO2A4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a4_e(&mut self) -> Gpio2a4EW<GrfGpio2aESpec> {
        Gpio2a4EW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO2A5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a5_e(&mut self) -> Gpio2a5EW<GrfGpio2aESpec> {
        Gpio2a5EW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO2A6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a6_e(&mut self) -> Gpio2a6EW<GrfGpio2aESpec> {
        Gpio2a6EW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO2A7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2a7_e(&mut self) -> Gpio2a7EW<GrfGpio2aESpec> {
        Gpio2a7EW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio2aESpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2A drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2a_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2a_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio2aESpec;
impl crate::RegisterSpec for GrfGpio2aESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio2a_e::R`](R) reader structure"]
impl crate::Readable for GrfGpio2aESpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio2a_e::W`](W) writer structure"]
impl crate::Writable for GrfGpio2aESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO2A_E to value 0"]
impl crate::Resettable for GrfGpio2aESpec {
    const RESET_VALUE: u32 = 0;
}
