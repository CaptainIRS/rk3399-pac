#[doc = "Register `VIDEO_CTL_8` reader"]
pub type R = crate::R<VideoCtl8Spec>;
#[doc = "Register `VIDEO_CTL_8` writer"]
pub type W = crate::W<VideoCtl8Spec>;
#[doc = "Field `VID_VRES_TH` reader - Video Frame Vertical Resolution variation threshold for video capture block. This bit field is used by CAPTURE block to determine whether STRM_VALID should be asserted."]
pub type VidVresThR = crate::FieldReader;
#[doc = "Field `VID_VRES_TH` writer - Video Frame Vertical Resolution variation threshold for video capture block. This bit field is used by CAPTURE block to determine whether STRM_VALID should be asserted."]
pub type VidVresThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `VID_HRES_TH` reader - Video Frame Horizontal Resolution variation threshold for video capture block. This bit field is used by CAPTURE block to determine whether STRM_VALID should be asserted."]
pub type VidHresThR = crate::FieldReader;
#[doc = "Field `VID_HRES_TH` writer - Video Frame Horizontal Resolution variation threshold for video capture block. This bit field is used by CAPTURE block to determine whether STRM_VALID should be asserted."]
pub type VidHresThW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Video Frame Vertical Resolution variation threshold for video capture block. This bit field is used by CAPTURE block to determine whether STRM_VALID should be asserted."]
    #[inline(always)]
    pub fn vid_vres_th(&self) -> VidVresThR {
        VidVresThR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Video Frame Horizontal Resolution variation threshold for video capture block. This bit field is used by CAPTURE block to determine whether STRM_VALID should be asserted."]
    #[inline(always)]
    pub fn vid_hres_th(&self) -> VidHresThR {
        VidHresThR::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Video Frame Vertical Resolution variation threshold for video capture block. This bit field is used by CAPTURE block to determine whether STRM_VALID should be asserted."]
    #[inline(always)]
    #[must_use]
    pub fn vid_vres_th(&mut self) -> VidVresThW<VideoCtl8Spec> {
        VidVresThW::new(self, 0)
    }
    #[doc = "Bits 4:7 - Video Frame Horizontal Resolution variation threshold for video capture block. This bit field is used by CAPTURE block to determine whether STRM_VALID should be asserted."]
    #[inline(always)]
    #[must_use]
    pub fn vid_hres_th(&mut self) -> VidHresThW<VideoCtl8Spec> {
        VidHresThW::new(self, 4)
    }
}
#[doc = "Video Control 8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`video_ctl_8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`video_ctl_8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VideoCtl8Spec;
impl crate::RegisterSpec for VideoCtl8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`video_ctl_8::R`](R) reader structure"]
impl crate::Readable for VideoCtl8Spec {}
#[doc = "`write(|w| ..)` method takes [`video_ctl_8::W`](W) writer structure"]
impl crate::Writable for VideoCtl8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VIDEO_CTL_8 to value 0x20"]
impl crate::Resettable for VideoCtl8Spec {
    const RESET_VALUE: u32 = 0x20;
}
