#[doc = "Register `PI_REG_99` reader"]
pub type R = crate::R<PiReg99Spec>;
#[doc = "Register `PI_REG_99` writer"]
pub type W = crate::W<PiReg99Spec>;
#[doc = "Field `PI_TDFI_CALVL_MAX` reader - Defines the DFI tCALVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_en assertion and a valid dfi_calvl_resp."]
pub type PiTdfiCalvlMaxR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_CALVL_MAX` writer - Defines the DFI tCALVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_en assertion and a valid dfi_calvl_resp."]
pub type PiTdfiCalvlMaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tCALVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_en assertion and a valid dfi_calvl_resp."]
    #[inline(always)]
    pub fn pi_tdfi_calvl_max(&self) -> PiTdfiCalvlMaxR {
        PiTdfiCalvlMaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tCALVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_en assertion and a valid dfi_calvl_resp."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_calvl_max(&mut self) -> PiTdfiCalvlMaxW<PiReg99Spec> {
        PiTdfiCalvlMaxW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 99\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pi_reg_99::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pi_reg_99::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PiReg99Spec;
impl crate::RegisterSpec for PiReg99Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pi_reg_99::R`](R) reader structure"]
impl crate::Readable for PiReg99Spec {}
#[doc = "`write(|w| ..)` method takes [`pi_reg_99::W`](W) writer structure"]
impl crate::Writable for PiReg99Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PI_REG_99 to value 0"]
impl crate::Resettable for PiReg99Spec {
    const RESET_VALUE: u32 = 0;
}
