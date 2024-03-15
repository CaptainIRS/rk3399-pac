#[doc = "Register `VIDEO_STATUS` reader"]
pub type R = crate::R<VideoStatusSpec>;
#[doc = "Register `VIDEO_STATUS` writer"]
pub type W = crate::W<VideoStatusSpec>;
#[doc = "Auto-detect HSYNC polarity:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HsyncPS {
    #[doc = "1: High is active. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    B1 = 1,
    #[doc = "0: High is active. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    B0 = 0,
}
impl From<HsyncPS> for bool {
    #[inline(always)]
    fn from(variant: HsyncPS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HSYNC_P_S` reader - Auto-detect HSYNC polarity:"]
pub type HsyncPSR = crate::BitReader<HsyncPS>;
impl HsyncPSR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HsyncPS {
        match self.bits {
            true => HsyncPS::B1,
            false => HsyncPS::B0,
        }
    }
    #[doc = "High is active. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HsyncPS::B1
    }
    #[doc = "High is active. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HsyncPS::B0
    }
}
#[doc = "Field `HSYNC_P_S` writer - Auto-detect HSYNC polarity:"]
pub type HsyncPSW<'a, REG> = crate::BitWriter1C<'a, REG, HsyncPS>;
impl<'a, REG> HsyncPSW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High is active. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HsyncPS::B1)
    }
    #[doc = "High is active. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HsyncPS::B0)
    }
}
#[doc = "Auto-detect VSYNC polarity:\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VsyncPS {
    #[doc = "1: High is active. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    B1 = 1,
    #[doc = "0: High is active. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    B0 = 0,
}
impl From<VsyncPS> for bool {
    #[inline(always)]
    fn from(variant: VsyncPS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VSYNC_P_S` reader - Auto-detect VSYNC polarity:"]
pub type VsyncPSR = crate::BitReader<VsyncPS>;
impl VsyncPSR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VsyncPS {
        match self.bits {
            true => VsyncPS::B1,
            false => VsyncPS::B0,
        }
    }
    #[doc = "High is active. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VsyncPS::B1
    }
    #[doc = "High is active. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VsyncPS::B0
    }
}
#[doc = "Field `VSYNC_P_S` writer - Auto-detect VSYNC polarity:"]
pub type VsyncPSW<'a, REG> = crate::BitWriter1C<'a, REG, VsyncPS>;
impl<'a, REG> VsyncPSW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "High is active. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VsyncPS::B1)
    }
    #[doc = "High is active. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VsyncPS::B0)
    }
}
#[doc = "Auto-detect interlace or progressive scan status:\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IScanS {
    #[doc = "1: Progressive scan. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    B1 = 1,
    #[doc = "0: Progressive scan. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    B0 = 0,
}
impl From<IScanS> for bool {
    #[inline(always)]
    fn from(variant: IScanS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `I_SCAN_S` reader - Auto-detect interlace or progressive scan status:"]
pub type IScanSR = crate::BitReader<IScanS>;
impl IScanSR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> IScanS {
        match self.bits {
            true => IScanS::B1,
            false => IScanS::B0,
        }
    }
    #[doc = "Progressive scan. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == IScanS::B1
    }
    #[doc = "Progressive scan. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == IScanS::B0
    }
}
#[doc = "Field `I_SCAN_S` writer - Auto-detect interlace or progressive scan status:"]
pub type IScanSW<'a, REG> = crate::BitWriter1C<'a, REG, IScanS>;
impl<'a, REG> IScanSW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Progressive scan. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(IScanS::B1)
    }
    #[doc = "Progressive scan. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(IScanS::B0)
    }
}
#[doc = "Interlace scan field status.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FieldS {
    #[doc = "1: First field. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    B1 = 1,
    #[doc = "0: First field. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    B0 = 0,
}
impl From<FieldS> for bool {
    #[inline(always)]
    fn from(variant: FieldS) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FIELD_S` reader - Interlace scan field status."]
pub type FieldSR = crate::BitReader<FieldS>;
impl FieldSR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FieldS {
        match self.bits {
            true => FieldS::B1,
            false => FieldS::B0,
        }
    }
    #[doc = "First field. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == FieldS::B1
    }
    #[doc = "First field. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == FieldS::B0
    }
}
#[doc = "Field `FIELD_S` writer - Interlace scan field status."]
pub type FieldSW<'a, REG> = crate::BitWriter<'a, REG, FieldS>;
impl<'a, REG> FieldSW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "First field. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(FieldS::B1)
    }
    #[doc = "First field. This bit field is valid only when STRM_VALID is high. And STRM_VALID becomes high when two successive frames are determined as stable."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(FieldS::B0)
    }
}
impl R {
    #[doc = "Bit 0 - Auto-detect HSYNC polarity:"]
    #[inline(always)]
    pub fn hsync_p_s(&self) -> HsyncPSR {
        HsyncPSR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Auto-detect VSYNC polarity:"]
    #[inline(always)]
    pub fn vsync_p_s(&self) -> VsyncPSR {
        VsyncPSR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Auto-detect interlace or progressive scan status:"]
    #[inline(always)]
    pub fn i_scan_s(&self) -> IScanSR {
        IScanSR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Interlace scan field status."]
    #[inline(always)]
    pub fn field_s(&self) -> FieldSR {
        FieldSR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Auto-detect HSYNC polarity:"]
    #[inline(always)]
    #[must_use]
    pub fn hsync_p_s(&mut self) -> HsyncPSW<VideoStatusSpec> {
        HsyncPSW::new(self, 0)
    }
    #[doc = "Bit 1 - Auto-detect VSYNC polarity:"]
    #[inline(always)]
    #[must_use]
    pub fn vsync_p_s(&mut self) -> VsyncPSW<VideoStatusSpec> {
        VsyncPSW::new(self, 1)
    }
    #[doc = "Bit 2 - Auto-detect interlace or progressive scan status:"]
    #[inline(always)]
    #[must_use]
    pub fn i_scan_s(&mut self) -> IScanSW<VideoStatusSpec> {
        IScanSW::new(self, 2)
    }
    #[doc = "Bit 3 - Interlace scan field status."]
    #[inline(always)]
    #[must_use]
    pub fn field_s(&mut self) -> FieldSW<VideoStatusSpec> {
        FieldSW::new(self, 3)
    }
}
#[doc = "Video Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`video_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`video_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VideoStatusSpec;
impl crate::RegisterSpec for VideoStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`video_status::R`](R) reader structure"]
impl crate::Readable for VideoStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`video_status::W`](W) writer structure"]
impl crate::Writable for VideoStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x07;
}
#[doc = "`reset()` method sets VIDEO_STATUS to value 0x03"]
impl crate::Resettable for VideoStatusSpec {
    const RESET_VALUE: u32 = 0x03;
}
