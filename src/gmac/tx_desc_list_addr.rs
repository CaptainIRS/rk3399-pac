#[doc = "Register `TX_DESC_LIST_ADDR` reader"]
pub type R = crate::R<TxDescListAddrSpec>;
#[doc = "Register `TX_DESC_LIST_ADDR` writer"]
pub type W = crate::W<TxDescListAddrSpec>;
#[doc = "Field `STL` reader - Start of Transmit List\n\nThis field contains the base address of the First Descriptor in the\n\nTransmit Descriptor list. The LSB bits \\[1/2/3:0\\]
for 32/64/128-bit\n\nbus width) will be ignored and taken as all-zero by the DMA\n\ninternally. Hence these LSB bits are Read Only."]
pub type StlR = crate::FieldReader<u32>;
#[doc = "Field `STL` writer - Start of Transmit List\n\nThis field contains the base address of the First Descriptor in the\n\nTransmit Descriptor list. The LSB bits \\[1/2/3:0\\]
for 32/64/128-bit\n\nbus width) will be ignored and taken as all-zero by the DMA\n\ninternally. Hence these LSB bits are Read Only."]
pub type StlW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start of Transmit List\n\nThis field contains the base address of the First Descriptor in the\n\nTransmit Descriptor list. The LSB bits \\[1/2/3:0\\]
for 32/64/128-bit\n\nbus width) will be ignored and taken as all-zero by the DMA\n\ninternally. Hence these LSB bits are Read Only."]
    #[inline(always)]
    pub fn stl(&self) -> StlR {
        StlR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start of Transmit List\n\nThis field contains the base address of the First Descriptor in the\n\nTransmit Descriptor list. The LSB bits \\[1/2/3:0\\]
for 32/64/128-bit\n\nbus width) will be ignored and taken as all-zero by the DMA\n\ninternally. Hence these LSB bits are Read Only."]
    #[inline(always)]
    #[must_use]
    pub fn stl(&mut self) -> StlW<TxDescListAddrSpec> {
        StlW::new(self, 0)
    }
}
#[doc = "Transmit Descriptor List Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_desc_list_addr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tx_desc_list_addr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxDescListAddrSpec;
impl crate::RegisterSpec for TxDescListAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_desc_list_addr::R`](R) reader structure"]
impl crate::Readable for TxDescListAddrSpec {}
#[doc = "`write(|w| ..)` method takes [`tx_desc_list_addr::W`](W) writer structure"]
impl crate::Writable for TxDescListAddrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TX_DESC_LIST_ADDR to value 0"]
impl crate::Resettable for TxDescListAddrSpec {
    const RESET_VALUE: u32 = 0;
}
