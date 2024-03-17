#[doc = "Register `DENALI_CTL_259` reader"]
pub type R = crate::R<DenaliCtl259Spec>;
#[doc = "Register `DENALI_CTL_259` writer"]
pub type W = crate::W<DenaliCtl259Spec>;
#[doc = "Field `CALVL_BG_PAT_1` reader - CA Training pattern 1 driven on the CA bus before and after a calibration command."]
pub type CalvlBgPat1R = crate::FieldReader<u32>;
#[doc = "Field `CALVL_BG_PAT_1` writer - CA Training pattern 1 driven on the CA bus before and after a calibration command."]
pub type CalvlBgPat1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA Training pattern 1 driven on the CA bus before and after a calibration command."]
    #[inline(always)]
    pub fn calvl_bg_pat_1(&self) -> CalvlBgPat1R {
        CalvlBgPat1R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA Training pattern 1 driven on the CA bus before and after a calibration command."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_bg_pat_1(&mut self) -> CalvlBgPat1W<DenaliCtl259Spec> {
        CalvlBgPat1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_259::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_259::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl259Spec;
impl crate::RegisterSpec for DenaliCtl259Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_259::R`](R) reader structure"]
impl crate::Readable for DenaliCtl259Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_259::W`](W) writer structure"]
impl crate::Writable for DenaliCtl259Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_259 to value 0"]
impl crate::Resettable for DenaliCtl259Spec {
    const RESET_VALUE: u32 = 0;
}
