#[doc = "Register `WIN0_SRC_ALPHA_CTRL` reader"]
pub type R = crate::R<Win0SrcAlphaCtrlSpec>;
#[doc = "Register `WIN0_SRC_ALPHA_CTRL` writer"]
pub type W = crate::W<Win0SrcAlphaCtrlSpec>;
#[doc = "Field `WIN0_SRC_ALPHA_EN` reader - src alpha en"]
pub type Win0SrcAlphaEnR = crate::BitReader;
#[doc = "Field `WIN0_SRC_ALPHA_EN` writer - src alpha en"]
pub type Win0SrcAlphaEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN0_SRC_COLOR_MODE` reader - src color mode"]
pub type Win0SrcColorModeR = crate::BitReader;
#[doc = "Field `WIN0_SRC_COLOR_MODE` writer - src color mode"]
pub type Win0SrcColorModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN0_SRC_ALPHA_MODE` reader - src alpha mode"]
pub type Win0SrcAlphaModeR = crate::BitReader;
#[doc = "Field `WIN0_SRC_ALPHA_MODE` writer - src alpha mode"]
pub type Win0SrcAlphaModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN0_SRC_BLEND_MODE` reader - src blend mode"]
pub type Win0SrcBlendModeR = crate::FieldReader;
#[doc = "Field `WIN0_SRC_BLEND_MODE` writer - src blend mode"]
pub type Win0SrcBlendModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WIN0_SRC_ALPHA_CAL_MODE` reader - src alpha calc mode"]
pub type Win0SrcAlphaCalModeR = crate::BitReader;
#[doc = "Field `WIN0_SRC_ALPHA_CAL_MODE` writer - src alpha calc mode"]
pub type Win0SrcAlphaCalModeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WIN0_SRC_FACTOR_MODE` reader - src factor mode"]
pub type Win0SrcFactorModeR = crate::FieldReader;
#[doc = "Field `WIN0_SRC_FACTOR_MODE` writer - src factor mode"]
pub type Win0SrcFactorModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WIN0_SRC_GLOBAL_ALPHA` reader - src global alpha"]
pub type Win0SrcGlobalAlphaR = crate::FieldReader;
#[doc = "Field `WIN0_SRC_GLOBAL_ALPHA` writer - src global alpha"]
pub type Win0SrcGlobalAlphaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WIN0_FADING_VALUE` reader - win0 fading value ,8bits"]
pub type Win0FadingValueR = crate::FieldReader;
#[doc = "Field `WIN0_FADING_VALUE` writer - win0 fading value ,8bits"]
pub type Win0FadingValueW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - src alpha en"]
    #[inline(always)]
    pub fn win0_src_alpha_en(&self) -> Win0SrcAlphaEnR {
        Win0SrcAlphaEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - src color mode"]
    #[inline(always)]
    pub fn win0_src_color_mode(&self) -> Win0SrcColorModeR {
        Win0SrcColorModeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - src alpha mode"]
    #[inline(always)]
    pub fn win0_src_alpha_mode(&self) -> Win0SrcAlphaModeR {
        Win0SrcAlphaModeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - src blend mode"]
    #[inline(always)]
    pub fn win0_src_blend_mode(&self) -> Win0SrcBlendModeR {
        Win0SrcBlendModeR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - src alpha calc mode"]
    #[inline(always)]
    pub fn win0_src_alpha_cal_mode(&self) -> Win0SrcAlphaCalModeR {
        Win0SrcAlphaCalModeR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:8 - src factor mode"]
    #[inline(always)]
    pub fn win0_src_factor_mode(&self) -> Win0SrcFactorModeR {
        Win0SrcFactorModeR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 16:23 - src global alpha"]
    #[inline(always)]
    pub fn win0_src_global_alpha(&self) -> Win0SrcGlobalAlphaR {
        Win0SrcGlobalAlphaR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - win0 fading value ,8bits"]
    #[inline(always)]
    pub fn win0_fading_value(&self) -> Win0FadingValueR {
        Win0FadingValueR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - src alpha en"]
    #[inline(always)]
    #[must_use]
    pub fn win0_src_alpha_en(&mut self) -> Win0SrcAlphaEnW<Win0SrcAlphaCtrlSpec> {
        Win0SrcAlphaEnW::new(self, 0)
    }
    #[doc = "Bit 1 - src color mode"]
    #[inline(always)]
    #[must_use]
    pub fn win0_src_color_mode(&mut self) -> Win0SrcColorModeW<Win0SrcAlphaCtrlSpec> {
        Win0SrcColorModeW::new(self, 1)
    }
    #[doc = "Bit 2 - src alpha mode"]
    #[inline(always)]
    #[must_use]
    pub fn win0_src_alpha_mode(&mut self) -> Win0SrcAlphaModeW<Win0SrcAlphaCtrlSpec> {
        Win0SrcAlphaModeW::new(self, 2)
    }
    #[doc = "Bits 3:4 - src blend mode"]
    #[inline(always)]
    #[must_use]
    pub fn win0_src_blend_mode(&mut self) -> Win0SrcBlendModeW<Win0SrcAlphaCtrlSpec> {
        Win0SrcBlendModeW::new(self, 3)
    }
    #[doc = "Bit 5 - src alpha calc mode"]
    #[inline(always)]
    #[must_use]
    pub fn win0_src_alpha_cal_mode(&mut self) -> Win0SrcAlphaCalModeW<Win0SrcAlphaCtrlSpec> {
        Win0SrcAlphaCalModeW::new(self, 5)
    }
    #[doc = "Bits 6:8 - src factor mode"]
    #[inline(always)]
    #[must_use]
    pub fn win0_src_factor_mode(&mut self) -> Win0SrcFactorModeW<Win0SrcAlphaCtrlSpec> {
        Win0SrcFactorModeW::new(self, 6)
    }
    #[doc = "Bits 16:23 - src global alpha"]
    #[inline(always)]
    #[must_use]
    pub fn win0_src_global_alpha(&mut self) -> Win0SrcGlobalAlphaW<Win0SrcAlphaCtrlSpec> {
        Win0SrcGlobalAlphaW::new(self, 16)
    }
    #[doc = "Bits 24:31 - win0 fading value ,8bits"]
    #[inline(always)]
    #[must_use]
    pub fn win0_fading_value(&mut self) -> Win0FadingValueW<Win0SrcAlphaCtrlSpec> {
        Win0FadingValueW::new(self, 24)
    }
}
#[doc = "Win0 alpha source control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_src_alpha_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_src_alpha_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0SrcAlphaCtrlSpec;
impl crate::RegisterSpec for Win0SrcAlphaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_src_alpha_ctrl::R`](R) reader structure"]
impl crate::Readable for Win0SrcAlphaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`win0_src_alpha_ctrl::W`](W) writer structure"]
impl crate::Writable for Win0SrcAlphaCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_SRC_ALPHA_CTRL to value 0"]
impl crate::Resettable for Win0SrcAlphaCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
