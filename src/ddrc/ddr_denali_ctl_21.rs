#[doc = "Register `DDR_DENALI_CTL_21` reader"]
pub type R = crate::R<DdrDenaliCtl21Spec>;
#[doc = "Register `DDR_DENALI_CTL_21` writer"]
pub type W = crate::W<DdrDenaliCtl21Spec>;
#[doc = "Field `CKE_INACTIVE` reader - Number of cycles after reset before CKE will be active."]
pub type CkeInactiveR = crate::FieldReader<u32>;
#[doc = "Field `CKE_INACTIVE` writer - Number of cycles after reset before CKE will be active."]
pub type CkeInactiveW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of cycles after reset before CKE will be active."]
    #[inline(always)]
    pub fn cke_inactive(&self) -> CkeInactiveR {
        CkeInactiveR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of cycles after reset before CKE will be active."]
    #[inline(always)]
    #[must_use]
    pub fn cke_inactive(&mut self) -> CkeInactiveW<DdrDenaliCtl21Spec> {
        CkeInactiveW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl21Spec;
impl crate::RegisterSpec for DdrDenaliCtl21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_21::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl21Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_21::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_21 to value 0"]
impl crate::Resettable for DdrDenaliCtl21Spec {
    const RESET_VALUE: u32 = 0;
}
