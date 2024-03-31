#[doc = "Register `GPIO2B_E` reader"]
pub type R = crate::R<Gpio2bESpec>;
#[doc = "Register `GPIO2B_E` writer"]
pub type W = crate::W<Gpio2bESpec>;
#[doc = "GPIO2B0 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b0E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2b0E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b0E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b0E {
    type Ux = u8;
}
#[doc = "Field `GPIO2B0_E` reader - GPIO2B0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b0ER = crate::FieldReader<Gpio2b0E>;
impl Gpio2b0ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b0E {
        match self.bits {
            0 => Gpio2b0E::B00,
            1 => Gpio2b0E::B01,
            2 => Gpio2b0E::B10,
            3 => Gpio2b0E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b0E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b0E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b0E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b0E::B11
    }
}
#[doc = "Field `GPIO2B0_E` writer - GPIO2B0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b0EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b0E>;
impl<'a, REG> Gpio2b0EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b0E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b0E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b0E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b0E::B11)
    }
}
#[doc = "GPIO2B1 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b1E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2b1E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b1E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b1E {
    type Ux = u8;
}
#[doc = "Field `GPIO2B1_E` reader - GPIO2B1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b1ER = crate::FieldReader<Gpio2b1E>;
impl Gpio2b1ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b1E {
        match self.bits {
            0 => Gpio2b1E::B00,
            1 => Gpio2b1E::B01,
            2 => Gpio2b1E::B10,
            3 => Gpio2b1E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b1E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b1E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b1E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b1E::B11
    }
}
#[doc = "Field `GPIO2B1_E` writer - GPIO2B1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b1EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b1E>;
impl<'a, REG> Gpio2b1EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b1E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b1E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b1E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b1E::B11)
    }
}
#[doc = "GPIO2B2 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b2E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2b2E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b2E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b2E {
    type Ux = u8;
}
#[doc = "Field `GPIO2B2_E` reader - GPIO2B2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b2ER = crate::FieldReader<Gpio2b2E>;
impl Gpio2b2ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b2E {
        match self.bits {
            0 => Gpio2b2E::B00,
            1 => Gpio2b2E::B01,
            2 => Gpio2b2E::B10,
            3 => Gpio2b2E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b2E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b2E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b2E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b2E::B11
    }
}
#[doc = "Field `GPIO2B2_E` writer - GPIO2B2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b2EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b2E>;
impl<'a, REG> Gpio2b2EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b2E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b2E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b2E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b2E::B11)
    }
}
#[doc = "GPIO2B3 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b3E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2b3E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b3E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b3E {
    type Ux = u8;
}
#[doc = "Field `GPIO2B3_E` reader - GPIO2B3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b3ER = crate::FieldReader<Gpio2b3E>;
impl Gpio2b3ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b3E {
        match self.bits {
            0 => Gpio2b3E::B00,
            1 => Gpio2b3E::B01,
            2 => Gpio2b3E::B10,
            3 => Gpio2b3E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b3E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b3E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b3E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b3E::B11
    }
}
#[doc = "Field `GPIO2B3_E` writer - GPIO2B3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b3EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b3E>;
impl<'a, REG> Gpio2b3EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b3E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b3E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b3E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b3E::B11)
    }
}
#[doc = "GPIO2B4 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b4E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2b4E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b4E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b4E {
    type Ux = u8;
}
#[doc = "Field `GPIO2B4_E` reader - GPIO2B4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b4ER = crate::FieldReader<Gpio2b4E>;
impl Gpio2b4ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b4E {
        match self.bits {
            0 => Gpio2b4E::B00,
            1 => Gpio2b4E::B01,
            2 => Gpio2b4E::B10,
            3 => Gpio2b4E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b4E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b4E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b4E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b4E::B11
    }
}
#[doc = "Field `GPIO2B4_E` writer - GPIO2B4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b4EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b4E>;
impl<'a, REG> Gpio2b4EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b4E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b4E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b4E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b4E::B11)
    }
}
#[doc = "GPIO2B5 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b5E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2b5E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b5E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b5E {
    type Ux = u8;
}
#[doc = "Field `GPIO2B5_E` reader - GPIO2B5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b5ER = crate::FieldReader<Gpio2b5E>;
impl Gpio2b5ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b5E {
        match self.bits {
            0 => Gpio2b5E::B00,
            1 => Gpio2b5E::B01,
            2 => Gpio2b5E::B10,
            3 => Gpio2b5E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b5E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b5E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b5E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b5E::B11
    }
}
#[doc = "Field `GPIO2B5_E` writer - GPIO2B5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b5EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b5E>;
impl<'a, REG> Gpio2b5EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b5E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b5E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b5E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b5E::B11)
    }
}
#[doc = "GPIO2B6 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b6E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2b6E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b6E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b6E {
    type Ux = u8;
}
#[doc = "Field `GPIO2B6_E` reader - GPIO2B6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b6ER = crate::FieldReader<Gpio2b6E>;
impl Gpio2b6ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b6E {
        match self.bits {
            0 => Gpio2b6E::B00,
            1 => Gpio2b6E::B01,
            2 => Gpio2b6E::B10,
            3 => Gpio2b6E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b6E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b6E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b6E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b6E::B11
    }
}
#[doc = "Field `GPIO2B6_E` writer - GPIO2B6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b6EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b6E>;
impl<'a, REG> Gpio2b6EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b6E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b6E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b6E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b6E::B11)
    }
}
#[doc = "GPIO2B7 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2b7E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2b7E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2b7E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2b7E {
    type Ux = u8;
}
#[doc = "Field `GPIO2B7_E` reader - GPIO2B7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b7ER = crate::FieldReader<Gpio2b7E>;
impl Gpio2b7ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2b7E {
        match self.bits {
            0 => Gpio2b7E::B00,
            1 => Gpio2b7E::B01,
            2 => Gpio2b7E::B10,
            3 => Gpio2b7E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2b7E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2b7E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2b7E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2b7E::B11
    }
}
#[doc = "Field `GPIO2B7_E` writer - GPIO2B7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio2b7EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2b7E>;
impl<'a, REG> Gpio2b7EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b7E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b7E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b7E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2b7E::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO2B0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2b0_e(&self) -> Gpio2b0ER {
        Gpio2b0ER::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO2B1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2b1_e(&self) -> Gpio2b1ER {
        Gpio2b1ER::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO2B2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2b2_e(&self) -> Gpio2b2ER {
        Gpio2b2ER::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO2B3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2b3_e(&self) -> Gpio2b3ER {
        Gpio2b3ER::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO2B4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2b4_e(&self) -> Gpio2b4ER {
        Gpio2b4ER::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO2B5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2b5_e(&self) -> Gpio2b5ER {
        Gpio2b5ER::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO2B6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2b6_e(&self) -> Gpio2b6ER {
        Gpio2b6ER::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO2B7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2b7_e(&self) -> Gpio2b7ER {
        Gpio2b7ER::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO2B0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b0_e(&mut self) -> Gpio2b0EW<Gpio2bESpec> {
        Gpio2b0EW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO2B1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b1_e(&mut self) -> Gpio2b1EW<Gpio2bESpec> {
        Gpio2b1EW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO2B2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b2_e(&mut self) -> Gpio2b2EW<Gpio2bESpec> {
        Gpio2b2EW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO2B3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b3_e(&mut self) -> Gpio2b3EW<Gpio2bESpec> {
        Gpio2b3EW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO2B4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b4_e(&mut self) -> Gpio2b4EW<Gpio2bESpec> {
        Gpio2b4EW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO2B5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b5_e(&mut self) -> Gpio2b5EW<Gpio2bESpec> {
        Gpio2b5EW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO2B6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b6_e(&mut self) -> Gpio2b6EW<Gpio2bESpec> {
        Gpio2b6EW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO2B7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2b7_e(&mut self) -> Gpio2b7EW<Gpio2bESpec> {
        Gpio2b7EW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio2bESpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2B drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio2b_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio2b_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio2bESpec;
impl crate::RegisterSpec for Gpio2bESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio2b_e::R`](R) reader structure"]
impl crate::Readable for Gpio2bESpec {}
#[doc = "`write(|w| ..)` method takes [`gpio2b_e::W`](W) writer structure"]
impl crate::Writable for Gpio2bESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO2B_E to value 0"]
impl crate::Resettable for Gpio2bESpec {
    const RESET_VALUE: u32 = 0;
}
