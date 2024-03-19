#[doc = "Register `DDR_PI_REG_65` reader"]
pub type R = crate::R<DdrPiReg65Spec>;
#[doc = "Register `DDR_PI_REG_65` writer"]
pub type W = crate::W<DdrPiReg65Spec>;
#[doc = "Field `PI_TDFI_WRLVL_MAX` reader - Defines the DFI tWRLVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_en assertion and a valid dfi_wrlvl_resp."]
pub type PiTdfiWrlvlMaxR = crate::FieldReader<u32>;
#[doc = "Field `PI_TDFI_WRLVL_MAX` writer - Defines the DFI tWRLVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_en assertion and a valid dfi_wrlvl_resp."]
pub type PiTdfiWrlvlMaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tWRLVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_en assertion and a valid dfi_wrlvl_resp."]
    #[inline(always)]
    pub fn pi_tdfi_wrlvl_max(&self) -> PiTdfiWrlvlMaxR {
        PiTdfiWrlvlMaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tWRLVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_en assertion and a valid dfi_wrlvl_resp."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wrlvl_max(&mut self) -> PiTdfiWrlvlMaxW<DdrPiReg65Spec> {
        PiTdfiWrlvlMaxW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 65\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_65::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_65::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg65Spec;
impl crate::RegisterSpec for DdrPiReg65Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_65::R`](R) reader structure"]
impl crate::Readable for DdrPiReg65Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_65::W`](W) writer structure"]
impl crate::Writable for DdrPiReg65Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_65 to value 0"]
impl crate::Resettable for DdrPiReg65Spec {
    const RESET_VALUE: u32 = 0;
}
