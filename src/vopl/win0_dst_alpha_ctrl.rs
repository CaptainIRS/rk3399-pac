#[doc = "Register `WIN0_DST_ALPHA_CTRL` reader"]
pub type R = crate::R<Win0DstAlphaCtrlSpec>;
#[doc = "Register `WIN0_DST_ALPHA_CTRL` writer"]
pub type W = crate::W<Win0DstAlphaCtrlSpec>;
#[doc = "Field `WIN0_DST_FACTOR_MODE` reader - dst factor mode"]
pub type Win0DstFactorModeR = crate::FieldReader;
#[doc = "Field `WIN0_DST_FACTOR_MODE` writer - dst factor mode"]
pub type Win0DstFactorModeW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 6:8 - dst factor mode"]
    #[inline(always)]
    pub fn win0_dst_factor_mode(&self) -> Win0DstFactorModeR {
        Win0DstFactorModeR::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 6:8 - dst factor mode"]
    #[inline(always)]
    #[must_use]
    pub fn win0_dst_factor_mode(&mut self) -> Win0DstFactorModeW<Win0DstAlphaCtrlSpec> {
        Win0DstFactorModeW::new(self, 6)
    }
}
#[doc = "Win0 alpha destination control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win0_dst_alpha_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win0_dst_alpha_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win0DstAlphaCtrlSpec;
impl crate::RegisterSpec for Win0DstAlphaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win0_dst_alpha_ctrl::R`](R) reader structure"]
impl crate::Readable for Win0DstAlphaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`win0_dst_alpha_ctrl::W`](W) writer structure"]
impl crate::Writable for Win0DstAlphaCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN0_DST_ALPHA_CTRL to value 0"]
impl crate::Resettable for Win0DstAlphaCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
