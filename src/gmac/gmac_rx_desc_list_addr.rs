#[doc = "Register `GMAC_RX_DESC_LIST_ADDR` reader"]
pub type R = crate::R<GmacRxDescListAddrSpec>;
#[doc = "Register `GMAC_RX_DESC_LIST_ADDR` writer"]
pub type W = crate::W<GmacRxDescListAddrSpec>;
#[doc = "Field `SRL` reader - Start of Receive List This field contains the base address of the First Descriptor in the Receive Descriptor list. The LSB bits \\[1/2/3:0\\]
for 32/64/128-bit bus width) will be ignored and taken as all-zero by the DMA internally. Hence these LSB bits are Read Only."]
pub type SrlR = crate::FieldReader<u32>;
#[doc = "Field `SRL` writer - Start of Receive List This field contains the base address of the First Descriptor in the Receive Descriptor list. The LSB bits \\[1/2/3:0\\]
for 32/64/128-bit bus width) will be ignored and taken as all-zero by the DMA internally. Hence these LSB bits are Read Only."]
pub type SrlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Receive List This field contains the base address of the First Descriptor in the Receive Descriptor list. The LSB bits \\[1/2/3:0\\]
for 32/64/128-bit bus width) will be ignored and taken as all-zero by the DMA internally. Hence these LSB bits are Read Only."]
    #[inline(always)]
    pub fn srl(&self) -> SrlR {
        SrlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Receive List This field contains the base address of the First Descriptor in the Receive Descriptor list. The LSB bits \\[1/2/3:0\\]
for 32/64/128-bit bus width) will be ignored and taken as all-zero by the DMA internally. Hence these LSB bits are Read Only."]
    #[inline(always)]
    #[must_use]
    pub fn srl(&mut self) -> SrlW<GmacRxDescListAddrSpec> {
        SrlW::new(self, 0)
    }
}
#[doc = "Receive Descriptor List Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_rx_desc_list_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`gmac_rx_desc_list_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacRxDescListAddrSpec;
impl crate::RegisterSpec for GmacRxDescListAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_rx_desc_list_addr::R`](R) reader structure"]
impl crate::Readable for GmacRxDescListAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`gmac_rx_desc_list_addr::W`](W) writer structure"]
impl crate::Writable for GmacRxDescListAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GMAC_RX_DESC_LIST_ADDR to value 0"]
impl crate::Resettable for GmacRxDescListAddrSpec {
    const RESET_VALUE: u32 = 0;
}
