#[doc = "Register `DMAC_INTEN` reader"]
pub type R = crate::R<DmacIntenSpec>;
#[doc = "Register `DMAC_INTEN` writer"]
pub type W = crate::W<DmacIntenSpec>;
#[doc = "Field `DMAC_INTEN_BITS_0` reader - Program the appropriate bit to control how the DMAC responds when it executes DMASEV: Bit \\[N\\]
= 0 If the DMAC executes DMASEV for the event-interrupt resource N then the DMAC signals event N to all of the threads. Set bit \\[N\\]
to 0 if your system design does not use irq\\[N\\]
to signal an interrupt request. Bit \\[N\\]
= 1 If the DMAC executes DMASEV for the event-interrupt resource N then the DMAC sets irq\\[N\\]
HIGH. Set bit \\[N\\]
to 1 if your system designer requires irq\\[N\\]
to signal an interruptrequest."]
pub type DmacIntenBits0R = crate::FieldReader<u32>;
#[doc = "Field `DMAC_INTEN_BITS_0` writer - Program the appropriate bit to control how the DMAC responds when it executes DMASEV: Bit \\[N\\]
= 0 If the DMAC executes DMASEV for the event-interrupt resource N then the DMAC signals event N to all of the threads. Set bit \\[N\\]
to 0 if your system design does not use irq\\[N\\]
to signal an interrupt request. Bit \\[N\\]
= 1 If the DMAC executes DMASEV for the event-interrupt resource N then the DMAC sets irq\\[N\\]
HIGH. Set bit \\[N\\]
to 1 if your system designer requires irq\\[N\\]
to signal an interruptrequest."]
pub type DmacIntenBits0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Program the appropriate bit to control how the DMAC responds when it executes DMASEV: Bit \\[N\\]
= 0 If the DMAC executes DMASEV for the event-interrupt resource N then the DMAC signals event N to all of the threads. Set bit \\[N\\]
to 0 if your system design does not use irq\\[N\\]
to signal an interrupt request. Bit \\[N\\]
= 1 If the DMAC executes DMASEV for the event-interrupt resource N then the DMAC sets irq\\[N\\]
HIGH. Set bit \\[N\\]
to 1 if your system designer requires irq\\[N\\]
to signal an interruptrequest."]
    #[inline(always)]
    pub fn dmac_inten_bits_0(&self) -> DmacIntenBits0R {
        DmacIntenBits0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Program the appropriate bit to control how the DMAC responds when it executes DMASEV: Bit \\[N\\]
= 0 If the DMAC executes DMASEV for the event-interrupt resource N then the DMAC signals event N to all of the threads. Set bit \\[N\\]
to 0 if your system design does not use irq\\[N\\]
to signal an interrupt request. Bit \\[N\\]
= 1 If the DMAC executes DMASEV for the event-interrupt resource N then the DMAC sets irq\\[N\\]
HIGH. Set bit \\[N\\]
to 1 if your system designer requires irq\\[N\\]
to signal an interruptrequest."]
    #[inline(always)]
    #[must_use]
    pub fn dmac_inten_bits_0(&mut self) -> DmacIntenBits0W<DmacIntenSpec> {
        DmacIntenBits0W::new(self, 0)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacIntenSpec;
impl crate::RegisterSpec for DmacIntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_inten::R`](R) reader structure"]
impl crate::Readable for DmacIntenSpec {}
#[doc = "`write(|w| ..)` method takes [`dmac_inten::W`](W) writer structure"]
impl crate::Writable for DmacIntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC_INTEN to value 0"]
impl crate::Resettable for DmacIntenSpec {
    const RESET_VALUE: u32 = 0;
}
