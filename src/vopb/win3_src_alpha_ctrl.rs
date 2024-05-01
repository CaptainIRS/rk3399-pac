#[doc = "Register `WIN3_SRC_ALPHA_CTRL` reader"]
pub type R = crate::R<Win3SrcAlphaCtrlSpec>;
#[doc = "Register `WIN3_SRC_ALPHA_CTRL` writer"]
pub type W = crate::W<Win3SrcAlphaCtrlSpec>;
#[doc = "Field `WIN3_SRC_ALPHA_EN` reader - src alpha en"]
pub type Win3SrcAlphaEnR = crate::BitReader;
#[doc = "Field `WIN3_SRC_ALPHA_EN` writer - src alpha en"]
pub type Win3SrcAlphaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN3_SRC_COLOR_MODE` reader - src color mode"]
pub type Win3SrcColorModeR = crate::BitReader;
#[doc = "Field `WIN3_SRC_COLOR_MODE` writer - src color mode"]
pub type Win3SrcColorModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN3_SRC_ALPHA_MODE` reader - src alpha mode"]
pub type Win3SrcAlphaModeR = crate::BitReader;
#[doc = "Field `WIN3_SRC_ALPHA_MODE` writer - src alpha mode"]
pub type Win3SrcAlphaModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN3_SRC_BLEND_MODE` reader - src blend mode"]
pub type Win3SrcBlendModeR = crate::FieldReader;
#[doc = "Field `WIN3_SRC_BLEND_MODE` writer - src blend mode"]
pub type Win3SrcBlendModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WIN3_SRC_ALPHA_CAL_MODE` reader - src alpha cal mode"]
pub type Win3SrcAlphaCalModeR = crate::BitReader;
#[doc = "Field `WIN3_SRC_ALPHA_CAL_MODE` writer - src alpha cal mode"]
pub type Win3SrcAlphaCalModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN3_SRC_FACTOR_MODE` reader - src factor mode"]
pub type Win3SrcFactorModeR = crate::FieldReader;
#[doc = "Field `WIN3_SRC_FACTOR_MODE` writer - src factor mode"]
pub type Win3SrcFactorModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WIN3_SRC_GLOBAL_ALPHA` reader - src global alpha"]
pub type Win3SrcGlobalAlphaR = crate::FieldReader;
#[doc = "Field `WIN3_SRC_GLOBAL_ALPHA` writer - src global alpha"]
pub type Win3SrcGlobalAlphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN3_FADING_VALUE` reader - fading value"]
pub type Win3FadingValueR = crate::FieldReader;
#[doc = "Field `WIN3_FADING_VALUE` writer - fading value"]
pub type Win3FadingValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - src alpha en"]
    #[inline(always)]
    pub fn win3_src_alpha_en(&self) -> Win3SrcAlphaEnR {
        Win3SrcAlphaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - src color mode"]
    #[inline(always)]
    pub fn win3_src_color_mode(&self) -> Win3SrcColorModeR {
        Win3SrcColorModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - src alpha mode"]
    #[inline(always)]
    pub fn win3_src_alpha_mode(&self) -> Win3SrcAlphaModeR {
        Win3SrcAlphaModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - src blend mode"]
    #[inline(always)]
    pub fn win3_src_blend_mode(&self) -> Win3SrcBlendModeR {
        Win3SrcBlendModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - src alpha cal mode"]
    #[inline(always)]
    pub fn win3_src_alpha_cal_mode(&self) -> Win3SrcAlphaCalModeR {
        Win3SrcAlphaCalModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - src factor mode"]
    #[inline(always)]
    pub fn win3_src_factor_mode(&self) -> Win3SrcFactorModeR {
        Win3SrcFactorModeR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 16:23 - src global alpha"]
    #[inline(always)]
    pub fn win3_src_global_alpha(&self) -> Win3SrcGlobalAlphaR {
        Win3SrcGlobalAlphaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - fading value"]
    #[inline(always)]
    pub fn win3_fading_value(&self) -> Win3FadingValueR {
        Win3FadingValueR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - src alpha en"]
    #[inline(always)]
    #[must_use]
    pub fn win3_src_alpha_en(&mut self) -> Win3SrcAlphaEnW<Win3SrcAlphaCtrlSpec> {
        Win3SrcAlphaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - src color mode"]
    #[inline(always)]
    #[must_use]
    pub fn win3_src_color_mode(&mut self) -> Win3SrcColorModeW<Win3SrcAlphaCtrlSpec> {
        Win3SrcColorModeW::new(self, 1)
    }
    #[doc = "Bit 2 - src alpha mode"]
    #[inline(always)]
    #[must_use]
    pub fn win3_src_alpha_mode(&mut self) -> Win3SrcAlphaModeW<Win3SrcAlphaCtrlSpec> {
        Win3SrcAlphaModeW::new(self, 2)
    }
    #[doc = "Bits 3:4 - src blend mode"]
    #[inline(always)]
    #[must_use]
    pub fn win3_src_blend_mode(&mut self) -> Win3SrcBlendModeW<Win3SrcAlphaCtrlSpec> {
        Win3SrcBlendModeW::new(self, 3)
    }
    #[doc = "Bit 5 - src alpha cal mode"]
    #[inline(always)]
    #[must_use]
    pub fn win3_src_alpha_cal_mode(&mut self) -> Win3SrcAlphaCalModeW<Win3SrcAlphaCtrlSpec> {
        Win3SrcAlphaCalModeW::new(self, 5)
    }
    #[doc = "Bits 6:8 - src factor mode"]
    #[inline(always)]
    #[must_use]
    pub fn win3_src_factor_mode(&mut self) -> Win3SrcFactorModeW<Win3SrcAlphaCtrlSpec> {
        Win3SrcFactorModeW::new(self, 6)
    }
    #[doc = "Bits 16:23 - src global alpha"]
    #[inline(always)]
    #[must_use]
    pub fn win3_src_global_alpha(&mut self) -> Win3SrcGlobalAlphaW<Win3SrcAlphaCtrlSpec> {
        Win3SrcGlobalAlphaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - fading value"]
    #[inline(always)]
    #[must_use]
    pub fn win3_fading_value(&mut self) -> Win3FadingValueW<Win3SrcAlphaCtrlSpec> {
        Win3FadingValueW::new(self, 24)
    }
}
#[doc = "Win3 alpha source control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win3_src_alpha_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win3_src_alpha_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win3SrcAlphaCtrlSpec;
impl crate::RegisterSpec for Win3SrcAlphaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win3_src_alpha_ctrl::R`](R) reader structure"]
impl crate::Readable for Win3SrcAlphaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`win3_src_alpha_ctrl::W`](W) writer structure"]
impl crate::Writable for Win3SrcAlphaCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN3_SRC_ALPHA_CTRL to value 0"]
impl crate::Resettable for Win3SrcAlphaCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
