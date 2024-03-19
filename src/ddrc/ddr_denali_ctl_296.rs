#[doc = "Register `DDR_DENALI_CTL_296` reader"]
pub type R = crate::R<DdrDenaliCtl296Spec>;
#[doc = "Register `DDR_DENALI_CTL_296` writer"]
pub type W = crate::W<DdrDenaliCtl296Spec>;
#[doc = "Field `TDFI_PHYUPD_RESP_F2` reader - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit (6) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdRespF2R = crate::FieldReader<u16>;
#[doc = "Field `TDFI_PHYUPD_RESP_F2` writer - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit (6) set in the UPDATE_ERROR_STATUS parameter."]
pub type TdfiPhyupdRespF2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit (6) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    pub fn tdfi_phyupd_resp_f2(&self) -> TdfiPhyupdRespF2R {
        TdfiPhyupdRespF2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Defines the DFI tPHYUPD_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_phyupd_req assertion and a dfi_phyupd_ack assertion. If programmed to a non-zero, a timing violation will cause an interrupt and bit (6) set in the UPDATE_ERROR_STATUS parameter."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_phyupd_resp_f2(&mut self) -> TdfiPhyupdRespF2W<DdrDenaliCtl296Spec> {
        TdfiPhyupdRespF2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_296::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_296::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl296Spec;
impl crate::RegisterSpec for DdrDenaliCtl296Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_296::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl296Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_296::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl296Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_296 to value 0"]
impl crate::Resettable for DdrDenaliCtl296Spec {
    const RESET_VALUE: u32 = 0;
}
