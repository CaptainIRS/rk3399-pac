#[doc = "Register `VIDEO_CTL_3` reader"]
pub type R = crate::R<VideoCtl3Spec>;
#[doc = "Register `VIDEO_CTL_3` writer"]
pub type W = crate::W<VideoCtl3Spec>;
#[doc = "Select video format stability check method in video capture block.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VidChkUpdateType {
    #[doc = "1: Check stability with the difference of differences between adjacent frames. Compares difference of 1st and 2nd to difference of 3rd and 4th frame."]
    B1 = 1,
    #[doc = "0: Check stability with the difference of differences between adjacent frames. Compares difference of 1st and 2nd to difference of 3rd and 4th frame."]
    B0 = 0,
}
impl From<VidChkUpdateType> for bool {
    #[inline(always)]
    fn from(variant: VidChkUpdateType) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `VID_CHK_UPDATE_TYPE` reader - Select video format stability check method in video capture block."]
pub type VidChkUpdateTypeR = crate::BitReader<VidChkUpdateType>;
impl VidChkUpdateTypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> VidChkUpdateType {
        match self.bits {
            true => VidChkUpdateType::B1,
            false => VidChkUpdateType::B0,
        }
    }
    #[doc = "Check stability with the difference of differences between adjacent frames. Compares difference of 1st and 2nd to difference of 3rd and 4th frame."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == VidChkUpdateType::B1
    }
    #[doc = "Check stability with the difference of differences between adjacent frames. Compares difference of 1st and 2nd to difference of 3rd and 4th frame."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == VidChkUpdateType::B0
    }
}
#[doc = "Field `VID_CHK_UPDATE_TYPE` writer - Select video format stability check method in video capture block."]
pub type VidChkUpdateTypeW<'a, REG> = crate::BitWriter1C<'a, REG, VidChkUpdateType>;
impl<'a, REG> VidChkUpdateTypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Check stability with the difference of differences between adjacent frames. Compares difference of 1st and 2nd to difference of 3rd and 4th frame."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(VidChkUpdateType::B1)
    }
    #[doc = "Check stability with the difference of differences between adjacent frames. Compares difference of 1st and 2nd to difference of 3rd and 4th frame."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(VidChkUpdateType::B0)
    }
}
#[doc = "YCbCr Coefficients of input video. This is used to specify video data format in main stream attribute data.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InYcCoeffi {
    #[doc = "1: ITU601."]
    B1 = 1,
    #[doc = "0: ITU601."]
    B0 = 0,
}
impl From<InYcCoeffi> for bool {
    #[inline(always)]
    fn from(variant: InYcCoeffi) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `IN_YC_COEFFI` reader - YCbCr Coefficients of input video. This is used to specify video data format in main stream attribute data."]
pub type InYcCoeffiR = crate::BitReader<InYcCoeffi>;
impl InYcCoeffiR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> InYcCoeffi {
        match self.bits {
            true => InYcCoeffi::B1,
            false => InYcCoeffi::B0,
        }
    }
    #[doc = "ITU601."]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == InYcCoeffi::B1
    }
    #[doc = "ITU601."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == InYcCoeffi::B0
    }
}
#[doc = "Field `IN_YC_COEFFI` writer - YCbCr Coefficients of input video. This is used to specify video data format in main stream attribute data."]
pub type InYcCoeffiW<'a, REG> = crate::BitWriter1C<'a, REG, InYcCoeffi>;
impl<'a, REG> InYcCoeffiW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "ITU601."]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(InYcCoeffi::B1)
    }
    #[doc = "ITU601."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(InYcCoeffi::B0)
    }
}
impl R {
    #[doc = "Bit 4 - Select video format stability check method in video capture block."]
    #[inline(always)]
    pub fn vid_chk_update_type(&self) -> VidChkUpdateTypeR {
        VidChkUpdateTypeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - YCbCr Coefficients of input video. This is used to specify video data format in main stream attribute data."]
    #[inline(always)]
    pub fn in_yc_coeffi(&self) -> InYcCoeffiR {
        InYcCoeffiR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - Select video format stability check method in video capture block."]
    #[inline(always)]
    #[must_use]
    pub fn vid_chk_update_type(&mut self) -> VidChkUpdateTypeW<VideoCtl3Spec> {
        VidChkUpdateTypeW::new(self, 4)
    }
    #[doc = "Bit 7 - YCbCr Coefficients of input video. This is used to specify video data format in main stream attribute data."]
    #[inline(always)]
    #[must_use]
    pub fn in_yc_coeffi(&mut self) -> InYcCoeffiW<VideoCtl3Spec> {
        InYcCoeffiW::new(self, 7)
    }
}
#[doc = "Video Control 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`video_ctl_3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`video_ctl_3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VideoCtl3Spec;
impl crate::RegisterSpec for VideoCtl3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`video_ctl_3::R`](R) reader structure"]
impl crate::Readable for VideoCtl3Spec {}
#[doc = "`write(|w| ..)` method takes [`video_ctl_3::W`](W) writer structure"]
impl crate::Writable for VideoCtl3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x90;
}
#[doc = "`reset()` method sets VIDEO_CTL_3 to value 0"]
impl crate::Resettable for VideoCtl3Spec {
    const RESET_VALUE: u32 = 0;
}
