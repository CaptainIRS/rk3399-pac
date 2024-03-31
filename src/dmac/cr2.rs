#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Field `CR2_BITS_0` reader - Provides the value of boot_addr\\[31:0\\]
when the DMAC exited from\n\nreset"]
pub type Cr2Bits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Provides the value of boot_addr\\[31:0\\]
when the DMAC exited from\n\nreset"]
    #[inline(always)]
    pub fn cr2_bits_0(&self) -> Cr2Bits0R {
        Cr2Bits0R::new(self.bits)
    }
}
#[doc = "Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {
    const RESET_VALUE: u32 = 0;
}
