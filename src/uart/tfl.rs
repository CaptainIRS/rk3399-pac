#[doc = "Register `TFL` reader"]
pub type R = crate::R<TflSpec>;
#[doc = "Register `TFL` writer"]
pub type W = crate::W<TflSpec>;
#[doc = "Field `TRANS_FIFO_LEVEL` reader - Transmit FIFO Level.\n\nThis is indicates the number\n\nof data entries in the transmit FIFO."]
pub type TransFifoLevelR = crate::FieldReader;
#[doc = "Field `TRANS_FIFO_LEVEL` writer - Transmit FIFO Level.\n\nThis is indicates the number\n\nof data entries in the transmit FIFO."]
pub type TransFifoLevelW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Transmit FIFO Level.\n\nThis is indicates the number\n\nof data entries in the transmit FIFO."]
    #[inline(always)]
    pub fn trans_fifo_level(&self) -> TransFifoLevelR {
        TransFifoLevelR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Transmit FIFO Level.\n\nThis is indicates the number\n\nof data entries in the transmit FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn trans_fifo_level(&mut self) -> TransFifoLevelW<TflSpec> {
        TransFifoLevelW::new(self, 0)
    }
}
#[doc = "Transmit FIFO Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tfl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`tfl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TflSpec;
impl crate::RegisterSpec for TflSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tfl::R`](R) reader structure"]
impl crate::Readable for TflSpec {}
#[doc = "`write(|w| ..)` method takes [`tfl::W`](W) writer structure"]
impl crate::Writable for TflSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TFL to value 0"]
impl crate::Resettable for TflSpec {
    const RESET_VALUE: u32 = 0;
}
