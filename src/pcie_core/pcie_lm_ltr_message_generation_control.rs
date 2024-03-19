#[doc = "Register `PCIE_LM_LTR_MESSAGE_GENERATION_CONTROL` reader"]
pub type R = crate::R<PcieLmLtrMessageGenerationControlSpec>;
#[doc = "Register `PCIE_LM_LTR_MESSAGE_GENERATION_CONTROL` writer"]
pub type W = crate::W<PcieLmLtrMessageGenerationControlSpec>;
#[doc = "Field `MLI` reader - Minimum LTR Interval \\[MLI\\]
This field specifies the minimum spacing between LTR messages transmitted by the core in units of microseconds. The PCI Express Specifications recommend sending no more than two LTR messages within a 500 microsecond interval. The core will wait for the minimum delay specified by this field after sending an LTR message, before transmitting a new LTR message."]
pub type MliR = crate::FieldReader<u16>;
#[doc = "Field `MLI` writer - Minimum LTR Interval \\[MLI\\]
This field specifies the minimum spacing between LTR messages transmitted by the core in units of microseconds. The PCI Express Specifications recommend sending no more than two LTR messages within a 500 microsecond interval. The core will wait for the minimum delay specified by this field after sending an LTR message, before transmitting a new LTR message."]
pub type MliW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SLM` reader - Send LTR Message \\[SLM\\]
Setting this bit causes the core to transmit an LTR message with the parameters specified in the LTR Snoop/No-Snoop Latency Register (Section 8.4.2.9). This bit is cleared by the core on transmitting the LTR message, and stays set until then. Client software must read this register and verify that this bit is 0 before setting it again to send a new message. This field becomes writable when LTR mechanism is enabled in device control-2 register."]
pub type SlmR = crate::BitReader;
#[doc = "Field `TMLMET` reader - Transmit Message on LTR Mechanism Enable Transition \\[TMLMET\\]
When this bit is set to 1, the core will automatically transmit an LTR message whenever the LTR Mechanism Enable bit in the Device Control 2 Register changes from 0 to 1, with the parameters specified in the LTR Snoop/No-Snoop Latency Register. When this bit is 1, the core will also transmit an LTR message whenever the LTR Mechanism Enable bit is cleared, if the following conditions are both true: 1. The core sent at least one LTR message since the LTR Mechanism Enable bit was last set. 2. The most recent LTR message transmitted by the core had as least one of the Requirement bits set. The core will set the Requirement bits in this LTR message to 0. When this bit 11 is 0, the core will not, by itself, send any LTR messages in response to state changes of the LTR Mechanism Enable bit. Client logic may monitor the state of the LTR_MECHANISM_ ENABLE output of the core and transmit LTR messages through the master interface, in response to its state changes."]
pub type TmlmetR = crate::BitReader;
#[doc = "Field `TMLMET` writer - Transmit Message on LTR Mechanism Enable Transition \\[TMLMET\\]
When this bit is set to 1, the core will automatically transmit an LTR message whenever the LTR Mechanism Enable bit in the Device Control 2 Register changes from 0 to 1, with the parameters specified in the LTR Snoop/No-Snoop Latency Register. When this bit is 1, the core will also transmit an LTR message whenever the LTR Mechanism Enable bit is cleared, if the following conditions are both true: 1. The core sent at least one LTR message since the LTR Mechanism Enable bit was last set. 2. The most recent LTR message transmitted by the core had as least one of the Requirement bits set. The core will set the Requirement bits in this LTR message to 0. When this bit 11 is 0, the core will not, by itself, send any LTR messages in response to state changes of the LTR Mechanism Enable bit. Client logic may monitor the state of the LTR_MECHANISM_ ENABLE output of the core and transmit LTR messages through the master interface, in response to its state changes."]
pub type TmlmetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMFPSC` reader - Transmit Message on Function Power State Change \\[TMFPSC\\]
When this bit is set to 1, the core will automatically transmit an LTR message when all the Functions in the core have transitioned to a non- D0 power state, provided that the following conditions are both true: 1. The core sent at least one LTR message since the Data Link layer last transitioned from down to up state. 2. The most recent LTR message transmitted by the core had as least one of the Requirement bits set. The core will set the Requirement bits in this LTR message to 0. When this bit 12 is 0, the core will not, by itself, send any LTR messages in response to Function Power State changes. Client logic may monitor the FUNCTION_POWER_STATE outputs of the core and transmit LTR messages through the master interface, in response to changes in their states."]
pub type TmfpscR = crate::BitReader;
#[doc = "Field `TMFPSC` writer - Transmit Message on Function Power State Change \\[TMFPSC\\]
When this bit is set to 1, the core will automatically transmit an LTR message when all the Functions in the core have transitioned to a non- D0 power state, provided that the following conditions are both true: 1. The core sent at least one LTR message since the Data Link layer last transitioned from down to up state. 2. The most recent LTR message transmitted by the core had as least one of the Requirement bits set. The core will set the Requirement bits in this LTR message to 0. When this bit 12 is 0, the core will not, by itself, send any LTR messages in response to Function Power State changes. Client logic may monitor the FUNCTION_POWER_STATE outputs of the core and transmit LTR messages through the master interface, in response to changes in their states."]
pub type TmfpscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Minimum LTR Interval \\[MLI\\]
This field specifies the minimum spacing between LTR messages transmitted by the core in units of microseconds. The PCI Express Specifications recommend sending no more than two LTR messages within a 500 microsecond interval. The core will wait for the minimum delay specified by this field after sending an LTR message, before transmitting a new LTR message."]
    #[inline(always)]
    pub fn mli(&self) -> MliR {
        MliR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Send LTR Message \\[SLM\\]
Setting this bit causes the core to transmit an LTR message with the parameters specified in the LTR Snoop/No-Snoop Latency Register (Section 8.4.2.9). This bit is cleared by the core on transmitting the LTR message, and stays set until then. Client software must read this register and verify that this bit is 0 before setting it again to send a new message. This field becomes writable when LTR mechanism is enabled in device control-2 register."]
    #[inline(always)]
    pub fn slm(&self) -> SlmR {
        SlmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit Message on LTR Mechanism Enable Transition \\[TMLMET\\]
When this bit is set to 1, the core will automatically transmit an LTR message whenever the LTR Mechanism Enable bit in the Device Control 2 Register changes from 0 to 1, with the parameters specified in the LTR Snoop/No-Snoop Latency Register. When this bit is 1, the core will also transmit an LTR message whenever the LTR Mechanism Enable bit is cleared, if the following conditions are both true: 1. The core sent at least one LTR message since the LTR Mechanism Enable bit was last set. 2. The most recent LTR message transmitted by the core had as least one of the Requirement bits set. The core will set the Requirement bits in this LTR message to 0. When this bit 11 is 0, the core will not, by itself, send any LTR messages in response to state changes of the LTR Mechanism Enable bit. Client logic may monitor the state of the LTR_MECHANISM_ ENABLE output of the core and transmit LTR messages through the master interface, in response to its state changes."]
    #[inline(always)]
    pub fn tmlmet(&self) -> TmlmetR {
        TmlmetR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Message on Function Power State Change \\[TMFPSC\\]
When this bit is set to 1, the core will automatically transmit an LTR message when all the Functions in the core have transitioned to a non- D0 power state, provided that the following conditions are both true: 1. The core sent at least one LTR message since the Data Link layer last transitioned from down to up state. 2. The most recent LTR message transmitted by the core had as least one of the Requirement bits set. The core will set the Requirement bits in this LTR message to 0. When this bit 12 is 0, the core will not, by itself, send any LTR messages in response to Function Power State changes. Client logic may monitor the FUNCTION_POWER_STATE outputs of the core and transmit LTR messages through the master interface, in response to changes in their states."]
    #[inline(always)]
    pub fn tmfpsc(&self) -> TmfpscR {
        TmfpscR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Minimum LTR Interval \\[MLI\\]
This field specifies the minimum spacing between LTR messages transmitted by the core in units of microseconds. The PCI Express Specifications recommend sending no more than two LTR messages within a 500 microsecond interval. The core will wait for the minimum delay specified by this field after sending an LTR message, before transmitting a new LTR message."]
    #[inline(always)]
    #[must_use]
    pub fn mli(&mut self) -> MliW<PcieLmLtrMessageGenerationControlSpec> {
        MliW::new(self, 0)
    }
    #[doc = "Bit 11 - Transmit Message on LTR Mechanism Enable Transition \\[TMLMET\\]
When this bit is set to 1, the core will automatically transmit an LTR message whenever the LTR Mechanism Enable bit in the Device Control 2 Register changes from 0 to 1, with the parameters specified in the LTR Snoop/No-Snoop Latency Register. When this bit is 1, the core will also transmit an LTR message whenever the LTR Mechanism Enable bit is cleared, if the following conditions are both true: 1. The core sent at least one LTR message since the LTR Mechanism Enable bit was last set. 2. The most recent LTR message transmitted by the core had as least one of the Requirement bits set. The core will set the Requirement bits in this LTR message to 0. When this bit 11 is 0, the core will not, by itself, send any LTR messages in response to state changes of the LTR Mechanism Enable bit. Client logic may monitor the state of the LTR_MECHANISM_ ENABLE output of the core and transmit LTR messages through the master interface, in response to its state changes."]
    #[inline(always)]
    #[must_use]
    pub fn tmlmet(&mut self) -> TmlmetW<PcieLmLtrMessageGenerationControlSpec> {
        TmlmetW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit Message on Function Power State Change \\[TMFPSC\\]
When this bit is set to 1, the core will automatically transmit an LTR message when all the Functions in the core have transitioned to a non- D0 power state, provided that the following conditions are both true: 1. The core sent at least one LTR message since the Data Link layer last transitioned from down to up state. 2. The most recent LTR message transmitted by the core had as least one of the Requirement bits set. The core will set the Requirement bits in this LTR message to 0. When this bit 12 is 0, the core will not, by itself, send any LTR messages in response to Function Power State changes. Client logic may monitor the FUNCTION_POWER_STATE outputs of the core and transmit LTR messages through the master interface, in response to changes in their states."]
    #[inline(always)]
    #[must_use]
    pub fn tmfpsc(&mut self) -> TmfpscW<PcieLmLtrMessageGenerationControlSpec> {
        TmfpscW::new(self, 12)
    }
}
#[doc = "LTR Message Generation Control Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_ltr_message_generation_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_ltr_message_generation_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmLtrMessageGenerationControlSpec;
impl crate::RegisterSpec for PcieLmLtrMessageGenerationControlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_ltr_message_generation_control::R`](R) reader structure"]
impl crate::Readable for PcieLmLtrMessageGenerationControlSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_ltr_message_generation_control::W`](W) writer structure"]
impl crate::Writable for PcieLmLtrMessageGenerationControlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_LM_LTR_MESSAGE_GENERATION_CONTROL to value 0x18fa"]
impl crate::Resettable for PcieLmLtrMessageGenerationControlSpec {
    const RESET_VALUE: u32 = 0x18fa;
}
