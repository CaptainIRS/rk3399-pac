#[doc = "Register `DENALI_CTL_306` reader"]
pub type R = crate::R<DenaliCtl306Spec>;
#[doc = "Register `DENALI_CTL_306` writer"]
pub type W = crate::W<DenaliCtl306Spec>;
#[doc = "Field `TDFI_RDLVL_MAX` reader - Defines the DFI tRDLVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion and a valid dfi_rdlvl_resp."]
pub type TdfiRdlvlMaxR = crate::FieldReader<u32>;
#[doc = "Field `TDFI_RDLVL_MAX` writer - Defines the DFI tRDLVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion and a valid dfi_rdlvl_resp."]
pub type TdfiRdlvlMaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tRDLVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion and a valid dfi_rdlvl_resp."]
    #[inline(always)]
    pub fn tdfi_rdlvl_max(&self) -> TdfiRdlvlMaxR {
        TdfiRdlvlMaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tRDLVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion and a valid dfi_rdlvl_resp."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_rdlvl_max(&mut self) -> TdfiRdlvlMaxW<DenaliCtl306Spec> {
        TdfiRdlvlMaxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_306::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_306::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl306Spec;
impl crate::RegisterSpec for DenaliCtl306Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_306::R`](R) reader structure"]
impl crate::Readable for DenaliCtl306Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_306::W`](W) writer structure"]
impl crate::Writable for DenaliCtl306Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_306 to value 0"]
impl crate::Resettable for DenaliCtl306Spec {
    const RESET_VALUE: u32 = 0;
}
