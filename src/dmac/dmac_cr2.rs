#[doc = "Register `DMAC_CR2` reader"]
pub type R = crate::R<DmacCr2Spec>;
#[doc = "Field `DMAC_CR2_BITS_0` reader - Provides the value of boot_addr\\[31:0\\]
when the DMAC exited from\n\nreset"]
pub type DmacCr2Bits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Provides the value of boot_addr\\[31:0\\]
when the DMAC exited from\n\nreset"]
    #[inline(always)]
    pub fn dmac_cr2_bits_0(&self) -> DmacCr2Bits0R {
        DmacCr2Bits0R::new(self.bits)
    }
}
#[doc = "Configuration Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cr2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacCr2Spec;
impl crate::RegisterSpec for DmacCr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_cr2::R`](R) reader structure"]
impl crate::Readable for DmacCr2Spec {}
#[doc = "`reset()` method sets DMAC_CR2 to value 0"]
impl crate::Resettable for DmacCr2Spec {
    const RESET_VALUE: u32 = 0;
}
