#[doc = "Register `MI_DMA_STATUS` reader"]
pub type R = crate::R<MiDmaStatusSpec>;
#[doc = "Field `dma_active` reader - If set DMA access is active.\n\n"]
pub type DmaActiveR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - If set DMA access is active.\n\n"]
    #[inline(always)]
    pub fn dma_active(&self) -> DmaActiveR {
        DmaActiveR::new((self.bits & 1) != 0)
    }
}
#[doc = "DMA status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mi_dma_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MiDmaStatusSpec;
impl crate::RegisterSpec for MiDmaStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mi_dma_status::R`](R) reader structure"]
impl crate::Readable for MiDmaStatusSpec {}
#[doc = "`reset()` method sets MI_DMA_STATUS to value 0"]
impl crate::Resettable for MiDmaStatusSpec {
    const RESET_VALUE: u32 = 0;
}
