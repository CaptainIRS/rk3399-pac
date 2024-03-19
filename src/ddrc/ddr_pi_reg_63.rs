#[doc = "Register `DDR_PI_REG_63` reader"]
pub type R = crate::R<DdrPiReg63Spec>;
#[doc = "Register `DDR_PI_REG_63` writer"]
pub type W = crate::W<DdrPiReg63Spec>;
#[doc = "Field `PI_TDFI_WRLVL_WW` reader - Defines the DFI tWRLVL_WW timing parameter (in DFI clocks), the minimum cycles between dfi_wrlvl_strobe assertions."]
pub type PiTdfiWrlvlWwR = crate::FieldReader<u16>;
#[doc = "Field `PI_TDFI_WRLVL_WW` writer - Defines the DFI tWRLVL_WW timing parameter (in DFI clocks), the minimum cycles between dfi_wrlvl_strobe assertions."]
pub type PiTdfiWrlvlWwW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Defines the DFI tWRLVL_WW timing parameter (in DFI clocks), the minimum cycles between dfi_wrlvl_strobe assertions."]
    #[inline(always)]
    pub fn pi_tdfi_wrlvl_ww(&self) -> PiTdfiWrlvlWwR {
        PiTdfiWrlvlWwR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Defines the DFI tWRLVL_WW timing parameter (in DFI clocks), the minimum cycles between dfi_wrlvl_strobe assertions."]
    #[inline(always)]
    #[must_use]
    pub fn pi_tdfi_wrlvl_ww(&mut self) -> PiTdfiWrlvlWwW<DdrPiReg63Spec> {
        PiTdfiWrlvlWwW::new(self, 0)
    }
}
#[doc = "DDR PHY Independent Register 63\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_pi_reg_63::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_pi_reg_63::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrPiReg63Spec;
impl crate::RegisterSpec for DdrPiReg63Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_pi_reg_63::R`](R) reader structure"]
impl crate::Readable for DdrPiReg63Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_pi_reg_63::W`](W) writer structure"]
impl crate::Writable for DdrPiReg63Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_PI_REG_63 to value 0"]
impl crate::Resettable for DdrPiReg63Spec {
    const RESET_VALUE: u32 = 0;
}
