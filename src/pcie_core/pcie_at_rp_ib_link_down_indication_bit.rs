#[doc = "Register `PCIE_AT_RP_IB_LINK_DOWN_INDICATION_BIT` reader"]
pub type R = crate::R<PcieAtRpIbLinkDownIndicationBitSpec>;
#[doc = "Register `PCIE_AT_RP_IB_LINK_DOWN_INDICATION_BIT` writer"]
pub type W = crate::W<PcieAtRpIbLinkDownIndicationBitSpec>;
#[doc = "Field `clear_link_down_bit` reader - Link down indication bit \\[clear_link_down_bit\\]\n\nThis bit will be set when link\n\ndown reset comes. client should\n\nclear this bit before issueing\n\nnew traffic"]
pub type ClearLinkDownBitR = crate::BitReader;
#[doc = "Field `clear_link_down_bit` writer - Link down indication bit \\[clear_link_down_bit\\]\n\nThis bit will be set when link\n\ndown reset comes. client should\n\nclear this bit before issueing\n\nnew traffic"]
pub type ClearLinkDownBitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Link down indication bit \\[clear_link_down_bit\\]\n\nThis bit will be set when link\n\ndown reset comes. client should\n\nclear this bit before issueing\n\nnew traffic"]
    #[inline(always)]
    pub fn clear_link_down_bit(&self) -> ClearLinkDownBitR {
        ClearLinkDownBitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Link down indication bit \\[clear_link_down_bit\\]\n\nThis bit will be set when link\n\ndown reset comes. client should\n\nclear this bit before issueing\n\nnew traffic"]
    #[inline(always)]
    #[must_use]
    pub fn clear_link_down_bit(
        &mut self,
    ) -> ClearLinkDownBitW<PcieAtRpIbLinkDownIndicationBitSpec> {
        ClearLinkDownBitW::new(self, 0)
    }
}
#[doc = "Link down indication bit\n\nRSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_at_rp_ib_link_down_indication_bit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_at_rp_ib_link_down_indication_bit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieAtRpIbLinkDownIndicationBitSpec;
impl crate::RegisterSpec for PcieAtRpIbLinkDownIndicationBitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_at_rp_ib_link_down_indication_bit::R`](R) reader structure"]
impl crate::Readable for PcieAtRpIbLinkDownIndicationBitSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_at_rp_ib_link_down_indication_bit::W`](W) writer structure"]
impl crate::Writable for PcieAtRpIbLinkDownIndicationBitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_AT_RP_IB_LINK_DOWN_INDICATION_BIT to value 0"]
impl crate::Resettable for PcieAtRpIbLinkDownIndicationBitSpec {
    const RESET_VALUE: u32 = 0;
}
