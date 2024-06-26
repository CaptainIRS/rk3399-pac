#[doc = "Register `RXFTLR` reader"]
pub type R = crate::R<RxftlrSpec>;
#[doc = "Register `RXFTLR` writer"]
pub type W = crate::W<RxftlrSpec>;
#[doc = "Field `RXFTLR` reader - Receive FIFO Threshold Level\n\nWhen the number of receive FIFO entries is greater than or equal\n\nto this value + 1, the receive FIFO full interrupt is triggered."]
pub type RxftlrR = crate::FieldReader;
#[doc = "Field `RXFTLR` writer - Receive FIFO Threshold Level\n\nWhen the number of receive FIFO entries is greater than or equal\n\nto this value + 1, the receive FIFO full interrupt is triggered."]
pub type RxftlrW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:4 - Receive FIFO Threshold Level\n\nWhen the number of receive FIFO entries is greater than or equal\n\nto this value + 1, the receive FIFO full interrupt is triggered."]
    #[inline(always)]
    pub fn rxftlr(&self) -> RxftlrR {
        RxftlrR::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4 - Receive FIFO Threshold Level\n\nWhen the number of receive FIFO entries is greater than or equal\n\nto this value + 1, the receive FIFO full interrupt is triggered."]
    #[inline(always)]
    #[must_use]
    pub fn rxftlr(&mut self) -> RxftlrW<RxftlrSpec> {
        RxftlrW::new(self, 0)
    }
}
#[doc = "Receive FIFO Threshold Level\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rxftlr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rxftlr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxftlrSpec;
impl crate::RegisterSpec for RxftlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxftlr::R`](R) reader structure"]
impl crate::Readable for RxftlrSpec {}
#[doc = "`write(|w| ..)` method takes [`rxftlr::W`](W) writer structure"]
impl crate::Writable for RxftlrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RXFTLR to value 0"]
impl crate::Resettable for RxftlrSpec {
    const RESET_VALUE: u32 = 0;
}
