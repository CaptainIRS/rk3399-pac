#[doc = "Register `WDR_CTRL` reader"]
pub type R = crate::R<WdrCtrlSpec>;
#[doc = "Register `WDR_CTRL` writer"]
pub type W = crate::W<WdrCtrlSpec>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdrEnable {
    #[doc = "1: enable WDR"]
    B1 = 1,
    #[doc = "0: bypass WDR *Default*"]
    B0 = 0,
}
impl From<WdrEnable> for bool {
    #[inline(always)]
    fn from(variant: WdrEnable) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDR_ENABLE` reader - "]
pub type WdrEnableR = crate::BitReader<WdrEnable>;
impl WdrEnableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdrEnable {
        match self.bits {
            true => WdrEnable::B1,
            false => WdrEnable::B0,
        }
    }
    #[doc = "enable WDR"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WdrEnable::B1
    }
    #[doc = "bypass WDR *Default*"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WdrEnable::B0
    }
}
#[doc = "Field `WDR_ENABLE` writer - "]
pub type WdrEnableW<'a, REG> = crate::BitWriter<'a, REG, WdrEnable>;
impl<'a, REG> WdrEnableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "enable WDR"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WdrEnable::B1)
    }
    #[doc = "bypass WDR *Default*"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WdrEnable::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdrUseIref {
    #[doc = "1: use Iref (Illumination reference) instead of Y for ToneMapping and Gain calculation"]
    B1 = 1,
    #[doc = "0: use Y for ToneMapping and Gain calculation *Default* Iref is calculated according to the following formula: Iref = (WDR_RGB_FACTOR * RGBmax_tr + (8 - WDR_RGB_FACTOR) * Y) / 8"]
    B0 = 0,
}
impl From<WdrUseIref> for bool {
    #[inline(always)]
    fn from(variant: WdrUseIref) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDR_USE_IREF` reader - "]
pub type WdrUseIrefR = crate::BitReader<WdrUseIref>;
impl WdrUseIrefR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdrUseIref {
        match self.bits {
            true => WdrUseIref::B1,
            false => WdrUseIref::B0,
        }
    }
    #[doc = "use Iref (Illumination reference) instead of Y for ToneMapping and Gain calculation"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WdrUseIref::B1
    }
    #[doc = "use Y for ToneMapping and Gain calculation *Default* Iref is calculated according to the following formula: Iref = (WDR_RGB_FACTOR * RGBmax_tr + (8 - WDR_RGB_FACTOR) * Y) / 8"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WdrUseIref::B0
    }
}
#[doc = "Field `WDR_USE_IREF` writer - "]
pub type WdrUseIrefW<'a, REG> = crate::BitWriter<'a, REG, WdrUseIref>;
impl<'a, REG> WdrUseIrefW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use Iref (Illumination reference) instead of Y for ToneMapping and Gain calculation"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WdrUseIref::B1)
    }
    #[doc = "use Y for ToneMapping and Gain calculation *Default* Iref is calculated according to the following formula: Iref = (WDR_RGB_FACTOR * RGBmax_tr + (8 - WDR_RGB_FACTOR) * Y) / 8"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WdrUseIref::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdrUseY9_8 {
    #[doc = "1: use R G B and Y*9/8 for maximum value calculation (for noise reduction)"]
    B1 = 1,
    #[doc = "0: only use R G B for maximum value calculation (RGBYmax approach) *Default* Use of this bit requires that Iref has been selected, see WDR_USE_IREF."]
    B0 = 0,
}
impl From<WdrUseY9_8> for bool {
    #[inline(always)]
    fn from(variant: WdrUseY9_8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDR_USE_Y9_8` reader - "]
pub type WdrUseY9_8R = crate::BitReader<WdrUseY9_8>;
impl WdrUseY9_8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdrUseY9_8 {
        match self.bits {
            true => WdrUseY9_8::B1,
            false => WdrUseY9_8::B0,
        }
    }
    #[doc = "use R G B and Y*9/8 for maximum value calculation (for noise reduction)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WdrUseY9_8::B1
    }
    #[doc = "only use R G B for maximum value calculation (RGBYmax approach) *Default* Use of this bit requires that Iref has been selected, see WDR_USE_IREF."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WdrUseY9_8::B0
    }
}
#[doc = "Field `WDR_USE_Y9_8` writer - "]
pub type WdrUseY9_8W<'a, REG> = crate::BitWriter<'a, REG, WdrUseY9_8>;
impl<'a, REG> WdrUseY9_8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "use R G B and Y*9/8 for maximum value calculation (for noise reduction)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WdrUseY9_8::B1)
    }
    #[doc = "only use R G B for maximum value calculation (RGBYmax approach) *Default* Use of this bit requires that Iref has been selected, see WDR_USE_IREF."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WdrUseY9_8::B0)
    }
}
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WdrUseRgb7_8 {
    #[doc = "1: decrease RGBmax by 7/8 (for noise reduction)"]
    B1 = 1,
    #[doc = "0: do not modify RGBmax *Default* Use of this bit requires that Iref has been selected, see WDR_USE_IREF."]
    B0 = 0,
}
impl From<WdrUseRgb7_8> for bool {
    #[inline(always)]
    fn from(variant: WdrUseRgb7_8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `WDR_USE_RGB7_8` reader - "]
pub type WdrUseRgb7_8R = crate::BitReader<WdrUseRgb7_8>;
impl WdrUseRgb7_8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> WdrUseRgb7_8 {
        match self.bits {
            true => WdrUseRgb7_8::B1,
            false => WdrUseRgb7_8::B0,
        }
    }
    #[doc = "decrease RGBmax by 7/8 (for noise reduction)"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == WdrUseRgb7_8::B1
    }
    #[doc = "do not modify RGBmax *Default* Use of this bit requires that Iref has been selected, see WDR_USE_IREF."]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == WdrUseRgb7_8::B0
    }
}
#[doc = "Field `WDR_USE_RGB7_8` writer - "]
pub type WdrUseRgb7_8W<'a, REG> = crate::BitWriter<'a, REG, WdrUseRgb7_8>;
impl<'a, REG> WdrUseRgb7_8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "decrease RGBmax by 7/8 (for noise reduction)"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(WdrUseRgb7_8::B1)
    }
    #[doc = "do not modify RGBmax *Default* Use of this bit requires that Iref has been selected, see WDR_USE_IREF."]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(WdrUseRgb7_8::B0)
    }
}
#[doc = "Field `WDR_RGB_FACTOR` reader - rgb_factor defines how much influence the\n\nRGBmax approach has in comparison to Y. The\n\nillumination reference Iref is calculated according to the\n\nfollowing formula:\n\nIref = (WDR_RGB_FACTOR * RGBYmax_tr + (8 - WDR_RGB_FACTOR) * Y) / 8\n\nSo, rgb_factor = 0 means that the standard\n\napproach is used. Use of this factor requires that Iref\n\nhas been selected, see WDR_USE_IREF.\n\nValue range of rgb_factor: 0...8"]
pub type WdrRgbFactorR = crate::FieldReader;
#[doc = "Field `WDR_RGB_FACTOR` writer - rgb_factor defines how much influence the\n\nRGBmax approach has in comparison to Y. The\n\nillumination reference Iref is calculated according to the\n\nfollowing formula:\n\nIref = (WDR_RGB_FACTOR * RGBYmax_tr + (8 - WDR_RGB_FACTOR) * Y) / 8\n\nSo, rgb_factor = 0 means that the standard\n\napproach is used. Use of this factor requires that Iref\n\nhas been selected, see WDR_USE_IREF.\n\nValue range of rgb_factor: 0...8"]
pub type WdrRgbFactorW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn wdr_enable(&self) -> WdrEnableR {
        WdrEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn wdr_use_iref(&self) -> WdrUseIrefR {
        WdrUseIrefR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn wdr_use_y9_8(&self) -> WdrUseY9_8R {
        WdrUseY9_8R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn wdr_use_rgb7_8(&self) -> WdrUseRgb7_8R {
        WdrUseRgb7_8R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:11 - rgb_factor defines how much influence the\n\nRGBmax approach has in comparison to Y. The\n\nillumination reference Iref is calculated according to the\n\nfollowing formula:\n\nIref = (WDR_RGB_FACTOR * RGBYmax_tr + (8 - WDR_RGB_FACTOR) * Y) / 8\n\nSo, rgb_factor = 0 means that the standard\n\napproach is used. Use of this factor requires that Iref\n\nhas been selected, see WDR_USE_IREF.\n\nValue range of rgb_factor: 0...8"]
    #[inline(always)]
    pub fn wdr_rgb_factor(&self) -> WdrRgbFactorR {
        WdrRgbFactorR::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_enable(&mut self) -> WdrEnableW<WdrCtrlSpec> {
        WdrEnableW::new(self, 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_use_iref(&mut self) -> WdrUseIrefW<WdrCtrlSpec> {
        WdrUseIrefW::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_use_y9_8(&mut self) -> WdrUseY9_8W<WdrCtrlSpec> {
        WdrUseY9_8W::new(self, 4)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_use_rgb7_8(&mut self) -> WdrUseRgb7_8W<WdrCtrlSpec> {
        WdrUseRgb7_8W::new(self, 5)
    }
    #[doc = "Bits 8:11 - rgb_factor defines how much influence the\n\nRGBmax approach has in comparison to Y. The\n\nillumination reference Iref is calculated according to the\n\nfollowing formula:\n\nIref = (WDR_RGB_FACTOR * RGBYmax_tr + (8 - WDR_RGB_FACTOR) * Y) / 8\n\nSo, rgb_factor = 0 means that the standard\n\napproach is used. Use of this factor requires that Iref\n\nhas been selected, see WDR_USE_IREF.\n\nValue range of rgb_factor: 0...8"]
    #[inline(always)]
    #[must_use]
    pub fn wdr_rgb_factor(&mut self) -> WdrRgbFactorW<WdrCtrlSpec> {
        WdrRgbFactorW::new(self, 8)
    }
}
#[doc = "Control Bits for Wide Dynamic Range Unit\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wdr_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdr_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdrCtrlSpec;
impl crate::RegisterSpec for WdrCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wdr_ctrl::R`](R) reader structure"]
impl crate::Readable for WdrCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`wdr_ctrl::W`](W) writer structure"]
impl crate::Writable for WdrCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WDR_CTRL to value 0"]
impl crate::Resettable for WdrCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
