#[doc = "Register `SWREG33_VP9_INFO_LASTFRAME` reader"]
pub type R = crate::R<Swreg33Vp9InfoLastframeSpec>;
#[doc = "Register `SWREG33_VP9_INFO_LASTFRAME` writer"]
pub type W = crate::W<Swreg33Vp9InfoLastframeSpec>;
#[doc = "Field `SW_VP9_MODE_DELTAS_LASTFRAME` reader - vp9 mode deltas\n\nvp9 mode deltas\n\nit is for last dec frame"]
pub type SwVp9ModeDeltasLastframeR = crate::FieldReader<u16>;
#[doc = "Field `SW_VP9_MODE_DELTAS_LASTFRAME` writer - vp9 mode deltas\n\nvp9 mode deltas\n\nit is for last dec frame"]
pub type SwVp9ModeDeltasLastframeW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `SW_SEGMENTATION_ENABLE_LSTFRAME` reader - segmentation_enable for last frame\n\n1'b1:sw_segmentation_enable for last frame\n\nit is for last_dec_frame"]
pub type SwSegmentationEnableLstframeR = crate::BitReader;
#[doc = "Field `SW_SEGMENTATION_ENABLE_LSTFRAME` writer - segmentation_enable for last frame\n\n1'b1:sw_segmentation_enable for last frame\n\nit is for last_dec_frame"]
pub type SwSegmentationEnableLstframeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9_LAST_SHOW_FRAME` reader - last show frame flag\n\nfor cal the flag use_prev_in_find_mv_refs which is to inter cmd\n\nit is for last_dec_frame"]
pub type SwVp9LastShowFrameR = crate::BitReader;
#[doc = "Field `SW_VP9_LAST_SHOW_FRAME` writer - last show frame flag\n\nfor cal the flag use_prev_in_find_mv_refs which is to inter cmd\n\nit is for last_dec_frame"]
pub type SwVp9LastShowFrameW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9_LAST_INTRA_ONLY` reader - last frame intra only flag\n\nvp9 last frame intra only flag\n\nto give inter command use\n\nit is for last_dec_frame"]
pub type SwVp9LastIntraOnlyR = crate::BitReader;
#[doc = "Field `SW_VP9_LAST_INTRA_ONLY` writer - last frame intra only flag\n\nvp9 last frame intra only flag\n\nto give inter command use\n\nit is for last_dec_frame"]
pub type SwVp9LastIntraOnlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9_LAST_WIDHHEIGHT_EQCUR` reader - last width and height equal cur\n\nlast width and height equal cur frame"]
pub type SwVp9LastWidhheightEqcurR = crate::BitReader;
#[doc = "Field `SW_VP9_LAST_WIDHHEIGHT_EQCUR` writer - last width and height equal cur\n\nlast width and height equal cur frame"]
pub type SwVp9LastWidhheightEqcurW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_VP9_COLOR_SPACE_LASTKEYFRAME` reader - vp9 last keyframe color_space\n\nvp9 last keyframe color_space"]
pub type SwVp9ColorSpaceLastkeyframeR = crate::FieldReader;
#[doc = "Field `SW_VP9_COLOR_SPACE_LASTKEYFRAME` writer - vp9 last keyframe color_space\n\nvp9 last keyframe color_space"]
pub type SwVp9ColorSpaceLastkeyframeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:13 - vp9 mode deltas\n\nvp9 mode deltas\n\nit is for last dec frame"]
    #[inline(always)]
    pub fn sw_vp9_mode_deltas_lastframe(&self) -> SwVp9ModeDeltasLastframeR {
        SwVp9ModeDeltasLastframeR::new((self.bits & 0x3fff) as u16)
    }
    #[doc = "Bit 16 - segmentation_enable for last frame\n\n1'b1:sw_segmentation_enable for last frame\n\nit is for last_dec_frame"]
    #[inline(always)]
    pub fn sw_segmentation_enable_lstframe(&self) -> SwSegmentationEnableLstframeR {
        SwSegmentationEnableLstframeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - last show frame flag\n\nfor cal the flag use_prev_in_find_mv_refs which is to inter cmd\n\nit is for last_dec_frame"]
    #[inline(always)]
    pub fn sw_vp9_last_show_frame(&self) -> SwVp9LastShowFrameR {
        SwVp9LastShowFrameR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - last frame intra only flag\n\nvp9 last frame intra only flag\n\nto give inter command use\n\nit is for last_dec_frame"]
    #[inline(always)]
    pub fn sw_vp9_last_intra_only(&self) -> SwVp9LastIntraOnlyR {
        SwVp9LastIntraOnlyR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - last width and height equal cur\n\nlast width and height equal cur frame"]
    #[inline(always)]
    pub fn sw_vp9_last_widhheight_eqcur(&self) -> SwVp9LastWidhheightEqcurR {
        SwVp9LastWidhheightEqcurR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - vp9 last keyframe color_space\n\nvp9 last keyframe color_space"]
    #[inline(always)]
    pub fn sw_vp9_color_space_lastkeyframe(&self) -> SwVp9ColorSpaceLastkeyframeR {
        SwVp9ColorSpaceLastkeyframeR::new(((self.bits >> 20) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:13 - vp9 mode deltas\n\nvp9 mode deltas\n\nit is for last dec frame"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_mode_deltas_lastframe(
        &mut self,
    ) -> SwVp9ModeDeltasLastframeW<Swreg33Vp9InfoLastframeSpec> {
        SwVp9ModeDeltasLastframeW::new(self, 0)
    }
    #[doc = "Bit 16 - segmentation_enable for last frame\n\n1'b1:sw_segmentation_enable for last frame\n\nit is for last_dec_frame"]
    #[inline(always)]
    #[must_use]
    pub fn sw_segmentation_enable_lstframe(
        &mut self,
    ) -> SwSegmentationEnableLstframeW<Swreg33Vp9InfoLastframeSpec> {
        SwSegmentationEnableLstframeW::new(self, 16)
    }
    #[doc = "Bit 17 - last show frame flag\n\nfor cal the flag use_prev_in_find_mv_refs which is to inter cmd\n\nit is for last_dec_frame"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_last_show_frame(&mut self) -> SwVp9LastShowFrameW<Swreg33Vp9InfoLastframeSpec> {
        SwVp9LastShowFrameW::new(self, 17)
    }
    #[doc = "Bit 18 - last frame intra only flag\n\nvp9 last frame intra only flag\n\nto give inter command use\n\nit is for last_dec_frame"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_last_intra_only(&mut self) -> SwVp9LastIntraOnlyW<Swreg33Vp9InfoLastframeSpec> {
        SwVp9LastIntraOnlyW::new(self, 18)
    }
    #[doc = "Bit 19 - last width and height equal cur\n\nlast width and height equal cur frame"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_last_widhheight_eqcur(
        &mut self,
    ) -> SwVp9LastWidhheightEqcurW<Swreg33Vp9InfoLastframeSpec> {
        SwVp9LastWidhheightEqcurW::new(self, 19)
    }
    #[doc = "Bits 20:22 - vp9 last keyframe color_space\n\nvp9 last keyframe color_space"]
    #[inline(always)]
    #[must_use]
    pub fn sw_vp9_color_space_lastkeyframe(
        &mut self,
    ) -> SwVp9ColorSpaceLastkeyframeW<Swreg33Vp9InfoLastframeSpec> {
        SwVp9ColorSpaceLastkeyframeW::new(self, 20)
    }
}
#[doc = "vp9 info for lastframe\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`swreg33_vp9_info_lastframe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`swreg33_vp9_info_lastframe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swreg33Vp9InfoLastframeSpec;
impl crate::RegisterSpec for Swreg33Vp9InfoLastframeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swreg33_vp9_info_lastframe::R`](R) reader structure"]
impl crate::Readable for Swreg33Vp9InfoLastframeSpec {}
#[doc = "`write(|w| ..)` method takes [`swreg33_vp9_info_lastframe::W`](W) writer structure"]
impl crate::Writable for Swreg33Vp9InfoLastframeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SWREG33_VP9_INFO_LASTFRAME to value 0"]
impl crate::Resettable for Swreg33Vp9InfoLastframeSpec {
    const RESET_VALUE: u32 = 0;
}
