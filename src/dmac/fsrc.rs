#[doc = "Register `FSRC` reader"]
pub type R = crate::R<FsrcSpec>;
#[doc = "Field `FSRC_BITS_0` reader - Each bit provides the fault status of the corresponding channel.\n\nRead as:\n\nBit \\[N\\]
= 0 No fault is present on DMA channel N.\n\nBit \\[N\\]
= 1 DMA channel N is in the Faulting or Faulting completing\n\nstate."]
pub type FsrcBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Each bit provides the fault status of the corresponding channel.\n\nRead as:\n\nBit \\[N\\]
= 0 No fault is present on DMA channel N.\n\nBit \\[N\\]
= 1 DMA channel N is in the Faulting or Faulting completing\n\nstate."]
    #[inline(always)]
    pub fn fsrc_bits_0(&self) -> FsrcBits0R {
        FsrcBits0R::new(self.bits)
    }
}
#[doc = "Fault Status DMA Channel Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsrc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FsrcSpec;
impl crate::RegisterSpec for FsrcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsrc::R`](R) reader structure"]
impl crate::Readable for FsrcSpec {}
#[doc = "`reset()` method sets FSRC to value 0"]
impl crate::Resettable for FsrcSpec {
    const RESET_VALUE: u32 = 0;
}
