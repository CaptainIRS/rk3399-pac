#[doc = "Register `PCIE_AT_OB_OUTBOUND_REGION_ADDRESS_0` reader"]
pub type R = crate::R<PcieAtObOutboundRegionAddress0Spec>;
#[doc = "Register `PCIE_AT_OB_OUTBOUND_REGION_ADDRESS_0` writer"]
pub type W = crate::W<PcieAtObOutboundRegionAddress0Spec>;
#[doc = "Field `num_bits` reader - Number_bits \\[5:0\\]
\\[num_bits\\]\n\nNumber of bits of the addres sthat\n\nare valid"]
pub type NumBitsR = crate::FieldReader;
#[doc = "Field `num_bits` writer - Number_bits \\[5:0\\]
\\[num_bits\\]\n\nNumber of bits of the addres sthat\n\nare valid"]
pub type NumBitsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `data` reader - Address bits \\[31:8\\]
\\[data\\]\n\nLower 32-bits of Address Register\n\nfor region N"]
pub type DataR = crate::FieldReader<u32>;
#[doc = "Field `data` writer - Address bits \\[31:8\\]
\\[data\\]\n\nLower 32-bits of Address Register\n\nfor region N"]
pub type DataW<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:5 - Number_bits \\[5:0\\]
\\[num_bits\\]\n\nNumber of bits of the addres sthat\n\nare valid"]
    #[inline(always)]
    pub fn num_bits(&self) -> NumBitsR {
        NumBitsR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:31 - Address bits \\[31:8\\]
\\[data\\]\n\nLower 32-bits of Address Register\n\nfor region N"]
    #[inline(always)]
    pub fn data(&self) -> DataR {
        DataR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:5 - Number_bits \\[5:0\\]
\\[num_bits\\]\n\nNumber of bits of the addres sthat\n\nare valid"]
    #[inline(always)]
    #[must_use]
    pub fn num_bits(&mut self) -> NumBitsW<PcieAtObOutboundRegionAddress0Spec> {
        NumBitsW::new(self, 0)
    }
    #[doc = "Bits 8:31 - Address bits \\[31:8\\]
\\[data\\]\n\nLower 32-bits of Address Register\n\nfor region N"]
    #[inline(always)]
    #[must_use]
    pub fn data(&mut self) -> DataW<PcieAtObOutboundRegionAddress0Spec> {
        DataW::new(self, 8)
    }
}
#[doc = "Outbound Region Address 0\n\nLower 32-bits of Address Register\n\nfor region N\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_ob_outbound_region_address_0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_at_ob_outbound_region_address_0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieAtObOutboundRegionAddress0Spec;
impl crate::RegisterSpec for PcieAtObOutboundRegionAddress0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_at_ob_outbound_region_address_0::R`](R) reader structure"]
impl crate::Readable for PcieAtObOutboundRegionAddress0Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_at_ob_outbound_region_address_0::W`](W) writer structure"]
impl crate::Writable for PcieAtObOutboundRegionAddress0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_AT_OB_OUTBOUND_REGION_ADDRESS_0 to value 0"]
impl crate::Resettable for PcieAtObOutboundRegionAddress0Spec {
    const RESET_VALUE: u32 = 0;
}
