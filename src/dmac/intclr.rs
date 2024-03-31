#[doc = "Register `INTCLR` writer"]
pub type W = crate::W<IntclrSpec>;
#[doc = "Field `INTCLR_BITS_0` writer - Controls the clearing of the irq outputs:\n\nBit \\[N\\]
= 0 The status of irq\\[N\\]
does not change.\n\nBit \\[N\\]
= 1 The DMAC sets irq\\[N\\]
LOW if the INTEN Register\n\nprograms the DMAC to signal an interrupt.\n\nOtherwise, the status of irq\\[N\\]
does not change."]
pub type IntclrBits0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Controls the clearing of the irq outputs:\n\nBit \\[N\\]
= 0 The status of irq\\[N\\]
does not change.\n\nBit \\[N\\]
= 1 The DMAC sets irq\\[N\\]
LOW if the INTEN Register\n\nprograms the DMAC to signal an interrupt.\n\nOtherwise, the status of irq\\[N\\]
does not change."]
    #[inline(always)]
    #[must_use]
    pub fn intclr_bits_0(&mut self) -> IntclrBits0W<IntclrSpec> {
        IntclrBits0W::new(self, 0)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`intclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntclrSpec;
impl crate::RegisterSpec for IntclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`intclr::W`](W) writer structure"]
impl crate::Writable for IntclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTCLR to value 0"]
impl crate::Resettable for IntclrSpec {
    const RESET_VALUE: u32 = 0;
}
