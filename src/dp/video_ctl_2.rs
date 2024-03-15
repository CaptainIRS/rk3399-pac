#[doc = "Register `VIDEO_CTL_2` reader"]
pub type R = crate::R<VideoCtl2Spec>;
#[doc = "Register `VIDEO_CTL_2` writer"]
pub type W = crate::W<VideoCtl2Spec>;
#[doc = "Colorimetric format of input video. This is used to specify video data format in main stream attribute data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InColorF {
    #[doc = "3: RGB."]
    B11 = 3,
    #[doc = "2: RGB."]
    B10 = 2,
    #[doc = "1: RGB."]
    B01 = 1,
    #[doc = "0: RGB."]
    B00 = 0,
}
impl From<InColorF> for u8 {
    #[inline(always)]
    fn from(variant: InColorF) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for InColorF {
    type Ux = u8;
}
#[doc = "Field `IN_COLOR_F` reader - Colorimetric format of input video. This is used to specify video data format in main stream attribute data."]
pub type InColorFR = crate::FieldReader<InColorF>;
impl InColorFR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InColorF {
        match self.bits {
            3 => InColorF::B11,
            2 => InColorF::B10,
            1 => InColorF::B01,
            0 => InColorF::B00,
            _ => unreachable!(),
        }
    }
    #[doc = "RGB."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == InColorF::B11
    }
    #[doc = "RGB."]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == InColorF::B10
    }
    #[doc = "RGB."]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == InColorF::B01
    }
    #[doc = "RGB."]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == InColorF::B00
    }
}
#[doc = "Field `IN_COLOR_F` writer - Colorimetric format of input video. This is used to specify video data format in main stream attribute data."]
pub type InColorFW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, InColorF>;
impl<'a, REG> InColorFW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "RGB."]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(InColorF::B11)
    }
    #[doc = "RGB."]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(InColorF::B10)
    }
    #[doc = "RGB."]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(InColorF::B01)
    }
    #[doc = "RGB."]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(InColorF::B00)
    }
}
#[doc = "Video input bit per color/ component (bpc). This bit field is used to specify video data format in main stream attribute data. Note that 6 bpc mode is invalid in YCbCr 422 mode. 100, 101, 110, 111: Reserved,\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum InBpc {
    #[doc = "3: 6 bits."]
    B011 = 3,
    #[doc = "2: 6 bits."]
    B010 = 2,
    #[doc = "1: 6 bits."]
    B001 = 1,
    #[doc = "0: 6 bits."]
    B000 = 0,
}
impl From<InBpc> for u8 {
    #[inline(always)]
    fn from(variant: InBpc) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for InBpc {
    type Ux = u8;
}
#[doc = "Field `IN_BPC` reader - Video input bit per color/ component (bpc). This bit field is used to specify video data format in main stream attribute data. Note that 6 bpc mode is invalid in YCbCr 422 mode. 100, 101, 110, 111: Reserved,"]
pub type InBpcR = crate::FieldReader<InBpc>;
impl InBpcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<InBpc> {
        match self.bits {
            3 => Some(InBpc::B011),
            2 => Some(InBpc::B010),
            1 => Some(InBpc::B001),
            0 => Some(InBpc::B000),
            _ => None,
        }
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn is_b011(&self) -> bool {
        *self == InBpc::B011
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn is_b010(&self) -> bool {
        *self == InBpc::B010
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn is_b001(&self) -> bool {
        *self == InBpc::B001
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn is_b000(&self) -> bool {
        *self == InBpc::B000
    }
}
#[doc = "Field `IN_BPC` writer - Video input bit per color/ component (bpc). This bit field is used to specify video data format in main stream attribute data. Note that 6 bpc mode is invalid in YCbCr 422 mode. 100, 101, 110, 111: Reserved,"]
pub type InBpcW<'a, REG> = crate::FieldWriter<'a, REG, 3, InBpc>;
impl<'a, REG> InBpcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn b011(self) -> &'a mut crate::W<REG> {
        self.variant(InBpc::B011)
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn b010(self) -> &'a mut crate::W<REG> {
        self.variant(InBpc::B010)
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn b001(self) -> &'a mut crate::W<REG> {
        self.variant(InBpc::B001)
    }
    #[doc = "6 bits."]
    #[inline(always)]
    pub fn b000(self) -> &'a mut crate::W<REG> {
        self.variant(InBpc::B000)
    }
}
#[doc = "Dynamic range. This bit field is used to specify video data format in main stream attribute data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InDRange {
    #[doc = "1: VESA range (0 ~ 255)."]
    B1 = 1,
    #[doc = "0: VESA range (0 ~ 255)."]
    B0 = 0,
}
impl From<InDRange> for bool {
    #[inline(always)]
    fn from(variant: InDRange) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN_D_RANGE` reader - Dynamic range. This bit field is used to specify video data format in main stream attribute data."]
pub type InDRangeR = crate::BitReader<InDRange>;
impl InDRangeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InDRange {
        match self.bits {
            true => InDRange::B1,
            false => InDRange::B0,
        }
    }
    #[doc = "VESA range (0 ~ 255)."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == InDRange::B1
    }
    #[doc = "VESA range (0 ~ 255)."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == InDRange::B0
    }
}
#[doc = "Field `IN_D_RANGE` writer - Dynamic range. This bit field is used to specify video data format in main stream attribute data."]
pub type InDRangeW<'a, REG> = crate::BitWriter<'a, REG, InDRange>;
impl<'a, REG> InDRangeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "VESA range (0 ~ 255)."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(InDRange::B1)
    }
    #[doc = "VESA range (0 ~ 255)."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(InDRange::B0)
    }
}
impl R {
    #[doc = "Bits 0:1 - Colorimetric format of input video. This is used to specify video data format in main stream attribute data."]
    #[inline(always)]
    pub fn in_color_f(&self) -> InColorFR {
        InColorFR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:6 - Video input bit per color/ component (bpc). This bit field is used to specify video data format in main stream attribute data. Note that 6 bpc mode is invalid in YCbCr 422 mode. 100, 101, 110, 111: Reserved,"]
    #[inline(always)]
    pub fn in_bpc(&self) -> InBpcR {
        InBpcR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Dynamic range. This bit field is used to specify video data format in main stream attribute data."]
    #[inline(always)]
    pub fn in_d_range(&self) -> InDRangeR {
        InDRangeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Colorimetric format of input video. This is used to specify video data format in main stream attribute data."]
    #[inline(always)]
    #[must_use]
    pub fn in_color_f(&mut self) -> InColorFW<VideoCtl2Spec> {
        InColorFW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Video input bit per color/ component (bpc). This bit field is used to specify video data format in main stream attribute data. Note that 6 bpc mode is invalid in YCbCr 422 mode. 100, 101, 110, 111: Reserved,"]
    #[inline(always)]
    #[must_use]
    pub fn in_bpc(&mut self) -> InBpcW<VideoCtl2Spec> {
        InBpcW::new(self, 4)
    }
    #[doc = "Bit 7 - Dynamic range. This bit field is used to specify video data format in main stream attribute data."]
    #[inline(always)]
    #[must_use]
    pub fn in_d_range(&mut self) -> InDRangeW<VideoCtl2Spec> {
        InDRangeW::new(self, 7)
    }
}
#[doc = "Video Control 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`video_ctl_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`video_ctl_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VideoCtl2Spec;
impl crate::RegisterSpec for VideoCtl2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`video_ctl_2::R`](R) reader structure"]
impl crate::Readable for VideoCtl2Spec {}
#[doc = "`write(|w| ..)` method takes [`video_ctl_2::W`](W) writer structure"]
impl crate::Writable for VideoCtl2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x73;
}
#[doc = "`reset()` method sets VIDEO_CTL_2 to value 0x10"]
impl crate::Resettable for VideoCtl2Spec {
    const RESET_VALUE: u32 = 0x10;
}
