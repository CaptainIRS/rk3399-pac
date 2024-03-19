#[doc = "Register `DDR_DENALI_CTL_304` reader"]
pub type R = crate::R<DdrDenaliCtl304Spec>;
#[doc = "Register `DDR_DENALI_CTL_304` writer"]
pub type W = crate::W<DdrDenaliCtl304Spec>;
#[doc = "Field `TDFI_RDLVL_RESP` reader - Defines the DFI tRDLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_rdlvl_req or dfi_rdlvl_gate_req assertion and a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion."]
pub type TdfiRdlvlRespR = crate::FieldReader<u32>;
#[doc = "Field `TDFI_RDLVL_RESP` writer - Defines the DFI tRDLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_rdlvl_req or dfi_rdlvl_gate_req assertion and a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion."]
pub type TdfiRdlvlRespW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tRDLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_rdlvl_req or dfi_rdlvl_gate_req assertion and a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion."]
    #[inline(always)]
    pub fn tdfi_rdlvl_resp(&self) -> TdfiRdlvlRespR {
        TdfiRdlvlRespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tRDLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_rdlvl_req or dfi_rdlvl_gate_req assertion and a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_rdlvl_resp(&mut self) -> TdfiRdlvlRespW<DdrDenaliCtl304Spec> {
        TdfiRdlvlRespW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_304::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_304::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl304Spec;
impl crate::RegisterSpec for DdrDenaliCtl304Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_304::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl304Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_304::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl304Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_304 to value 0"]
impl crate::Resettable for DdrDenaliCtl304Spec {
    const RESET_VALUE: u32 = 0;
}
