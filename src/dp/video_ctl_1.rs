#[doc = "Register `VIDEO_CTL_1` reader"]
pub type R = crate::R<VideoCtl1Spec>;
#[doc = "Register `VIDEO_CTL_1` writer"]
pub type W = crate::W<VideoCtl1Spec>;
#[doc = "Field `VIDEO_MUTE` reader - Video mute enable. In video mute mode, the solid \n\ncolor, specified in Base + 0x04A8 ~ Base + 0x04B0, \n\nis displayed. \n\n0: Disable, 1: Enable. \n\nOutput video data is changed properly as soon as this \n\nbit is configured."]
pub type VideoMuteR = crate::BitReader;
#[doc = "Field `VIDEO_MUTE` writer - Video mute enable. In video mute mode, the solid \n\ncolor, specified in Base + 0x04A8 ~ Base + 0x04B0, \n\nis displayed. \n\n0: Disable, 1: Enable. \n\nOutput video data is changed properly as soon as this \n\nbit is configured."]
pub type VideoMuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Video data input enable.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VideoEn {
    #[doc = "0: Disable video data input."]
    B0 = 0,
    #[doc = "1: Enable video data input, It takes effect at next video frame."]
    B1 = 1,
}
impl From<VideoEn> for bool {
    #[inline(always)]
    fn from(variant: VideoEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIDEO_EN` reader - Video data input enable."]
pub type VideoEnR = crate::BitReader<VideoEn>;
impl VideoEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VideoEn {
        match self.bits {
            false => VideoEn::B0,
            true => VideoEn::B1,
        }
    }
    #[doc = "Disable video data input."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VideoEn::B0
    }
    #[doc = "Enable video data input, It takes effect at next video frame."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VideoEn::B1
    }
}
#[doc = "Field `VIDEO_EN` writer - Video data input enable."]
pub type VideoEnW<'a, REG> = crate::BitWriter<'a, REG, VideoEn>;
impl<'a, REG> VideoEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable video data input."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VideoEn::B0)
    }
    #[doc = "Enable video data input, It takes effect at next video frame."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VideoEn::B1)
    }
}
impl R {
    #[doc = "Bit 6 - Video mute enable. In video mute mode, the solid \n\ncolor, specified in Base + 0x04A8 ~ Base + 0x04B0, \n\nis displayed. \n\n0: Disable, 1: Enable. \n\nOutput video data is changed properly as soon as this \n\nbit is configured."]
    #[inline(always)]
    pub fn video_mute(&self) -> VideoMuteR {
        VideoMuteR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Video data input enable."]
    #[inline(always)]
    pub fn video_en(&self) -> VideoEnR {
        VideoEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 6 - Video mute enable. In video mute mode, the solid \n\ncolor, specified in Base + 0x04A8 ~ Base + 0x04B0, \n\nis displayed. \n\n0: Disable, 1: Enable. \n\nOutput video data is changed properly as soon as this \n\nbit is configured."]
    #[inline(always)]
    #[must_use]
    pub fn video_mute(&mut self) -> VideoMuteW<VideoCtl1Spec> {
        VideoMuteW::new(self, 6)
    }
    #[doc = "Bit 7 - Video data input enable."]
    #[inline(always)]
    #[must_use]
    pub fn video_en(&mut self) -> VideoEnW<VideoCtl1Spec> {
        VideoEnW::new(self, 7)
    }
}
#[doc = "Video Control 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`video_ctl_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`video_ctl_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VideoCtl1Spec;
impl crate::RegisterSpec for VideoCtl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`video_ctl_1::R`](R) reader structure"]
impl crate::Readable for VideoCtl1Spec {}
#[doc = "`write(|w| ..)` method takes [`video_ctl_1::W`](W) writer structure"]
impl crate::Writable for VideoCtl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VIDEO_CTL_1 to value 0"]
impl crate::Resettable for VideoCtl1Spec {
    const RESET_VALUE: u32 = 0;
}
