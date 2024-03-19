#[doc = "Register `DDR_DENALI_CTL_257` reader"]
pub type R = crate::R<DdrDenaliCtl257Spec>;
#[doc = "Register `DDR_DENALI_CTL_257` writer"]
pub type W = crate::W<DdrDenaliCtl257Spec>;
#[doc = "Field `CALVL_BG_PAT_0` reader - CA Training pattern 0 driven on the CA bus before and after a calibration command."]
pub type CalvlBgPat0R = crate::FieldReader<u32>;
#[doc = "Field `CALVL_BG_PAT_0` writer - CA Training pattern 0 driven on the CA bus before and after a calibration command."]
pub type CalvlBgPat0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA Training pattern 0 driven on the CA bus before and after a calibration command."]
    #[inline(always)]
    pub fn calvl_bg_pat_0(&self) -> CalvlBgPat0R {
        CalvlBgPat0R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA Training pattern 0 driven on the CA bus before and after a calibration command."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_bg_pat_0(&mut self) -> CalvlBgPat0W<DdrDenaliCtl257Spec> {
        CalvlBgPat0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_257::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_257::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl257Spec;
impl crate::RegisterSpec for DdrDenaliCtl257Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_257::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl257Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_257::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl257Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_257 to value 0"]
impl crate::Resettable for DdrDenaliCtl257Spec {
    const RESET_VALUE: u32 = 0;
}
