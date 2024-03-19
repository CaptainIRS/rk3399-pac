#[doc = "Register `DDR_PI_REG_64` reader"]
pub type R = crate::R<DdrPiReg64Spec>;
#[doc = "Register `DDR_PI_REG_64` writer"]
pub type W = crate::W<DdrPiReg64Spec>;
#[doc = "Field `PI_TDFI_WRLVL_RESP` reader - Defines the DFI tWRLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_req assertion and a dfi_wrlvl_en assertion."]
pub type PiTdfiWrlvlRespR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_WRLVL_RESP` writer - Defines the DFI tWRLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_req assertion and a dfi_wrlvl_en assertion."]
pub type PiTdfiWrlvlRespW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tWRLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_req assertion and a dfi_wrlvl_en assertion."]
    #[inline(always)]
    pub fn pi_tdfi_wrlvl_resp(&self) -> PiTdfiWrlvlRespR {
        PiTdfiWrlvlRespR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tWRLVL_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_req assertion and a dfi_wrlvl_en assertion."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wrlvl_resp(&mut self) -> PiTdfiWrlvlRespW<DdrPiReg64Spec> {
        PiTdfiWrlvlRespW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 64\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_64::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_64::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg64Spec;
impl crate::RegisterSpec for DdrPiReg64Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_64::R`](R) reader structure"]
impl crate::Readable for DdrPiReg64Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_64::W`](W) writer structure"]
impl crate::Writable for DdrPiReg64Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_64 to value 0"]
impl crate::Resettable for DdrPiReg64Spec {
    const RESET_VALUE: u32 = 0;
}
