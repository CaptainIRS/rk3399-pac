#[doc = "Register `WD` reader"]
pub type R = crate::R<WdSpec>;
#[doc = "Register `WD` writer"]
pub type W = crate::W<WdSpec>;
#[doc = "Field `WD_BITS_1` reader - Controls how the DMAC responds when it detects a lock-up\n\ncondition:\n\n0 = the DMAC aborts all of the contributing DMA channels and sets\n\nirq_abort HIGH\n\n1 = the DMAC sets irq_abort HIGH."]
pub type WdBits1R = crate::BitReader;
#[doc = "Field `WD_BITS_1` writer - Controls how the DMAC responds when it detects a lock-up\n\ncondition:\n\n0 = the DMAC aborts all of the contributing DMA channels and sets\n\nirq_abort HIGH\n\n1 = the DMAC sets irq_abort HIGH."]
pub type WdBits1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Controls how the DMAC responds when it detects a lock-up\n\ncondition:\n\n0 = the DMAC aborts all of the contributing DMA channels and sets\n\nirq_abort HIGH\n\n1 = the DMAC sets irq_abort HIGH."]
    #[inline(always)]
    pub fn wd_bits_1(&self) -> WdBits1R {
        WdBits1R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls how the DMAC responds when it detects a lock-up\n\ncondition:\n\n0 = the DMAC aborts all of the contributing DMA channels and sets\n\nirq_abort HIGH\n\n1 = the DMAC sets irq_abort HIGH."]
    #[inline(always)]
    #[must_use]
    pub fn wd_bits_1(&mut self) -> WdBits1W<WdSpec> {
        WdBits1W::new(self, 0)
    }
}
#[doc = "DMA Watchdog Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`wd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WdSpec;
impl crate::RegisterSpec for WdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wd::R`](R) reader structure"]
impl crate::Readable for WdSpec {}
#[doc = "`write(|w| ..)` method takes [`wd::W`](W) writer structure"]
impl crate::Writable for WdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WD to value 0"]
impl crate::Resettable for WdSpec {
    const RESET_VALUE: u32 = 0;
}
