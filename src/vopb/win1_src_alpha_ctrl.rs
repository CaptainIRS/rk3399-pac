#[doc = "Register `WIN1_SRC_ALPHA_CTRL` reader"]
pub type R = crate::R<Win1SrcAlphaCtrlSpec>;
#[doc = "Register `WIN1_SRC_ALPHA_CTRL` writer"]
pub type W = crate::W<Win1SrcAlphaCtrlSpec>;
#[doc = "Field `WIN1_SRC_ALPHA_EN` reader - src alpha en"]
pub type Win1SrcAlphaEnR = crate::BitReader;
#[doc = "Field `WIN1_SRC_ALPHA_EN` writer - src alpha en"]
pub type Win1SrcAlphaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN1_SRC_COLOR_MODE` reader - src color mode"]
pub type Win1SrcColorModeR = crate::BitReader;
#[doc = "Field `WIN1_SRC_COLOR_MODE` writer - src color mode"]
pub type Win1SrcColorModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN1_SRC_ALPHA_MODE` reader - src alpha mode"]
pub type Win1SrcAlphaModeR = crate::BitReader;
#[doc = "Field `WIN1_SRC_ALPHA_MODE` writer - src alpha mode"]
pub type Win1SrcAlphaModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN1_SRC_BLEND_MODE` reader - src blend mode"]
pub type Win1SrcBlendModeR = crate::FieldReader;
#[doc = "Field `WIN1_SRC_BLEND_MODE` writer - src blend mode"]
pub type Win1SrcBlendModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WIN1_SRC_ALPHA_CAL_MODE` reader - src alpha calc mode"]
pub type Win1SrcAlphaCalModeR = crate::BitReader;
#[doc = "Field `WIN1_SRC_ALPHA_CAL_MODE` writer - src alpha calc mode"]
pub type Win1SrcAlphaCalModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN1_SRC_FACTOR_MODE` reader - src factor mode"]
pub type Win1SrcFactorModeR = crate::FieldReader;
#[doc = "Field `WIN1_SRC_FACTOR_MODE` writer - src factor mode"]
pub type Win1SrcFactorModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WIN1_SRC_GLOBAL_ALPHA` reader - src global alpha"]
pub type Win1SrcGlobalAlphaR = crate::FieldReader;
#[doc = "Field `WIN1_SRC_GLOBAL_ALPHA` writer - src global alpha"]
pub type Win1SrcGlobalAlphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN1_FADING_VALUE` reader - fading value,8bit"]
pub type Win1FadingValueR = crate::FieldReader;
#[doc = "Field `WIN1_FADING_VALUE` writer - fading value,8bit"]
pub type Win1FadingValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - src alpha en"]
    #[inline(always)]
    pub fn win1_src_alpha_en(&self) -> Win1SrcAlphaEnR {
        Win1SrcAlphaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - src color mode"]
    #[inline(always)]
    pub fn win1_src_color_mode(&self) -> Win1SrcColorModeR {
        Win1SrcColorModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - src alpha mode"]
    #[inline(always)]
    pub fn win1_src_alpha_mode(&self) -> Win1SrcAlphaModeR {
        Win1SrcAlphaModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - src blend mode"]
    #[inline(always)]
    pub fn win1_src_blend_mode(&self) -> Win1SrcBlendModeR {
        Win1SrcBlendModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - src alpha calc mode"]
    #[inline(always)]
    pub fn win1_src_alpha_cal_mode(&self) -> Win1SrcAlphaCalModeR {
        Win1SrcAlphaCalModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - src factor mode"]
    #[inline(always)]
    pub fn win1_src_factor_mode(&self) -> Win1SrcFactorModeR {
        Win1SrcFactorModeR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 16:23 - src global alpha"]
    #[inline(always)]
    pub fn win1_src_global_alpha(&self) -> Win1SrcGlobalAlphaR {
        Win1SrcGlobalAlphaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - fading value,8bit"]
    #[inline(always)]
    pub fn win1_fading_value(&self) -> Win1FadingValueR {
        Win1FadingValueR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - src alpha en"]
    #[inline(always)]
    #[must_use]
    pub fn win1_src_alpha_en(&mut self) -> Win1SrcAlphaEnW<Win1SrcAlphaCtrlSpec> {
        Win1SrcAlphaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - src color mode"]
    #[inline(always)]
    #[must_use]
    pub fn win1_src_color_mode(&mut self) -> Win1SrcColorModeW<Win1SrcAlphaCtrlSpec> {
        Win1SrcColorModeW::new(self, 1)
    }
    #[doc = "Bit 2 - src alpha mode"]
    #[inline(always)]
    #[must_use]
    pub fn win1_src_alpha_mode(&mut self) -> Win1SrcAlphaModeW<Win1SrcAlphaCtrlSpec> {
        Win1SrcAlphaModeW::new(self, 2)
    }
    #[doc = "Bits 3:4 - src blend mode"]
    #[inline(always)]
    #[must_use]
    pub fn win1_src_blend_mode(&mut self) -> Win1SrcBlendModeW<Win1SrcAlphaCtrlSpec> {
        Win1SrcBlendModeW::new(self, 3)
    }
    #[doc = "Bit 5 - src alpha calc mode"]
    #[inline(always)]
    #[must_use]
    pub fn win1_src_alpha_cal_mode(&mut self) -> Win1SrcAlphaCalModeW<Win1SrcAlphaCtrlSpec> {
        Win1SrcAlphaCalModeW::new(self, 5)
    }
    #[doc = "Bits 6:8 - src factor mode"]
    #[inline(always)]
    #[must_use]
    pub fn win1_src_factor_mode(&mut self) -> Win1SrcFactorModeW<Win1SrcAlphaCtrlSpec> {
        Win1SrcFactorModeW::new(self, 6)
    }
    #[doc = "Bits 16:23 - src global alpha"]
    #[inline(always)]
    #[must_use]
    pub fn win1_src_global_alpha(&mut self) -> Win1SrcGlobalAlphaW<Win1SrcAlphaCtrlSpec> {
        Win1SrcGlobalAlphaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - fading value,8bit"]
    #[inline(always)]
    #[must_use]
    pub fn win1_fading_value(&mut self) -> Win1FadingValueW<Win1SrcAlphaCtrlSpec> {
        Win1FadingValueW::new(self, 24)
    }
}
#[doc = "Win1 alpha source control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_src_alpha_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_src_alpha_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1SrcAlphaCtrlSpec;
impl crate::RegisterSpec for Win1SrcAlphaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_src_alpha_ctrl::R`](R) reader structure"]
impl crate::Readable for Win1SrcAlphaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`win1_src_alpha_ctrl::W`](W) writer structure"]
impl crate::Writable for Win1SrcAlphaCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_SRC_ALPHA_CTRL to value 0"]
impl crate::Resettable for Win1SrcAlphaCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
