#[doc = "Register `WIN2_SRC_ALPHA_CTRL` reader"]
pub type R = crate::R<Win2SrcAlphaCtrlSpec>;
#[doc = "Register `WIN2_SRC_ALPHA_CTRL` writer"]
pub type W = crate::W<Win2SrcAlphaCtrlSpec>;
#[doc = "Field `WIN2_SRC_ALPHA_EN` reader - src alpha en"]
pub type Win2SrcAlphaEnR = crate::BitReader;
#[doc = "Field `WIN2_SRC_ALPHA_EN` writer - src alpha en"]
pub type Win2SrcAlphaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN2_SRC_COLOR_MODE` reader - src color mode"]
pub type Win2SrcColorModeR = crate::BitReader;
#[doc = "Field `WIN2_SRC_COLOR_MODE` writer - src color mode"]
pub type Win2SrcColorModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN2_SRC_ALPHA_MODE` reader - src alpha mode"]
pub type Win2SrcAlphaModeR = crate::BitReader;
#[doc = "Field `WIN2_SRC_ALPHA_MODE` writer - src alpha mode"]
pub type Win2SrcAlphaModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN2_SRC_BLEND_MODE` reader - src blend mode"]
pub type Win2SrcBlendModeR = crate::FieldReader;
#[doc = "Field `WIN2_SRC_BLEND_MODE` writer - src blend mode"]
pub type Win2SrcBlendModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WIN2_SRC_ALPHA_CAL_MODE` reader - src alpha cal mode"]
pub type Win2SrcAlphaCalModeR = crate::BitReader;
#[doc = "Field `WIN2_SRC_ALPHA_CAL_MODE` writer - src alpha cal mode"]
pub type Win2SrcAlphaCalModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN2_SRC_FACTOR_MODE` reader - src factor mode"]
pub type Win2SrcFactorModeR = crate::FieldReader;
#[doc = "Field `WIN2_SRC_FACTOR_MODE` writer - src factor mode"]
pub type Win2SrcFactorModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WIN2_SRC_GLOBAL_ALPHA` reader - src global alpha"]
pub type Win2SrcGlobalAlphaR = crate::FieldReader;
#[doc = "Field `WIN2_SRC_GLOBAL_ALPHA` writer - src global alpha"]
pub type Win2SrcGlobalAlphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN2_FADING_VALUE` reader - fading value,8bits"]
pub type Win2FadingValueR = crate::FieldReader;
#[doc = "Field `WIN2_FADING_VALUE` writer - fading value,8bits"]
pub type Win2FadingValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - src alpha en"]
    #[inline(always)]
    pub fn win2_src_alpha_en(&self) -> Win2SrcAlphaEnR {
        Win2SrcAlphaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - src color mode"]
    #[inline(always)]
    pub fn win2_src_color_mode(&self) -> Win2SrcColorModeR {
        Win2SrcColorModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - src alpha mode"]
    #[inline(always)]
    pub fn win2_src_alpha_mode(&self) -> Win2SrcAlphaModeR {
        Win2SrcAlphaModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - src blend mode"]
    #[inline(always)]
    pub fn win2_src_blend_mode(&self) -> Win2SrcBlendModeR {
        Win2SrcBlendModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - src alpha cal mode"]
    #[inline(always)]
    pub fn win2_src_alpha_cal_mode(&self) -> Win2SrcAlphaCalModeR {
        Win2SrcAlphaCalModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - src factor mode"]
    #[inline(always)]
    pub fn win2_src_factor_mode(&self) -> Win2SrcFactorModeR {
        Win2SrcFactorModeR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 16:23 - src global alpha"]
    #[inline(always)]
    pub fn win2_src_global_alpha(&self) -> Win2SrcGlobalAlphaR {
        Win2SrcGlobalAlphaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - fading value,8bits"]
    #[inline(always)]
    pub fn win2_fading_value(&self) -> Win2FadingValueR {
        Win2FadingValueR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - src alpha en"]
    #[inline(always)]
    #[must_use]
    pub fn win2_src_alpha_en(&mut self) -> Win2SrcAlphaEnW<Win2SrcAlphaCtrlSpec> {
        Win2SrcAlphaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - src color mode"]
    #[inline(always)]
    #[must_use]
    pub fn win2_src_color_mode(&mut self) -> Win2SrcColorModeW<Win2SrcAlphaCtrlSpec> {
        Win2SrcColorModeW::new(self, 1)
    }
    #[doc = "Bit 2 - src alpha mode"]
    #[inline(always)]
    #[must_use]
    pub fn win2_src_alpha_mode(&mut self) -> Win2SrcAlphaModeW<Win2SrcAlphaCtrlSpec> {
        Win2SrcAlphaModeW::new(self, 2)
    }
    #[doc = "Bits 3:4 - src blend mode"]
    #[inline(always)]
    #[must_use]
    pub fn win2_src_blend_mode(&mut self) -> Win2SrcBlendModeW<Win2SrcAlphaCtrlSpec> {
        Win2SrcBlendModeW::new(self, 3)
    }
    #[doc = "Bit 5 - src alpha cal mode"]
    #[inline(always)]
    #[must_use]
    pub fn win2_src_alpha_cal_mode(&mut self) -> Win2SrcAlphaCalModeW<Win2SrcAlphaCtrlSpec> {
        Win2SrcAlphaCalModeW::new(self, 5)
    }
    #[doc = "Bits 6:8 - src factor mode"]
    #[inline(always)]
    #[must_use]
    pub fn win2_src_factor_mode(&mut self) -> Win2SrcFactorModeW<Win2SrcAlphaCtrlSpec> {
        Win2SrcFactorModeW::new(self, 6)
    }
    #[doc = "Bits 16:23 - src global alpha"]
    #[inline(always)]
    #[must_use]
    pub fn win2_src_global_alpha(&mut self) -> Win2SrcGlobalAlphaW<Win2SrcAlphaCtrlSpec> {
        Win2SrcGlobalAlphaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - fading value,8bits"]
    #[inline(always)]
    #[must_use]
    pub fn win2_fading_value(&mut self) -> Win2FadingValueW<Win2SrcAlphaCtrlSpec> {
        Win2FadingValueW::new(self, 24)
    }
}
#[doc = "Win2 alpha source control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win2_src_alpha_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win2_src_alpha_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win2SrcAlphaCtrlSpec;
impl crate::RegisterSpec for Win2SrcAlphaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win2_src_alpha_ctrl::R`](R) reader structure"]
impl crate::Readable for Win2SrcAlphaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`win2_src_alpha_ctrl::W`](W) writer structure"]
impl crate::Writable for Win2SrcAlphaCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN2_SRC_ALPHA_CTRL to value 0"]
impl crate::Resettable for Win2SrcAlphaCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
