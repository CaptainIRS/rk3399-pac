#[doc = "Register `PCIE_DMA_CHANNEL_0_START_POINTER_UPPER` reader"]
pub type R = crate::R<PcieDmaChannel0StartPointerUpperSpec>;
#[doc = "Register `PCIE_DMA_CHANNEL_0_START_POINTER_UPPER` writer"]
pub type W = crate::W<PcieDmaChannel0StartPointerUpperSpec>;
#[doc = "Field `ptr` reader - Start Pointer Upper DWORD \\[ptr\\]
Upper 32-bits Pointer Address Registers"]
pub type PtrR = crate::FieldReader<u32>;
#[doc = "Field `ptr` writer - Start Pointer Upper DWORD \\[ptr\\]
Upper 32-bits Pointer Address Registers"]
pub type PtrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start Pointer Upper DWORD \\[ptr\\]
Upper 32-bits Pointer Address Registers"]
    #[inline(always)]
    pub fn ptr(&self) -> PtrR {
        PtrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start Pointer Upper DWORD \\[ptr\\]
Upper 32-bits Pointer Address Registers"]
    #[inline(always)]
    #[must_use]
    pub fn ptr(&mut self) -> PtrW<PcieDmaChannel0StartPointerUpperSpec> {
        PtrW::new(self, 0)
    }
}
#[doc = "PCIe DMA Channel 0 Start Pointer Upper Register Upper 32-bits Pointer Address Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_dma_channel_0_start_pointer_upper::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_dma_channel_0_start_pointer_upper::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieDmaChannel0StartPointerUpperSpec;
impl crate::RegisterSpec for PcieDmaChannel0StartPointerUpperSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_dma_channel_0_start_pointer_upper::R`](R) reader structure"]
impl crate::Readable for PcieDmaChannel0StartPointerUpperSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_dma_channel_0_start_pointer_upper::W`](W) writer structure"]
impl crate::Writable for PcieDmaChannel0StartPointerUpperSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_DMA_CHANNEL_0_START_POINTER_UPPER to value 0"]
impl crate::Resettable for PcieDmaChannel0StartPointerUpperSpec {
    const RESET_VALUE: u32 = 0;
}
