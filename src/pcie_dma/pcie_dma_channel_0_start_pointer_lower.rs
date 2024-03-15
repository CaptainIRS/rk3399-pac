#[doc = "Register `PCIE_DMA_CHANNEL_0_START_POINTER_LOWER` reader"]
pub type R = crate::R<PcieDmaChannel0StartPointerLowerSpec>;
#[doc = "Register `PCIE_DMA_CHANNEL_0_START_POINTER_LOWER` writer"]
pub type W = crate::W<PcieDmaChannel0StartPointerLowerSpec>;
#[doc = "Field `ptr` reader - Start pointer Lower DWORD \\[ptr\\]
Lower 32-bits Pointer Address Registers"]
pub type PtrR = crate::FieldReader<u32>;
#[doc = "Field `ptr` writer - Start pointer Lower DWORD \\[ptr\\]
Lower 32-bits Pointer Address Registers"]
pub type PtrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start pointer Lower DWORD \\[ptr\\]
Lower 32-bits Pointer Address Registers"]
    #[inline(always)]
    pub fn ptr(&self) -> PtrR {
        PtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start pointer Lower DWORD \\[ptr\\]
Lower 32-bits Pointer Address Registers"]
    #[inline(always)]
    #[must_use]
    pub fn ptr(&mut self) -> PtrW<PcieDmaChannel0StartPointerLowerSpec> {
        PtrW::new(self, 0)
    }
}
#[doc = "PCIe DMA Channel 0 Start Pointer Lower Register Lower 32-bits Pointer Address Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_0_start_pointer_lower::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_0_start_pointer_lower::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieDmaChannel0StartPointerLowerSpec;
impl crate::RegisterSpec for PcieDmaChannel0StartPointerLowerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_dma_channel_0_start_pointer_lower::R`](R) reader structure"]
impl crate::Readable for PcieDmaChannel0StartPointerLowerSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_dma_channel_0_start_pointer_lower::W`](W) writer structure"]
impl crate::Writable for PcieDmaChannel0StartPointerLowerSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_DMA_CHANNEL_0_START_POINTER_LOWER to value 0"]
impl crate::Resettable for PcieDmaChannel0StartPointerLowerSpec {
    const RESET_VALUE: u32 = 0;
}
