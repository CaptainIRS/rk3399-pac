#[doc = "Register `RAW_ENH_YUV_CNFG_2` reader"]
pub type R = crate::R<RawEnhYuvCnfg2Spec>;
#[doc = "Field `COLOR_BAR_Y` reader - color bar y value"]
pub type ColorBarYR = crate::FieldReader;
#[doc = "Field `COLOR_BAR_U` reader - color bar u value"]
pub type ColorBarUR = crate::FieldReader;
#[doc = "Field `COLOR_BAR_V` reader - color bar v value"]
pub type ColorBarVR = crate::FieldReader;
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
#[doc = "color bar configuration\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_enh_yuv_cnfg_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RawEnhYuvCnfg2Spec;
impl crate::RegisterSpec for RawEnhYuvCnfg2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_enh_yuv_cnfg_2::R`](R) reader structure"]
impl crate::Readable for RawEnhYuvCnfg2Spec {}
#[doc = "`reset()` method sets RAW_ENH_YUV_CNFG_2 to value 0"]
impl crate::Resettable for RawEnhYuvCnfg2Spec {
    const RESET_VALUE: u32 = 0;
}
