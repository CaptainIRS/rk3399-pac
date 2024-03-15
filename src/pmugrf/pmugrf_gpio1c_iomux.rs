#[doc = "Register `PMUGRF_GPIO1C_IOMUX` reader"]
pub type R = crate::R<PmugrfGpio1cIomuxSpec>;
#[doc = "Register `PMUGRF_GPIO1C_IOMUX` writer"]
pub type W = crate::W<PmugrfGpio1cIomuxSpec>;
#[doc = "GPIO1C\\[0\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1c0Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio1c0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1c0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1c0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1C0_SEL` reader - GPIO1C\\[0\\]
iomux select"]
pub type Gpio1c0SelR = crate::FieldReader<Gpio1c0Sel>;
impl Gpio1c0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1c0Sel {
        match self.bits {
            0 => Gpio1c0Sel::B00,
            1 => Gpio1c0Sel::B01,
            2 => Gpio1c0Sel::B10,
            3 => Gpio1c0Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1c0Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1c0Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1c0Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1c0Sel::B11
    }
}
#[doc = "Field `GPIO1C0_SEL` writer - GPIO1C\\[0\\]
iomux select"]
pub type Gpio1c0SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio1c0Sel>;
impl<'a, REG> Gpio1c0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c0Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c0Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c0Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c0Sel::B11)
    }
}
#[doc = "GPIO1C\\[1\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1c1Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio1c1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1c1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1c1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1C1_SEL` reader - GPIO1C\\[1\\]
iomux select"]
pub type Gpio1c1SelR = crate::FieldReader<Gpio1c1Sel>;
impl Gpio1c1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1c1Sel {
        match self.bits {
            0 => Gpio1c1Sel::B00,
            1 => Gpio1c1Sel::B01,
            2 => Gpio1c1Sel::B10,
            3 => Gpio1c1Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1c1Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1c1Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1c1Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1c1Sel::B11
    }
}
#[doc = "Field `GPIO1C1_SEL` writer - GPIO1C\\[1\\]
iomux select"]
pub type Gpio1c1SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio1c1Sel>;
impl<'a, REG> Gpio1c1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c1Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c1Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c1Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c1Sel::B11)
    }
}
#[doc = "GPIO1C\\[2\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1c2Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio1c2Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1c2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1c2Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1C2_SEL` reader - GPIO1C\\[2\\]
iomux select"]
pub type Gpio1c2SelR = crate::FieldReader<Gpio1c2Sel>;
impl Gpio1c2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1c2Sel {
        match self.bits {
            0 => Gpio1c2Sel::B00,
            1 => Gpio1c2Sel::B01,
            2 => Gpio1c2Sel::B10,
            3 => Gpio1c2Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1c2Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1c2Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1c2Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1c2Sel::B11
    }
}
#[doc = "Field `GPIO1C2_SEL` writer - GPIO1C\\[2\\]
iomux select"]
pub type Gpio1c2SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio1c2Sel>;
impl<'a, REG> Gpio1c2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c2Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c2Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c2Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c2Sel::B11)
    }
}
#[doc = "GPIO1C\\[3\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1c3Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio1c3Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1c3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1c3Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1C3_SEL` reader - GPIO1C\\[3\\]
iomux select"]
pub type Gpio1c3SelR = crate::FieldReader<Gpio1c3Sel>;
impl Gpio1c3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1c3Sel {
        match self.bits {
            0 => Gpio1c3Sel::B00,
            1 => Gpio1c3Sel::B01,
            2 => Gpio1c3Sel::B10,
            3 => Gpio1c3Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1c3Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1c3Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1c3Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1c3Sel::B11
    }
}
#[doc = "Field `GPIO1C3_SEL` writer - GPIO1C\\[3\\]
iomux select"]
pub type Gpio1c3SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio1c3Sel>;
impl<'a, REG> Gpio1c3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c3Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c3Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c3Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c3Sel::B11)
    }
}
#[doc = "GPIO1C\\[4\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1c4Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio1c4Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1c4Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1c4Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1C4_SEL` reader - GPIO1C\\[4\\]
iomux select"]
pub type Gpio1c4SelR = crate::FieldReader<Gpio1c4Sel>;
impl Gpio1c4SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1c4Sel {
        match self.bits {
            0 => Gpio1c4Sel::B00,
            1 => Gpio1c4Sel::B01,
            2 => Gpio1c4Sel::B10,
            3 => Gpio1c4Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1c4Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1c4Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1c4Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1c4Sel::B11
    }
}
#[doc = "Field `GPIO1C4_SEL` writer - GPIO1C\\[4\\]
iomux select"]
pub type Gpio1c4SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio1c4Sel>;
impl<'a, REG> Gpio1c4SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c4Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c4Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c4Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c4Sel::B11)
    }
}
#[doc = "GPIO1C\\[5\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1c5Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio1c5Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1c5Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1c5Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1C5_SEL` reader - GPIO1C\\[5\\]
iomux select"]
pub type Gpio1c5SelR = crate::FieldReader<Gpio1c5Sel>;
impl Gpio1c5SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1c5Sel {
        match self.bits {
            0 => Gpio1c5Sel::B00,
            1 => Gpio1c5Sel::B01,
            2 => Gpio1c5Sel::B10,
            3 => Gpio1c5Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1c5Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1c5Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1c5Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1c5Sel::B11
    }
}
#[doc = "Field `GPIO1C5_SEL` writer - GPIO1C\\[5\\]
iomux select"]
pub type Gpio1c5SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio1c5Sel>;
impl<'a, REG> Gpio1c5SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c5Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c5Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c5Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c5Sel::B11)
    }
}
#[doc = "GPIO1C\\[6\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1c6Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio1c6Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1c6Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1c6Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1C6_SEL` reader - GPIO1C\\[6\\]
iomux select"]
pub type Gpio1c6SelR = crate::FieldReader<Gpio1c6Sel>;
impl Gpio1c6SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1c6Sel {
        match self.bits {
            0 => Gpio1c6Sel::B00,
            1 => Gpio1c6Sel::B01,
            2 => Gpio1c6Sel::B10,
            3 => Gpio1c6Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1c6Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1c6Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1c6Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1c6Sel::B11
    }
}
#[doc = "Field `GPIO1C6_SEL` writer - GPIO1C\\[6\\]
iomux select"]
pub type Gpio1c6SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio1c6Sel>;
impl<'a, REG> Gpio1c6SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c6Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c6Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c6Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c6Sel::B11)
    }
}
#[doc = "GPIO1C\\[7\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio1c7Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio1c7Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio1c7Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio1c7Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO1C7_SEL` reader - GPIO1C\\[7\\]
iomux select"]
pub type Gpio1c7SelR = crate::FieldReader<Gpio1c7Sel>;
impl Gpio1c7SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio1c7Sel {
        match self.bits {
            0 => Gpio1c7Sel::B00,
            1 => Gpio1c7Sel::B01,
            2 => Gpio1c7Sel::B10,
            3 => Gpio1c7Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio1c7Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio1c7Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio1c7Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio1c7Sel::B11
    }
}
#[doc = "Field `GPIO1C7_SEL` writer - GPIO1C\\[7\\]
iomux select"]
pub type Gpio1c7SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio1c7Sel>;
impl<'a, REG> Gpio1c7SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c7Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c7Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c7Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio1c7Sel::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO1C\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1c0_sel(&self) -> Gpio1c0SelR {
        Gpio1c0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO1C\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1c1_sel(&self) -> Gpio1c1SelR {
        Gpio1c1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO1C\\[2\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1c2_sel(&self) -> Gpio1c2SelR {
        Gpio1c2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO1C\\[3\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1c3_sel(&self) -> Gpio1c3SelR {
        Gpio1c3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO1C\\[4\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1c4_sel(&self) -> Gpio1c4SelR {
        Gpio1c4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO1C\\[5\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1c5_sel(&self) -> Gpio1c5SelR {
        Gpio1c5SelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO1C\\[6\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1c6_sel(&self) -> Gpio1c6SelR {
        Gpio1c6SelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO1C\\[7\\]
iomux select"]
    #[inline(always)]
    pub fn gpio1c7_sel(&self) -> Gpio1c7SelR {
        Gpio1c7SelR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO1C\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1c0_sel(&mut self) -> Gpio1c0SelW<PmugrfGpio1cIomuxSpec> {
        Gpio1c0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO1C\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1c1_sel(&mut self) -> Gpio1c1SelW<PmugrfGpio1cIomuxSpec> {
        Gpio1c1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO1C\\[2\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1c2_sel(&mut self) -> Gpio1c2SelW<PmugrfGpio1cIomuxSpec> {
        Gpio1c2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO1C\\[3\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1c3_sel(&mut self) -> Gpio1c3SelW<PmugrfGpio1cIomuxSpec> {
        Gpio1c3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO1C\\[4\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1c4_sel(&mut self) -> Gpio1c4SelW<PmugrfGpio1cIomuxSpec> {
        Gpio1c4SelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO1C\\[5\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1c5_sel(&mut self) -> Gpio1c5SelW<PmugrfGpio1cIomuxSpec> {
        Gpio1c5SelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO1C\\[6\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1c6_sel(&mut self) -> Gpio1c6SelW<PmugrfGpio1cIomuxSpec> {
        Gpio1c6SelW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO1C\\[7\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio1c7_sel(&mut self) -> Gpio1c7SelW<PmugrfGpio1cIomuxSpec> {
        Gpio1c7SelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<PmugrfGpio1cIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO1C iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmugrf_gpio1c_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmugrf_gpio1c_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmugrfGpio1cIomuxSpec;
impl crate::RegisterSpec for PmugrfGpio1cIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmugrf_gpio1c_iomux::R`](R) reader structure"]
impl crate::Readable for PmugrfGpio1cIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`pmugrf_gpio1c_iomux::W`](W) writer structure"]
impl crate::Writable for PmugrfGpio1cIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUGRF_GPIO1C_IOMUX to value 0"]
impl crate::Resettable for PmugrfGpio1cIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
