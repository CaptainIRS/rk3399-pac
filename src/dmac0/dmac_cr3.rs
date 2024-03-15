#[doc = "Register `DMAC_CR3` reader"]
pub type R = crate::R<DmacCr3Spec>;
#[doc = "Field `DMAC_CR3_BITS_0` reader - Provides the security state of an event-interrupt resource: Bit \\[N\\]
= 0 Assigns event&lt;N> or irq\\[N\\]
to the Secure state. Bit \\[N\\]
= 1 Assigns event&lt;N> or irq\\[N\\]
to the Non-secure state."]
pub type DmacCr3Bits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Provides the security state of an event-interrupt resource: Bit \\[N\\]
= 0 Assigns event&lt;N> or irq\\[N\\]
to the Secure state. Bit \\[N\\]
= 1 Assigns event&lt;N> or irq\\[N\\]
to the Non-secure state."]
    #[inline(always)]
    pub fn dmac_cr3_bits_0(&self) -> DmacCr3Bits0R {
        DmacCr3Bits0R::new(self.bits)
    }
}
#[doc = "Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacCr3Spec;
impl crate::RegisterSpec for DmacCr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_cr3::R`](R) reader structure"]
impl crate::Readable for DmacCr3Spec {}
#[doc = "`reset()` method sets DMAC_CR3 to value 0"]
impl crate::Resettable for DmacCr3Spec {
    const RESET_VALUE: u32 = 0;
}
