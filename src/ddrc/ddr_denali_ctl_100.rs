#[doc = "Register `DDR_DENALI_CTL_100` reader"]
pub type R = crate::R<DdrDenaliCtl100Spec>;
#[doc = "Register `DDR_DENALI_CTL_100` writer"]
pub type W = crate::W<DdrDenaliCtl100Spec>;
#[doc = "Field `LPI_WAKEUP_TIMEOUT` reader - Defines the LPI timeout time, the maximum cycles between a dfi_lp_req de-assertion and a dfi_lp_ack de-assertion. If this value is exceeded, an interrupt will occur."]
pub type LpiWakeupTimeoutR = crate::FieldReader<u16>;
#[doc = "Field `LPI_WAKEUP_TIMEOUT` writer - Defines the LPI timeout time, the maximum cycles between a dfi_lp_req de-assertion and a dfi_lp_ack de-assertion. If this value is exceeded, an interrupt will occur."]
pub type LpiWakeupTimeoutW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
#[doc = "Field `TDFI_LP_RESP` reader - Defines the DFI tLP_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_lp_req assertion and a dfi_lp_ack assertion."]
pub type TdfiLpRespR = crate::FieldReader;
#[doc = "Field `TDFI_LP_RESP` writer - Defines the DFI tLP_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_lp_req assertion and a dfi_lp_ack assertion."]
pub type TdfiLpRespW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_STATE` reader - Low power state status parameter. Bits (4:0) indicate the current low power state and bit (5) set indicates that status bits are valid. READ- ONLY"]
pub type LpStateR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:11 - Defines the LPI timeout time, the maximum cycles between a dfi_lp_req de-assertion and a dfi_lp_ack de-assertion. If this value is exceeded, an interrupt will occur."]
    #[inline(always)]
    pub fn lpi_wakeup_timeout(&self) -> LpiWakeupTimeoutR {
        LpiWakeupTimeoutR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:18 - Defines the DFI tLP_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_lp_req assertion and a dfi_lp_ack assertion."]
    #[inline(always)]
    pub fn tdfi_lp_resp(&self) -> TdfiLpRespR {
        TdfiLpRespR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:30 - Low power state status parameter. Bits (4:0) indicate the current low power state and bit (5) set indicates that status bits are valid. READ- ONLY"]
    #[inline(always)]
    pub fn lp_state(&self) -> LpStateR {
        LpStateR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:11 - Defines the LPI timeout time, the maximum cycles between a dfi_lp_req de-assertion and a dfi_lp_ack de-assertion. If this value is exceeded, an interrupt will occur."]
    #[inline(always)]
    #[must_use]
    pub fn lpi_wakeup_timeout(&mut self) -> LpiWakeupTimeoutW<DdrDenaliCtl100Spec> {
        LpiWakeupTimeoutW::new(self, 0)
    }
    #[doc = "Bits 16:18 - Defines the DFI tLP_RESP timing parameter (in DFI clocks), the maximum cycles between a dfi_lp_req assertion and a dfi_lp_ack assertion."]
    #[inline(always)]
    #[must_use]
    pub fn tdfi_lp_resp(&mut self) -> TdfiLpRespW<DdrDenaliCtl100Spec> {
        TdfiLpRespW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_ctl_100::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_ctl_100::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliCtl100Spec;
impl crate::RegisterSpec for DdrDenaliCtl100Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_ctl_100::R`](R) reader structure"]
impl crate::Readable for DdrDenaliCtl100Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_ctl_100::W`](W) writer structure"]
impl crate::Writable for DdrDenaliCtl100Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_CTL_100 to value 0x2000_0000"]
impl crate::Resettable for DdrDenaliCtl100Spec {
    const RESET_VALUE: u32 = 0x2000_0000;
}
