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
#[doc = "Start address for the current burst operation\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_bstraddr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
