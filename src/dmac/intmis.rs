#[doc = "Register `INTMIS` reader"]
pub type R = crate::R<IntmisSpec>;
#[doc = "Field `INTMIS_BITS_0` reader - Provides the status of the interrupts that are active in the DMAC:\n\nBit \\[N\\]
= 0 Interrupt N is inactive and therefore irq\\[N\\]
is LOW.\n\nBit \\[N\\]
= 1 Interrupt N is active and therefore irq\\[N\\]
is HIGH"]
pub type IntmisBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Provides the status of the interrupts that are active in the DMAC:\n\nBit \\[N\\]
= 0 Interrupt N is inactive and therefore irq\\[N\\]
is LOW.\n\nBit \\[N\\]
= 1 Interrupt N is active and therefore irq\\[N\\]
is HIGH"]
    #[inline(always)]
    pub fn intmis_bits_0(&self) -> IntmisBits0R {
        IntmisBits0R::new(self.bits)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intmis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntmisSpec;
impl crate::RegisterSpec for IntmisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intmis::R`](R) reader structure"]
impl crate::Readable for IntmisSpec {}
#[doc = "`reset()` method sets INTMIS to value 0"]
impl crate::Resettable for IntmisSpec {
    const RESET_VALUE: u32 = 0;
}
