#[doc = "Register `GRF_GPIO3A_IOMUX` reader"]
pub type R = crate::R<GrfGpio3aIomuxSpec>;
#[doc = "Register `GRF_GPIO3A_IOMUX` writer"]
pub type W = crate::W<GrfGpio3aIomuxSpec>;
#[doc = "GPIO3A\\[0\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3a0Sel {
    #[doc = "0: trace_data12"]
    B00 = 0,
    #[doc = "1: trace_data12"]
    B01 = 1,
    #[doc = "2: trace_data12"]
    B10 = 2,
    #[doc = "3: trace_data12"]
    B11 = 3,
}
impl From<Gpio3a0Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3a0Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3a0Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3A0_SEL` reader - GPIO3A\\[0\\]
iomux select"]
pub type Gpio3a0SelR = crate::FieldReader<Gpio3a0Sel>;
impl Gpio3a0SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3a0Sel {
        match self.bits {
            0 => Gpio3a0Sel::B00,
            1 => Gpio3a0Sel::B01,
            2 => Gpio3a0Sel::B10,
            3 => Gpio3a0Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "trace_data12"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3a0Sel::B00
    }
    #[doc = "trace_data12"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3a0Sel::B01
    }
    #[doc = "trace_data12"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3a0Sel::B10
    }
    #[doc = "trace_data12"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3a0Sel::B11
    }
}
#[doc = "Field `GPIO3A0_SEL` writer - GPIO3A\\[0\\]
iomux select"]
pub type Gpio3a0SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3a0Sel>;
impl<'a, REG> Gpio3a0SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "trace_data12"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a0Sel::B00)
    }
    #[doc = "trace_data12"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a0Sel::B01)
    }
    #[doc = "trace_data12"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a0Sel::B10)
    }
    #[doc = "trace_data12"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a0Sel::B11)
    }
}
#[doc = "GPIO3A\\[1\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3a1Sel {
    #[doc = "0: trace_data13"]
    B00 = 0,
    #[doc = "1: trace_data13"]
    B01 = 1,
    #[doc = "2: trace_data13"]
    B10 = 2,
    #[doc = "3: trace_data13"]
    B11 = 3,
}
impl From<Gpio3a1Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3a1Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3a1Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3A1_SEL` reader - GPIO3A\\[1\\]
iomux select"]
pub type Gpio3a1SelR = crate::FieldReader<Gpio3a1Sel>;
impl Gpio3a1SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3a1Sel {
        match self.bits {
            0 => Gpio3a1Sel::B00,
            1 => Gpio3a1Sel::B01,
            2 => Gpio3a1Sel::B10,
            3 => Gpio3a1Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "trace_data13"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3a1Sel::B00
    }
    #[doc = "trace_data13"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3a1Sel::B01
    }
    #[doc = "trace_data13"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3a1Sel::B10
    }
    #[doc = "trace_data13"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3a1Sel::B11
    }
}
#[doc = "Field `GPIO3A1_SEL` writer - GPIO3A\\[1\\]
iomux select"]
pub type Gpio3a1SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3a1Sel>;
impl<'a, REG> Gpio3a1SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "trace_data13"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a1Sel::B00)
    }
    #[doc = "trace_data13"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a1Sel::B01)
    }
    #[doc = "trace_data13"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a1Sel::B10)
    }
    #[doc = "trace_data13"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a1Sel::B11)
    }
}
#[doc = "GPIO3A\\[2\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3a2Sel {
    #[doc = "0: trace_data14"]
    B00 = 0,
    #[doc = "1: trace_data14"]
    B01 = 1,
    #[doc = "2: trace_data14"]
    B10 = 2,
    #[doc = "3: trace_data14"]
    B11 = 3,
}
impl From<Gpio3a2Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3a2Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3a2Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3A2_SEL` reader - GPIO3A\\[2\\]
iomux select"]
pub type Gpio3a2SelR = crate::FieldReader<Gpio3a2Sel>;
impl Gpio3a2SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3a2Sel {
        match self.bits {
            0 => Gpio3a2Sel::B00,
            1 => Gpio3a2Sel::B01,
            2 => Gpio3a2Sel::B10,
            3 => Gpio3a2Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "trace_data14"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3a2Sel::B00
    }
    #[doc = "trace_data14"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3a2Sel::B01
    }
    #[doc = "trace_data14"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3a2Sel::B10
    }
    #[doc = "trace_data14"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3a2Sel::B11
    }
}
#[doc = "Field `GPIO3A2_SEL` writer - GPIO3A\\[2\\]
iomux select"]
pub type Gpio3a2SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3a2Sel>;
impl<'a, REG> Gpio3a2SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "trace_data14"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a2Sel::B00)
    }
    #[doc = "trace_data14"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a2Sel::B01)
    }
    #[doc = "trace_data14"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a2Sel::B10)
    }
    #[doc = "trace_data14"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a2Sel::B11)
    }
}
#[doc = "GPIO3A\\[3\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3a3Sel {
    #[doc = "0: trace_data15"]
    B00 = 0,
    #[doc = "1: trace_data15"]
    B01 = 1,
    #[doc = "2: trace_data15"]
    B10 = 2,
    #[doc = "3: trace_data15"]
    B11 = 3,
}
impl From<Gpio3a3Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3a3Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3a3Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3A3_SEL` reader - GPIO3A\\[3\\]
iomux select"]
pub type Gpio3a3SelR = crate::FieldReader<Gpio3a3Sel>;
impl Gpio3a3SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3a3Sel {
        match self.bits {
            0 => Gpio3a3Sel::B00,
            1 => Gpio3a3Sel::B01,
            2 => Gpio3a3Sel::B10,
            3 => Gpio3a3Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "trace_data15"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3a3Sel::B00
    }
    #[doc = "trace_data15"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3a3Sel::B01
    }
    #[doc = "trace_data15"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3a3Sel::B10
    }
    #[doc = "trace_data15"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3a3Sel::B11
    }
}
#[doc = "Field `GPIO3A3_SEL` writer - GPIO3A\\[3\\]
iomux select"]
pub type Gpio3a3SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3a3Sel>;
impl<'a, REG> Gpio3a3SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "trace_data15"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a3Sel::B00)
    }
    #[doc = "trace_data15"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a3Sel::B01)
    }
    #[doc = "trace_data15"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a3Sel::B10)
    }
    #[doc = "trace_data15"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a3Sel::B11)
    }
}
#[doc = "GPIO3A\\[4\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3a4Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio3a4Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3a4Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3a4Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3A4_SEL` reader - GPIO3A\\[4\\]
iomux select"]
pub type Gpio3a4SelR = crate::FieldReader<Gpio3a4Sel>;
impl Gpio3a4SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3a4Sel {
        match self.bits {
            0 => Gpio3a4Sel::B00,
            1 => Gpio3a4Sel::B01,
            2 => Gpio3a4Sel::B10,
            3 => Gpio3a4Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3a4Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3a4Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3a4Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3a4Sel::B11
    }
}
#[doc = "Field `GPIO3A4_SEL` writer - GPIO3A\\[4\\]
iomux select"]
pub type Gpio3a4SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3a4Sel>;
impl<'a, REG> Gpio3a4SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a4Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a4Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a4Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a4Sel::B11)
    }
}
#[doc = "GPIO3A\\[5\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3a5Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio3a5Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3a5Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3a5Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3A5_SEL` reader - GPIO3A\\[5\\]
iomux select"]
pub type Gpio3a5SelR = crate::FieldReader<Gpio3a5Sel>;
impl Gpio3a5SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3a5Sel {
        match self.bits {
            0 => Gpio3a5Sel::B00,
            1 => Gpio3a5Sel::B01,
            2 => Gpio3a5Sel::B10,
            3 => Gpio3a5Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3a5Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3a5Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3a5Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3a5Sel::B11
    }
}
#[doc = "Field `GPIO3A5_SEL` writer - GPIO3A\\[5\\]
iomux select"]
pub type Gpio3a5SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3a5Sel>;
impl<'a, REG> Gpio3a5SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a5Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a5Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a5Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a5Sel::B11)
    }
}
#[doc = "GPIO3A\\[6\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3a6Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio3a6Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3a6Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3a6Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3A6_SEL` reader - GPIO3A\\[6\\]
iomux select"]
pub type Gpio3a6SelR = crate::FieldReader<Gpio3a6Sel>;
impl Gpio3a6SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3a6Sel {
        match self.bits {
            0 => Gpio3a6Sel::B00,
            1 => Gpio3a6Sel::B01,
            2 => Gpio3a6Sel::B10,
            3 => Gpio3a6Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3a6Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3a6Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3a6Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3a6Sel::B11
    }
}
#[doc = "Field `GPIO3A6_SEL` writer - GPIO3A\\[6\\]
iomux select"]
pub type Gpio3a6SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3a6Sel>;
impl<'a, REG> Gpio3a6SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a6Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a6Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a6Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a6Sel::B11)
    }
}
#[doc = "GPIO3A\\[7\\]
iomux select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Gpio3a7Sel {
    #[doc = "0: reserved"]
    B00 = 0,
    #[doc = "1: reserved"]
    B01 = 1,
    #[doc = "2: reserved"]
    B10 = 2,
    #[doc = "3: reserved"]
    B11 = 3,
}
impl From<Gpio3a7Sel> for u8 {
    #[inline(always)]
    fn from(variant: Gpio3a7Sel) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Gpio3a7Sel {
    type Ux = u8;
}
#[doc = "Field `GPIO3A7_SEL` reader - GPIO3A\\[7\\]
iomux select"]
pub type Gpio3a7SelR = crate::FieldReader<Gpio3a7Sel>;
impl Gpio3a7SelR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Gpio3a7Sel {
        match self.bits {
            0 => Gpio3a7Sel::B00,
            1 => Gpio3a7Sel::B01,
            2 => Gpio3a7Sel::B10,
            3 => Gpio3a7Sel::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == Gpio3a7Sel::B00
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == Gpio3a7Sel::B01
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == Gpio3a7Sel::B10
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == Gpio3a7Sel::B11
    }
}
#[doc = "Field `GPIO3A7_SEL` writer - GPIO3A\\[7\\]
iomux select"]
pub type Gpio3a7SelW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, Gpio3a7Sel>;
impl<'a, REG> Gpio3a7SelW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a7Sel::B00)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a7Sel::B01)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a7Sel::B10)
    }
    #[doc = "reserved"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(Gpio3a7Sel::B11)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - GPIO3A\\[0\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3a0_sel(&self) -> Gpio3a0SelR {
        Gpio3a0SelR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - GPIO3A\\[1\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3a1_sel(&self) -> Gpio3a1SelR {
        Gpio3a1SelR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - GPIO3A\\[2\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3a2_sel(&self) -> Gpio3a2SelR {
        Gpio3a2SelR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - GPIO3A\\[3\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3a3_sel(&self) -> Gpio3a3SelR {
        Gpio3a3SelR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - GPIO3A\\[4\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3a4_sel(&self) -> Gpio3a4SelR {
        Gpio3a4SelR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - GPIO3A\\[5\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3a5_sel(&self) -> Gpio3a5SelR {
        Gpio3a5SelR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - GPIO3A\\[6\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3a6_sel(&self) -> Gpio3a6SelR {
        Gpio3a6SelR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - GPIO3A\\[7\\]
iomux select"]
    #[inline(always)]
    pub fn gpio3a7_sel(&self) -> Gpio3a7SelR {
        Gpio3a7SelR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:1 - GPIO3A\\[0\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3a0_sel(&mut self) -> Gpio3a0SelW<GrfGpio3aIomuxSpec> {
        Gpio3a0SelW::new(self, 0)
    }
    #[doc = "Bits 2:3 - GPIO3A\\[1\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3a1_sel(&mut self) -> Gpio3a1SelW<GrfGpio3aIomuxSpec> {
        Gpio3a1SelW::new(self, 2)
    }
    #[doc = "Bits 4:5 - GPIO3A\\[2\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3a2_sel(&mut self) -> Gpio3a2SelW<GrfGpio3aIomuxSpec> {
        Gpio3a2SelW::new(self, 4)
    }
    #[doc = "Bits 6:7 - GPIO3A\\[3\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3a3_sel(&mut self) -> Gpio3a3SelW<GrfGpio3aIomuxSpec> {
        Gpio3a3SelW::new(self, 6)
    }
    #[doc = "Bits 8:9 - GPIO3A\\[4\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3a4_sel(&mut self) -> Gpio3a4SelW<GrfGpio3aIomuxSpec> {
        Gpio3a4SelW::new(self, 8)
    }
    #[doc = "Bits 10:11 - GPIO3A\\[5\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3a5_sel(&mut self) -> Gpio3a5SelW<GrfGpio3aIomuxSpec> {
        Gpio3a5SelW::new(self, 10)
    }
    #[doc = "Bits 12:13 - GPIO3A\\[6\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3a6_sel(&mut self) -> Gpio3a6SelW<GrfGpio3aIomuxSpec> {
        Gpio3a6SelW::new(self, 12)
    }
    #[doc = "Bits 14:15 - GPIO3A\\[7\\]
iomux select"]
    #[inline(always)]
    #[must_use]
    pub fn gpio3a7_sel(&mut self) -> Gpio3a7SelW<GrfGpio3aIomuxSpec> {
        Gpio3a7SelW::new(self, 14)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfGpio3aIomuxSpec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "GPIO3A iomux control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_gpio3a_iomux::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_gpio3a_iomux::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfGpio3aIomuxSpec;
impl crate::RegisterSpec for GrfGpio3aIomuxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_gpio3a_iomux::R`](R) reader structure"]
impl crate::Readable for GrfGpio3aIomuxSpec {}
#[doc = "`write(|w| ..)` method takes [`grf_gpio3a_iomux::W`](W) writer structure"]
impl crate::Writable for GrfGpio3aIomuxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_GPIO3A_IOMUX to value 0"]
impl crate::Resettable for GrfGpio3aIomuxSpec {
    const RESET_VALUE: u32 = 0;
}
