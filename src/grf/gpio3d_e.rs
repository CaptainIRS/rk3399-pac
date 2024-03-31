#[doc = "Register `GPIO3D_E` reader"]
pub type R = crate::R<Gpio3dESpec>;
#[doc = "Register `GPIO3D_E` writer"]
pub type W = crate::W<Gpio3dESpec>;
#[doc = "GPIO3D0 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d0E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio3d0E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d0E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d0E {
    type Ux = u8;
}
#[doc = "Field `GPIO3D0_E` reader - GPIO3D0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d0ER = crate::FieldReader<Gpio3d0E>;
impl Gpio3d0ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d0E {
        match self.bits {
            0 => Gpio3d0E::B00,
            1 => Gpio3d0E::B01,
            2 => Gpio3d0E::B10,
            3 => Gpio3d0E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d0E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d0E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d0E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d0E::B11
    }
}
#[doc = "Field `GPIO3D0_E` writer - GPIO3D0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d0EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d0E>;
impl<'a, REG> Gpio3d0EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d0E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d0E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d0E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d0E::B11)
    }
}
#[doc = "GPIO3D1 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d1E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio3d1E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d1E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d1E {
    type Ux = u8;
}
#[doc = "Field `GPIO3D1_E` reader - GPIO3D1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d1ER = crate::FieldReader<Gpio3d1E>;
impl Gpio3d1ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d1E {
        match self.bits {
            0 => Gpio3d1E::B00,
            1 => Gpio3d1E::B01,
            2 => Gpio3d1E::B10,
            3 => Gpio3d1E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d1E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d1E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d1E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d1E::B11
    }
}
#[doc = "Field `GPIO3D1_E` writer - GPIO3D1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d1EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d1E>;
impl<'a, REG> Gpio3d1EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d1E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d1E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d1E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d1E::B11)
    }
}
#[doc = "GPIO3D2 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d2E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio3d2E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d2E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d2E {
    type Ux = u8;
}
#[doc = "Field `GPIO3D2_E` reader - GPIO3D2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d2ER = crate::FieldReader<Gpio3d2E>;
impl Gpio3d2ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d2E {
        match self.bits {
            0 => Gpio3d2E::B00,
            1 => Gpio3d2E::B01,
            2 => Gpio3d2E::B10,
            3 => Gpio3d2E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d2E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d2E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d2E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d2E::B11
    }
}
#[doc = "Field `GPIO3D2_E` writer - GPIO3D2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d2EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d2E>;
impl<'a, REG> Gpio3d2EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d2E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d2E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d2E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d2E::B11)
    }
}
#[doc = "GPIO3D3 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d3E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio3d3E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d3E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d3E {
    type Ux = u8;
}
#[doc = "Field `GPIO3D3_E` reader - GPIO3D3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d3ER = crate::FieldReader<Gpio3d3E>;
impl Gpio3d3ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d3E {
        match self.bits {
            0 => Gpio3d3E::B00,
            1 => Gpio3d3E::B01,
            2 => Gpio3d3E::B10,
            3 => Gpio3d3E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d3E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d3E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d3E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d3E::B11
    }
}
#[doc = "Field `GPIO3D3_E` writer - GPIO3D3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d3EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d3E>;
impl<'a, REG> Gpio3d3EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d3E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d3E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d3E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d3E::B11)
    }
}
#[doc = "GPIO3D4 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d4E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio3d4E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d4E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d4E {
    type Ux = u8;
}
#[doc = "Field `GPIO3D4_E` reader - GPIO3D4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d4ER = crate::FieldReader<Gpio3d4E>;
impl Gpio3d4ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d4E {
        match self.bits {
            0 => Gpio3d4E::B00,
            1 => Gpio3d4E::B01,
            2 => Gpio3d4E::B10,
            3 => Gpio3d4E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d4E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d4E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d4E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d4E::B11
    }
}
#[doc = "Field `GPIO3D4_E` writer - GPIO3D4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d4EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d4E>;
impl<'a, REG> Gpio3d4EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d4E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d4E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d4E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d4E::B11)
    }
}
#[doc = "GPIO3D5 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d5E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio3d5E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d5E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d5E {
    type Ux = u8;
}
#[doc = "Field `GPIO3D5_E` reader - GPIO3D5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d5ER = crate::FieldReader<Gpio3d5E>;
impl Gpio3d5ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d5E {
        match self.bits {
            0 => Gpio3d5E::B00,
            1 => Gpio3d5E::B01,
            2 => Gpio3d5E::B10,
            3 => Gpio3d5E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d5E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d5E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d5E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d5E::B11
    }
}
#[doc = "Field `GPIO3D5_E` writer - GPIO3D5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d5EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d5E>;
impl<'a, REG> Gpio3d5EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d5E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d5E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d5E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d5E::B11)
    }
}
#[doc = "GPIO3D6 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d6E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio3d6E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d6E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d6E {
    type Ux = u8;
}
#[doc = "Field `GPIO3D6_E` reader - GPIO3D6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d6ER = crate::FieldReader<Gpio3d6E>;
impl Gpio3d6ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d6E {
        match self.bits {
            0 => Gpio3d6E::B00,
            1 => Gpio3d6E::B01,
            2 => Gpio3d6E::B10,
            3 => Gpio3d6E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d6E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d6E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d6E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d6E::B11
    }
}
#[doc = "Field `GPIO3D6_E` writer - GPIO3D6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d6EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d6E>;
impl<'a, REG> Gpio3d6EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d6E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d6E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d6E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d6E::B11)
    }
}
#[doc = "GPIO3D7 drive strength control, every GPIO\n\nbit corresponding to 2bits\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3d7E {
    #[doc = "0: level 0"]
    B00 = 0,
    #[doc = "1: level 1"]
    B01 = 1,
    #[doc = "2: level 2"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio3d7E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3d7E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3d7E {
    type Ux = u8;
}
#[doc = "Field `GPIO3D7_E` reader - GPIO3D7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d7ER = crate::FieldReader<Gpio3d7E>;
impl Gpio3d7ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3d7E {
        match self.bits {
            0 => Gpio3d7E::B00,
            1 => Gpio3d7E::B01,
            2 => Gpio3d7E::B10,
            3 => Gpio3d7E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 0"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3d7E::B00
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3d7E::B01
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3d7E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3d7E::B11
    }
}
#[doc = "Field `GPIO3D7_E` writer - GPIO3D7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
pub type Gpio3d7EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3d7E>;
impl<'a, REG> Gpio3d7EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 0"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d7E::B00)
    }
    #[doc = "level 1"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d7E::B01)
    }
    #[doc = "level 2"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d7E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3d7E::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO3D0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d0_e(&self) -> Gpio3d0ER {
        Gpio3d0ER::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO3D1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d1_e(&self) -> Gpio3d1ER {
        Gpio3d1ER::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO3D2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d2_e(&self) -> Gpio3d2ER {
        Gpio3d2ER::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO3D3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d3_e(&self) -> Gpio3d3ER {
        Gpio3d3ER::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO3D4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d4_e(&self) -> Gpio3d4ER {
        Gpio3d4ER::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO3D5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d5_e(&self) -> Gpio3d5ER {
        Gpio3d5ER::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO3D6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d6_e(&self) -> Gpio3d6ER {
        Gpio3d6ER::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO3D7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio3d7_e(&self) -> Gpio3d7ER {
        Gpio3d7ER::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO3D0 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d0_e(&mut self) -> Gpio3d0EW<Gpio3dESpec> {
        Gpio3d0EW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO3D1 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d1_e(&mut self) -> Gpio3d1EW<Gpio3dESpec> {
        Gpio3d1EW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO3D2 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d2_e(&mut self) -> Gpio3d2EW<Gpio3dESpec> {
        Gpio3d2EW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO3D3 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d3_e(&mut self) -> Gpio3d3EW<Gpio3dESpec> {
        Gpio3d3EW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO3D4 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d4_e(&mut self) -> Gpio3d4EW<Gpio3dESpec> {
        Gpio3d4EW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO3D5 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d5_e(&mut self) -> Gpio3d5EW<Gpio3dESpec> {
        Gpio3d5EW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO3D6 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d6_e(&mut self) -> Gpio3d6EW<Gpio3dESpec> {
        Gpio3d6EW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO3D7 drive strength control, every GPIO\n\nbit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3d7_e(&mut self) -> Gpio3d7EW<Gpio3dESpec> {
        Gpio3d7EW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<Gpio3dESpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO3D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gpio3d_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gpio3d_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Gpio3dESpec;
impl crate::RegisterSpec for Gpio3dESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gpio3d_e::R`](R) reader structure"]
impl crate::Readable for Gpio3dESpec {}
#[doc = "`write(|w| ..)` method takes [`gpio3d_e::W`](W) writer structure"]
impl crate::Writable for Gpio3dESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GPIO3D_E to value 0"]
impl crate::Resettable for Gpio3dESpec {
    const RESET_VALUE: u32 = 0;
}
