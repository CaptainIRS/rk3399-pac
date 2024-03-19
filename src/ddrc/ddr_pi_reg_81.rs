#[doc = "Register `DDR_PI_REG_81` reader"]
pub type R = crate::R<DdrPiReg81Spec>;
#[doc = "Register `DDR_PI_REG_81` writer"]
pub type W = crate::W<DdrPiReg81Spec>;
#[doc = "Field `PI_TDFI_RDLVL_MAX` reader - Defines the DFI tRDLVL_MAX timing parameter (in DFI clocks), the\n\nmaximum cycles between a dfi_rdlvl_en or dfi_rdlvl_gate_en\n\nassertion and a valid dfi_rdlvl_resp."]
pub type PiTdfiRdlvlMaxR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_RDLVL_MAX` writer - Defines the DFI tRDLVL_MAX timing parameter (in DFI clocks), the\n\nmaximum cycles between a dfi_rdlvl_en or dfi_rdlvl_gate_en\n\nassertion and a valid dfi_rdlvl_resp."]
pub type PiTdfiRdlvlMaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tRDLVL_MAX timing parameter (in DFI clocks), the\n\nmaximum cycles between a dfi_rdlvl_en or dfi_rdlvl_gate_en\n\nassertion and a valid dfi_rdlvl_resp."]
    #[inline(always)]
    pub fn pi_tdfi_rdlvl_max(&self) -> PiTdfiRdlvlMaxR {
        PiTdfiRdlvlMaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tRDLVL_MAX timing parameter (in DFI clocks), the\n\nmaximum cycles between a dfi_rdlvl_en or dfi_rdlvl_gate_en\n\nassertion and a valid dfi_rdlvl_resp."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_rdlvl_max(&mut self) -> PiTdfiRdlvlMaxW<DdrPiReg81Spec> {
        PiTdfiRdlvlMaxW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 81\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_81::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_81::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg81Spec;
impl crate::RegisterSpec for DdrPiReg81Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_81::R`](R) reader structure"]
impl crate::Readable for DdrPiReg81Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_81::W`](W) writer structure"]
impl crate::Writable for DdrPiReg81Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_81 to value 0"]
impl crate::Resettable for DdrPiReg81Spec {
    const RESET_VALUE: u32 = 0;
}
