#[doc = "Register `HWC_DST_ALPHA_CTRL` reader"]
pub type R = crate::R<HwcDstAlphaCtrlSpec>;
#[doc = "Register `HWC_DST_ALPHA_CTRL` writer"]
pub type W = crate::W<HwcDstAlphaCtrlSpec>;
#[doc = "Field `HWC_DST_FACTOR_MODE` reader - dst factor mode"]
pub type HwcDstFactorModeR = crate::FieldReader;
#[doc = "Field `HWC_DST_FACTOR_MODE` writer - dst factor mode"]
pub type HwcDstFactorModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 6:8 - dst factor mode"]
    #[inline(always)]
    pub fn hwc_dst_factor_mode(&self) -> HwcDstFactorModeR {
        HwcDstFactorModeR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 6:8 - dst factor mode"]
    #[inline(always)]
    #[must_use]
    pub fn hwc_dst_factor_mode(&mut self) -> HwcDstFactorModeW<HwcDstAlphaCtrlSpec> {
        HwcDstFactorModeW::new(self, 6)
    }
}
#[doc = "Hwc alpha destination control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hwc_dst_alpha_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hwc_dst_alpha_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HwcDstAlphaCtrlSpec;
impl crate::RegisterSpec for HwcDstAlphaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hwc_dst_alpha_ctrl::R`](R) reader structure"]
impl crate::Readable for HwcDstAlphaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`hwc_dst_alpha_ctrl::W`](W) writer structure"]
impl crate::Writable for HwcDstAlphaCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HWC_DST_ALPHA_CTRL to value 0"]
impl crate::Resettable for HwcDstAlphaCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
