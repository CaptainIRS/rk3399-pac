#[doc = "Register `PCIE_LM_NEGOTIATED_LANE_MAP` reader"]
pub type R = crate::R<PcieLmNegotiatedLaneMapSpec>;
#[doc = "Field `NLM` reader - Negotiated Lane Map \\[NLM\\]\n\nBit i of this field is set to 1 at the end\n\nof link training if Lane i is part of the\n\nPCIe link. The value of this field is\n\nvalid only when the link is in L0 or\n\nL0s states."]
pub type NlmR = crate::FieldReader;
#[doc = "Field `R70` reader - Reserved \\[R70\\]\n\nReserved"]
pub type R70R = crate::FieldReader<u16>;
#[doc = "Field `LRS` reader - Lane Reversal Status \\[LRS\\]\n\nThis bit set by the core at the end of\n\nlink training if the LTSSM had to\n\nreverse the lane numbers to form\n\nthe link."]
pub type LrsR = crate::BitReader;
#[doc = "Field `R71` reader - Reserved \\[R71\\]\n\nReserved"]
pub type R71R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Negotiated Lane Map \\[NLM\\]\n\nBit i of this field is set to 1 at the end\n\nof link training if Lane i is part of the\n\nPCIe link. The value of this field is\n\nvalid only when the link is in L0 or\n\nL0s states."]
    #[inline(always)]
    pub fn nlm(&self) -> NlmR {
        NlmR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:15 - Reserved \\[R70\\]\n\nReserved"]
    #[inline(always)]
    pub fn r70(&self) -> R70R {
        R70R::new(((self.bits >> 4) & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Lane Reversal Status \\[LRS\\]\n\nThis bit set by the core at the end of\n\nlink training if the LTSSM had to\n\nreverse the lane numbers to form\n\nthe link."]
    #[inline(always)]
    pub fn lrs(&self) -> LrsR {
        LrsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - Reserved \\[R71\\]\n\nReserved"]
    #[inline(always)]
    pub fn r71(&self) -> R71R {
        R71R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
#[doc = "Negotiated Lane Map Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_negotiated_lane_map::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmNegotiatedLaneMapSpec;
impl crate::RegisterSpec for PcieLmNegotiatedLaneMapSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_negotiated_lane_map::R`](R) reader structure"]
impl crate::Readable for PcieLmNegotiatedLaneMapSpec {}
#[doc = "`reset()` method sets PCIE_LM_NEGOTIATED_LANE_MAP to value 0"]
impl crate::Resettable for PcieLmNegotiatedLaneMapSpec {
    const RESET_VALUE: u32 = 0;
}
