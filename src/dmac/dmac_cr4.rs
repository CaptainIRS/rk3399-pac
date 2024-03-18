#[doc = "Register `DMAC_CR4` reader"]
pub type R = crate::R<DmacCr4Spec>;
#[doc = "Field `DMAC_CR4_BITS_0` reader - Provides the security state of the peripheral request interfaces: Bit \\[N\\]
= 0 Assigns peripheral request interface N to the Secure state. Bit \\[N\\]
= 1 Assigns peripheral request interface N to the Non-secure state"]
pub type DmacCr4Bits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Provides the security state of the peripheral request interfaces: Bit \\[N\\]
= 0 Assigns peripheral request interface N to the Secure state. Bit \\[N\\]
= 1 Assigns peripheral request interface N to the Non-secure state"]
    #[inline(always)]
    pub fn dmac_cr4_bits_0(&self) -> DmacCr4Bits0R {
        DmacCr4Bits0R::new(self.bits)
    }
}
#[doc = "Configuration Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cr4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacCr4Spec;
impl crate::RegisterSpec for DmacCr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_cr4::R`](R) reader structure"]
impl crate::Readable for DmacCr4Spec {}
#[doc = "`reset()` method sets DMAC_CR4 to value 0"]
impl crate::Resettable for DmacCr4Spec {
    const RESET_VALUE: u32 = 0;
}
