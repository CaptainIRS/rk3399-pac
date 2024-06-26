#[doc = "Register `DENALI_CTL_300` reader"]
pub type R = crate::R<DenaliCtl300Spec>;
#[doc = "Register `DENALI_CTL_300` writer"]
pub type W = crate::W<DenaliCtl300Spec>;
#[doc = "Field `TDFI_WRLVL_WW` reader - Defines the DFI tWRLVL_WW timing parameter (in DFI clocks), the minimum cycles between dfi_wrlvl_strobe assertions."]
pub type TdfiWrlvlWwR = crate::FieldReader<u16>;
#[doc = "Field `TDFI_WRLVL_WW` writer - Defines the DFI tWRLVL_WW timing parameter (in DFI clocks), the minimum cycles between dfi_wrlvl_strobe assertions."]
pub type TdfiWrlvlWwW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Defines the DFI tWRLVL_WW timing parameter (in DFI clocks), the minimum cycles between dfi_wrlvl_strobe assertions."]
    #[inline(always)]
    pub fn tdfi_wrlvl_ww(&self) -> TdfiWrlvlWwR {
        TdfiWrlvlWwR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Defines the DFI tWRLVL_WW timing parameter (in DFI clocks), the minimum cycles between dfi_wrlvl_strobe assertions."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_wrlvl_ww(&mut self) -> TdfiWrlvlWwW<DenaliCtl300Spec> {
        TdfiWrlvlWwW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_300::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_300::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl300Spec;
impl crate::RegisterSpec for DenaliCtl300Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_300::R`](R) reader structure"]
impl crate::Readable for DenaliCtl300Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_300::W`](W) writer structure"]
impl crate::Writable for DenaliCtl300Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_300 to value 0"]
impl crate::Resettable for DenaliCtl300Spec {
    const RESET_VALUE: u32 = 0;
}
