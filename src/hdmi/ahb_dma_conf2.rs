#[doc = "Register `AHB_DMA_CONF2` reader"]
pub type R = crate::R<AhbDmaConf2Spec>;
#[doc = "Register `AHB_DMA_CONF2` writer"]
pub type W = crate::W<AhbDmaConf2Spec>;
#[doc = "Field `AUTOSTART_ENABLE` reader - Enables the AHB audio DMA auto-start feature"]
pub type AutostartEnableR = crate::BitReader;
#[doc = "Field `AUTOSTART_ENABLE` writer - Enables the AHB audio DMA auto-start feature"]
pub type AutostartEnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTOSTART_LOOP` reader - Enables the AHB audio DMA auto-start loop mode"]
pub type AutostartLoopR = crate::BitReader;
#[doc = "Field `AUTOSTART_LOOP` writer - Enables the AHB audio DMA auto-start loop mode"]
pub type AutostartLoopW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Enables the AHB audio DMA auto-start feature"]
    #[inline(always)]
    pub fn autostart_enable(&self) -> AutostartEnableR {
        AutostartEnableR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enables the AHB audio DMA auto-start loop mode"]
    #[inline(always)]
    pub fn autostart_loop(&self) -> AutostartLoopR {
        AutostartLoopR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enables the AHB audio DMA auto-start feature"]
    #[inline(always)]
    #[must_use]
    pub fn autostart_enable(&mut self) -> AutostartEnableW<AhbDmaConf2Spec> {
        AutostartEnableW::new(self, 0)
    }
    #[doc = "Bit 1 - Enables the AHB audio DMA auto-start loop mode"]
    #[inline(always)]
    #[must_use]
    pub fn autostart_loop(&mut self) -> AutostartLoopW<AhbDmaConf2Spec> {
        AutostartLoopW::new(self, 1)
    }
}
#[doc = "Audio DMA Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_conf2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_conf2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbDmaConf2Spec;
impl crate::RegisterSpec for AhbDmaConf2Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ahb_dma_conf2::R`](R) reader structure"]
impl crate::Readable for AhbDmaConf2Spec {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_conf2::W`](W) writer structure"]
impl crate::Writable for AhbDmaConf2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AHB_DMA_CONF2 to value 0x02"]
impl crate::Resettable for AhbDmaConf2Spec {
    const RESET_VALUE: u8 = 0x02;
}
