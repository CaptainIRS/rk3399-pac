#[doc = "Register `PCIE_LM_LTR_MESSAGE_GENERATION_CONTROL` reader"]
pub type R = crate::R<PcieLmLtrMessageGenerationControlSpec>;
#[doc = "Register `PCIE_LM_LTR_MESSAGE_GENERATION_CONTROL` writer"]
pub type W = crate::W<PcieLmLtrMessageGenerationControlSpec>;
#[doc = "Field `MLI` reader - Minimum LTR Interval \\[MLI\\]\n\nThis field specifies the minimum\n\nspacing between LTR messages\n\ntransmitted by the core in units of\n\nmicroseconds. The PCI Express\n\nSpecifications recommend sending\n\nno more than two LTR messages\n\nwithin a 500 microsecond interval.\n\nThe core will wait for the minimum\n\ndelay specified by this field after\n\nsending an LTR message, before\n\ntransmitting a new LTR message."]
pub type MliR = crate::FieldReader<u16>;
#[doc = "Field `MLI` writer - Minimum LTR Interval \\[MLI\\]\n\nThis field specifies the minimum\n\nspacing between LTR messages\n\ntransmitted by the core in units of\n\nmicroseconds. The PCI Express\n\nSpecifications recommend sending\n\nno more than two LTR messages\n\nwithin a 500 microsecond interval.\n\nThe core will wait for the minimum\n\ndelay specified by this field after\n\nsending an LTR message, before\n\ntransmitting a new LTR message."]
pub type MliW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `SLM` reader - Send LTR Message \\[SLM\\]\n\nSetting this bit causes the core to\n\ntransmit an LTR message with the\n\nparameters specified in the LTR\n\nSnoop/No-Snoop Latency Register\n\n(Section 8.4.2.9). This bit is cleared\n\nby the core on transmitting the LTR\n\nmessage, and stays set until then.\n\nClient software must read this\n\nregister and verify that this bit is 0\n\nbefore setting it again to send a new\n\nmessage. This field becomes writable\n\nwhen LTR mechanism is enabled in\n\ndevice control-2 register."]
pub type SlmR = crate::BitReader;
#[doc = "Field `TMLMET` reader - Transmit Message on LTR Mechanism Enable Transition \\[TMLMET\\]\n\nWhen this bit is set to 1, the core will\n\nautomatically transmit an LTR\n\nmessage whenever the LTR\n\nMechanism Enable bit in the Device\n\nControl 2 Register changes from 0 to\n\n1, with the parameters specified in\n\nthe LTR Snoop/No-Snoop Latency\n\nRegister. When this bit is 1, the core\n\nwill also transmit an LTR message\n\nwhenever the LTR Mechanism Enable\n\nbit is cleared, if the following\n\nconditions are both true: 1. The core\n\nsent at least\n\none LTR message since the LTR\n\nMechanism Enable bit was last set.\n\n2. The most recent LTR message\n\ntransmitted by the core had as least\n\none of the Requirement bits set. The\n\ncore will set the Requirement bits in\n\nthis LTR message to 0. When this bit\n\n11 is 0, the core will not, by itself,\n\nsend any LTR messages in response\n\nto state changes of the LTR\n\nMechanism Enable bit. Client logic\n\nmay monitor the state of the\n\nLTR_MECHANISM_ ENABLE output of\n\nthe core and transmit LTR messages\n\nthrough the master interface, in\n\nresponse to its state changes."]
pub type TmlmetR = crate::BitReader;
#[doc = "Field `TMLMET` writer - Transmit Message on LTR Mechanism Enable Transition \\[TMLMET\\]\n\nWhen this bit is set to 1, the core will\n\nautomatically transmit an LTR\n\nmessage whenever the LTR\n\nMechanism Enable bit in the Device\n\nControl 2 Register changes from 0 to\n\n1, with the parameters specified in\n\nthe LTR Snoop/No-Snoop Latency\n\nRegister. When this bit is 1, the core\n\nwill also transmit an LTR message\n\nwhenever the LTR Mechanism Enable\n\nbit is cleared, if the following\n\nconditions are both true: 1. The core\n\nsent at least\n\none LTR message since the LTR\n\nMechanism Enable bit was last set.\n\n2. The most recent LTR message\n\ntransmitted by the core had as least\n\none of the Requirement bits set. The\n\ncore will set the Requirement bits in\n\nthis LTR message to 0. When this bit\n\n11 is 0, the core will not, by itself,\n\nsend any LTR messages in response\n\nto state changes of the LTR\n\nMechanism Enable bit. Client logic\n\nmay monitor the state of the\n\nLTR_MECHANISM_ ENABLE output of\n\nthe core and transmit LTR messages\n\nthrough the master interface, in\n\nresponse to its state changes."]
pub type TmlmetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMFPSC` reader - Transmit Message on Function Power State Change \\[TMFPSC\\]\n\nWhen this bit is set to 1, the core will\n\nautomatically transmit an LTR\n\nmessage when all the Functions in\n\nthe core have transitioned to a non-\n\nD0 power state, provided that the\n\nfollowing conditions are both true: 1.\n\nThe core sent at least one LTR\n\nmessage since the Data Link layer\n\nlast transitioned from down to up\n\nstate. 2. The most recent LTR\n\nmessage transmitted by the core had\n\nas least one of the Requirement bits\n\nset. The core will set the\n\nRequirement bits in this LTR\n\nmessage to 0. When this bit 12 is 0,\n\nthe core will not, by itself, send any\n\nLTR messages in response to\n\nFunction Power State changes. Client\n\nlogic may monitor the\n\nFUNCTION_POWER_STATE outputs\n\nof the core and transmit LTR\n\nmessages through the master\n\ninterface, in response to changes in\n\ntheir states."]
pub type TmfpscR = crate::BitReader;
#[doc = "Field `TMFPSC` writer - Transmit Message on Function Power State Change \\[TMFPSC\\]\n\nWhen this bit is set to 1, the core will\n\nautomatically transmit an LTR\n\nmessage when all the Functions in\n\nthe core have transitioned to a non-\n\nD0 power state, provided that the\n\nfollowing conditions are both true: 1.\n\nThe core sent at least one LTR\n\nmessage since the Data Link layer\n\nlast transitioned from down to up\n\nstate. 2. The most recent LTR\n\nmessage transmitted by the core had\n\nas least one of the Requirement bits\n\nset. The core will set the\n\nRequirement bits in this LTR\n\nmessage to 0. When this bit 12 is 0,\n\nthe core will not, by itself, send any\n\nLTR messages in response to\n\nFunction Power State changes. Client\n\nlogic may monitor the\n\nFUNCTION_POWER_STATE outputs\n\nof the core and transmit LTR\n\nmessages through the master\n\ninterface, in response to changes in\n\ntheir states."]
pub type TmfpscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - Minimum LTR Interval \\[MLI\\]\n\nThis field specifies the minimum\n\nspacing between LTR messages\n\ntransmitted by the core in units of\n\nmicroseconds. The PCI Express\n\nSpecifications recommend sending\n\nno more than two LTR messages\n\nwithin a 500 microsecond interval.\n\nThe core will wait for the minimum\n\ndelay specified by this field after\n\nsending an LTR message, before\n\ntransmitting a new LTR message."]
    #[inline(always)]
    pub fn mli(&self) -> MliR {
        MliR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Send LTR Message \\[SLM\\]\n\nSetting this bit causes the core to\n\ntransmit an LTR message with the\n\nparameters specified in the LTR\n\nSnoop/No-Snoop Latency Register\n\n(Section 8.4.2.9). This bit is cleared\n\nby the core on transmitting the LTR\n\nmessage, and stays set until then.\n\nClient software must read this\n\nregister and verify that this bit is 0\n\nbefore setting it again to send a new\n\nmessage. This field becomes writable\n\nwhen LTR mechanism is enabled in\n\ndevice control-2 register."]
    #[inline(always)]
    pub fn slm(&self) -> SlmR {
        SlmR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Transmit Message on LTR Mechanism Enable Transition \\[TMLMET\\]\n\nWhen this bit is set to 1, the core will\n\nautomatically transmit an LTR\n\nmessage whenever the LTR\n\nMechanism Enable bit in the Device\n\nControl 2 Register changes from 0 to\n\n1, with the parameters specified in\n\nthe LTR Snoop/No-Snoop Latency\n\nRegister. When this bit is 1, the core\n\nwill also transmit an LTR message\n\nwhenever the LTR Mechanism Enable\n\nbit is cleared, if the following\n\nconditions are both true: 1. The core\n\nsent at least\n\none LTR message since the LTR\n\nMechanism Enable bit was last set.\n\n2. The most recent LTR message\n\ntransmitted by the core had as least\n\none of the Requirement bits set. The\n\ncore will set the Requirement bits in\n\nthis LTR message to 0. When this bit\n\n11 is 0, the core will not, by itself,\n\nsend any LTR messages in response\n\nto state changes of the LTR\n\nMechanism Enable bit. Client logic\n\nmay monitor the state of the\n\nLTR_MECHANISM_ ENABLE output of\n\nthe core and transmit LTR messages\n\nthrough the master interface, in\n\nresponse to its state changes."]
    #[inline(always)]
    pub fn tmlmet(&self) -> TmlmetR {
        TmlmetR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Transmit Message on Function Power State Change \\[TMFPSC\\]\n\nWhen this bit is set to 1, the core will\n\nautomatically transmit an LTR\n\nmessage when all the Functions in\n\nthe core have transitioned to a non-\n\nD0 power state, provided that the\n\nfollowing conditions are both true: 1.\n\nThe core sent at least one LTR\n\nmessage since the Data Link layer\n\nlast transitioned from down to up\n\nstate. 2. The most recent LTR\n\nmessage transmitted by the core had\n\nas least one of the Requirement bits\n\nset. The core will set the\n\nRequirement bits in this LTR\n\nmessage to 0. When this bit 12 is 0,\n\nthe core will not, by itself, send any\n\nLTR messages in response to\n\nFunction Power State changes. Client\n\nlogic may monitor the\n\nFUNCTION_POWER_STATE outputs\n\nof the core and transmit LTR\n\nmessages through the master\n\ninterface, in response to changes in\n\ntheir states."]
    #[inline(always)]
    pub fn tmfpsc(&self) -> TmfpscR {
        TmfpscR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Minimum LTR Interval \\[MLI\\]\n\nThis field specifies the minimum\n\nspacing between LTR messages\n\ntransmitted by the core in units of\n\nmicroseconds. The PCI Express\n\nSpecifications recommend sending\n\nno more than two LTR messages\n\nwithin a 500 microsecond interval.\n\nThe core will wait for the minimum\n\ndelay specified by this field after\n\nsending an LTR message, before\n\ntransmitting a new LTR message."]
    #[inline(always)]
    #[must_use]
    pub fn mli(&mut self) -> MliW<PcieLmLtrMessageGenerationControlSpec> {
        MliW::new(self, 0)
    }
    #[doc = "Bit 11 - Transmit Message on LTR Mechanism Enable Transition \\[TMLMET\\]\n\nWhen this bit is set to 1, the core will\n\nautomatically transmit an LTR\n\nmessage whenever the LTR\n\nMechanism Enable bit in the Device\n\nControl 2 Register changes from 0 to\n\n1, with the parameters specified in\n\nthe LTR Snoop/No-Snoop Latency\n\nRegister. When this bit is 1, the core\n\nwill also transmit an LTR message\n\nwhenever the LTR Mechanism Enable\n\nbit is cleared, if the following\n\nconditions are both true: 1. The core\n\nsent at least\n\none LTR message since the LTR\n\nMechanism Enable bit was last set.\n\n2. The most recent LTR message\n\ntransmitted by the core had as least\n\none of the Requirement bits set. The\n\ncore will set the Requirement bits in\n\nthis LTR message to 0. When this bit\n\n11 is 0, the core will not, by itself,\n\nsend any LTR messages in response\n\nto state changes of the LTR\n\nMechanism Enable bit. Client logic\n\nmay monitor the state of the\n\nLTR_MECHANISM_ ENABLE output of\n\nthe core and transmit LTR messages\n\nthrough the master interface, in\n\nresponse to its state changes."]
    #[inline(always)]
    #[must_use]
    pub fn tmlmet(&mut self) -> TmlmetW<PcieLmLtrMessageGenerationControlSpec> {
        TmlmetW::new(self, 11)
    }
    #[doc = "Bit 12 - Transmit Message on Function Power State Change \\[TMFPSC\\]\n\nWhen this bit is set to 1, the core will\n\nautomatically transmit an LTR\n\nmessage when all the Functions in\n\nthe core have transitioned to a non-\n\nD0 power state, provided that the\n\nfollowing conditions are both true: 1.\n\nThe core sent at least one LTR\n\nmessage since the Data Link layer\n\nlast transitioned from down to up\n\nstate. 2. The most recent LTR\n\nmessage transmitted by the core had\n\nas least one of the Requirement bits\n\nset. The core will set the\n\nRequirement bits in this LTR\n\nmessage to 0. When this bit 12 is 0,\n\nthe core will not, by itself, send any\n\nLTR messages in response to\n\nFunction Power State changes. Client\n\nlogic may monitor the\n\nFUNCTION_POWER_STATE outputs\n\nof the core and transmit LTR\n\nmessages through the master\n\ninterface, in response to changes in\n\ntheir states."]
    #[inline(always)]
    #[must_use]
    pub fn tmfpsc(&mut self) -> TmfpscW<PcieLmLtrMessageGenerationControlSpec> {
        TmfpscW::new(self, 12)
    }
}
#[doc = "LTR Message Generation Control Register\n\nRSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_ltr_message_generation_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_ltr_message_generation_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
