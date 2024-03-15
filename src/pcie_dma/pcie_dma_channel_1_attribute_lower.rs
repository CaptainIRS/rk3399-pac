#[doc = "Register `PCIE_DMA_CHANNEL_1_ATTRIBUTE_LOWER` reader"]
pub type R = crate::R<PcieDmaChannel1AttributeLowerSpec>;
#[doc = "Register `PCIE_DMA_CHANNEL_1_ATTRIBUTE_LOWER` writer"]
pub type W = crate::W<PcieDmaChannel1AttributeLowerSpec>;
#[doc = "Field `attr` reader - Descriptor Attributes Lower DWORD \\[attr\\]
Lower 32-bits Attribute Values used when fetching and returning link list descriptors"]
pub type AttrR = crate::FieldReader<u32>;
#[doc = "Field `attr` writer - Descriptor Attributes Lower DWORD \\[attr\\]
Lower 32-bits Attribute Values used when fetching and returning link list descriptors"]
pub type AttrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Descriptor Attributes Lower DWORD \\[attr\\]
Lower 32-bits Attribute Values used when fetching and returning link list descriptors"]
    #[inline(always)]
    pub fn attr(&self) -> AttrR {
        AttrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Descriptor Attributes Lower DWORD \\[attr\\]
Lower 32-bits Attribute Values used when fetching and returning link list descriptors"]
    #[inline(always)]
    #[must_use]
    pub fn attr(&mut self) -> AttrW<PcieDmaChannel1AttributeLowerSpec> {
        AttrW::new(self, 0)
    }
}
#[doc = "PCIe DMA Channel 1 Attribute Lower Register Lower 32-bits Attribute Values used when fetching and returning link list descriptors\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_1_attribute_lower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_1_attribute_lower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieDmaChannel1AttributeLowerSpec;
impl crate::RegisterSpec for PcieDmaChannel1AttributeLowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_dma_channel_1_attribute_lower::R`](R) reader structure"]
impl crate::Readable for PcieDmaChannel1AttributeLowerSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_dma_channel_1_attribute_lower::W`](W) writer structure"]
impl crate::Writable for PcieDmaChannel1AttributeLowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_DMA_CHANNEL_1_ATTRIBUTE_LOWER to value 0"]
impl crate::Resettable for PcieDmaChannel1AttributeLowerSpec {
    const RESET_VALUE: u32 = 0;
}
