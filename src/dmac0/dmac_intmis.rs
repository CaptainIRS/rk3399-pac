#[doc = "Register `DMAC_INTMIS` reader"]
pub type R = crate::R<DmacIntmisSpec>;
#[doc = "Field `DMAC_INTMIS_BITS_0` reader - Provides the status of the interrupts that are active in the DMAC: Bit \\[N\\]
= 0 Interrupt N is inactive and therefore irq\\[N\\]
is LOW. Bit \\[N\\]
= 1 Interrupt N is active and therefore irq\\[N\\]
is HIGH"]
pub type DmacIntmisBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Provides the status of the interrupts that are active in the DMAC: Bit \\[N\\]
= 0 Interrupt N is inactive and therefore irq\\[N\\]
is LOW. Bit \\[N\\]
= 1 Interrupt N is active and therefore irq\\[N\\]
is HIGH"]
    #[inline(always)]
    pub fn dmac_intmis_bits_0(&self) -> DmacIntmisBits0R {
        DmacIntmisBits0R::new(self.bits)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_intmis::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacIntmisSpec;
impl crate::RegisterSpec for DmacIntmisSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_intmis::R`](R) reader structure"]
impl crate::Readable for DmacIntmisSpec {}
#[doc = "`reset()` method sets DMAC_INTMIS to value 0"]
impl crate::Resettable for DmacIntmisSpec {
    const RESET_VALUE: u32 = 0;
}
