#[doc = "Register `AHB_DMA_STRADDR_SET1[%s]` reader"]
pub type R = crate::R<AhbDmaStraddrSet1Spec>;
#[doc = "Register `AHB_DMA_STRADDR_SET1[%s]` writer"]
pub type W = crate::W<AhbDmaStraddrSet1Spec>;
#[doc = "Field `INITIAL_ADDR_1` reader - Defines init_addr_1\\[7:0\\]
to initiate DMA burst transactions"]
pub type InitialAddr1R = crate::FieldReader;
#[doc = "Field `INITIAL_ADDR_1` writer - Defines init_addr_1\\[7:0\\]
to initiate DMA burst transactions"]
pub type InitialAddr1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Defines init_addr_1\\[7:0\\]
to initiate DMA burst transactions"]
    #[inline(always)]
    pub fn initial_addr_1(&self) -> InitialAddr1R {
        InitialAddr1R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Defines init_addr_1\\[7:0\\]
to initiate DMA burst transactions"]
    #[inline(always)]
    #[must_use]
    pub fn initial_addr_1(&mut self) -> InitialAddr1W<AhbDmaStraddrSet1Spec> {
        InitialAddr1W::new(self, 0)
    }
}
#[doc = "Defines init_addr_1\\[7:0\\]
to initiate DMA burst transactions\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_straddr_set1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_straddr_set1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbDmaStraddrSet1Spec;
impl crate::RegisterSpec for AhbDmaStraddrSet1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ahb_dma_straddr_set1::R`](R) reader structure"]
impl crate::Readable for AhbDmaStraddrSet1Spec {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_straddr_set1::W`](W) writer structure"]
impl crate::Writable for AhbDmaStraddrSet1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AHB_DMA_STRADDR_SET1[%s]
to value 0"]
impl crate::Resettable for AhbDmaStraddrSet1Spec {
    const RESET_VALUE: u8 = 0;
}
