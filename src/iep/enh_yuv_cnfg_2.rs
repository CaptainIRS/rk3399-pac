#[doc = "Register `ENH_YUV_CNFG_2` reader"]
pub type R = crate::R<EnhYuvCnfg2Spec>;
#[doc = "Register `ENH_YUV_CNFG_2` writer"]
pub type W = crate::W<EnhYuvCnfg2Spec>;
#[doc = "Field `COLOR_BAR_Y` reader - color bar y value"]
pub type ColorBarYR = crate::FieldReader;
#[doc = "Field `COLOR_BAR_Y` writer - color bar y value"]
pub type ColorBarYW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLOR_BAR_U` reader - color bar u value"]
pub type ColorBarUR = crate::FieldReader;
#[doc = "Field `COLOR_BAR_U` writer - color bar u value"]
pub type ColorBarUW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `COLOR_BAR_V` reader - color bar v value"]
pub type ColorBarVR = crate::FieldReader;
#[doc = "Field `COLOR_BAR_V` writer - color bar v value"]
pub type ColorBarVW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "video mode\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VideoMode {
    #[doc = "0: black screen"]
    B00 = 0,
    #[doc = "1: blue screen"]
    B01 = 1,
    #[doc = "2: color bars"]
    B10 = 2,
    #[doc = "3: normal video"]
    B11 = 3,
}
impl From<VideoMode> for u8 {
    #[inline(always)]
    fn from(variant: VideoMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VideoMode {
    type Ux = u8;
}
#[doc = "Field `VIDEO_MODE` reader - video mode"]
pub type VideoModeR = crate::FieldReader<VideoMode>;
impl VideoModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VideoMode {
        match self.bits {
            0 => VideoMode::B00,
            1 => VideoMode::B01,
            2 => VideoMode::B10,
            3 => VideoMode::B11,
            _ => unreachable!(),
        }
    }
    #[doc = "black screen"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == VideoMode::B00
    }
    #[doc = "blue screen"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == VideoMode::B01
    }
    #[doc = "color bars"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == VideoMode::B10
    }
    #[doc = "normal video"]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == VideoMode::B11
    }
}
#[doc = "Field `VIDEO_MODE` writer - video mode"]
pub type VideoModeW<'a, REG> = crate::FieldWriterSafe<'a, REG, 2, VideoMode>;
impl<'a, REG> VideoModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "black screen"]
    #[inline(always)]
    pub fn b00(self) -> &'a mut crate::W<REG> {
        self.variant(VideoMode::B00)
    }
    #[doc = "blue screen"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(VideoMode::B01)
    }
    #[doc = "color bars"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(VideoMode::B10)
    }
    #[doc = "normal video"]
    #[inline(always)]
    pub fn b11(self) -> &'a mut crate::W<REG> {
        self.variant(VideoMode::B11)
    }
}
impl R {
    #[doc = "Bits 0:7 - color bar y value"]
    #[inline(always)]
    pub fn color_bar_y(&self) -> ColorBarYR {
        ColorBarYR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - color bar u value"]
    #[inline(always)]
    pub fn color_bar_u(&self) -> ColorBarUR {
        ColorBarUR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - color bar v value"]
    #[inline(always)]
    pub fn color_bar_v(&self) -> ColorBarVR {
        ColorBarVR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:25 - video mode"]
    #[inline(always)]
    pub fn video_mode(&self) -> VideoModeR {
        VideoModeR::new(((self.bits >> 24) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - color bar y value"]
    #[inline(always)]
    #[must_use]
    pub fn color_bar_y(&mut self) -> ColorBarYW<EnhYuvCnfg2Spec> {
        ColorBarYW::new(self, 0)
    }
    #[doc = "Bits 8:15 - color bar u value"]
    #[inline(always)]
    #[must_use]
    pub fn color_bar_u(&mut self) -> ColorBarUW<EnhYuvCnfg2Spec> {
        ColorBarUW::new(self, 8)
    }
    #[doc = "Bits 16:23 - color bar v value"]
    #[inline(always)]
    #[must_use]
    pub fn color_bar_v(&mut self) -> ColorBarVW<EnhYuvCnfg2Spec> {
        ColorBarVW::new(self, 16)
    }
    #[doc = "Bits 24:25 - video mode"]
    #[inline(always)]
    #[must_use]
    pub fn video_mode(&mut self) -> VideoModeW<EnhYuvCnfg2Spec> {
        VideoModeW::new(self, 24)
    }
}
#[doc = "color bar configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`enh_yuv_cnfg_2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`enh_yuv_cnfg_2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EnhYuvCnfg2Spec;
impl crate::RegisterSpec for EnhYuvCnfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`enh_yuv_cnfg_2::R`](R) reader structure"]
impl crate::Readable for EnhYuvCnfg2Spec {}
#[doc = "`write(|w| ..)` method takes [`enh_yuv_cnfg_2::W`](W) writer structure"]
impl crate::Writable for EnhYuvCnfg2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ENH_YUV_CNFG_2 to value 0"]
impl crate::Resettable for EnhYuvCnfg2Spec {
    const RESET_VALUE: u32 = 0;
}
