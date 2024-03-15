#[doc = "Register `AHB_DMA_CONF1` reader"]
pub type R = crate::R<AhbDmaConf1Spec>;
#[doc = "Register `AHB_DMA_CONF1` writer"]
pub type W = crate::W<AhbDmaConf1Spec>;
#[doc = "Field `CH_IN_EN` reader - Each bit controls the enabling of the respective audio channel. For instance, when bit 1 is set (1'b1) the audio Channel 1 is enabled. When cleared, the referred channel is disabled."]
pub type ChInEnR = crate::FieldReader;
#[doc = "Field `CH_IN_EN` writer - Each bit controls the enabling of the respective audio channel. For instance, when bit 1 is set (1'b1) the audio Channel 1 is enabled. When cleared, the referred channel is disabled."]
pub type ChInEnW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Each bit controls the enabling of the respective audio channel. For instance, when bit 1 is set (1'b1) the audio Channel 1 is enabled. When cleared, the referred channel is disabled."]
    #[inline(always)]
    pub fn ch_in_en(&self) -> ChInEnR {
        ChInEnR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - Each bit controls the enabling of the respective audio channel. For instance, when bit 1 is set (1'b1) the audio Channel 1 is enabled. When cleared, the referred channel is disabled."]
    #[inline(always)]
    #[must_use]
    pub fn ch_in_en(&mut self) -> ChInEnW<AhbDmaConf1Spec> {
        ChInEnW::new(self, 0)
    }
}
#[doc = "Each bit controls the enabling of the respective audio channel. For instance, when bit 1 is set (1'b1) the audio Channel 1 is enabled. When cleared, the referred channel is disabled.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ahb_dma_conf1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ahb_dma_conf1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AhbDmaConf1Spec;
impl crate::RegisterSpec for AhbDmaConf1Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ahb_dma_conf1::R`](R) reader structure"]
impl crate::Readable for AhbDmaConf1Spec {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_conf1::W`](W) writer structure"]
impl crate::Writable for AhbDmaConf1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets AHB_DMA_CONF1 to value 0"]
impl crate::Resettable for AhbDmaConf1Spec {
    const RESET_VALUE: u8 = 0;
}
