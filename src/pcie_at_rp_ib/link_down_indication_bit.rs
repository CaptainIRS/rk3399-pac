#[doc = "Register `LINK_DOWN_INDICATION_BIT` reader"]
pub type R = crate::R<LinkDownIndicationBitSpec>;
#[doc = "Register `LINK_DOWN_INDICATION_BIT` writer"]
pub type W = crate::W<LinkDownIndicationBitSpec>;
#[doc = "Field `clear_link_down_bit` reader - Link down indication bit \\[clear_link_down_bit\\]
This bit will be set when link down reset comes. client should clear this bit before issueing new traffic"]
pub type ClearLinkDownBitR = crate::BitReader;
#[doc = "Field `clear_link_down_bit` writer - Link down indication bit \\[clear_link_down_bit\\]
This bit will be set when link down reset comes. client should clear this bit before issueing new traffic"]
pub type ClearLinkDownBitW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Link down indication bit \\[clear_link_down_bit\\]
This bit will be set when link down reset comes. client should clear this bit before issueing new traffic"]
    #[inline(always)]
    pub fn clear_link_down_bit(&self) -> ClearLinkDownBitR {
        ClearLinkDownBitR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Link down indication bit \\[clear_link_down_bit\\]
This bit will be set when link down reset comes. client should clear this bit before issueing new traffic"]
    #[inline(always)]
    #[must_use]
    pub fn clear_link_down_bit(&mut self) -> ClearLinkDownBitW<LinkDownIndicationBitSpec> {
        ClearLinkDownBitW::new(self, 0)
    }
}
#[doc = "Link down indication bit RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`link_down_indication_bit::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`link_down_indication_bit::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LinkDownIndicationBitSpec;
impl crate::RegisterSpec for LinkDownIndicationBitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`link_down_indication_bit::R`](R) reader structure"]
impl crate::Readable for LinkDownIndicationBitSpec {}
#[doc = "`write(|w| ..)` method takes [`link_down_indication_bit::W`](W) writer structure"]
impl crate::Writable for LinkDownIndicationBitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LINK_DOWN_INDICATION_BIT to value 0"]
impl crate::Resettable for LinkDownIndicationBitSpec {
    const RESET_VALUE: u32 = 0;
}
