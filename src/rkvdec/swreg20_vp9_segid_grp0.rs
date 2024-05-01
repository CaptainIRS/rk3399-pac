#[doc = "Register `SWREG20_VP9_SEGID_GRP0` reader"]
pub type R = crate::R<Swreg20Vp9SegidGrp0Spec>;
#[doc = "Register `SWREG20_VP9_SEGID_GRP0` writer"]
pub type W = crate::W<Swreg20Vp9SegidGrp0Spec>;
#[doc = "Field `SW_VP9SEGID_ABS_DELTA` reader - abs delta\n\nused to decide quant and loopfilter param"]
pub type SwVp9segidAbsDeltaR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID_ABS_DELTA` writer - abs delta\n\nused to decide quant and loopfilter param"]
pub type SwVp9segidAbsDeltaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID0_FRAME_QP_DELTA_EN` reader - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
pub type SwVp9segid0FrameQpDeltaEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID0_FRAME_QP_DELTA_EN` writer - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
pub type SwVp9segid0FrameQpDeltaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID0_FRAME_QP_DELTA` reader - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
pub type SwVp9segid0FrameQpDeltaR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9SEGID0_FRAME_QP_DELTA` writer - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
pub type SwVp9segid0FrameQpDeltaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SW_VP9SEGID0_FRAME_LOOPFITLER_VALUE_EN` reader - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
pub type SwVp9segid0FrameLoopfitlerValueEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID0_FRAME_LOOPFITLER_VALUE_EN` writer - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
pub type SwVp9segid0FrameLoopfitlerValueEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID0_FRAME_LOOPFILTER_VALUE` reader - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
pub type SwVp9segid0FrameLoopfilterValueR = crate::FieldReader;
#[doc = "Field `SW_VP9SEGID0_FRAME_LOOPFILTER_VALUE` writer - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
pub type SwVp9segid0FrameLoopfilterValueW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SW_VP9SEGID0_REFERINFO_EN` reader - frame reference info enable\n\nframe reference info enable"]
pub type SwVp9segid0ReferinfoEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID0_REFERINFO_EN` writer - frame reference info enable\n\nframe reference info enable"]
pub type SwVp9segid0ReferinfoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID0_REFERINFO` reader - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
pub type SwVp9segid0ReferinfoR = crate::FieldReader;
#[doc = "Field `SW_VP9SEGID0_REFERINFO` writer - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
pub type SwVp9segid0ReferinfoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_VP9SEGID0_FRAME_SKIP_EN` reader - frame skip feature enable\n\nframe skip feature enable"]
pub type SwVp9segid0FrameSkipEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID0_FRAME_SKIP_EN` writer - frame skip feature enable\n\nframe skip feature enable"]
pub type SwVp9segid0FrameSkipEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - abs delta\n\nused to decide quant and loopfilter param"]
    #[inline(always)]
    pub fn sw_vp9segid_abs_delta(&self) -> SwVp9segidAbsDeltaR {
        SwVp9segidAbsDeltaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
    #[inline(always)]
    pub fn sw_vp9segid0_frame_qp_delta_en(&self) -> SwVp9segid0FrameQpDeltaEnR {
        SwVp9segid0FrameQpDeltaEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10 - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
    #[inline(always)]
    pub fn sw_vp9segid0_frame_qp_delta(&self) -> SwVp9segid0FrameQpDeltaR {
        SwVp9segid0FrameQpDeltaR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
    #[inline(always)]
    pub fn sw_vp9segid0_frame_loopfitler_value_en(&self) -> SwVp9segid0FrameLoopfitlerValueEnR {
        SwVp9segid0FrameLoopfitlerValueEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:18 - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
    #[inline(always)]
    pub fn sw_vp9segid0_frame_loopfilter_value(&self) -> SwVp9segid0FrameLoopfilterValueR {
        SwVp9segid0FrameLoopfilterValueR::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bit 19 - frame reference info enable\n\nframe reference info enable"]
    #[inline(always)]
    pub fn sw_vp9segid0_referinfo_en(&self) -> SwVp9segid0ReferinfoEnR {
        SwVp9segid0ReferinfoEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
    #[inline(always)]
    pub fn sw_vp9segid0_referinfo(&self) -> SwVp9segid0ReferinfoR {
        SwVp9segid0ReferinfoR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - frame skip feature enable\n\nframe skip feature enable"]
    #[inline(always)]
    pub fn sw_vp9segid0_frame_skip_en(&self) -> SwVp9segid0FrameSkipEnR {
        SwVp9segid0FrameSkipEnR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - abs delta\n\nused to decide quant and loopfilter param"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid_abs_delta(&mut self) -> SwVp9segidAbsDeltaW<Swreg20Vp9SegidGrp0Spec> {
        SwVp9segidAbsDeltaW::new(self, 0)
    }
    #[doc = "Bit 1 - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid0_frame_qp_delta_en(
        &mut self,
    ) -> SwVp9segid0FrameQpDeltaEnW<Swreg20Vp9SegidGrp0Spec> {
        SwVp9segid0FrameQpDeltaEnW::new(self, 1)
    }
    #[doc = "Bits 2:10 - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid0_frame_qp_delta(
        &mut self,
    ) -> SwVp9segid0FrameQpDeltaW<Swreg20Vp9SegidGrp0Spec> {
        SwVp9segid0FrameQpDeltaW::new(self, 2)
    }
    #[doc = "Bit 11 - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid0_frame_loopfitler_value_en(
        &mut self,
    ) -> SwVp9segid0FrameLoopfitlerValueEnW<Swreg20Vp9SegidGrp0Spec> {
        SwVp9segid0FrameLoopfitlerValueEnW::new(self, 11)
    }
    #[doc = "Bits 12:18 - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid0_frame_loopfilter_value(
        &mut self,
    ) -> SwVp9segid0FrameLoopfilterValueW<Swreg20Vp9SegidGrp0Spec> {
        SwVp9segid0FrameLoopfilterValueW::new(self, 12)
    }
    #[doc = "Bit 19 - frame reference info enable\n\nframe reference info enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid0_referinfo_en(
        &mut self,
    ) -> SwVp9segid0ReferinfoEnW<Swreg20Vp9SegidGrp0Spec> {
        SwVp9segid0ReferinfoEnW::new(self, 19)
    }
    #[doc = "Bits 20:21 - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid0_referinfo(&mut self) -> SwVp9segid0ReferinfoW<Swreg20Vp9SegidGrp0Spec> {
        SwVp9segid0ReferinfoW::new(self, 20)
    }
    #[doc = "Bit 22 - frame skip feature enable\n\nframe skip feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid0_frame_skip_en(
        &mut self,
    ) -> SwVp9segid0FrameSkipEnW<Swreg20Vp9SegidGrp0Spec> {
        SwVp9segid0FrameSkipEnW::new(self, 22)
    }
}
#[doc = "vp9 segid syntax grp0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg20_vp9_segid_grp0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg20_vp9_segid_grp0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg20Vp9SegidGrp0Spec;
impl crate::RegisterSpec for Swreg20Vp9SegidGrp0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg20_vp9_segid_grp0::R`](R) reader structure"]
impl crate::Readable for Swreg20Vp9SegidGrp0Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg20_vp9_segid_grp0::W`](W) writer structure"]
impl crate::Writable for Swreg20Vp9SegidGrp0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG20_VP9_SEGID_GRP0 to value 0"]
impl crate::Resettable for Swreg20Vp9SegidGrp0Spec {
    const RESET_VALUE: u32 = 0;
}
