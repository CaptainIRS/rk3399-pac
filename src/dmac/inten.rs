#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Field `INTEN_BITS_0` reader - Program the appropriate bit to control how the DMAC responds\n\nwhen it executes DMASEV:\n\nBit \\[N\\]
= 0 If the DMAC executes DMASEV for the event-interrupt\n\nresource N then the DMAC signals event N to all of the threads. Set\n\nbit \\[N\\]
to 0 if your system design does not use irq\\[N\\]
to\n\nsignal an interrupt request.\n\nBit \\[N\\]
= 1 If the DMAC executes DMASEV for the event-interrupt\n\nresource N then the DMAC sets irq\\[N\\]
HIGH. Set bit \\[N\\]
to 1 if your\n\nsystem designer requires irq\\[N\\]
to signal an interruptrequest."]
pub type IntenBits0R = crate::FieldReader<u32>;
#[doc = "Field `INTEN_BITS_0` writer - Program the appropriate bit to control how the DMAC responds\n\nwhen it executes DMASEV:\n\nBit \\[N\\]
= 0 If the DMAC executes DMASEV for the event-interrupt\n\nresource N then the DMAC signals event N to all of the threads. Set\n\nbit \\[N\\]
to 0 if your system design does not use irq\\[N\\]
to\n\nsignal an interrupt request.\n\nBit \\[N\\]
= 1 If the DMAC executes DMASEV for the event-interrupt\n\nresource N then the DMAC sets irq\\[N\\]
HIGH. Set bit \\[N\\]
to 1 if your\n\nsystem designer requires irq\\[N\\]
to signal an interruptrequest."]
pub type IntenBits0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Program the appropriate bit to control how the DMAC responds\n\nwhen it executes DMASEV:\n\nBit \\[N\\]
= 0 If the DMAC executes DMASEV for the event-interrupt\n\nresource N then the DMAC signals event N to all of the threads. Set\n\nbit \\[N\\]
to 0 if your system design does not use irq\\[N\\]
to\n\nsignal an interrupt request.\n\nBit \\[N\\]
= 1 If the DMAC executes DMASEV for the event-interrupt\n\nresource N then the DMAC sets irq\\[N\\]
HIGH. Set bit \\[N\\]
to 1 if your\n\nsystem designer requires irq\\[N\\]
to signal an interruptrequest."]
    #[inline(always)]
    pub fn inten_bits_0(&self) -> IntenBits0R {
        IntenBits0R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Program the appropriate bit to control how the DMAC responds\n\nwhen it executes DMASEV:\n\nBit \\[N\\]
= 0 If the DMAC executes DMASEV for the event-interrupt\n\nresource N then the DMAC signals event N to all of the threads. Set\n\nbit \\[N\\]
to 0 if your system design does not use irq\\[N\\]
to\n\nsignal an interrupt request.\n\nBit \\[N\\]
= 1 If the DMAC executes DMASEV for the event-interrupt\n\nresource N then the DMAC sets irq\\[N\\]
HIGH. Set bit \\[N\\]
to 1 if your\n\nsystem designer requires irq\\[N\\]
to signal an interruptrequest."]
    #[inline(always)]
    #[must_use]
    pub fn inten_bits_0(&mut self) -> IntenBits0W<IntenSpec> {
        IntenBits0W::new(self, 0)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`inten::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
