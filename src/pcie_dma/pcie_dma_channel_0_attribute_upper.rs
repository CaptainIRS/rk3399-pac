#[doc = "Register `PCIE_DMA_CHANNEL_0_ATTRIBUTE_UPPER` reader"]
pub type R = crate::R<PcieDmaChannel0AttributeUpperSpec>;
#[doc = "Register `PCIE_DMA_CHANNEL_0_ATTRIBUTE_UPPER` writer"]
pub type W = crate::W<PcieDmaChannel0AttributeUpperSpec>;
#[doc = "Field `attr` reader - Descriptor Attributes Upper DWORD \\[attr\\]
Upper 32-bit Attribute Values used when fetching and returning link list descriptors"]
pub type AttrR = crate::FieldReader<u32>;
#[doc = "Field `attr` writer - Descriptor Attributes Upper DWORD \\[attr\\]
Upper 32-bit Attribute Values used when fetching and returning link list descriptors"]
pub type AttrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Descriptor Attributes Upper DWORD \\[attr\\]
Upper 32-bit Attribute Values used when fetching and returning link list descriptors"]
    #[inline(always)]
    pub fn attr(&self) -> AttrR {
        AttrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Descriptor Attributes Upper DWORD \\[attr\\]
Upper 32-bit Attribute Values used when fetching and returning link list descriptors"]
    #[inline(always)]
    #[must_use]
    pub fn attr(&mut self) -> AttrW<PcieDmaChannel0AttributeUpperSpec> {
        AttrW::new(self, 0)
    }
}
#[doc = "PCIe DMA Channel 0 Attribute Upper Register Upper 32-bit Attribute Values used when fetching and returning link list descriptors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_0_attribute_upper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_0_attribute_upper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieDmaChannel0AttributeUpperSpec;
impl crate::RegisterSpec for PcieDmaChannel0AttributeUpperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_dma_channel_0_attribute_upper::R`](R) reader structure"]
impl crate::Readable for PcieDmaChannel0AttributeUpperSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_dma_channel_0_attribute_upper::W`](W) writer structure"]
impl crate::Writable for PcieDmaChannel0AttributeUpperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_DMA_CHANNEL_0_ATTRIBUTE_UPPER to value 0"]
impl crate::Resettable for PcieDmaChannel0AttributeUpperSpec {
    const RESET_VALUE: u32 = 0;
}
