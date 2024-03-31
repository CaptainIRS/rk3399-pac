#[doc = "Register `DPC` reader"]
pub type R = crate::R<DpcSpec>;
#[doc = "Field `DPC_BITS_0` reader - Program counter for the DMA manager thread"]
pub type DpcBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Program counter for the DMA manager thread"]
    #[inline(always)]
    pub fn dpc_bits_0(&self) -> DpcBits0R {
        DpcBits0R::new(self.bits)
    }
}
#[doc = "DMA Program Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dpc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpcSpec;
impl crate::RegisterSpec for DpcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dpc::R`](R) reader structure"]
impl crate::Readable for DpcSpec {}
#[doc = "`reset()` method sets DPC to value 0"]
impl crate::Resettable for DpcSpec {
    const RESET_VALUE: u32 = 0;
}
