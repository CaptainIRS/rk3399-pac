#[doc = "Register `GRF_GPIO4C_E` reader"]
pub type R = crate::R<GrfGpio4cESpec>;
#[doc = "Register `GRF_GPIO4C_E` writer"]
pub type W = crate::W<GrfGpio4cESpec>;
#[doc = "GPIO4C0 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c0E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4c0E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c0E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c0E {
    type Ux = u8;
}
#[doc = "Field `GPIO4C0_E` reader - GPIO4C0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c0ER = crate::FieldReader<Gpio4c0E>;
impl Gpio4c0ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c0E {
        match self.bits {
            0 => Gpio4c0E::B00,
            1 => Gpio4c0E::B01,
            2 => Gpio4c0E::B10,
            3 => Gpio4c0E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c0E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c0E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c0E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c0E::B11
    }
}
#[doc = "Field `GPIO4C0_E` writer - GPIO4C0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c0EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c0E>;
impl<'a, REG> Gpio4c0EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c0E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c0E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c0E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c0E::B11)
    }
}
#[doc = "GPIO4C1 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c1E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4c1E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c1E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c1E {
    type Ux = u8;
}
#[doc = "Field `GPIO4C1_E` reader - GPIO4C1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c1ER = crate::FieldReader<Gpio4c1E>;
impl Gpio4c1ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c1E {
        match self.bits {
            0 => Gpio4c1E::B00,
            1 => Gpio4c1E::B01,
            2 => Gpio4c1E::B10,
            3 => Gpio4c1E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c1E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c1E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c1E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c1E::B11
    }
}
#[doc = "Field `GPIO4C1_E` writer - GPIO4C1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c1EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c1E>;
impl<'a, REG> Gpio4c1EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c1E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c1E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c1E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c1E::B11)
    }
}
#[doc = "GPIO4C2 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c2E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4c2E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c2E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c2E {
    type Ux = u8;
}
#[doc = "Field `GPIO4C2_E` reader - GPIO4C2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c2ER = crate::FieldReader<Gpio4c2E>;
impl Gpio4c2ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c2E {
        match self.bits {
            0 => Gpio4c2E::B00,
            1 => Gpio4c2E::B01,
            2 => Gpio4c2E::B10,
            3 => Gpio4c2E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c2E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c2E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c2E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c2E::B11
    }
}
#[doc = "Field `GPIO4C2_E` writer - GPIO4C2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c2EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c2E>;
impl<'a, REG> Gpio4c2EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c2E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c2E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c2E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c2E::B11)
    }
}
#[doc = "GPIO4C3 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c3E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4c3E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c3E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c3E {
    type Ux = u8;
}
#[doc = "Field `GPIO4C3_E` reader - GPIO4C3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c3ER = crate::FieldReader<Gpio4c3E>;
impl Gpio4c3ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c3E {
        match self.bits {
            0 => Gpio4c3E::B00,
            1 => Gpio4c3E::B01,
            2 => Gpio4c3E::B10,
            3 => Gpio4c3E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c3E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c3E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c3E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c3E::B11
    }
}
#[doc = "Field `GPIO4C3_E` writer - GPIO4C3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c3EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c3E>;
impl<'a, REG> Gpio4c3EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c3E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c3E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c3E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c3E::B11)
    }
}
#[doc = "GPIO4C4 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c4E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4c4E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c4E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c4E {
    type Ux = u8;
}
#[doc = "Field `GPIO4C4_E` reader - GPIO4C4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c4ER = crate::FieldReader<Gpio4c4E>;
impl Gpio4c4ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c4E {
        match self.bits {
            0 => Gpio4c4E::B00,
            1 => Gpio4c4E::B01,
            2 => Gpio4c4E::B10,
            3 => Gpio4c4E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c4E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c4E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c4E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c4E::B11
    }
}
#[doc = "Field `GPIO4C4_E` writer - GPIO4C4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c4EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c4E>;
impl<'a, REG> Gpio4c4EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c4E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c4E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c4E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c4E::B11)
    }
}
#[doc = "GPIO4C5 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c5E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4c5E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c5E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c5E {
    type Ux = u8;
}
#[doc = "Field `GPIO4C5_E` reader - GPIO4C5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c5ER = crate::FieldReader<Gpio4c5E>;
impl Gpio4c5ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c5E {
        match self.bits {
            0 => Gpio4c5E::B00,
            1 => Gpio4c5E::B01,
            2 => Gpio4c5E::B10,
            3 => Gpio4c5E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c5E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c5E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c5E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c5E::B11
    }
}
#[doc = "Field `GPIO4C5_E` writer - GPIO4C5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c5EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c5E>;
impl<'a, REG> Gpio4c5EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c5E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c5E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c5E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c5E::B11)
    }
}
#[doc = "GPIO4C6 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c6E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4c6E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c6E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c6E {
    type Ux = u8;
}
#[doc = "Field `GPIO4C6_E` reader - GPIO4C6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c6ER = crate::FieldReader<Gpio4c6E>;
impl Gpio4c6ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c6E {
        match self.bits {
            0 => Gpio4c6E::B00,
            1 => Gpio4c6E::B01,
            2 => Gpio4c6E::B10,
            3 => Gpio4c6E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c6E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c6E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c6E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c6E::B11
    }
}
#[doc = "Field `GPIO4C6_E` writer - GPIO4C6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c6EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c6E>;
impl<'a, REG> Gpio4c6EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c6E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c6E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c6E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c6E::B11)
    }
}
#[doc = "GPIO4C7 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4c7E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio4c7E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4c7E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4c7E {
    type Ux = u8;
}
#[doc = "Field `GPIO4C7_E` reader - GPIO4C7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c7ER = crate::FieldReader<Gpio4c7E>;
impl Gpio4c7ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4c7E {
        match self.bits {
            0 => Gpio4c7E::B00,
            1 => Gpio4c7E::B01,
            2 => Gpio4c7E::B10,
            3 => Gpio4c7E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4c7E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4c7E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4c7E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4c7E::B11
    }
}
#[doc = "Field `GPIO4C7_E` writer - GPIO4C7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio4c7EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4c7E>;
impl<'a, REG> Gpio4c7EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c7E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c7E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c7E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4c7E::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO4C0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c0_e(&self) -> Gpio4c0ER {
        Gpio4c0ER::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO4C1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c1_e(&self) -> Gpio4c1ER {
        Gpio4c1ER::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO4C2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c2_e(&self) -> Gpio4c2ER {
        Gpio4c2ER::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO4C3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c3_e(&self) -> Gpio4c3ER {
        Gpio4c3ER::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO4C4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c4_e(&self) -> Gpio4c4ER {
        Gpio4c4ER::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO4C5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c5_e(&self) -> Gpio4c5ER {
        Gpio4c5ER::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO4C6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c6_e(&self) -> Gpio4c6ER {
        Gpio4c6ER::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO4C7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio4c7_e(&self) -> Gpio4c7ER {
        Gpio4c7ER::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO4C0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c0_e(&mut self) -> Gpio4c0EW<GrfGpio4cESpec> {
        Gpio4c0EW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO4C1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c1_e(&mut self) -> Gpio4c1EW<GrfGpio4cESpec> {
        Gpio4c1EW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO4C2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c2_e(&mut self) -> Gpio4c2EW<GrfGpio4cESpec> {
        Gpio4c2EW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO4C3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c3_e(&mut self) -> Gpio4c3EW<GrfGpio4cESpec> {
        Gpio4c3EW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO4C4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c4_e(&mut self) -> Gpio4c4EW<GrfGpio4cESpec> {
        Gpio4c4EW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO4C5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c5_e(&mut self) -> Gpio4c5EW<GrfGpio4cESpec> {
        Gpio4c5EW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO4C6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c6_e(&mut self) -> Gpio4c6EW<GrfGpio4cESpec> {
        Gpio4c6EW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO4C7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4c7_e(&mut self) -> Gpio4c7EW<GrfGpio4cESpec> {
        Gpio4c7EW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio4cESpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO4C drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4c_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4c_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio4cESpec;
impl crate::RegisterSpec for GrfGpio4cESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio4c_e::R`](R) reader structure"]
impl crate::Readable for GrfGpio4cESpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio4c_e::W`](W) writer structure"]
impl crate::Writable for GrfGpio4cESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO4C_E to value 0"]
impl crate::Resettable for GrfGpio4cESpec {
    const RESET_VALUE: u32 = 0;
}
