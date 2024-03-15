#[doc = "Register `PI_REG_98` reader"]
pub type R = crate::R<PiReg98Spec>;
#[doc = "Register `PI_REG_98` writer"]
pub type W = crate::W<PiReg98Spec>;
#[doc = "Field `PI_TDFI_CALVL_RESP` reader - Defines the DFI tCALVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_req assertion and a dfi_calvl_en assertion."]
pub type PiTdfiCalvlRespR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_CALVL_RESP` writer - Defines the DFI tCALVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_req assertion and a dfi_calvl_en assertion."]
pub type PiTdfiCalvlRespW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tCALVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_req assertion and a dfi_calvl_en assertion."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_resp(&self) -> PiTdfiCalvlRespR {
        PiTdfiCalvlRespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tCALVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_req assertion and a dfi_calvl_en assertion."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_resp(&mut self) -> PiTdfiCalvlRespW<PiReg98Spec> {
        PiTdfiCalvlRespW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 98\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_98::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_98::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg98Spec;
impl crate::RegisterSpec for PiReg98Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_98::R`](R) reader structure"]
impl crate::Readable for PiReg98Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_98::W`](W) writer structure"]
impl crate::Writable for PiReg98Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_98 to value 0"]
impl crate::Resettable for PiReg98Spec {
    const RESET_VALUE: u32 = 0;
}
