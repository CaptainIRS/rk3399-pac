#[doc = "Register `DMAC_INTCLR` writer"]
pub type W = crate::W<DmacIntclrSpec>;
#[doc = "Field `DMAC_INTCLR_BITS_0` writer - Controls the clearing of the irq outputs:\n\nBit \\[N\\]
= 0 The status of irq\\[N\\]
does not change.\n\nBit \\[N\\]
= 1 The DMAC sets irq\\[N\\]
LOW if the INTEN Register\n\nprograms the DMAC to signal an interrupt.\n\nOtherwise, the status of irq\\[N\\]
does not change."]
pub type DmacIntclrBits0W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl W {
    #[doc = "Bits 0:31 - Controls the clearing of the irq outputs:\n\nBit \\[N\\]
= 0 The status of irq\\[N\\]
does not change.\n\nBit \\[N\\]
= 1 The DMAC sets irq\\[N\\]
LOW if the INTEN Register\n\nprograms the DMAC to signal an interrupt.\n\nOtherwise, the status of irq\\[N\\]
does not change."]
    #[inline(always)]
    #[must_use]
    pub fn dmac_intclr_bits_0(&mut self) -> DmacIntclrBits0W<DmacIntclrSpec> {
        DmacIntclrBits0W::new(self, 0)
    }
}
#[doc = "Interrupt Clear Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dmac_intclr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacIntclrSpec;
impl crate::RegisterSpec for DmacIntclrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dmac_intclr::W`](W) writer structure"]
impl crate::Writable for DmacIntclrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMAC_INTCLR to value 0"]
impl crate::Resettable for DmacIntclrSpec {
    const RESET_VALUE: u32 = 0;
}
