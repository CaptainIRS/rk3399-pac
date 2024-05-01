#[doc = "Register `SWREG26_VP9_SEGID_GRP6` reader"]
pub type R = crate::R<Swreg26Vp9SegidGrp6Spec>;
#[doc = "Register `SWREG26_VP9_SEGID_GRP6` writer"]
pub type W = crate::W<Swreg26Vp9SegidGrp6Spec>;
#[doc = "Field `SW_VP9SEGID6_FRAME_QP_DELTA_EN` reader - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
pub type SwVp9segid6FrameQpDeltaEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID6_FRAME_QP_DELTA_EN` writer - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
pub type SwVp9segid6FrameQpDeltaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID6_FRAME_QP_DELTA` reader - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
pub type SwVp9segid6FrameQpDeltaR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9SEGID6_FRAME_QP_DELTA` writer - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
pub type SwVp9segid6FrameQpDeltaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SW_VP9SEGID6_FRAME_LOOPFITLER_VALUE_EN` reader - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
pub type SwVp9segid6FrameLoopfitlerValueEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID6_FRAME_LOOPFITLER_VALUE_EN` writer - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
pub type SwVp9segid6FrameLoopfitlerValueEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID6_FRAME_LOOPFILTER_VALUE` reader - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
pub type SwVp9segid6FrameLoopfilterValueR = crate::FieldReader;
#[doc = "Field `SW_VP9SEGID6_FRAME_LOOPFILTER_VALUE` writer - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
pub type SwVp9segid6FrameLoopfilterValueW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SW_VP9SEGID6_REFERINFO_EN` reader - frame reference info enable\n\nframe reference info enable"]
pub type SwVp9segid6ReferinfoEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID6_REFERINFO_EN` writer - frame reference info enable\n\nframe reference info enable"]
pub type SwVp9segid6ReferinfoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID6_REFERINFO` reader - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
pub type SwVp9segid6ReferinfoR = crate::FieldReader;
#[doc = "Field `SW_VP9SEGID6_REFERINFO` writer - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
pub type SwVp9segid6ReferinfoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_VP9SEGID6_FRAME_SKIP_EN` reader - frame skip feature enable\n\nframe skip feature enable"]
pub type SwVp9segid6FrameSkipEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID6_FRAME_SKIP_EN` writer - frame skip feature enable\n\nframe skip feature enable"]
pub type SwVp9segid6FrameSkipEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
    #[inline(always)]
    pub fn sw_vp9segid6_frame_qp_delta_en(&self) -> SwVp9segid6FrameQpDeltaEnR {
        SwVp9segid6FrameQpDeltaEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10 - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
    #[inline(always)]
    pub fn sw_vp9segid6_frame_qp_delta(&self) -> SwVp9segid6FrameQpDeltaR {
        SwVp9segid6FrameQpDeltaR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
    #[inline(always)]
    pub fn sw_vp9segid6_frame_loopfitler_value_en(&self) -> SwVp9segid6FrameLoopfitlerValueEnR {
        SwVp9segid6FrameLoopfitlerValueEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:18 - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
    #[inline(always)]
    pub fn sw_vp9segid6_frame_loopfilter_value(&self) -> SwVp9segid6FrameLoopfilterValueR {
        SwVp9segid6FrameLoopfilterValueR::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bit 19 - frame reference info enable\n\nframe reference info enable"]
    #[inline(always)]
    pub fn sw_vp9segid6_referinfo_en(&self) -> SwVp9segid6ReferinfoEnR {
        SwVp9segid6ReferinfoEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
    #[inline(always)]
    pub fn sw_vp9segid6_referinfo(&self) -> SwVp9segid6ReferinfoR {
        SwVp9segid6ReferinfoR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - frame skip feature enable\n\nframe skip feature enable"]
    #[inline(always)]
    pub fn sw_vp9segid6_frame_skip_en(&self) -> SwVp9segid6FrameSkipEnR {
        SwVp9segid6FrameSkipEnR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid6_frame_qp_delta_en(
        &mut self,
    ) -> SwVp9segid6FrameQpDeltaEnW<Swreg26Vp9SegidGrp6Spec> {
        SwVp9segid6FrameQpDeltaEnW::new(self, 1)
    }
    #[doc = "Bits 2:10 - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid6_frame_qp_delta(
        &mut self,
    ) -> SwVp9segid6FrameQpDeltaW<Swreg26Vp9SegidGrp6Spec> {
        SwVp9segid6FrameQpDeltaW::new(self, 2)
    }
    #[doc = "Bit 11 - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid6_frame_loopfitler_value_en(
        &mut self,
    ) -> SwVp9segid6FrameLoopfitlerValueEnW<Swreg26Vp9SegidGrp6Spec> {
        SwVp9segid6FrameLoopfitlerValueEnW::new(self, 11)
    }
    #[doc = "Bits 12:18 - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid6_frame_loopfilter_value(
        &mut self,
    ) -> SwVp9segid6FrameLoopfilterValueW<Swreg26Vp9SegidGrp6Spec> {
        SwVp9segid6FrameLoopfilterValueW::new(self, 12)
    }
    #[doc = "Bit 19 - frame reference info enable\n\nframe reference info enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid6_referinfo_en(
        &mut self,
    ) -> SwVp9segid6ReferinfoEnW<Swreg26Vp9SegidGrp6Spec> {
        SwVp9segid6ReferinfoEnW::new(self, 19)
    }
    #[doc = "Bits 20:21 - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid6_referinfo(&mut self) -> SwVp9segid6ReferinfoW<Swreg26Vp9SegidGrp6Spec> {
        SwVp9segid6ReferinfoW::new(self, 20)
    }
    #[doc = "Bit 22 - frame skip feature enable\n\nframe skip feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid6_frame_skip_en(
        &mut self,
    ) -> SwVp9segid6FrameSkipEnW<Swreg26Vp9SegidGrp6Spec> {
        SwVp9segid6FrameSkipEnW::new(self, 22)
    }
}
#[doc = "vp9 segid syntax grp6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg26_vp9_segid_grp6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg26_vp9_segid_grp6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg26Vp9SegidGrp6Spec;
impl crate::RegisterSpec for Swreg26Vp9SegidGrp6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg26_vp9_segid_grp6::R`](R) reader structure"]
impl crate::Readable for Swreg26Vp9SegidGrp6Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg26_vp9_segid_grp6::W`](W) writer structure"]
impl crate::Writable for Swreg26Vp9SegidGrp6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG26_VP9_SEGID_GRP6 to value 0"]
impl crate::Resettable for Swreg26Vp9SegidGrp6Spec {
    const RESET_VALUE: u32 = 0;
}
