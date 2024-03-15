#[doc = "Register `UART_SDMAM` reader"]
pub type R = crate::R<UartSdmamSpec>;
#[doc = "Register `UART_SDMAM` writer"]
pub type W = crate::W<UartSdmamSpec>;
#[doc = "Field `SHADOW_DMA_MODE` reader - Shadow DMA Mode. This is a shadow register for the DMA mode bit (FCR\\[3\\])."]
pub type ShadowDmaModeR = crate::BitReader;
#[doc = "Field `SHADOW_DMA_MODE` writer - Shadow DMA Mode. This is a shadow register for the DMA mode bit (FCR\\[3\\])."]
pub type ShadowDmaModeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shadow DMA Mode. This is a shadow register for the DMA mode bit (FCR\\[3\\])."]
    #[inline(always)]
    pub fn shadow_dma_mode(&self) -> ShadowDmaModeR {
        ShadowDmaModeR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow DMA Mode. This is a shadow register for the DMA mode bit (FCR\\[3\\])."]
    #[inline(always)]
    #[must_use]
    pub fn shadow_dma_mode(&mut self) -> ShadowDmaModeW<UartSdmamSpec> {
        ShadowDmaModeW::new(self, 0)
    }
}
#[doc = "Shadow DMA Mode\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_sdmam::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_sdmam::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartSdmamSpec;
impl crate::RegisterSpec for UartSdmamSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_sdmam::R`](R) reader structure"]
impl crate::Readable for UartSdmamSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_sdmam::W`](W) writer structure"]
impl crate::Writable for UartSdmamSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_SDMAM to value 0"]
impl crate::Resettable for UartSdmamSpec {
    const RESET_VALUE: u32 = 0;
}
