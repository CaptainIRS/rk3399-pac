#[doc = "Register `DMAC_DPC` reader"]
pub type R = crate::R<DmacDpcSpec>;
#[doc = "Field `DMAC_DPC_BITS_0` reader - Program counter for the DMA manager thread"]
pub type DmacDpcBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Program counter for the DMA manager thread"]
    #[inline(always)]
    pub fn dmac_dpc_bits_0(&self) -> DmacDpcBits0R {
        DmacDpcBits0R::new(self.bits)
    }
}
#[doc = "DMA Program Counter Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_dpc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacDpcSpec;
impl crate::RegisterSpec for DmacDpcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_dpc::R`](R) reader structure"]
impl crate::Readable for DmacDpcSpec {}
#[doc = "`reset()` method sets DMAC_DPC to value 0"]
impl crate::Resettable for DmacDpcSpec {
    const RESET_VALUE: u32 = 0;
}
