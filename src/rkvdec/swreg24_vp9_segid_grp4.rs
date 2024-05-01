#[doc = "Register `SWREG24_VP9_SEGID_GRP4` reader"]
pub type R = crate::R<Swreg24Vp9SegidGrp4Spec>;
#[doc = "Register `SWREG24_VP9_SEGID_GRP4` writer"]
pub type W = crate::W<Swreg24Vp9SegidGrp4Spec>;
#[doc = "Field `SW_VP9SEGID4_FRAME_QP_DELTA_EN` reader - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
pub type SwVp9segid4FrameQpDeltaEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID4_FRAME_QP_DELTA_EN` writer - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
pub type SwVp9segid4FrameQpDeltaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID4_FRAME_QP_DELTA` reader - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
pub type SwVp9segid4FrameQpDeltaR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9SEGID4_FRAME_QP_DELTA` writer - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
pub type SwVp9segid4FrameQpDeltaW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SW_VP9SEGID4_FRAME_LOOPFITLER_VALUE_EN` reader - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
pub type SwVp9segid4FrameLoopfitlerValueEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID4_FRAME_LOOPFITLER_VALUE_EN` writer - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
pub type SwVp9segid4FrameLoopfitlerValueEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID4_FRAME_LOOPFILTER_VALUE` reader - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
pub type SwVp9segid4FrameLoopfilterValueR = crate::FieldReader;
#[doc = "Field `SW_VP9SEGID4_FRAME_LOOPFILTER_VALUE` writer - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
pub type SwVp9segid4FrameLoopfilterValueW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `SW_VP9SEGID4_REFERINFO_EN` reader - frame reference info enable\n\nframe reference info enable"]
pub type SwVp9segid4ReferinfoEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID4_REFERINFO_EN` writer - frame reference info enable\n\nframe reference info enable"]
pub type SwVp9segid4ReferinfoEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9SEGID4_REFERINFO` reader - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
pub type SwVp9segid4ReferinfoR = crate::FieldReader;
#[doc = "Field `SW_VP9SEGID4_REFERINFO` writer - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
pub type SwVp9segid4ReferinfoW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_VP9SEGID4_FRAME_SKIP_EN` reader - frame skip feature enable\n\nframe skip feature enable"]
pub type SwVp9segid4FrameSkipEnR = crate::BitReader;
#[doc = "Field `SW_VP9SEGID4_FRAME_SKIP_EN` writer - frame skip feature enable\n\nframe skip feature enable"]
pub type SwVp9segid4FrameSkipEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
    #[inline(always)]
    pub fn sw_vp9segid4_frame_qp_delta_en(&self) -> SwVp9segid4FrameQpDeltaEnR {
        SwVp9segid4FrameQpDeltaEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:10 - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
    #[inline(always)]
    pub fn sw_vp9segid4_frame_qp_delta(&self) -> SwVp9segid4FrameQpDeltaR {
        SwVp9segid4FrameQpDeltaR::new(((self.bits >> 2) & 0x01ff) as u16)
    }
    #[doc = "Bit 11 - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
    #[inline(always)]
    pub fn sw_vp9segid4_frame_loopfitler_value_en(&self) -> SwVp9segid4FrameLoopfitlerValueEnR {
        SwVp9segid4FrameLoopfitlerValueEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:18 - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
    #[inline(always)]
    pub fn sw_vp9segid4_frame_loopfilter_value(&self) -> SwVp9segid4FrameLoopfilterValueR {
        SwVp9segid4FrameLoopfilterValueR::new(((self.bits >> 12) & 0x7f) as u8)
    }
    #[doc = "Bit 19 - frame reference info enable\n\nframe reference info enable"]
    #[inline(always)]
    pub fn sw_vp9segid4_referinfo_en(&self) -> SwVp9segid4ReferinfoEnR {
        SwVp9segid4ReferinfoEnR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:21 - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
    #[inline(always)]
    pub fn sw_vp9segid4_referinfo(&self) -> SwVp9segid4ReferinfoR {
        SwVp9segid4ReferinfoR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - frame skip feature enable\n\nframe skip feature enable"]
    #[inline(always)]
    pub fn sw_vp9segid4_frame_skip_en(&self) -> SwVp9segid4FrameSkipEnR {
        SwVp9segid4FrameSkipEnR::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - frame_qp_delta feature enable\n\nframe_qp_delta feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid4_frame_qp_delta_en(
        &mut self,
    ) -> SwVp9segid4FrameQpDeltaEnW<Swreg24Vp9SegidGrp4Spec> {
        SwVp9segid4FrameQpDeltaEnW::new(self, 1)
    }
    #[doc = "Bits 2:10 - frame qp delta\n\nspecifies segment i's qp_delta value which is used to calculate\n\ny_dequant and uv_dequant"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid4_frame_qp_delta(
        &mut self,
    ) -> SwVp9segid4FrameQpDeltaW<Swreg24Vp9SegidGrp4Spec> {
        SwVp9segid4FrameQpDeltaW::new(self, 2)
    }
    #[doc = "Bit 11 - frame_loopfilter_value feature enable\n\nframe_loopfilter_value feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid4_frame_loopfitler_value_en(
        &mut self,
    ) -> SwVp9segid4FrameLoopfitlerValueEnW<Swreg24Vp9SegidGrp4Spec> {
        SwVp9segid4FrameLoopfitlerValueEnW::new(self, 11)
    }
    #[doc = "Bits 12:18 - frame loopfitler value\n\nspecifies segment i's loopfilter_delta value which is used to\n\ncalculate filter level"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid4_frame_loopfilter_value(
        &mut self,
    ) -> SwVp9segid4FrameLoopfilterValueW<Swreg24Vp9SegidGrp4Spec> {
        SwVp9segid4FrameLoopfilterValueW::new(self, 12)
    }
    #[doc = "Bit 19 - frame reference info enable\n\nframe reference info enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid4_referinfo_en(
        &mut self,
    ) -> SwVp9segid4ReferinfoEnW<Swreg24Vp9SegidGrp4Spec> {
        SwVp9segid4ReferinfoEnW::new(self, 19)
    }
    #[doc = "Bits 20:21 - specifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]\n\nspecifies segment i's reference_info which is used to get\n\nref_frame\\[0\\]"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid4_referinfo(&mut self) -> SwVp9segid4ReferinfoW<Swreg24Vp9SegidGrp4Spec> {
        SwVp9segid4ReferinfoW::new(self, 20)
    }
    #[doc = "Bit 22 - frame skip feature enable\n\nframe skip feature enable"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9segid4_frame_skip_en(
        &mut self,
    ) -> SwVp9segid4FrameSkipEnW<Swreg24Vp9SegidGrp4Spec> {
        SwVp9segid4FrameSkipEnW::new(self, 22)
    }
}
#[doc = "vp9 segid syntax grp4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg24_vp9_segid_grp4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg24_vp9_segid_grp4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg24Vp9SegidGrp4Spec;
impl crate::RegisterSpec for Swreg24Vp9SegidGrp4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg24_vp9_segid_grp4::R`](R) reader structure"]
impl crate::Readable for Swreg24Vp9SegidGrp4Spec {}
#[doc = "`write(|w| ..)` method takes [`swreg24_vp9_segid_grp4::W`](W) writer structure"]
impl crate::Writable for Swreg24Vp9SegidGrp4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG24_VP9_SEGID_GRP4 to value 0"]
impl crate::Resettable for Swreg24Vp9SegidGrp4Spec {
    const RESET_VALUE: u32 = 0;
}
