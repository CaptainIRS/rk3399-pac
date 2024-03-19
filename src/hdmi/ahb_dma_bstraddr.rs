#[doc = "Register `AHB_DMA_BSTRADDR[%s]` reader"]
pub type R = crate::R<AhbDmaBstraddrSpec>;
#[doc = "Field `BURST_ADDR` reader - Start address for the current burst operation"]
pub type BurstAddrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Start address for the current burst operation"]
    #[inline(always)]
    pub fn burst_addr(&self) -> BurstAddrR {
        BurstAddrR::new(self.bits)
    }
}
#[doc = "Audio DMA Burst Start Address Register Array Address offset: i = 0 to 3\n\nThese read-only registers compose the start address of the current burst operation.\n\nburst_start_addr\\[31:0\\]
= haddr\\[31:0\\]
= initial_addr\\[31:0\\]
+ 16.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_bstraddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbDmaBstraddrSpec;
impl crate::RegisterSpec for AhbDmaBstraddrSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ahb_dma_bstraddr::R`](R) reader structure"]
impl crate::Readable for AhbDmaBstraddrSpec {}
#[doc = "`reset()` method sets AHB_DMA_BSTRADDR[%s]
to value 0"]
impl crate::Resettable for AhbDmaBstraddrSpec {
    const RESET_VALUE: u8 = 0;
}
