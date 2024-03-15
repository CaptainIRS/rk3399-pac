#[doc = "Register `GRF_GPIO4A_IOMUX` reader"]
pub type R = crate::R<GrfGpio4aIomuxSpec>;
#[doc = "Register `GRF_GPIO4A_IOMUX` writer"]
pub type W = crate::W<GrfGpio4aIomuxSpec>;
#[doc = "GPIO4A\\[0\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a0Sel {
    #[doc = "0: lpm0_wfi"]
    B00 = 0,
    #[doc = "1: lpm0_wfi"]
    B01 = 1,
    #[doc = "2: lpm0_wfi"]
    B10 = 2,
    #[doc = "3: lpm0_wfi"]
    B11 = 3,
}
impl From<Gpio4a0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4A0_SEL` reader - GPIO4A\\[0\\]
iomux select"]
pub type Gpio4a0SelR = crate::FieldReader<Gpio4a0Sel>;
impl Gpio4a0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a0Sel {
        match self.bits {
            0 => Gpio4a0Sel::B00,
            1 => Gpio4a0Sel::B01,
            2 => Gpio4a0Sel::B10,
            3 => Gpio4a0Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "lpm0_wfi"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a0Sel::B00
    }
    #[doc = "lpm0_wfi"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a0Sel::B01
    }
    #[doc = "lpm0_wfi"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a0Sel::B10
    }
    #[doc = "lpm0_wfi"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a0Sel::B11
    }
}
#[doc = "Field `GPIO4A0_SEL` writer - GPIO4A\\[0\\]
iomux select"]
pub type Gpio4a0SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a0Sel>;
impl<'a, REG> Gpio4a0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "lpm0_wfi"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a0Sel::B00)
    }
    #[doc = "lpm0_wfi"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a0Sel::B01)
    }
    #[doc = "lpm0_wfi"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a0Sel::B10)
    }
    #[doc = "lpm0_wfi"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a0Sel::B11)
    }
}
#[doc = "GPIO4A\\[1\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a1Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio4a1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4A1_SEL` reader - GPIO4A\\[1\\]
iomux select"]
pub type Gpio4a1SelR = crate::FieldReader<Gpio4a1Sel>;
impl Gpio4a1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a1Sel {
        match self.bits {
            0 => Gpio4a1Sel::B00,
            1 => Gpio4a1Sel::B01,
            2 => Gpio4a1Sel::B10,
            3 => Gpio4a1Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a1Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a1Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a1Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a1Sel::B11
    }
}
#[doc = "Field `GPIO4A1_SEL` writer - GPIO4A\\[1\\]
iomux select"]
pub type Gpio4a1SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a1Sel>;
impl<'a, REG> Gpio4a1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a1Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a1Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a1Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a1Sel::B11)
    }
}
#[doc = "GPIO4A\\[2\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a2Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio4a2Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a2Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4A2_SEL` reader - GPIO4A\\[2\\]
iomux select"]
pub type Gpio4a2SelR = crate::FieldReader<Gpio4a2Sel>;
impl Gpio4a2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a2Sel {
        match self.bits {
            0 => Gpio4a2Sel::B00,
            1 => Gpio4a2Sel::B01,
            2 => Gpio4a2Sel::B10,
            3 => Gpio4a2Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a2Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a2Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a2Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a2Sel::B11
    }
}
#[doc = "Field `GPIO4A2_SEL` writer - GPIO4A\\[2\\]
iomux select"]
pub type Gpio4a2SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a2Sel>;
impl<'a, REG> Gpio4a2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a2Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a2Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a2Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a2Sel::B11)
    }
}
#[doc = "GPIO4A\\[3\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a3Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio4a3Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a3Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4A3_SEL` reader - GPIO4A\\[3\\]
iomux select"]
pub type Gpio4a3SelR = crate::FieldReader<Gpio4a3Sel>;
impl Gpio4a3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a3Sel {
        match self.bits {
            0 => Gpio4a3Sel::B00,
            1 => Gpio4a3Sel::B01,
            2 => Gpio4a3Sel::B10,
            3 => Gpio4a3Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a3Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a3Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a3Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a3Sel::B11
    }
}
#[doc = "Field `GPIO4A3_SEL` writer - GPIO4A\\[3\\]
iomux select"]
pub type Gpio4a3SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a3Sel>;
impl<'a, REG> Gpio4a3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a3Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a3Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a3Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a3Sel::B11)
    }
}
#[doc = "GPIO4A\\[4\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a4Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio4a4Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a4Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a4Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4A4_SEL` reader - GPIO4A\\[4\\]
iomux select"]
pub type Gpio4a4SelR = crate::FieldReader<Gpio4a4Sel>;
impl Gpio4a4SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a4Sel {
        match self.bits {
            0 => Gpio4a4Sel::B00,
            1 => Gpio4a4Sel::B01,
            2 => Gpio4a4Sel::B10,
            3 => Gpio4a4Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a4Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a4Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a4Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a4Sel::B11
    }
}
#[doc = "Field `GPIO4A4_SEL` writer - GPIO4A\\[4\\]
iomux select"]
pub type Gpio4a4SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a4Sel>;
impl<'a, REG> Gpio4a4SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a4Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a4Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a4Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a4Sel::B11)
    }
}
#[doc = "GPIO4A\\[5\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a5Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio4a5Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a5Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a5Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4A5_SEL` reader - GPIO4A\\[5\\]
iomux select"]
pub type Gpio4a5SelR = crate::FieldReader<Gpio4a5Sel>;
impl Gpio4a5SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a5Sel {
        match self.bits {
            0 => Gpio4a5Sel::B00,
            1 => Gpio4a5Sel::B01,
            2 => Gpio4a5Sel::B10,
            3 => Gpio4a5Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a5Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a5Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a5Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a5Sel::B11
    }
}
#[doc = "Field `GPIO4A5_SEL` writer - GPIO4A\\[5\\]
iomux select"]
pub type Gpio4a5SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a5Sel>;
impl<'a, REG> Gpio4a5SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a5Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a5Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a5Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a5Sel::B11)
    }
}
#[doc = "GPIO4A\\[6\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a6Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio4a6Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a6Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a6Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4A6_SEL` reader - GPIO4A\\[6\\]
iomux select"]
pub type Gpio4a6SelR = crate::FieldReader<Gpio4a6Sel>;
impl Gpio4a6SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a6Sel {
        match self.bits {
            0 => Gpio4a6Sel::B00,
            1 => Gpio4a6Sel::B01,
            2 => Gpio4a6Sel::B10,
            3 => Gpio4a6Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a6Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a6Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a6Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a6Sel::B11
    }
}
#[doc = "Field `GPIO4A6_SEL` writer - GPIO4A\\[6\\]
iomux select"]
pub type Gpio4a6SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a6Sel>;
impl<'a, REG> Gpio4a6SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a6Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a6Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a6Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a6Sel::B11)
    }
}
#[doc = "GPIO4A\\[7\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio4a7Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio4a7Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio4a7Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio4a7Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO4A7_SEL` reader - GPIO4A\\[7\\]
iomux select"]
pub type Gpio4a7SelR = crate::FieldReader<Gpio4a7Sel>;
impl Gpio4a7SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio4a7Sel {
        match self.bits {
            0 => Gpio4a7Sel::B00,
            1 => Gpio4a7Sel::B01,
            2 => Gpio4a7Sel::B10,
            3 => Gpio4a7Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio4a7Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio4a7Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio4a7Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio4a7Sel::B11
    }
}
#[doc = "Field `GPIO4A7_SEL` writer - GPIO4A\\[7\\]
iomux select"]
pub type Gpio4a7SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio4a7Sel>;
impl<'a, REG> Gpio4a7SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a7Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a7Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a7Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio4a7Sel::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO4A\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4a0_sel(&self) -> Gpio4a0SelR {
        Gpio4a0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO4A\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4a1_sel(&self) -> Gpio4a1SelR {
        Gpio4a1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO4A\\[2\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4a2_sel(&self) -> Gpio4a2SelR {
        Gpio4a2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO4A\\[3\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4a3_sel(&self) -> Gpio4a3SelR {
        Gpio4a3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO4A\\[4\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4a4_sel(&self) -> Gpio4a4SelR {
        Gpio4a4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO4A\\[5\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4a5_sel(&self) -> Gpio4a5SelR {
        Gpio4a5SelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO4A\\[6\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4a6_sel(&self) -> Gpio4a6SelR {
        Gpio4a6SelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO4A\\[7\\]
iomux select"]
    #[inline(always)]
    pub fn gpio4a7_sel(&self) -> Gpio4a7SelR {
        Gpio4a7SelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO4A\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a0_sel(&mut self) -> Gpio4a0SelW<GrfGpio4aIomuxSpec> {
        Gpio4a0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO4A\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a1_sel(&mut self) -> Gpio4a1SelW<GrfGpio4aIomuxSpec> {
        Gpio4a1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO4A\\[2\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a2_sel(&mut self) -> Gpio4a2SelW<GrfGpio4aIomuxSpec> {
        Gpio4a2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO4A\\[3\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a3_sel(&mut self) -> Gpio4a3SelW<GrfGpio4aIomuxSpec> {
        Gpio4a3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO4A\\[4\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a4_sel(&mut self) -> Gpio4a4SelW<GrfGpio4aIomuxSpec> {
        Gpio4a4SelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO4A\\[5\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a5_sel(&mut self) -> Gpio4a5SelW<GrfGpio4aIomuxSpec> {
        Gpio4a5SelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO4A\\[6\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a6_sel(&mut self) -> Gpio4a6SelW<GrfGpio4aIomuxSpec> {
        Gpio4a6SelW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO4A\\[7\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio4a7_sel(&mut self) -> Gpio4a7SelW<GrfGpio4aIomuxSpec> {
        Gpio4a7SelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio4aIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO4A iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio4a_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio4a_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio4aIomuxSpec;
impl crate::RegisterSpec for GrfGpio4aIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio4a_iomux::R`](R) reader structure"]
impl crate::Readable for GrfGpio4aIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio4a_iomux::W`](W) writer structure"]
impl crate::Writable for GrfGpio4aIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO4A_IOMUX to value 0"]
impl crate::Resettable for GrfGpio4aIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
