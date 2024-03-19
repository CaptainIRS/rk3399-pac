#[doc = "Register `AHB_DMA_STPADDR_SET0[%s]` reader"]
pub type R = crate::R<AhbDmaStpaddrSet0Spec>;
#[doc = "Register `AHB_DMA_STPADDR_SET0[%s]` writer"]
pub type W = crate::W<AhbDmaStpaddrSet0Spec>;
#[doc = "Field `FINAL_ADDR` reader - Defines final_addr\\[7:0\\]
to end DMA burst transactions"]
pub type FinalAddrR = crate::FieldReader;
#[doc = "Field `FINAL_ADDR` writer - Defines final_addr\\[7:0\\]
to end DMA burst transactions"]
pub type FinalAddrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Defines final_addr\\[7:0\\]
to end DMA burst transactions"]
    #[inline(always)]
    pub fn final_addr(&self) -> FinalAddrR {
        FinalAddrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines final_addr\\[7:0\\]
to end DMA burst transactions"]
    #[inline(always)]
    #[must_use]
    pub fn final_addr(&mut self) -> FinalAddrW<AhbDmaStpaddrSet0Spec> {
        FinalAddrW::new(self, 0)
    }
}
#[doc = "Audio DMA Stop Address Set0 Register Array Address offset: i = 0 to 3\n\nThis registers define the final_addr\\[31:0\\]
used as the final point to the DMA burst read\n\ntransactions.\n\nUpon start_dma_transaction configuration, the DMA engine starts requesting burst reads\n\nfrom the external system memory. Each burst read can have a maximum theoretical length\n\nof 256 words (due to the AMBA AHB specification 1 Kbyte boundary burst limitation).\n\nThe DMA engine is responsible for incrementing the burst starting address and defining its\n\ncorresponding burst length to reach the final_addr\\[31:0\\]
address. The last burst request\n\nissued by the DMA engine takes into account that it should only request data until the\n\nfinal_addr\\[31:0\\]
address (included) and for that should calculate the correct burst length.\n\nAfter reaching the final_addr\\[31:0\\]
address, the done interrupt is active to signal\n\ncompletion of DMA operation.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_stpaddr_set0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_stpaddr_set0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbDmaStpaddrSet0Spec;
impl crate::RegisterSpec for AhbDmaStpaddrSet0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ahb_dma_stpaddr_set0::R`](R) reader structure"]
impl crate::Readable for AhbDmaStpaddrSet0Spec {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_stpaddr_set0::W`](W) writer structure"]
impl crate::Writable for AhbDmaStpaddrSet0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AHB_DMA_STPADDR_SET0[%s]
to value 0"]
impl crate::Resettable for AhbDmaStpaddrSet0Spec {
    const RESET_VALUE: u8 = 0;
}
