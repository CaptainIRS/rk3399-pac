#[doc = "Register `DENALI_CTL_302` reader"]
pub type R = crate::R<DenaliCtl302Spec>;
#[doc = "Register `DENALI_CTL_302` writer"]
pub type W = crate::W<DenaliCtl302Spec>;
#[doc = "Field `TDFI_WRLVL_MAX` reader - Defines the DFI tWRLVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_en assertion and a valid dfi_wrlvl_resp."]
pub type TdfiWrlvlMaxR = crate::FieldReader<u32>;
#[doc = "Field `TDFI_WRLVL_MAX` writer - Defines the DFI tWRLVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_en assertion and a valid dfi_wrlvl_resp."]
pub type TdfiWrlvlMaxW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Defines the DFI tWRLVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_en assertion and a valid dfi_wrlvl_resp."]
    #[inline(always)]
    pub fn tdfi_wrlvl_max(&self) -> TdfiWrlvlMaxR {
        TdfiWrlvlMaxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Defines the DFI tWRLVL_MAX timing parameter (in DFI clocks), the maximum cycles between a dfi_wrlvl_en assertion and a valid dfi_wrlvl_resp."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_wrlvl_max(&mut self) -> TdfiWrlvlMaxW<DenaliCtl302Spec> {
        TdfiWrlvlMaxW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_ctl_302::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_ctl_302::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliCtl302Spec;
impl crate::RegisterSpec for DenaliCtl302Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_ctl_302::R`](R) reader structure"]
impl crate::Readable for DenaliCtl302Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_ctl_302::W`](W) writer structure"]
impl crate::Writable for DenaliCtl302Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_CTL_302 to value 0"]
impl crate::Resettable for DenaliCtl302Spec {
    const RESET_VALUE: u32 = 0;
}
