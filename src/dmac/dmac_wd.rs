#[doc = "Register `DMAC_WD` reader"]
pub type R = crate::R<DmacWdSpec>;
#[doc = "Register `DMAC_WD` writer"]
pub type W = crate::W<DmacWdSpec>;
#[doc = "Field `DMAC_WD_BITS_1` reader - Controls how the DMAC responds when it detects a lock-up condition: 0 = the DMAC aborts all of the contributing DMA channels and sets irq_abort HIGH 1 = the DMAC sets irq_abort HIGH."]
pub type DmacWdBits1R = crate::BitReader;
#[doc = "Field `DMAC_WD_BITS_1` writer - Controls how the DMAC responds when it detects a lock-up condition: 0 = the DMAC aborts all of the contributing DMA channels and sets irq_abort HIGH 1 = the DMAC sets irq_abort HIGH."]
pub type DmacWdBits1W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Controls how the DMAC responds when it detects a lock-up condition: 0 = the DMAC aborts all of the contributing DMA channels and sets irq_abort HIGH 1 = the DMAC sets irq_abort HIGH."]
    #[inline(always)]
    pub fn dmac_wd_bits_1(&self) -> DmacWdBits1R {
        DmacWdBits1R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Controls how the DMAC responds when it detects a lock-up condition: 0 = the DMAC aborts all of the contributing DMA channels and sets irq_abort HIGH 1 = the DMAC sets irq_abort HIGH."]
    #[inline(always)]
    #[must_use]
    pub fn dmac_wd_bits_1(&mut self) -> DmacWdBits1W<DmacWdSpec> {
        DmacWdBits1W::new(self, 0)
    }
}
#[doc = "DMA Watchdog Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_wd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_wd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacWdSpec;
impl crate::RegisterSpec for DmacWdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_wd::R`](R) reader structure"]
impl crate::Readable for DmacWdSpec {}
#[doc = "`write(|w| ..)` method takes [`dmac_wd::W`](W) writer structure"]
impl crate::Writable for DmacWdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC_WD to value 0"]
impl crate::Resettable for DmacWdSpec {
    const RESET_VALUE: u32 = 0;
}
