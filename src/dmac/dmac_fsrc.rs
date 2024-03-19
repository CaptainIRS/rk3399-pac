#[doc = "Register `DMAC_FSRC` reader"]
pub type R = crate::R<DmacFsrcSpec>;
#[doc = "Field `DMAC_FSRC_BITS_0` reader - Each bit provides the fault status of the corresponding channel.\n\nRead as:\n\nBit \\[N\\]
= 0 No fault is present on DMA channel N.\n\nBit \\[N\\]
= 1 DMA channel N is in the Faulting or Faulting completing\n\nstate."]
pub type DmacFsrcBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit provides the fault status of the corresponding channel.\n\nRead as:\n\nBit \\[N\\]
= 0 No fault is present on DMA channel N.\n\nBit \\[N\\]
= 1 DMA channel N is in the Faulting or Faulting completing\n\nstate."]
    #[inline(always)]
    pub fn dmac_fsrc_bits_0(&self) -> DmacFsrcBits0R {
        DmacFsrcBits0R::new(self.bits)
    }
}
#[doc = "Fault Status DMA Channel Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_fsrc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacFsrcSpec;
impl crate::RegisterSpec for DmacFsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_fsrc::R`](R) reader structure"]
impl crate::Readable for DmacFsrcSpec {}
#[doc = "`reset()` method sets DMAC_FSRC to value 0"]
impl crate::Resettable for DmacFsrcSpec {
    const RESET_VALUE: u32 = 0;
}
