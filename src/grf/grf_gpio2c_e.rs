#[doc = "Register `GRF_GPIO2C_E` reader"]
pub type R = crate::R<GrfGpio2cESpec>;
#[doc = "Register `GRF_GPIO2C_E` writer"]
pub type W = crate::W<GrfGpio2cESpec>;
#[doc = "GPIO2C0 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c0E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2c0E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c0E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c0E {
    type Ux = u8;
}
#[doc = "Field `GPIO2C0_E` reader - GPIO2C0 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c0ER = crate::FieldReader<Gpio2c0E>;
impl Gpio2c0ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c0E {
        match self.bits {
            0 => Gpio2c0E::B00,
            1 => Gpio2c0E::B01,
            2 => Gpio2c0E::B10,
            3 => Gpio2c0E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c0E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c0E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c0E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c0E::B11
    }
}
#[doc = "Field `GPIO2C0_E` writer - GPIO2C0 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c0EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c0E>;
impl<'a, REG> Gpio2c0EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c0E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c0E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c0E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c0E::B11)
    }
}
#[doc = "GPIO2C1 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c1E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2c1E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c1E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c1E {
    type Ux = u8;
}
#[doc = "Field `GPIO2C1_E` reader - GPIO2C1 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c1ER = crate::FieldReader<Gpio2c1E>;
impl Gpio2c1ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c1E {
        match self.bits {
            0 => Gpio2c1E::B00,
            1 => Gpio2c1E::B01,
            2 => Gpio2c1E::B10,
            3 => Gpio2c1E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c1E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c1E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c1E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c1E::B11
    }
}
#[doc = "Field `GPIO2C1_E` writer - GPIO2C1 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c1EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c1E>;
impl<'a, REG> Gpio2c1EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c1E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c1E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c1E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c1E::B11)
    }
}
#[doc = "GPIO2C2 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c2E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2c2E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c2E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c2E {
    type Ux = u8;
}
#[doc = "Field `GPIO2C2_E` reader - GPIO2C2 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c2ER = crate::FieldReader<Gpio2c2E>;
impl Gpio2c2ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c2E {
        match self.bits {
            0 => Gpio2c2E::B00,
            1 => Gpio2c2E::B01,
            2 => Gpio2c2E::B10,
            3 => Gpio2c2E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c2E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c2E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c2E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c2E::B11
    }
}
#[doc = "Field `GPIO2C2_E` writer - GPIO2C2 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c2EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c2E>;
impl<'a, REG> Gpio2c2EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c2E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c2E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c2E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c2E::B11)
    }
}
#[doc = "GPIO2C3 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c3E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2c3E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c3E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c3E {
    type Ux = u8;
}
#[doc = "Field `GPIO2C3_E` reader - GPIO2C3 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c3ER = crate::FieldReader<Gpio2c3E>;
impl Gpio2c3ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c3E {
        match self.bits {
            0 => Gpio2c3E::B00,
            1 => Gpio2c3E::B01,
            2 => Gpio2c3E::B10,
            3 => Gpio2c3E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c3E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c3E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c3E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c3E::B11
    }
}
#[doc = "Field `GPIO2C3_E` writer - GPIO2C3 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c3EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c3E>;
impl<'a, REG> Gpio2c3EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c3E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c3E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c3E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c3E::B11)
    }
}
#[doc = "GPIO2C4 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c4E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2c4E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c4E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c4E {
    type Ux = u8;
}
#[doc = "Field `GPIO2C4_E` reader - GPIO2C4 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c4ER = crate::FieldReader<Gpio2c4E>;
impl Gpio2c4ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c4E {
        match self.bits {
            0 => Gpio2c4E::B00,
            1 => Gpio2c4E::B01,
            2 => Gpio2c4E::B10,
            3 => Gpio2c4E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c4E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c4E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c4E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c4E::B11
    }
}
#[doc = "Field `GPIO2C4_E` writer - GPIO2C4 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c4EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c4E>;
impl<'a, REG> Gpio2c4EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c4E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c4E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c4E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c4E::B11)
    }
}
#[doc = "GPIO2C5 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c5E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2c5E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c5E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c5E {
    type Ux = u8;
}
#[doc = "Field `GPIO2C5_E` reader - GPIO2C5 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c5ER = crate::FieldReader<Gpio2c5E>;
impl Gpio2c5ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c5E {
        match self.bits {
            0 => Gpio2c5E::B00,
            1 => Gpio2c5E::B01,
            2 => Gpio2c5E::B10,
            3 => Gpio2c5E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c5E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c5E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c5E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c5E::B11
    }
}
#[doc = "Field `GPIO2C5_E` writer - GPIO2C5 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c5EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c5E>;
impl<'a, REG> Gpio2c5EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c5E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c5E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c5E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c5E::B11)
    }
}
#[doc = "GPIO2C6 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c6E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2c6E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c6E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c6E {
    type Ux = u8;
}
#[doc = "Field `GPIO2C6_E` reader - GPIO2C6 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c6ER = crate::FieldReader<Gpio2c6E>;
impl Gpio2c6ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c6E {
        match self.bits {
            0 => Gpio2c6E::B00,
            1 => Gpio2c6E::B01,
            2 => Gpio2c6E::B10,
            3 => Gpio2c6E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c6E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c6E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c6E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c6E::B11
    }
}
#[doc = "Field `GPIO2C6_E` writer - GPIO2C6 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c6EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c6E>;
impl<'a, REG> Gpio2c6EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c6E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c6E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c6E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c6E::B11)
    }
}
#[doc = "GPIO2C7 drive strength control, every GPIO bit corresponding to 2bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio2c7E {
    #[doc = "0: level 3"]
    B00 = 0,
    #[doc = "1: level 3"]
    B01 = 1,
    #[doc = "2: level 3"]
    B10 = 2,
    #[doc = "3: level 3"]
    B11 = 3,
}
impl From<Gpio2c7E> for u8 {
    #[inline(always)]
    fn from(variant: Gpio2c7E) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio2c7E {
    type Ux = u8;
}
#[doc = "Field `GPIO2C7_E` reader - GPIO2C7 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c7ER = crate::FieldReader<Gpio2c7E>;
impl Gpio2c7ER {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio2c7E {
        match self.bits {
            0 => Gpio2c7E::B00,
            1 => Gpio2c7E::B01,
            2 => Gpio2c7E::B10,
            3 => Gpio2c7E::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio2c7E::B00
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio2c7E::B01
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio2c7E::B10
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio2c7E::B11
    }
}
#[doc = "Field `GPIO2C7_E` writer - GPIO2C7 drive strength control, every GPIO bit corresponding to 2bits"]
pub type Gpio2c7EW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio2c7E>;
impl<'a, REG> Gpio2c7EW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c7E::B00)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c7E::B01)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c7E::B10)
    }
    #[doc = "level 3"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio2c7E::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO2C0 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c0_e(&self) -> Gpio2c0ER {
        Gpio2c0ER::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO2C1 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c1_e(&self) -> Gpio2c1ER {
        Gpio2c1ER::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO2C2 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c2_e(&self) -> Gpio2c2ER {
        Gpio2c2ER::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO2C3 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c3_e(&self) -> Gpio2c3ER {
        Gpio2c3ER::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO2C4 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c4_e(&self) -> Gpio2c4ER {
        Gpio2c4ER::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO2C5 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c5_e(&self) -> Gpio2c5ER {
        Gpio2c5ER::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO2C6 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c6_e(&self) -> Gpio2c6ER {
        Gpio2c6ER::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO2C7 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    pub fn gpio2c7_e(&self) -> Gpio2c7ER {
        Gpio2c7ER::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO2C0 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c0_e(&mut self) -> Gpio2c0EW<GrfGpio2cESpec> {
        Gpio2c0EW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO2C1 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c1_e(&mut self) -> Gpio2c1EW<GrfGpio2cESpec> {
        Gpio2c1EW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO2C2 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c2_e(&mut self) -> Gpio2c2EW<GrfGpio2cESpec> {
        Gpio2c2EW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO2C3 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c3_e(&mut self) -> Gpio2c3EW<GrfGpio2cESpec> {
        Gpio2c3EW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO2C4 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c4_e(&mut self) -> Gpio2c4EW<GrfGpio2cESpec> {
        Gpio2c4EW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO2C5 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c5_e(&mut self) -> Gpio2c5EW<GrfGpio2cESpec> {
        Gpio2c5EW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO2C6 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c6_e(&mut self) -> Gpio2c6EW<GrfGpio2cESpec> {
        Gpio2c6EW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO2C7 drive strength control, every GPIO bit corresponding to 2bits"]
    #[inline(always)]
    #[must_use]
    pub fn gpio2c7_e(&mut self) -> Gpio2c7EW<GrfGpio2cESpec> {
        Gpio2c7EW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio2cESpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO2C drive strength control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio2c_e::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio2c_e::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio2cESpec;
impl crate::RegisterSpec for GrfGpio2cESpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio2c_e::R`](R) reader structure"]
impl crate::Readable for GrfGpio2cESpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio2c_e::W`](W) writer structure"]
impl crate::Writable for GrfGpio2cESpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO2C_E to value 0"]
impl crate::Resettable for GrfGpio2cESpec {
    const RESET_VALUE: u32 = 0;
}
