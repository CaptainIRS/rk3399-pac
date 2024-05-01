#[doc = "Register `HWC_SRC_ALPHA_CTRL` reader"]
pub type R = crate::R<HwcSrcAlphaCtrlSpec>;
#[doc = "Register `HWC_SRC_ALPHA_CTRL` writer"]
pub type W = crate::W<HwcSrcAlphaCtrlSpec>;
#[doc = "Field `HWC_SRC_ALPHA_EN` reader - src alpha enable"]
pub type HwcSrcAlphaEnR = crate::BitReader;
#[doc = "Field `HWC_SRC_ALPHA_EN` writer - src alpha enable"]
pub type HwcSrcAlphaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWC_SRC_COLOR_MODE` reader - src color mode"]
pub type HwcSrcColorModeR = crate::BitReader;
#[doc = "Field `HWC_SRC_COLOR_MODE` writer - src color mode"]
pub type HwcSrcColorModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWC_SRC_ALPHA_MODE` reader - src alpha mode"]
pub type HwcSrcAlphaModeR = crate::BitReader;
#[doc = "Field `HWC_SRC_ALPHA_MODE` writer - src alpha mode"]
pub type HwcSrcAlphaModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWC_SRC_BLEND_MODE` reader - src blend mode"]
pub type HwcSrcBlendModeR = crate::FieldReader;
#[doc = "Field `HWC_SRC_BLEND_MODE` writer - src blend mode"]
pub type HwcSrcBlendModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HWC_SRC_ALPHA_CAL_MODE` reader - src alpha calc mode"]
pub type HwcSrcAlphaCalModeR = crate::BitReader;
#[doc = "Field `HWC_SRC_ALPHA_CAL_MODE` writer - src alpha calc mode"]
pub type HwcSrcAlphaCalModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HWC_SRC_FACTOR_MODE` reader - src factor mode"]
pub type HwcSrcFactorModeR = crate::FieldReader;
#[doc = "Field `HWC_SRC_FACTOR_MODE` writer - src factor mode"]
pub type HwcSrcFactorModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HWC_SRC_GLOBAL_ALPHA` reader - src global alpha"]
pub type HwcSrcGlobalAlphaR = crate::FieldReader;
#[doc = "Field `HWC_SRC_GLOBAL_ALPHA` writer - src global alpha"]
pub type HwcSrcGlobalAlphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HWC_FADING_VALUE` reader - fading value"]
pub type HwcFadingValueR = crate::FieldReader;
#[doc = "Field `HWC_FADING_VALUE` writer - fading value"]
pub type HwcFadingValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - src alpha enable"]
    #[inline(always)]
    pub fn hwc_src_alpha_en(&self) -> HwcSrcAlphaEnR {
        HwcSrcAlphaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - src color mode"]
    #[inline(always)]
    pub fn hwc_src_color_mode(&self) -> HwcSrcColorModeR {
        HwcSrcColorModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - src alpha mode"]
    #[inline(always)]
    pub fn hwc_src_alpha_mode(&self) -> HwcSrcAlphaModeR {
        HwcSrcAlphaModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - src blend mode"]
    #[inline(always)]
    pub fn hwc_src_blend_mode(&self) -> HwcSrcBlendModeR {
        HwcSrcBlendModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - src alpha calc mode"]
    #[inline(always)]
    pub fn hwc_src_alpha_cal_mode(&self) -> HwcSrcAlphaCalModeR {
        HwcSrcAlphaCalModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - src factor mode"]
    #[inline(always)]
    pub fn hwc_src_factor_mode(&self) -> HwcSrcFactorModeR {
        HwcSrcFactorModeR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 16:23 - src global alpha"]
    #[inline(always)]
    pub fn hwc_src_global_alpha(&self) -> HwcSrcGlobalAlphaR {
        HwcSrcGlobalAlphaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - fading value"]
    #[inline(always)]
    pub fn hwc_fading_value(&self) -> HwcFadingValueR {
        HwcFadingValueR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - src alpha enable"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_src_alpha_en(&mut self) -> HwcSrcAlphaEnW<HwcSrcAlphaCtrlSpec> {
        HwcSrcAlphaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - src color mode"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_src_color_mode(&mut self) -> HwcSrcColorModeW<HwcSrcAlphaCtrlSpec> {
        HwcSrcColorModeW::new(self, 1)
    }
    #[doc = "Bit 2 - src alpha mode"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_src_alpha_mode(&mut self) -> HwcSrcAlphaModeW<HwcSrcAlphaCtrlSpec> {
        HwcSrcAlphaModeW::new(self, 2)
    }
    #[doc = "Bits 3:4 - src blend mode"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_src_blend_mode(&mut self) -> HwcSrcBlendModeW<HwcSrcAlphaCtrlSpec> {
        HwcSrcBlendModeW::new(self, 3)
    }
    #[doc = "Bit 5 - src alpha calc mode"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_src_alpha_cal_mode(&mut self) -> HwcSrcAlphaCalModeW<HwcSrcAlphaCtrlSpec> {
        HwcSrcAlphaCalModeW::new(self, 5)
    }
    #[doc = "Bits 6:8 - src factor mode"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_src_factor_mode(&mut self) -> HwcSrcFactorModeW<HwcSrcAlphaCtrlSpec> {
        HwcSrcFactorModeW::new(self, 6)
    }
    #[doc = "Bits 16:23 - src global alpha"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_src_global_alpha(&mut self) -> HwcSrcGlobalAlphaW<HwcSrcAlphaCtrlSpec> {
        HwcSrcGlobalAlphaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - fading value"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_fading_value(&mut self) -> HwcFadingValueW<HwcSrcAlphaCtrlSpec> {
        HwcFadingValueW::new(self, 24)
    }
}
#[doc = "Hwc alpha source control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_src_alpha_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_src_alpha_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwcSrcAlphaCtrlSpec;
impl crate::RegisterSpec for HwcSrcAlphaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwc_src_alpha_ctrl::R`](R) reader structure"]
impl crate::Readable for HwcSrcAlphaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hwc_src_alpha_ctrl::W`](W) writer structure"]
impl crate::Writable for HwcSrcAlphaCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWC_SRC_ALPHA_CTRL to value 0"]
impl crate::Resettable for HwcSrcAlphaCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
