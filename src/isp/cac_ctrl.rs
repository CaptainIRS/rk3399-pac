#[doc = "Register `CAC_CTRL` reader"]
pub type R = crate::R<CacCtrlSpec>;
#[doc = "Register `CAC_CTRL` writer"]
pub type W = crate::W<CacCtrlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CacEn {
    #[doc = "0: chromatic aberration correction off"]
    B0 = 0,
    #[doc = "1: chromatic aberration correction on"]
    B1 = 1,
}
impl From<CacEn> for bool {
    #[inline(always)]
    fn from(variant: CacEn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `cac_en` reader - "]
pub type CacEnR = crate::BitReader<CacEn>;
impl CacEnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CacEn {
        match self.bits {
            false => CacEn::B0,
            true => CacEn::B1,
        }
    }
    #[doc = "chromatic aberration correction off"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == CacEn::B0
    }
    #[doc = "chromatic aberration correction on"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == CacEn::B1
    }
}
#[doc = "Field `cac_en` writer - "]
pub type CacEnW<'a, REG> = crate::BitWriter<'a, REG, CacEn>;
impl<'a, REG> CacEnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "chromatic aberration correction off"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(CacEn::B0)
    }
    #[doc = "chromatic aberration correction on"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(CacEn::B1)
    }
}
#[doc = "Defines the maximum red/blue pixel shift in vertical\n\ndirection 00: Set vertical vector clipping to +/-2 pixel ; fix filter_enable (Default)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum VClipMode {
    #[doc = "1: Set vertical vector clipping to +/-3 pixel; dynamic filter_enable for chroma low pass filter"]
    B01 = 1,
    #[doc = "2: Set vertical vector clipping +/-3 or +/-4 pixel displacement depending on pixel position inside the Bayer raster (dynamic switching between +/-3 and +/-4)"]
    B10 = 2,
}
impl From<VClipMode> for u8 {
    #[inline(always)]
    fn from(variant: VClipMode) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for VClipMode {
    type Ux = u8;
}
#[doc = "Field `v_clip_mode` reader - Defines the maximum red/blue pixel shift in vertical\n\ndirection 00: Set vertical vector clipping to +/-2 pixel ; fix filter_enable (Default)"]
pub type VClipModeR = crate::FieldReader<VClipMode>;
impl VClipModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<VClipMode> {
        match self.bits {
            1 => Some(VClipMode::B01),
            2 => Some(VClipMode::B10),
            _ => None,
        }
    }
    #[doc = "Set vertical vector clipping to +/-3 pixel; dynamic filter_enable for chroma low pass filter"]
    #[inline(always)]
    pub fn is_b01(&self) -> bool {
        *self == VClipMode::B01
    }
    #[doc = "Set vertical vector clipping +/-3 or +/-4 pixel displacement depending on pixel position inside the Bayer raster (dynamic switching between +/-3 and +/-4)"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == VClipMode::B10
    }
}
#[doc = "Field `v_clip_mode` writer - Defines the maximum red/blue pixel shift in vertical\n\ndirection 00: Set vertical vector clipping to +/-2 pixel ; fix filter_enable (Default)"]
pub type VClipModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, VClipMode>;
impl<'a, REG> VClipModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Set vertical vector clipping to +/-3 pixel; dynamic filter_enable for chroma low pass filter"]
    #[inline(always)]
    pub fn b01(self) -> &'a mut crate::W<REG> {
        self.variant(VClipMode::B01)
    }
    #[doc = "Set vertical vector clipping +/-3 or +/-4 pixel displacement depending on pixel position inside the Bayer raster (dynamic switching between +/-3 and +/-4)"]
    #[inline(always)]
    pub fn b10(self) -> &'a mut crate::W<REG> {
        self.variant(VClipMode::B10)
    }
}
#[doc = "Defines the maximum red/blue pixel shift in horizontal\n\ndirection At pixel positions, that require a larger\n\ndisplacement, the maximum shift value is used instead\n\n(vector clipping)\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HClipMode {
    #[doc = "0: Set horizontal vector clipping to +/-4 pixel displacement (Default)"]
    B0 = 0,
    #[doc = "1: Set horizontal vector clipping to +/-4 or +/-5 pixel displacement depending on pixel position inside the Bayer raster (dynamic switching between +/-4 and +/-5)"]
    B1 = 1,
}
impl From<HClipMode> for bool {
    #[inline(always)]
    fn from(variant: HClipMode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `h_clip_mode` reader - Defines the maximum red/blue pixel shift in horizontal\n\ndirection At pixel positions, that require a larger\n\ndisplacement, the maximum shift value is used instead\n\n(vector clipping)"]
pub type HClipModeR = crate::BitReader<HClipMode>;
impl HClipModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> HClipMode {
        match self.bits {
            false => HClipMode::B0,
            true => HClipMode::B1,
        }
    }
    #[doc = "Set horizontal vector clipping to +/-4 pixel displacement (Default)"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == HClipMode::B0
    }
    #[doc = "Set horizontal vector clipping to +/-4 or +/-5 pixel displacement depending on pixel position inside the Bayer raster (dynamic switching between +/-4 and +/-5)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == HClipMode::B1
    }
}
#[doc = "Field `h_clip_mode` writer - Defines the maximum red/blue pixel shift in horizontal\n\ndirection At pixel positions, that require a larger\n\ndisplacement, the maximum shift value is used instead\n\n(vector clipping)"]
pub type HClipModeW<'a, REG> = crate::BitWriter<'a, REG, HClipMode>;
impl<'a, REG> HClipModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Set horizontal vector clipping to +/-4 pixel displacement (Default)"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(HClipMode::B0)
    }
    #[doc = "Set horizontal vector clipping to +/-4 or +/-5 pixel displacement depending on pixel position inside the Bayer raster (dynamic switching between +/-4 and +/-5)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(HClipMode::B1)
    }
}
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cac_en(&self) -> CacEnR {
        CacEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Defines the maximum red/blue pixel shift in vertical\n\ndirection 00: Set vertical vector clipping to +/-2 pixel ; fix filter_enable (Default)"]
    #[inline(always)]
    pub fn v_clip_mode(&self) -> VClipModeR {
        VClipModeR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - Defines the maximum red/blue pixel shift in horizontal\n\ndirection At pixel positions, that require a larger\n\ndisplacement, the maximum shift value is used instead\n\n(vector clipping)"]
    #[inline(always)]
    pub fn h_clip_mode(&self) -> HClipModeR {
        HClipModeR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn cac_en(&mut self) -> CacEnW<CacCtrlSpec> {
        CacEnW::new(self, 0)
    }
    #[doc = "Bits 1:2 - Defines the maximum red/blue pixel shift in vertical\n\ndirection 00: Set vertical vector clipping to +/-2 pixel ; fix filter_enable (Default)"]
    #[inline(always)]
    #[must_use]
    pub fn v_clip_mode(&mut self) -> VClipModeW<CacCtrlSpec> {
        VClipModeW::new(self, 1)
    }
    #[doc = "Bit 3 - Defines the maximum red/blue pixel shift in horizontal\n\ndirection At pixel positions, that require a larger\n\ndisplacement, the maximum shift value is used instead\n\n(vector clipping)"]
    #[inline(always)]
    #[must_use]
    pub fn h_clip_mode(&mut self) -> HClipModeW<CacCtrlSpec> {
        HClipModeW::new(self, 3)
    }
}
#[doc = "Control register for chromatic aberration correction\n\nNote: Clipping behavior can be controlled by clip_mode bits. If no clipping occurs, because \n\ndisplacement is below the maximum correctable displacement, then it does not matter which \n\nmode is selected. \n\n\n\n \n\n\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cac_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cac_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CacCtrlSpec;
impl crate::RegisterSpec for CacCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cac_ctrl::R`](R) reader structure"]
impl crate::Readable for CacCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`cac_ctrl::W`](W) writer structure"]
impl crate::Writable for CacCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAC_CTRL to value 0"]
impl crate::Resettable for CacCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
