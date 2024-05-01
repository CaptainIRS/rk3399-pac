#[doc = "Register `SWREG25_VP9_SEGID_GRP5` reader"]
pub type R = crate::R<Swreg25Vp9SegidGrp5Spec>;
#[doc = "Register `SWREG25_VP9_SEGID_GRP5` writer"]
pub type W = crate::W<Swreg25Vp9SegidGrp5Spec>;
#[doc = "Field `SW_VP9SEGID5_FRAME_QP_DELTA_EN` reader - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
pub type SwVp9segid5FrameQpDeltaEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID5_FRAME_QP_DELTA_EN` writer - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
pub type SwVp9segid5FrameQpDeltaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID5_FRAME_QP_DELTA` reader - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
pub type SwVp9segid5FrameQpDeltaR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9SEGID5_FRAME_QP_DELTA` writer - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
pub type SwVp9segid5FrameQpDeltaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SW_VP9SEGID5_FRAME_LOOPFITLER_VALUE_EN` reader - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
pub type SwVp9segid5FrameLoopfitlerValueEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID5_FRAME_LOOPFITLER_VALUE_EN` writer - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
pub type SwVp9segid5FrameLoopfitlerValueEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID5_FRAME_LOOPFILTER_VALUE` reader - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
pub type SwVp9segid5FrameLoopfilterValueR = crate::FieldReader;
#[doc = "Field `SW_VP9SEGID5_FRAME_LOOPFILTER_VALUE` writer - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
pub type SwVp9segid5FrameLoopfilterValueW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SW_VP9SEGID5_REFERINFO_EN` reader - frame reference info enable\n\nframe reference info enable"]
pub type SwVp9segid5ReferinfoEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID5_REFERINFO_EN` writer - frame reference info enable\n\nframe reference info enable"]
pub type SwVp9segid5ReferinfoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID5_REFERINFO` reader - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
pub type SwVp9segid5ReferinfoR = crate::FieldReader;
#[doc = "Field `SW_VP9SEGID5_REFERINFO` writer - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
pub type SwVp9segid5ReferinfoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_VP9SEGID5_FRAME_SKIP_EN` reader - frame skip feature enable\n\nframe skip feature enable"]
pub type SwVp9segid5FrameSkipEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID5_FRAME_SKIP_EN` writer - frame skip feature enable\n\nframe skip feature enable"]
pub type SwVp9segid5FrameSkipEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
    #[inline(always)]
    pub fn sw_vp9segid5_frame_qp_delta_en(&self) -> SwVp9segid5FrameQpDeltaEnR {
        SwVp9segid5FrameQpDeltaEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10 - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
    #[inline(always)]
    pub fn sw_vp9segid5_frame_qp_delta(&self) -> SwVp9segid5FrameQpDeltaR {
        SwVp9segid5FrameQpDeltaR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
    #[inline(always)]
    pub fn sw_vp9segid5_frame_loopfitler_value_en(&self) -> SwVp9segid5FrameLoopfitlerValueEnR {
        SwVp9segid5FrameLoopfitlerValueEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:18 - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
    #[inline(always)]
    pub fn sw_vp9segid5_frame_loopfilter_value(&self) -> SwVp9segid5FrameLoopfilterValueR {
        SwVp9segid5FrameLoopfilterValueR::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bit 19 - frame reference info enable\n\nframe reference info enable"]
    #[inline(always)]
    pub fn sw_vp9segid5_referinfo_en(&self) -> SwVp9segid5ReferinfoEnR {
        SwVp9segid5ReferinfoEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
    #[inline(always)]
    pub fn sw_vp9segid5_referinfo(&self) -> SwVp9segid5ReferinfoR {
        SwVp9segid5ReferinfoR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - frame skip feature enable\n\nframe skip feature enable"]
    #[inline(always)]
    pub fn sw_vp9segid5_frame_skip_en(&self) -> SwVp9segid5FrameSkipEnR {
        SwVp9segid5FrameSkipEnR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid5_frame_qp_delta_en(
        &mut self,
    ) -> SwVp9segid5FrameQpDeltaEnW<Swreg25Vp9SegidGrp5Spec> {
        SwVp9segid5FrameQpDeltaEnW::new(self, 1)
    }
    #[doc = "Bits 2:10 - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid5_frame_qp_delta(
        &mut self,
    ) -> SwVp9segid5FrameQpDeltaW<Swreg25Vp9SegidGrp5Spec> {
        SwVp9segid5FrameQpDeltaW::new(self, 2)
    }
    #[doc = "Bit 11 - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid5_frame_loopfitler_value_en(
        &mut self,
    ) -> SwVp9segid5FrameLoopfitlerValueEnW<Swreg25Vp9SegidGrp5Spec> {
        SwVp9segid5FrameLoopfitlerValueEnW::new(self, 11)
    }
    #[doc = "Bits 12:18 - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid5_frame_loopfilter_value(
        &mut self,
    ) -> SwVp9segid5FrameLoopfilterValueW<Swreg25Vp9SegidGrp5Spec> {
        SwVp9segid5FrameLoopfilterValueW::new(self, 12)
    }
    #[doc = "Bit 19 - frame reference info enable\n\nframe reference info enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid5_referinfo_en(
        &mut self,
    ) -> SwVp9segid5ReferinfoEnW<Swreg25Vp9SegidGrp5Spec> {
        SwVp9segid5ReferinfoEnW::new(self, 19)
    }
    #[doc = "Bits 20:21 - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid5_referinfo(&mut self) -> SwVp9segid5ReferinfoW<Swreg25Vp9SegidGrp5Spec> {
        SwVp9segid5ReferinfoW::new(self, 20)
    }
    #[doc = "Bit 22 - frame skip feature enable\n\nframe skip feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid5_frame_skip_en(
        &mut self,
    ) -> SwVp9segid5FrameSkipEnW<Swreg25Vp9SegidGrp5Spec> {
        SwVp9segid5FrameSkipEnW::new(self, 22)
    }
}
#[doc = "vp9 segid syntax grp5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg25_vp9_segid_grp5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg25_vp9_segid_grp5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg25Vp9SegidGrp5Spec;
impl crate::RegisterSpec for Swreg25Vp9SegidGrp5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg25_vp9_segid_grp5::R`](R) reader structure"]
impl crate::Readable for Swreg25Vp9SegidGrp5Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg25_vp9_segid_grp5::W`](W) writer structure"]
impl crate::Writable for Swreg25Vp9SegidGrp5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG25_VP9_SEGID_GRP5 to value 0"]
impl crate::Resettable for Swreg25Vp9SegidGrp5Spec {
    const RESET_VALUE: u32 = 0;
}
