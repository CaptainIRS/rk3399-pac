#[doc = "Register `CR3` reader"]
pub type R = crate::R<Cr3Spec>;
#[doc = "Field `CR3_BITS_0` reader - Provides the security state of an event-interrupt resource:\n\nBit \\[N\\]
= 0 Assigns event&lt;N> or irq\\[N\\]
to the Secure state.\n\nBit \\[N\\]
= 1 Assigns event&lt;N> or irq\\[N\\]
to the Non-secure state."]
pub type Cr3Bits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Provides the security state of an event-interrupt resource:\n\nBit \\[N\\]
= 0 Assigns event&lt;N> or irq\\[N\\]
to the Secure state.\n\nBit \\[N\\]
= 1 Assigns event&lt;N> or irq\\[N\\]
to the Non-secure state."]
    #[inline(always)]
    pub fn cr3_bits_0(&self) -> Cr3Bits0R {
        Cr3Bits0R::new(self.bits)
    }
}
#[doc = "Configuration Register 3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr3Spec;
impl crate::RegisterSpec for Cr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr3::R`](R) reader structure"]
impl crate::Readable for Cr3Spec {}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for Cr3Spec {
    const RESET_VALUE: u32 = 0;
}
