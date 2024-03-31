#[doc = "Register `CR4` reader"]
pub type R = crate::R<Cr4Spec>;
#[doc = "Field `CR4_BITS_0` reader - Provides the security state of the peripheral request interfaces:\n\nBit \\[N\\]
= 0 Assigns peripheral request interface N to the Secure\n\nstate.\n\nBit \\[N\\]
= 1 Assigns peripheral request interface N to the Non-secure\n\nstate"]
pub type Cr4Bits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Provides the security state of the peripheral request interfaces:\n\nBit \\[N\\]
= 0 Assigns peripheral request interface N to the Secure\n\nstate.\n\nBit \\[N\\]
= 1 Assigns peripheral request interface N to the Non-secure\n\nstate"]
    #[inline(always)]
    pub fn cr4_bits_0(&self) -> Cr4Bits0R {
        Cr4Bits0R::new(self.bits)
    }
}
#[doc = "Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr4Spec;
impl crate::RegisterSpec for Cr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr4::R`](R) reader structure"]
impl crate::Readable for Cr4Spec {}
#[doc = "`reset()` method sets CR4 to value 0"]
impl crate::Resettable for Cr4Spec {
    const RESET_VALUE: u32 = 0;
}
