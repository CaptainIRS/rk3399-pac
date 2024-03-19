#[doc = "Register `PCIE_LM_PME_TURNOFF_ACK_DELAY` reader"]
pub type R = crate::R<PcieLmPmeTurnoffAckDelaySpec>;
#[doc = "Register `PCIE_LM_PME_TURNOFF_ACK_DELAY` writer"]
pub type W = crate::W<PcieLmPmeTurnoffAckDelaySpec>;
#[doc = "Field `PTOAD` reader - PME Turnoff Ack Delay \\[PTOAD\\]\n\nTime in microseconds between the\n\ncore receiving a PME_TurnOff\n\nmessage TLP and the core sending a\n\nPME_TO_Ack response to it. This\n\nfield must be set to a non-zero value\n\nin order for the core to send a\n\nresponse. Setting this field to 0\n\nsuppresses the core's response to\n\nPME_TurnOff message, so that the\n\nclient may transmit the PME_TO_Ack\n\nmessage through the master\n\ninterface."]
pub type PtoadR = crate::FieldReader<u16>;
#[doc = "Field `PTOAD` writer - PME Turnoff Ack Delay \\[PTOAD\\]\n\nTime in microseconds between the\n\ncore receiving a PME_TurnOff\n\nmessage TLP and the core sending a\n\nPME_TO_Ack response to it. This\n\nfield must be set to a non-zero value\n\nin order for the core to send a\n\nresponse. Setting this field to 0\n\nsuppresses the core's response to\n\nPME_TurnOff message, so that the\n\nclient may transmit the PME_TO_Ack\n\nmessage through the master\n\ninterface."]
pub type PtoadW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `R7` reader - Reserved \\[R7\\]\n\nReserved"]
pub type R7R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - PME Turnoff Ack Delay \\[PTOAD\\]\n\nTime in microseconds between the\n\ncore receiving a PME_TurnOff\n\nmessage TLP and the core sending a\n\nPME_TO_Ack response to it. This\n\nfield must be set to a non-zero value\n\nin order for the core to send a\n\nresponse. Setting this field to 0\n\nsuppresses the core's response to\n\nPME_TurnOff message, so that the\n\nclient may transmit the PME_TO_Ack\n\nmessage through the master\n\ninterface."]
    #[inline(always)]
    pub fn ptoad(&self) -> PtoadR {
        PtoadR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Reserved \\[R7\\]\n\nReserved"]
    #[inline(always)]
    pub fn r7(&self) -> R7R {
        R7R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - PME Turnoff Ack Delay \\[PTOAD\\]\n\nTime in microseconds between the\n\ncore receiving a PME_TurnOff\n\nmessage TLP and the core sending a\n\nPME_TO_Ack response to it. This\n\nfield must be set to a non-zero value\n\nin order for the core to send a\n\nresponse. Setting this field to 0\n\nsuppresses the core's response to\n\nPME_TurnOff message, so that the\n\nclient may transmit the PME_TO_Ack\n\nmessage through the master\n\ninterface."]
    #[inline(always)]
    #[must_use]
    pub fn ptoad(&mut self) -> PtoadW<PcieLmPmeTurnoffAckDelaySpec> {
        PtoadW::new(self, 0)
    }
}
#[doc = "PME TurnOff Ack Delay Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_pme_turnoff_ack_delay::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_pme_turnoff_ack_delay::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmPmeTurnoffAckDelaySpec;
impl crate::RegisterSpec for PcieLmPmeTurnoffAckDelaySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_pme_turnoff_ack_delay::R`](R) reader structure"]
impl crate::Readable for PcieLmPmeTurnoffAckDelaySpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_pme_turnoff_ack_delay::W`](W) writer structure"]
impl crate::Writable for PcieLmPmeTurnoffAckDelaySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_PME_TURNOFF_ACK_DELAY to value 0x64"]
impl crate::Resettable for PcieLmPmeTurnoffAckDelaySpec {
    const RESET_VALUE: u32 = 0x64;
}
