#[doc = "Register `DENALI_CTL_311` reader"]
pub type R = crate::R<DenaliCtl311Spec>;
#[doc = "Register `DENALI_CTL_311` writer"]
pub type W = crate::W<DenaliCtl311Spec>;
#[doc = "Field `TDFI_CALVL_RESP` reader - Defines the DFI tCALVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_req assertion and a dfi_calvl_en assertion."]
pub type TdfiCalvlRespR = crate::FieldReader<u32>;
#[doc = "Field `TDFI_CALVL_RESP` writer - Defines the DFI tCALVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_req assertion and a dfi_calvl_en assertion."]
pub type TdfiCalvlRespW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tCALVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_req assertion and a dfi_calvl_en assertion."]
    #[inline(always)]
    pub fn tdfi_calvl_resp(&self) -> TdfiCalvlRespR {
        TdfiCalvlRespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tCALVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_req assertion and a dfi_calvl_en assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_calvl_resp(&mut self) -> TdfiCalvlRespW<DenaliCtl311Spec> {
        TdfiCalvlRespW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_311::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_311::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl311Spec;
impl crate::RegisterSpec for DenaliCtl311Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_311::R`](R) reader structure"]
impl crate::Readable for DenaliCtl311Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_311::W`](W) writer structure"]
impl crate::Writable for DenaliCtl311Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_311 to value 0"]
impl crate::Resettable for DenaliCtl311Spec {
    const RESET_VALUE: u32 = 0;
}
