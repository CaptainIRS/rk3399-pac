#[doc = "Register `PCIE_AT_RP_IB_RP_INBOUND_BAR_ADDRESS_TRANSLATION_0` reader"]
pub type R = crate::R<PcieAtRpIbRpInboundBarAddressTranslation0Spec>;
#[doc = "Register `PCIE_AT_RP_IB_RP_INBOUND_BAR_ADDRESS_TRANSLATION_0` writer"]
pub type W = crate::W<PcieAtRpIbRpInboundBarAddressTranslation0Spec>;
#[doc = "Field `num_bits` reader - Number_bits \\[5:0\\]
\\[num_bits\\]
Number of bits - 1 of the PCIE address passed through"]
pub type NumBitsR = crate::FieldReader;
#[doc = "Field `num_bits` writer - Number_bits \\[5:0\\]
\\[num_bits\\]
Number of bits - 1 of the PCIE address passed through"]
pub type NumBitsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `rsvd0` reader - Reserved \\[rsvd0\\]
Bits 7 and 6 are reserved"]
pub type Rsvd0R = crate::FieldReader;
#[doc = "Field `data` reader - Address bits \\[31:8\\]
\\[data\\]
Bits \\[31:8\\]
of Address Register for BAR N"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `data` writer - Address bits \\[31:8\\]
\\[data\\]
Bits \\[31:8\\]
of Address Register for BAR N"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:5 - Number_bits \\[5:0\\]
\\[num_bits\\]
Number of bits - 1 of the PCIE address passed through"]
    #[inline(always)]
    pub fn num_bits(&self) -> NumBitsR {
        NumBitsR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - Reserved \\[rsvd0\\]
Bits 7 and 6 are reserved"]
    #[inline(always)]
    pub fn rsvd0(&self) -> Rsvd0R {
        Rsvd0R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:31 - Address bits \\[31:8\\]
\\[data\\]
Bits \\[31:8\\]
of Address Register for BAR N"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number_bits \\[5:0\\]
\\[num_bits\\]
Number of bits - 1 of the PCIE address passed through"]
    #[inline(always)]
    #[must_use]
    pub fn num_bits(&mut self) -> NumBitsW<PcieAtRpIbRpInboundBarAddressTranslation0Spec> {
        NumBitsW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Address bits \\[31:8\\]
\\[data\\]
Bits \\[31:8\\]
of Address Register for BAR N"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<PcieAtRpIbRpInboundBarAddressTranslation0Spec> {
        DataW::new(self, 8)
    }
}
#[doc = "RP Inbound BAR Address Translation 0 Bits \\[31:8\\]
of Address Register for BAR N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_rp_ib_rp_inbound_bar_address_translation_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_at_rp_ib_rp_inbound_bar_address_translation_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieAtRpIbRpInboundBarAddressTranslation0Spec;
impl crate::RegisterSpec for PcieAtRpIbRpInboundBarAddressTranslation0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_at_rp_ib_rp_inbound_bar_address_translation_0::R`](R) reader structure"]
impl crate::Readable for PcieAtRpIbRpInboundBarAddressTranslation0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_at_rp_ib_rp_inbound_bar_address_translation_0::W`](W) writer structure"]
impl crate::Writable for PcieAtRpIbRpInboundBarAddressTranslation0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_AT_RP_IB_RP_INBOUND_BAR_ADDRESS_TRANSLATION_0 to value 0"]
impl crate::Resettable for PcieAtRpIbRpInboundBarAddressTranslation0Spec {
    const RESET_VALUE: u32 = 0;
}
