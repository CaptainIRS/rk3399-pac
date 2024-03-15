#[doc = "Register `GRF_GPIO2D_E` reader"]
pub type R = crate::R<GrfGpio2dESpec>;
#[doc = "Register `GRF_GPIO2D_E` writer"]
pub type W = crate::W<GrfGpio2dESpec>;
#[doc = "GPIO2D0 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d0E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2d0E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d0E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d0E {
    type Ux = u8;
}
#[doc = "Field `GPIO2D0_E` reader - GPIO2D0 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d0ER = crate::FieldReader<Gpio2d0E>;
impl Gpio2d0ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d0E {
        match self.bits {
            0 => Gpio2d0E::B00,
            1 => Gpio2d0E::B01,
            2 => Gpio2d0E::B10,
            3 => Gpio2d0E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d0E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d0E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d0E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d0E::B11
    }
}
#[doc = "Field `GPIO2D0_E` writer - GPIO2D0 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d0EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d0E>;
impl<'a, REG> Gpio2d0EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d0E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d0E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d0E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d0E::B11)
    }
}
#[doc = "GPIO2D1 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d1E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2d1E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d1E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d1E {
    type Ux = u8;
}
#[doc = "Field `GPIO2D1_E` reader - GPIO2D1 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d1ER = crate::FieldReader<Gpio2d1E>;
impl Gpio2d1ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d1E {
        match self.bits {
            0 => Gpio2d1E::B00,
            1 => Gpio2d1E::B01,
            2 => Gpio2d1E::B10,
            3 => Gpio2d1E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d1E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d1E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d1E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d1E::B11
    }
}
#[doc = "Field `GPIO2D1_E` writer - GPIO2D1 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d1EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d1E>;
impl<'a, REG> Gpio2d1EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d1E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d1E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d1E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d1E::B11)
    }
}
#[doc = "GPIO2D2 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d2E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2d2E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d2E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d2E {
    type Ux = u8;
}
#[doc = "Field `GPIO2D2_E` reader - GPIO2D2 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d2ER = crate::FieldReader<Gpio2d2E>;
impl Gpio2d2ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d2E {
        match self.bits {
            0 => Gpio2d2E::B00,
            1 => Gpio2d2E::B01,
            2 => Gpio2d2E::B10,
            3 => Gpio2d2E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d2E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d2E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d2E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d2E::B11
    }
}
#[doc = "Field `GPIO2D2_E` writer - GPIO2D2 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d2EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d2E>;
impl<'a, REG> Gpio2d2EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d2E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d2E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d2E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d2E::B11)
    }
}
#[doc = "GPIO2D3 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d3E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2d3E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d3E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d3E {
    type Ux = u8;
}
#[doc = "Field `GPIO2D3_E` reader - GPIO2D3 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d3ER = crate::FieldReader<Gpio2d3E>;
impl Gpio2d3ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d3E {
        match self.bits {
            0 => Gpio2d3E::B00,
            1 => Gpio2d3E::B01,
            2 => Gpio2d3E::B10,
            3 => Gpio2d3E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d3E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d3E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d3E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d3E::B11
    }
}
#[doc = "Field `GPIO2D3_E` writer - GPIO2D3 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d3EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d3E>;
impl<'a, REG> Gpio2d3EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d3E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d3E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d3E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d3E::B11)
    }
}
#[doc = "GPIO2D4 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d4E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2d4E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d4E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d4E {
    type Ux = u8;
}
#[doc = "Field `GPIO2D4_E` reader - GPIO2D4 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d4ER = crate::FieldReader<Gpio2d4E>;
impl Gpio2d4ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d4E {
        match self.bits {
            0 => Gpio2d4E::B00,
            1 => Gpio2d4E::B01,
            2 => Gpio2d4E::B10,
            3 => Gpio2d4E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d4E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d4E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d4E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d4E::B11
    }
}
#[doc = "Field `GPIO2D4_E` writer - GPIO2D4 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d4EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d4E>;
impl<'a, REG> Gpio2d4EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d4E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d4E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d4E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d4E::B11)
    }
}
#[doc = "GPIO2D5 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d5E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2d5E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d5E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d5E {
    type Ux = u8;
}
#[doc = "Field `GPIO2D5_E` reader - GPIO2D5 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d5ER = crate::FieldReader<Gpio2d5E>;
impl Gpio2d5ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d5E {
        match self.bits {
            0 => Gpio2d5E::B00,
            1 => Gpio2d5E::B01,
            2 => Gpio2d5E::B10,
            3 => Gpio2d5E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d5E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d5E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d5E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d5E::B11
    }
}
#[doc = "Field `GPIO2D5_E` writer - GPIO2D5 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d5EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d5E>;
impl<'a, REG> Gpio2d5EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d5E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d5E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d5E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d5E::B11)
    }
}
#[doc = "GPIO2D6 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d6E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2d6E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d6E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d6E {
    type Ux = u8;
}
#[doc = "Field `GPIO2D6_E` reader - GPIO2D6 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d6ER = crate::FieldReader<Gpio2d6E>;
impl Gpio2d6ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d6E {
        match self.bits {
            0 => Gpio2d6E::B00,
            1 => Gpio2d6E::B01,
            2 => Gpio2d6E::B10,
            3 => Gpio2d6E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d6E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d6E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d6E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d6E::B11
    }
}
#[doc = "Field `GPIO2D6_E` writer - GPIO2D6 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d6EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d6E>;
impl<'a, REG> Gpio2d6EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d6E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d6E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d6E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d6E::B11)
    }
}
#[doc = "GPIO2D7 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2d7E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2d7E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2d7E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2d7E {
    type Ux = u8;
}
#[doc = "Field `GPIO2D7_E` reader - GPIO2D7 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d7ER = crate::FieldReader<Gpio2d7E>;
impl Gpio2d7ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2d7E {
        match self.bits {
            0 => Gpio2d7E::B00,
            1 => Gpio2d7E::B01,
            2 => Gpio2d7E::B10,
            3 => Gpio2d7E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2d7E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2d7E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2d7E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2d7E::B11
    }
}
#[doc = "Field `GPIO2D7_E` writer - GPIO2D7 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2d7EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2d7E>;
impl<'a, REG> Gpio2d7EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d7E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d7E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d7E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2d7E::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO2D0 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2d0_e(&self) -> Gpio2d0ER {
        Gpio2d0ER::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO2D1 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2d1_e(&self) -> Gpio2d1ER {
        Gpio2d1ER::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO2D2 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2d2_e(&self) -> Gpio2d2ER {
        Gpio2d2ER::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO2D3 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2d3_e(&self) -> Gpio2d3ER {
        Gpio2d3ER::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO2D4 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2d4_e(&self) -> Gpio2d4ER {
        Gpio2d4ER::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO2D5 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2d5_e(&self) -> Gpio2d5ER {
        Gpio2d5ER::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO2D6 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2d6_e(&self) -> Gpio2d6ER {
        Gpio2d6ER::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO2D7 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2d7_e(&self) -> Gpio2d7ER {
        Gpio2d7ER::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO2D0 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d0_e(&mut self) -> Gpio2d0EW<GrfGpio2dESpec> {
        Gpio2d0EW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO2D1 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d1_e(&mut self) -> Gpio2d1EW<GrfGpio2dESpec> {
        Gpio2d1EW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO2D2 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d2_e(&mut self) -> Gpio2d2EW<GrfGpio2dESpec> {
        Gpio2d2EW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO2D3 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d3_e(&mut self) -> Gpio2d3EW<GrfGpio2dESpec> {
        Gpio2d3EW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO2D4 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d4_e(&mut self) -> Gpio2d4EW<GrfGpio2dESpec> {
        Gpio2d4EW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO2D5 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d5_e(&mut self) -> Gpio2d5EW<GrfGpio2dESpec> {
        Gpio2d5EW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO2D6 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d6_e(&mut self) -> Gpio2d6EW<GrfGpio2dESpec> {
        Gpio2d6EW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO2D7 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2d7_e(&mut self) -> Gpio2d7EW<GrfGpio2dESpec> {
        Gpio2d7EW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio2dESpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2D drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2d_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2d_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio2dESpec;
impl crate::RegisterSpec for GrfGpio2dESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio2d_e::R`](R) reader structure"]
impl crate::Readable for GrfGpio2dESpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio2d_e::W`](W) writer structure"]
impl crate::Writable for GrfGpio2dESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO2D_E to value 0"]
impl crate::Resettable for GrfGpio2dESpec {
    const RESET_VALUE: u32 = 0;
}
