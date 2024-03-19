#[doc = "Register `PCIE_PF_DPA_CAPABILITY` reader"]
pub type R = crate::R<PciePfDpaCapabilitySpec>;
#[doc = "Field `MNS` reader - Maximum Number of Substates \\[MNS\\]\n\nMaximum number of DPA substates\n\nsupported by the Function (the value\n\nin this field is the number of\n\nsubstates minus 1)."]
pub type MnsR = crate::FieldReader;
#[doc = "Field `R0` reader - Reserved \\[R0\\]\n\nReserved"]
pub type R0R = crate::FieldReader;
#[doc = "Field `TLU` reader - Transition Latency Unit \\[TLU\\]\n\nThis is the unit of the transition\n\nlatencies specified in the Transition\n\nLatency Value 0 and Transition\n\nLatency Value 1 fields of this register\n\n(00 = 1ms, 01 = 10ms, 10 =\n\n100ms, 11 = reserved)."]
pub type TluR = crate::FieldReader;
#[doc = "Field `R1` reader - Reserved \\[R1\\]\n\nReserved"]
pub type R1R = crate::FieldReader;
#[doc = "Field `PAS` reader - Power Allocation Scale \\[PAS\\]\n\nThis is the scale used to compute the\n\nactual power from the values\n\nspecified in the Dynamic Power\n\nAllocation Array Registers 0 - 7. The\n\nactual power in Watts is obtained\n\nby multiplying the value in the\n\nDynamic Power Allocation Array\n\nRegister by this scale factor (00 =\n\n10x, 01 = 1x, 10 = 0.1x, 11 =\n\n0.01x)."]
pub type PasR = crate::FieldReader;
#[doc = "Field `R2` reader - Reserved \\[R2\\]\n\nReserved"]
pub type R2R = crate::FieldReader;
#[doc = "Field `TLV0` reader - Transition Latency Value 0 \\[TLV0\\]\n\nSpecifies the transition latency for\n\nthe substate. Each of the 32\n\nsubstates may specify one of the two\n\ntransition latency values. This field\n\ncontains the first of the two latency\n\nvalues. The unit of latency is\n\nspecified by the Transition Latency\n\nUnit field of this register."]
pub type Tlv0R = crate::FieldReader;
#[doc = "Field `TLV1` reader - Transition Latency Value 1 \\[TLV1\\]\n\nSpecifies the second of the two\n\ntransition latency values for the\n\nsubstates. The unit of latency is\n\nspecified by the Transition Latency\n\nUnit field of this register."]
pub type Tlv1R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - Maximum Number of Substates \\[MNS\\]\n\nMaximum number of DPA substates\n\nsupported by the Function (the value\n\nin this field is the number of\n\nsubstates minus 1)."]
    #[inline(always)]
    pub fn mns(&self) -> MnsR {
        MnsR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:7 - Reserved \\[R0\\]\n\nReserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bits 8:9 - Transition Latency Unit \\[TLU\\]\n\nThis is the unit of the transition\n\nlatencies specified in the Transition\n\nLatency Value 0 and Transition\n\nLatency Value 1 fields of this register\n\n(00 = 1ms, 01 = 10ms, 10 =\n\n100ms, 11 = reserved)."]
    #[inline(always)]
    pub fn tlu(&self) -> TluR {
        TluR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Reserved \\[R1\\]\n\nReserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Power Allocation Scale \\[PAS\\]\n\nThis is the scale used to compute the\n\nactual power from the values\n\nspecified in the Dynamic Power\n\nAllocation Array Registers 0 - 7. The\n\nactual power in Watts is obtained\n\nby multiplying the value in the\n\nDynamic Power Allocation Array\n\nRegister by this scale factor (00 =\n\n10x, 01 = 1x, 10 = 0.1x, 11 =\n\n0.01x)."]
    #[inline(always)]
    pub fn pas(&self) -> PasR {
        PasR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - Reserved \\[R2\\]\n\nReserved"]
    #[inline(always)]
    pub fn r2(&self) -> R2R {
        R2R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:23 - Transition Latency Value 0 \\[TLV0\\]\n\nSpecifies the transition latency for\n\nthe substate. Each of the 32\n\nsubstates may specify one of the two\n\ntransition latency values. This field\n\ncontains the first of the two latency\n\nvalues. The unit of latency is\n\nspecified by the Transition Latency\n\nUnit field of this register."]
    #[inline(always)]
    pub fn tlv0(&self) -> Tlv0R {
        Tlv0R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Transition Latency Value 1 \\[TLV1\\]\n\nSpecifies the second of the two\n\ntransition latency values for the\n\nsubstates. The unit of latency is\n\nspecified by the Transition Latency\n\nUnit field of this register."]
    #[inline(always)]
    pub fn tlv1(&self) -> Tlv1R {
        Tlv1R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "DPA Capability Register\n\nSpecifies the second of the two\n\ntransition latency values for the\n\nsubstates. The unit of latency is\n\nspecified by the Transition Latency\n\nUnit field of this register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_dpa_capability::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfDpaCapabilitySpec;
impl crate::RegisterSpec for PciePfDpaCapabilitySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_dpa_capability::R`](R) reader structure"]
impl crate::Readable for PciePfDpaCapabilitySpec {}
#[doc = "`reset()` method sets PCIE_PF_DPA_CAPABILITY to value 0x0010_0007"]
impl crate::Resettable for PciePfDpaCapabilitySpec {
    const RESET_VALUE: u32 = 0x0010_0007;
}
