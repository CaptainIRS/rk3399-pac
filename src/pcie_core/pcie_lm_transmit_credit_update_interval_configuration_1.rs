#[doc = "Register `PCIE_LM_TRANSMIT_CREDIT_UPDATE_INTERVAL_CONFIGURATION_1` reader"]
pub type R = crate::R<PcieLmTransmitCreditUpdateIntervalConfiguration1Spec>;
#[doc = "Register `PCIE_LM_TRANSMIT_CREDIT_UPDATE_INTERVAL_CONFIGURATION_1` writer"]
pub type W = crate::W<PcieLmTransmitCreditUpdateIntervalConfiguration1Spec>;
#[doc = "Field `CUI` reader - Minimum Completion Update Interval \\[CUI\\]
has elapsed since the last update. The value is in units of 4 ns. This parameter is not used when the Completion credit is infinity."]
pub type CuiR = crate::FieldReader<u16>;
#[doc = "Field `CUI` writer - Minimum Completion Update Interval \\[CUI\\]
has elapsed since the last update. The value is in units of 4 ns. This parameter is not used when the Completion credit is infinity."]
pub type CuiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MUI` reader - Maximum Update Interval \\[MUI\\]
Maximum credit update interval for all transactions. If no new credit has become available since the last update, the core will repeat the last update after this interval. This is to recover from any losses of credit update packets. The value is in units of 4 ns. This field could be re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
pub type MuiR = crate::FieldReader<u16>;
#[doc = "Field `MUI` writer - Maximum Update Interval \\[MUI\\]
Maximum credit update interval for all transactions. If no new credit has become available since the last update, the core will repeat the last update after this interval. This is to recover from any losses of credit update packets. The value is in units of 4 ns. This field could be re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
pub type MuiW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Minimum Completion Update Interval \\[CUI\\]
has elapsed since the last update. The value is in units of 4 ns. This parameter is not used when the Completion credit is infinity."]
    #[inline(always)]
    pub fn cui(&self) -> CuiR {
        CuiR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Maximum Update Interval \\[MUI\\]
Maximum credit update interval for all transactions. If no new credit has become available since the last update, the core will repeat the last update after this interval. This is to recover from any losses of credit update packets. The value is in units of 4 ns. This field could be re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
    #[inline(always)]
    pub fn mui(&self) -> MuiR {
        MuiR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Minimum Completion Update Interval \\[CUI\\]
has elapsed since the last update. The value is in units of 4 ns. This parameter is not used when the Completion credit is infinity."]
    #[inline(always)]
    #[must_use]
    pub fn cui(&mut self) -> CuiW<PcieLmTransmitCreditUpdateIntervalConfiguration1Spec> {
        CuiW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Maximum Update Interval \\[MUI\\]
Maximum credit update interval for all transactions. If no new credit has become available since the last update, the core will repeat the last update after this interval. This is to recover from any losses of credit update packets. The value is in units of 4 ns. This field could be re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed."]
    #[inline(always)]
    #[must_use]
    pub fn mui(&mut self) -> MuiW<PcieLmTransmitCreditUpdateIntervalConfiguration1Spec> {
        MuiW::new(self, 16)
    }
}
#[doc = "Transmit Credit Update Interval Configuration Register 1 Maximum credit update interval for all transactions. If no new credit has become available since the last update, the core will repeat the last update after this interval. This is to recover from any losses of credit update packets. The value is in units of 4 ns. This field could be re-written by the internal logic when the negotiated link width or link speed changes, to correspond to the default values defined in defines.h. The user may override this default value by writing into this register field. The value written will be lost on a change in the negotiated link width/speed.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_transmit_credit_update_interval_configuration_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_transmit_credit_update_interval_configuration_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmTransmitCreditUpdateIntervalConfiguration1Spec;
impl crate::RegisterSpec for PcieLmTransmitCreditUpdateIntervalConfiguration1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_transmit_credit_update_interval_configuration_1::R`](R) reader structure"]
impl crate::Readable for PcieLmTransmitCreditUpdateIntervalConfiguration1Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_transmit_credit_update_interval_configuration_1::W`](W) writer structure"]
impl crate::Writable for PcieLmTransmitCreditUpdateIntervalConfiguration1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_TRANSMIT_CREDIT_UPDATE_INTERVAL_CONFIGURATION_1 to value 0x03aa_0000"]
impl crate::Resettable for PcieLmTransmitCreditUpdateIntervalConfiguration1Spec {
    const RESET_VALUE: u32 = 0x03aa_0000;
}
