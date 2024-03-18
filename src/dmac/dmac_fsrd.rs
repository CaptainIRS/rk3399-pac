#[doc = "Register `DMAC_FSRD` reader"]
pub type R = crate::R<DmacFsrdSpec>;
#[doc = "Field `DMAC_FSRD_BITS_0` reader - Provides the fault status of the DMA manager. Read as: 0 = the DMA manager thread is not in the Faulting state 1 = the DMA manager thread is in the Faulting state."]
pub type DmacFsrdBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Provides the fault status of the DMA manager. Read as: 0 = the DMA manager thread is not in the Faulting state 1 = the DMA manager thread is in the Faulting state."]
    #[inline(always)]
    pub fn dmac_fsrd_bits_0(&self) -> DmacFsrdBits0R {
        DmacFsrdBits0R::new(self.bits)
    }
}
#[doc = "Fault Status DMA Manager Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_fsrd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacFsrdSpec;
impl crate::RegisterSpec for DmacFsrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_fsrd::R`](R) reader structure"]
impl crate::Readable for DmacFsrdSpec {}
#[doc = "`reset()` method sets DMAC_FSRD to value 0"]
impl crate::Resettable for DmacFsrdSpec {
    const RESET_VALUE: u32 = 0;
}
