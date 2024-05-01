#[doc = "Register `WIN1_DST_ALPHA_CTRL` reader"]
pub type R = crate::R<Win1DstAlphaCtrlSpec>;
#[doc = "Register `WIN1_DST_ALPHA_CTRL` writer"]
pub type W = crate::W<Win1DstAlphaCtrlSpec>;
#[doc = "Field `WIN1_DST_FACTOR_M0` reader - dst factor mode"]
pub type Win1DstFactorM0R = crate::FieldReader;
#[doc = "Field `WIN1_DST_FACTOR_M0` writer - dst factor mode"]
pub type Win1DstFactorM0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 6:8 - dst factor mode"]
    #[inline(always)]
    pub fn win1_dst_factor_m0(&self) -> Win1DstFactorM0R {
        Win1DstFactorM0R::new(((self.bits >> 6) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 6:8 - dst factor mode"]
    #[inline(always)]
    #[must_use]
    pub fn win1_dst_factor_m0(&mut self) -> Win1DstFactorM0W<Win1DstAlphaCtrlSpec> {
        Win1DstFactorM0W::new(self, 6)
    }
}
#[doc = "Win1 alpha destination control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`win1_dst_alpha_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`win1_dst_alpha_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Win1DstAlphaCtrlSpec;
impl crate::RegisterSpec for Win1DstAlphaCtrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`win1_dst_alpha_ctrl::R`](R) reader structure"]
impl crate::Readable for Win1DstAlphaCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`win1_dst_alpha_ctrl::W`](W) writer structure"]
impl crate::Writable for Win1DstAlphaCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WIN1_DST_ALPHA_CTRL to value 0"]
impl crate::Resettable for Win1DstAlphaCtrlSpec {
    const RESET_VALUE: u32 = 0;
}
