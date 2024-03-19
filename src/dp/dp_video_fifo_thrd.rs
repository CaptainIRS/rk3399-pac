#[doc = "Register `DP_VIDEO_FIFO_THRD` reader"]
pub type R = crate::R<DpVideoFifoThrdSpec>;
#[doc = "Register `DP_VIDEO_FIFO_THRD` writer"]
pub type W = crate::W<DpVideoFifoThrdSpec>;
#[doc = "Field `VIDEO_TH_VALUE` reader - Video Data FIFO threshold value. If \n\nVIDEO_TH_CTRL is 1, and data count in video \n\ndata FIFO have reached FIFO threshold value, \n\nvideo data is read out from FIFO."]
pub type VideoThValueR = crate::FieldReader;
#[doc = "Field `VIDEO_TH_VALUE` writer - Video Data FIFO threshold value. If \n\nVIDEO_TH_CTRL is 1, and data count in video \n\ndata FIFO have reached FIFO threshold value, \n\nvideo data is read out from FIFO."]
pub type VideoThValueW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Video Data FIFO threshold control enables.\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VideoThCtrl {
    #[doc = "1: Video Data FIFO threshold uses VIDEO_TH_VALUE."]
    B1 = 1,
    #[doc = "0: Video Data FIFO threshold uses internal calculate value automatically."]
    B0 = 0,
}
impl From<VideoThCtrl> for bool {
    #[inline(always)]
    fn from(variant: VideoThCtrl) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VIDEO_TH_CTRL` reader - Video Data FIFO threshold control enables."]
pub type VideoThCtrlR = crate::BitReader<VideoThCtrl>;
impl VideoThCtrlR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VideoThCtrl {
        match self.bits {
            true => VideoThCtrl::B1,
            false => VideoThCtrl::B0,
        }
    }
    #[doc = "Video Data FIFO threshold uses VIDEO_TH_VALUE."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VideoThCtrl::B1
    }
    #[doc = "Video Data FIFO threshold uses internal calculate value automatically."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VideoThCtrl::B0
    }
}
#[doc = "Field `VIDEO_TH_CTRL` writer - Video Data FIFO threshold control enables."]
pub type VideoThCtrlW<'a, REG> = crate::BitWriter1C<'a, REG, VideoThCtrl>;
impl<'a, REG> VideoThCtrlW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Video Data FIFO threshold uses VIDEO_TH_VALUE."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VideoThCtrl::B1)
    }
    #[doc = "Video Data FIFO threshold uses internal calculate value automatically."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VideoThCtrl::B0)
    }
}
impl R {
    #[doc = "Bits 0:3 - Video Data FIFO threshold value. If \n\nVIDEO_TH_CTRL is 1, and data count in video \n\ndata FIFO have reached FIFO threshold value, \n\nvideo data is read out from FIFO."]
    #[inline(always)]
    pub fn video_th_value(&self) -> VideoThValueR {
        VideoThValueR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Video Data FIFO threshold control enables."]
    #[inline(always)]
    pub fn video_th_ctrl(&self) -> VideoThCtrlR {
        VideoThCtrlR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - Video Data FIFO threshold value. If \n\nVIDEO_TH_CTRL is 1, and data count in video \n\ndata FIFO have reached FIFO threshold value, \n\nvideo data is read out from FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn video_th_value(&mut self) -> VideoThValueW<DpVideoFifoThrdSpec> {
        VideoThValueW::new(self, 0)
    }
    #[doc = "Bit 4 - Video Data FIFO threshold control enables."]
    #[inline(always)]
    #[must_use]
    pub fn video_th_ctrl(&mut self) -> VideoThCtrlW<DpVideoFifoThrdSpec> {
        VideoThCtrlW::new(self, 4)
    }
}
#[doc = "DP FIFO Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_video_fifo_thrd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_video_fifo_thrd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpVideoFifoThrdSpec;
impl crate::RegisterSpec for DpVideoFifoThrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dp_video_fifo_thrd::R`](R) reader structure"]
impl crate::Readable for DpVideoFifoThrdSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_video_fifo_thrd::W`](W) writer structure"]
impl crate::Writable for DpVideoFifoThrdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x10;
}
#[doc = "`reset()` method sets DP_VIDEO_FIFO_THRD to value 0"]
impl crate::Resettable for DpVideoFifoThrdSpec {
    const RESET_VALUE: u32 = 0;
}
