#[doc = "Register `UART_SFE` reader"]
pub type R = crate::R<UartSfeSpec>;
#[doc = "Register `UART_SFE` writer"]
pub type W = crate::W<UartSfeSpec>;
#[doc = "Field `SHADOW_FIFO_EN` reader - Shadow FIFO Enable. Shadow FIFO Enable. This is a shadow register for the FIFO enable bit (FCR\\[0\\])."]
pub type ShadowFifoEnR = crate::BitReader;
#[doc = "Field `SHADOW_FIFO_EN` writer - Shadow FIFO Enable. Shadow FIFO Enable. This is a shadow register for the FIFO enable bit (FCR\\[0\\])."]
pub type ShadowFifoEnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Shadow FIFO Enable. Shadow FIFO Enable. This is a shadow register for the FIFO enable bit (FCR\\[0\\])."]
    #[inline(always)]
    pub fn shadow_fifo_en(&self) -> ShadowFifoEnR {
        ShadowFifoEnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Shadow FIFO Enable. Shadow FIFO Enable. This is a shadow register for the FIFO enable bit (FCR\\[0\\])."]
    #[inline(always)]
    #[must_use]
    pub fn shadow_fifo_en(&mut self) -> ShadowFifoEnW<UartSfeSpec> {
        ShadowFifoEnW::new(self, 0)
    }
}
#[doc = "Shadow FIFO Enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uart_sfe::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uart_sfe::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UartSfeSpec;
impl crate::RegisterSpec for UartSfeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uart_sfe::R`](R) reader structure"]
impl crate::Readable for UartSfeSpec {}
#[doc = "`write(|w| ..)` method takes [`uart_sfe::W`](W) writer structure"]
impl crate::Writable for UartSfeSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UART_SFE to value 0"]
impl crate::Resettable for UartSfeSpec {
    const RESET_VALUE: u32 = 0;
}
