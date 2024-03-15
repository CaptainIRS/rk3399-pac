#[doc = "Register `SSC_REG` reader"]
pub type R = crate::R<SscRegSpec>;
#[doc = "Register `SSC_REG` writer"]
pub type W = crate::W<SscRegSpec>;
#[doc = "\n\nValue on reset: 10"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SscDepth {
    #[doc = "0: 5500ppm 1100-1111:6000ppm"]
    B0000 = 0,
    #[doc = "1: 5500ppm 1100-1111:6000ppm"]
    B0001 = 1,
    #[doc = "2: 5500ppm 1100-1111:6000ppm"]
    B0010 = 2,
    #[doc = "3: 5500ppm 1100-1111:6000ppm"]
    B0011 = 3,
    #[doc = "4: 5500ppm 1100-1111:6000ppm"]
    B0100 = 4,
    #[doc = "5: 5500ppm 1100-1111:6000ppm"]
    B0101 = 5,
    #[doc = "6: 5500ppm 1100-1111:6000ppm"]
    B0110 = 6,
    #[doc = "7: 5500ppm 1100-1111:6000ppm"]
    B0111 = 7,
    #[doc = "8: 5500ppm 1100-1111:6000ppm"]
    B1000 = 8,
    #[doc = "9: 5500ppm 1100-1111:6000ppm"]
    B1001 = 9,
    #[doc = "10: 5500ppm 1100-1111:6000ppm"]
    B1010 = 10,
    #[doc = "11: 5500ppm 1100-1111:6000ppm"]
    B1011 = 11,
}
impl From<SscDepth> for u8 {
    #[inline(always)]
    fn from(variant: SscDepth) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SscDepth {
    type Ux = u8;
}
#[doc = "Field `SSC_DEPTH` reader - "]
pub type SscDepthR = crate::FieldReader<SscDepth>;
impl SscDepthR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SscDepth> {
        match self.bits {
            0 => Some(SscDepth::B0000),
            1 => Some(SscDepth::B0001),
            2 => Some(SscDepth::B0010),
            3 => Some(SscDepth::B0011),
            4 => Some(SscDepth::B0100),
            5 => Some(SscDepth::B0101),
            6 => Some(SscDepth::B0110),
            7 => Some(SscDepth::B0111),
            8 => Some(SscDepth::B1000),
            9 => Some(SscDepth::B1001),
            10 => Some(SscDepth::B1010),
            11 => Some(SscDepth::B1011),
            _ => None,
        }
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == SscDepth::B0000
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == SscDepth::B0001
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == SscDepth::B0010
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == SscDepth::B0011
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == SscDepth::B0100
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == SscDepth::B0101
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == SscDepth::B0110
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == SscDepth::B0111
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == SscDepth::B1000
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == SscDepth::B1001
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn is_b1010(&self) -> bool {
        *self == SscDepth::B1010
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn is_b1011(&self) -> bool {
        *self == SscDepth::B1011
    }
}
#[doc = "Field `SSC_DEPTH` writer - "]
pub type SscDepthW<'a, REG> = crate::FieldWriter<'a, REG, 4, SscDepth>;
impl<'a, REG> SscDepthW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(SscDepth::B0000)
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(SscDepth::B0001)
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(SscDepth::B0010)
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(SscDepth::B0011)
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(SscDepth::B0100)
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(SscDepth::B0101)
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(SscDepth::B0110)
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn b0111(self) -> &'a mut crate::W<REG> {
        self.variant(SscDepth::B0111)
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(SscDepth::B1000)
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(SscDepth::B1001)
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn b1010(self) -> &'a mut crate::W<REG> {
        self.variant(SscDepth::B1010)
    }
    #[doc = "5500ppm 1100-1111:6000ppm"]
    #[inline(always)]
    pub fn b1011(self) -> &'a mut crate::W<REG> {
        self.variant(SscDepth::B1011)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SscMode {
    #[doc = "0: up spread"]
    B00 = 0,
    #[doc = "1: up spread"]
    B01 = 1,
    #[doc = "2: up spread"]
    B10 = 2,
    #[doc = "3: up spread"]
    B11 = 3,
}
impl From<SscMode> for u8 {
    #[inline(always)]
    fn from(variant: SscMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SscMode {
    type Ux = u8;
}
#[doc = "Field `SSC_MODE` reader - "]
pub type SscModeR = crate::FieldReader<SscMode>;
impl SscModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SscMode {
        match self.bits {
            0 => SscMode::B00,
            1 => SscMode::B01,
            2 => SscMode::B10,
            3 => SscMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "up spread"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SscMode::B00
    }
    #[doc = "up spread"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SscMode::B01
    }
    #[doc = "up spread"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SscMode::B10
    }
    #[doc = "up spread"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == SscMode::B11
    }
}
#[doc = "Field `SSC_MODE` writer - "]
pub type SscModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SscMode>;
impl<'a, REG> SscModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "up spread"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SscMode::B00)
    }
    #[doc = "up spread"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SscMode::B01)
    }
    #[doc = "up spread"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SscMode::B10)
    }
    #[doc = "up spread"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(SscMode::B11)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SscOffset {
    #[doc = "0: down 200ppm"]
    B00 = 0,
    #[doc = "1: down 200ppm"]
    B01 = 1,
    #[doc = "2: down 200ppm"]
    B10 = 2,
    #[doc = "3: down 200ppm"]
    B11 = 3,
}
impl From<SscOffset> for u8 {
    #[inline(always)]
    fn from(variant: SscOffset) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SscOffset {
    type Ux = u8;
}
#[doc = "Field `SSC_OFFSET` reader - "]
pub type SscOffsetR = crate::FieldReader<SscOffset>;
impl SscOffsetR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SscOffset {
        match self.bits {
            0 => SscOffset::B00,
            1 => SscOffset::B01,
            2 => SscOffset::B10,
            3 => SscOffset::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "down 200ppm"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == SscOffset::B00
    }
    #[doc = "down 200ppm"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == SscOffset::B01
    }
    #[doc = "down 200ppm"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == SscOffset::B10
    }
    #[doc = "down 200ppm"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == SscOffset::B11
    }
}
#[doc = "Field `SSC_OFFSET` writer - "]
pub type SscOffsetW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, SscOffset>;
impl<'a, REG> SscOffsetW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "down 200ppm"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(SscOffset::B00)
    }
    #[doc = "down 200ppm"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(SscOffset::B01)
    }
    #[doc = "down 200ppm"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(SscOffset::B10)
    }
    #[doc = "down 200ppm"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(SscOffset::B11)
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn ssc_depth(&self) -> SscDepthR {
        SscDepthR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn ssc_mode(&self) -> SscModeR {
        SscModeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn ssc_offset(&self) -> SscOffsetR {
        SscOffsetR::new(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    #[must_use]
    pub fn ssc_depth(&mut self) -> SscDepthW<SscRegSpec> {
        SscDepthW::new(self, 0)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    #[must_use]
    pub fn ssc_mode(&mut self) -> SscModeW<SscRegSpec> {
        SscModeW::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    #[must_use]
    pub fn ssc_offset(&mut self) -> SscOffsetW<SscRegSpec> {
        SscOffsetW::new(self, 6)
    }
}
#[doc = "SSC control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ssc_reg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ssc_reg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SscRegSpec;
impl crate::RegisterSpec for SscRegSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ssc_reg::R`](R) reader structure"]
impl crate::Readable for SscRegSpec {}
#[doc = "`write(|w| ..)` method takes [`ssc_reg::W`](W) writer structure"]
impl crate::Writable for SscRegSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xff;
}
#[doc = "`reset()` method sets SSC_REG to value 0x0a"]
impl crate::Resettable for SscRegSpec {
    const RESET_VALUE: u32 = 0x0a;
}
