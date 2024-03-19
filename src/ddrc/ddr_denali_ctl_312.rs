#[doc = "Register `DDR_DENALI_CTL_312` reader"]
pub type R = crate::R<DdrDenaliCtl312Spec>;
#[doc = "Register `DDR_DENALI_CTL_312` writer"]
pub type W = crate::W<DdrDenaliCtl312Spec>;
#[doc = "Field `TDFI_CALVL_MAX` reader - Defines the DFI tCALVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_en assertion and a valid dfi_calvl_resp."]
pub type TdfiCalvlMaxR = crate::FieldReader<u32>;
#[doc = "Field `TDFI_CALVL_MAX` writer - Defines the DFI tCALVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_en assertion and a valid dfi_calvl_resp."]
pub type TdfiCalvlMaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tCALVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_en assertion and a valid dfi_calvl_resp."]
    #[inline(always)]
    pub fn tdfi_calvl_max(&self) -> TdfiCalvlMaxR {
        TdfiCalvlMaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tCALVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_calvl_en assertion and a valid dfi_calvl_resp."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_calvl_max(&mut self) -> TdfiCalvlMaxW<DdrDenaliCtl312Spec> {
        TdfiCalvlMaxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_312::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_312::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl312Spec;
impl crate::RegisterSpec for DdrDenaliCtl312Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_312::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl312Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_312::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl312Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_312 to value 0"]
impl crate::Resettable for DdrDenaliCtl312Spec {
    const RESET_VALUE: u32 = 0;
}
