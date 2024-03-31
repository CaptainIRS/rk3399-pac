#[doc = "Register `DENALI_CTL_263` reader"]
pub type R = crate::R<DenaliCtl263Spec>;
#[doc = "Register `DENALI_CTL_263` writer"]
pub type W = crate::W<DenaliCtl263Spec>;
#[doc = "Field `CALVL_BG_PAT_3` reader - CA Training pattern 3 driven on the CA bus before and after a calibration command."]
pub type CalvlBgPat3R = crate::FieldReader<u32>;
#[doc = "Field `CALVL_BG_PAT_3` writer - CA Training pattern 3 driven on the CA bus before and after a calibration command."]
pub type CalvlBgPat3W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA Training pattern 3 driven on the CA bus before and after a calibration command."]
    #[inline(always)]
    pub fn calvl_bg_pat_3(&self) -> CalvlBgPat3R {
        CalvlBgPat3R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA Training pattern 3 driven on the CA bus before and after a calibration command."]
    #[inline(always)]
    #[must_use]
    pub fn calvl_bg_pat_3(&mut self) -> CalvlBgPat3W<DenaliCtl263Spec> {
        CalvlBgPat3W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_263::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_263::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl263Spec;
impl crate::RegisterSpec for DenaliCtl263Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_263::R`](R) reader structure"]
impl crate::Readable for DenaliCtl263Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_263::W`](W) writer structure"]
impl crate::Writable for DenaliCtl263Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_263 to value 0"]
impl crate::Resettable for DenaliCtl263Spec {
    const RESET_VALUE: u32 = 0;
}
