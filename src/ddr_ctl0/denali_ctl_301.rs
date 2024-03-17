#[doc = "Register `DENALI_CTL_301` reader"]
pub type R = crate::R<DenaliCtl301Spec>;
#[doc = "Register `DENALI_CTL_301` writer"]
pub type W = crate::W<DenaliCtl301Spec>;
#[doc = "Field `TDFI_WRLVL_RESP` reader - Defines the DFI tWRLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_req assertion and a dfi_wrlvl_en assertion."]
pub type TdfiWrlvlRespR = crate::FieldReader<u32>;
#[doc = "Field `TDFI_WRLVL_RESP` writer - Defines the DFI tWRLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_req assertion and a dfi_wrlvl_en assertion."]
pub type TdfiWrlvlRespW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tWRLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_req assertion and a dfi_wrlvl_en assertion."]
    #[inline(always)]
    pub fn tdfi_wrlvl_resp(&self) -> TdfiWrlvlRespR {
        TdfiWrlvlRespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tWRLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_req assertion and a dfi_wrlvl_en assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_wrlvl_resp(&mut self) -> TdfiWrlvlRespW<DenaliCtl301Spec> {
        TdfiWrlvlRespW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_301::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_301::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl301Spec;
impl crate::RegisterSpec for DenaliCtl301Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_301::R`](R) reader structure"]
impl crate::Readable for DenaliCtl301Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_301::W`](W) writer structure"]
impl crate::Writable for DenaliCtl301Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_301 to value 0"]
impl crate::Resettable for DenaliCtl301Spec {
    const RESET_VALUE: u32 = 0;
}
