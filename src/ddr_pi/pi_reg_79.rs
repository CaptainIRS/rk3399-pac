#[doc = "Register `PI_REG_79` reader"]
pub type R = crate::R<PiReg79Spec>;
#[doc = "Register `PI_REG_79` writer"]
pub type W = crate::W<PiReg79Spec>;
#[doc = "Field `PI_TDFI_RDLVL_RESP` reader - Defines the DFI tRDLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_rdlvl_req or dfi_rdlvl_gate_req assertion and a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion."]
pub type PiTdfiRdlvlRespR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_RDLVL_RESP` writer - Defines the DFI tRDLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_rdlvl_req or dfi_rdlvl_gate_req assertion and a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion."]
pub type PiTdfiRdlvlRespW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tRDLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_rdlvl_req or dfi_rdlvl_gate_req assertion and a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion."]
    #[inline(always)]
    pub fn pi_tdfi_rdlvl_resp(&self) -> PiTdfiRdlvlRespR {
        PiTdfiRdlvlRespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tRDLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_rdlvl_req or dfi_rdlvl_gate_req assertion and a dfi_rdlvl_en or dfi_rdlvl_gate_en assertion."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_rdlvl_resp(&mut self) -> PiTdfiRdlvlRespW<PiReg79Spec> {
        PiTdfiRdlvlRespW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 79\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_79::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_79::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg79Spec;
impl crate::RegisterSpec for PiReg79Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_79::R`](R) reader structure"]
impl crate::Readable for PiReg79Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_79::W`](W) writer structure"]
impl crate::Writable for PiReg79Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_79 to value 0"]
impl crate::Resettable for PiReg79Spec {
    const RESET_VALUE: u32 = 0;
}
