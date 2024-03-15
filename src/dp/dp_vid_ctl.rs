#[doc = "Register `DP_VID_CTL` reader"]
pub type R = crate::R<DpVidCtlSpec>;
#[doc = "Register `DP_VID_CTL` writer"]
pub type W = crate::W<DpVidCtlSpec>;
#[doc = "Colorimetric format with video which transferred via DP main link\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ColorF {
    #[doc = "3: RGB."]
    B11 = 3,
    #[doc = "2: RGB."]
    B10 = 2,
    #[doc = "1: RGB."]
    B01 = 1,
    #[doc = "0: RGB."]
    B00 = 0,
}
impl From<ColorF> for u8 {
    #[inline(always)]
    fn from(variant: ColorF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ColorF {
    type Ux = u8;
}
#[doc = "Field `COLOR_F` reader - Colorimetric format with video which transferred via DP main link"]
pub type ColorFR = crate::FieldReader<ColorF>;
impl ColorFR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ColorF {
        match self.bits {
            3 => ColorF::B11,
            2 => ColorF::B10,
            1 => ColorF::B01,
            0 => ColorF::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "RGB."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == ColorF::B11
    }
    #[doc = "RGB."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ColorF::B10
    }
    #[doc = "RGB."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == ColorF::B01
    }
    #[doc = "RGB."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ColorF::B00
    }
}
#[doc = "Field `COLOR_F` writer - Colorimetric format with video which transferred via DP main link"]
pub type ColorFW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, ColorF>;
impl<'a, REG> ColorFW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RGB."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(ColorF::B11)
    }
    #[doc = "RGB."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(ColorF::B10)
    }
    #[doc = "RGB."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(ColorF::B01)
    }
    #[doc = "RGB."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(ColorF::B00)
    }
}
#[doc = "Dynamic range\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DRange {
    #[doc = "1: VESA range (from 0 to the maximum)."]
    B1 = 1,
    #[doc = "0: VESA range (from 0 to the maximum)."]
    B0 = 0,
}
impl From<DRange> for bool {
    #[inline(always)]
    fn from(variant: DRange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `D_RANGE` reader - Dynamic range"]
pub type DRangeR = crate::BitReader<DRange>;
impl DRangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DRange {
        match self.bits {
            true => DRange::B1,
            false => DRange::B0,
        }
    }
    #[doc = "VESA range (from 0 to the maximum)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == DRange::B1
    }
    #[doc = "VESA range (from 0 to the maximum)."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == DRange::B0
    }
}
#[doc = "Field `D_RANGE` writer - Dynamic range"]
pub type DRangeW<'a, REG> = crate::BitWriter<'a, REG, DRange>;
impl<'a, REG> DRangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VESA range (from 0 to the maximum)."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(DRange::B1)
    }
    #[doc = "VESA range (from 0 to the maximum)."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(DRange::B0)
    }
}
#[doc = "YcbCr Coefficients with video which transferred via DP main link\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum YcCoeff {
    #[doc = "1: ITU601."]
    B1 = 1,
    #[doc = "0: ITU601."]
    B0 = 0,
}
impl From<YcCoeff> for bool {
    #[inline(always)]
    fn from(variant: YcCoeff) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `YC_COEFF` reader - YcbCr Coefficients with video which transferred via DP main link"]
pub type YcCoeffR = crate::BitReader<YcCoeff>;
impl YcCoeffR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> YcCoeff {
        match self.bits {
            true => YcCoeff::B1,
            false => YcCoeff::B0,
        }
    }
    #[doc = "ITU601."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == YcCoeff::B1
    }
    #[doc = "ITU601."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == YcCoeff::B0
    }
}
#[doc = "Field `YC_COEFF` writer - YcbCr Coefficients with video which transferred via DP main link"]
pub type YcCoeffW<'a, REG> = crate::BitWriter1C<'a, REG, YcCoeff>;
impl<'a, REG> YcCoeffW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ITU601."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(YcCoeff::B1)
    }
    #[doc = "ITU601."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(YcCoeff::B0)
    }
}
#[doc = "Bit per color/ component with video which transferred via DP main link 101, 110, 111, 100: Reserved,\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Bpc {
    #[doc = "3: 6 bits."]
    B011 = 3,
    #[doc = "2: 6 bits."]
    B010 = 2,
    #[doc = "1: 6 bits."]
    B001 = 1,
    #[doc = "0: 6 bits."]
    B000 = 0,
}
impl From<Bpc> for u8 {
    #[inline(always)]
    fn from(variant: Bpc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Bpc {
    type Ux = u8;
}
#[doc = "Field `BPC` reader - Bit per color/ component with video which transferred via DP main link 101, 110, 111, 100: Reserved,"]
pub type BpcR = crate::FieldReader<Bpc>;
impl BpcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Bpc> {
        match self.bits {
            3 => Some(Bpc::B011),
            2 => Some(Bpc::B010),
            1 => Some(Bpc::B001),
            0 => Some(Bpc::B000),
            _ => None,
        }
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == Bpc::B011
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == Bpc::B010
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == Bpc::B001
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == Bpc::B000
    }
}
#[doc = "Field `BPC` writer - Bit per color/ component with video which transferred via DP main link 101, 110, 111, 100: Reserved,"]
pub type BpcW<'a, REG> = crate::FieldWriter<'a, REG, 3, Bpc>;
impl<'a, REG> BpcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(Bpc::B011)
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(Bpc::B010)
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(Bpc::B001)
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(Bpc::B000)
    }
}
impl R {
    #[doc = "Bits 1:2 - Colorimetric format with video which transferred via DP main link"]
    #[inline(always)]
    pub fn color_f(&self) -> ColorFR {
        ColorFR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Dynamic range"]
    #[inline(always)]
    pub fn d_range(&self) -> DRangeR {
        DRangeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - YcbCr Coefficients with video which transferred via DP main link"]
    #[inline(always)]
    pub fn yc_coeff(&self) -> YcCoeffR {
        YcCoeffR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:7 - Bit per color/ component with video which transferred via DP main link 101, 110, 111, 100: Reserved,"]
    #[inline(always)]
    pub fn bpc(&self) -> BpcR {
        BpcR::new(((self.bits >> 5) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 1:2 - Colorimetric format with video which transferred via DP main link"]
    #[inline(always)]
    #[must_use]
    pub fn color_f(&mut self) -> ColorFW<DpVidCtlSpec> {
        ColorFW::new(self, 1)
    }
    #[doc = "Bit 3 - Dynamic range"]
    #[inline(always)]
    #[must_use]
    pub fn d_range(&mut self) -> DRangeW<DpVidCtlSpec> {
        DRangeW::new(self, 3)
    }
    #[doc = "Bit 4 - YcbCr Coefficients with video which transferred via DP main link"]
    #[inline(always)]
    #[must_use]
    pub fn yc_coeff(&mut self) -> YcCoeffW<DpVidCtlSpec> {
        YcCoeffW::new(self, 4)
    }
    #[doc = "Bits 5:7 - Bit per color/ component with video which transferred via DP main link 101, 110, 111, 100: Reserved,"]
    #[inline(always)]
    #[must_use]
    pub fn bpc(&mut self) -> BpcW<DpVidCtlSpec> {
        BpcW::new(self, 5)
    }
}
#[doc = "DP Video Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_vid_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_vid_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpVidCtlSpec;
impl crate::RegisterSpec for DpVidCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_vid_ctl::R`](R) reader structure"]
impl crate::Readable for DpVidCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_vid_ctl::W`](W) writer structure"]
impl crate::Writable for DpVidCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf6;
}
#[doc = "`reset()` method sets DP_VID_CTL to value 0x20"]
impl crate::Resettable for DpVidCtlSpec {
    const RESET_VALUE: u32 = 0x20;
}
