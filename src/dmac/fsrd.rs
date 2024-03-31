#[doc = "Register `FSRD` reader"]
pub type R = crate::R<FsrdSpec>;
#[doc = "Field `FSRD_BITS_0` reader - Provides the fault status of the DMA manager. Read as:\n\n0 = the DMA manager thread is not in the Faulting state\n\n1 = the DMA manager thread is in the Faulting state."]
pub type FsrdBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Provides the fault status of the DMA manager. Read as:\n\n0 = the DMA manager thread is not in the Faulting state\n\n1 = the DMA manager thread is in the Faulting state."]
    #[inline(always)]
    pub fn fsrd_bits_0(&self) -> FsrdBits0R {
        FsrdBits0R::new(self.bits)
    }
}
#[doc = "Fault Status DMA Manager Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsrd::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsrdSpec;
impl crate::RegisterSpec for FsrdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsrd::R`](R) reader structure"]
impl crate::Readable for FsrdSpec {}
#[doc = "`reset()` method sets FSRD to value 0"]
impl crate::Resettable for FsrdSpec {
    const RESET_VALUE: u32 = 0;
}
