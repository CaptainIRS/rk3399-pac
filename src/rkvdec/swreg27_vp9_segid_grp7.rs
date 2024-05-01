#[doc = "Register `SWREG27_VP9_SEGID_GRP7` reader"]
pub type R = crate::R<Swreg27Vp9SegidGrp7Spec>;
#[doc = "Register `SWREG27_VP9_SEGID_GRP7` writer"]
pub type W = crate::W<Swreg27Vp9SegidGrp7Spec>;
#[doc = "Field `SW_VP9SEGID7_FRAME_QP_DELTA_EN` reader - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
pub type SwVp9segid7FrameQpDeltaEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID7_FRAME_QP_DELTA_EN` writer - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
pub type SwVp9segid7FrameQpDeltaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID7_FRAME_QP_DELTA` reader - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
pub type SwVp9segid7FrameQpDeltaR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9SEGID7_FRAME_QP_DELTA` writer - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
pub type SwVp9segid7FrameQpDeltaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SW_VP9SEGID7_FRAME_LOOPFITLER_VALUE_EN` reader - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
pub type SwVp9segid7FrameLoopfitlerValueEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID7_FRAME_LOOPFITLER_VALUE_EN` writer - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
pub type SwVp9segid7FrameLoopfitlerValueEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID7_FRAME_LOOPFILTER_VALUE` reader - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
pub type SwVp9segid7FrameLoopfilterValueR = crate::FieldReader;
#[doc = "Field `SW_VP9SEGID7_FRAME_LOOPFILTER_VALUE` writer - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
pub type SwVp9segid7FrameLoopfilterValueW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SW_VP9SEGID7_REFERINFO_EN` reader - frame reference info enable\n\nframe reference info enable"]
pub type SwVp9segid7ReferinfoEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID7_REFERINFO_EN` writer - frame reference info enable\n\nframe reference info enable"]
pub type SwVp9segid7ReferinfoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID7_REFERINFO` reader - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
pub type SwVp9segid7ReferinfoR = crate::FieldReader;
#[doc = "Field `SW_VP9SEGID7_REFERINFO` writer - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
pub type SwVp9segid7ReferinfoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_VP9SEGID7_FRAME_SKIP_EN` reader - frame skip feature enable\n\nframe skip feature enable"]
pub type SwVp9segid7FrameSkipEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID7_FRAME_SKIP_EN` writer - frame skip feature enable\n\nframe skip feature enable"]
pub type SwVp9segid7FrameSkipEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
    #[inline(always)]
    pub fn sw_vp9segid7_frame_qp_delta_en(&self) -> SwVp9segid7FrameQpDeltaEnR {
        SwVp9segid7FrameQpDeltaEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10 - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
    #[inline(always)]
    pub fn sw_vp9segid7_frame_qp_delta(&self) -> SwVp9segid7FrameQpDeltaR {
        SwVp9segid7FrameQpDeltaR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
    #[inline(always)]
    pub fn sw_vp9segid7_frame_loopfitler_value_en(&self) -> SwVp9segid7FrameLoopfitlerValueEnR {
        SwVp9segid7FrameLoopfitlerValueEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:18 - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
    #[inline(always)]
    pub fn sw_vp9segid7_frame_loopfilter_value(&self) -> SwVp9segid7FrameLoopfilterValueR {
        SwVp9segid7FrameLoopfilterValueR::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bit 19 - frame reference info enable\n\nframe reference info enable"]
    #[inline(always)]
    pub fn sw_vp9segid7_referinfo_en(&self) -> SwVp9segid7ReferinfoEnR {
        SwVp9segid7ReferinfoEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
    #[inline(always)]
    pub fn sw_vp9segid7_referinfo(&self) -> SwVp9segid7ReferinfoR {
        SwVp9segid7ReferinfoR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - frame skip feature enable\n\nframe skip feature enable"]
    #[inline(always)]
    pub fn sw_vp9segid7_frame_skip_en(&self) -> SwVp9segid7FrameSkipEnR {
        SwVp9segid7FrameSkipEnR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid7_frame_qp_delta_en(
        &mut self,
    ) -> SwVp9segid7FrameQpDeltaEnW<Swreg27Vp9SegidGrp7Spec> {
        SwVp9segid7FrameQpDeltaEnW::new(self, 1)
    }
    #[doc = "Bits 2:10 - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid7_frame_qp_delta(
        &mut self,
    ) -> SwVp9segid7FrameQpDeltaW<Swreg27Vp9SegidGrp7Spec> {
        SwVp9segid7FrameQpDeltaW::new(self, 2)
    }
    #[doc = "Bit 11 - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid7_frame_loopfitler_value_en(
        &mut self,
    ) -> SwVp9segid7FrameLoopfitlerValueEnW<Swreg27Vp9SegidGrp7Spec> {
        SwVp9segid7FrameLoopfitlerValueEnW::new(self, 11)
    }
    #[doc = "Bits 12:18 - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid7_frame_loopfilter_value(
        &mut self,
    ) -> SwVp9segid7FrameLoopfilterValueW<Swreg27Vp9SegidGrp7Spec> {
        SwVp9segid7FrameLoopfilterValueW::new(self, 12)
    }
    #[doc = "Bit 19 - frame reference info enable\n\nframe reference info enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid7_referinfo_en(
        &mut self,
    ) -> SwVp9segid7ReferinfoEnW<Swreg27Vp9SegidGrp7Spec> {
        SwVp9segid7ReferinfoEnW::new(self, 19)
    }
    #[doc = "Bits 20:21 - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid7_referinfo(&mut self) -> SwVp9segid7ReferinfoW<Swreg27Vp9SegidGrp7Spec> {
        SwVp9segid7ReferinfoW::new(self, 20)
    }
    #[doc = "Bit 22 - frame skip feature enable\n\nframe skip feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid7_frame_skip_en(
        &mut self,
    ) -> SwVp9segid7FrameSkipEnW<Swreg27Vp9SegidGrp7Spec> {
        SwVp9segid7FrameSkipEnW::new(self, 22)
    }
}
#[doc = "vp9 segid syntax grp7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg27_vp9_segid_grp7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg27_vp9_segid_grp7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg27Vp9SegidGrp7Spec;
impl crate::RegisterSpec for Swreg27Vp9SegidGrp7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg27_vp9_segid_grp7::R`](R) reader structure"]
impl crate::Readable for Swreg27Vp9SegidGrp7Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg27_vp9_segid_grp7::W`](W) writer structure"]
impl crate::Writable for Swreg27Vp9SegidGrp7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG27_VP9_SEGID_GRP7 to value 0"]
impl crate::Resettable for Swreg27Vp9SegidGrp7Spec {
    const RESET_VALUE: u32 = 0;
}
