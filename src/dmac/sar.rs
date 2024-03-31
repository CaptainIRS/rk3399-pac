#[doc = "Register `SAR%s` reader"]
pub type R = crate::R<SarSpec>;
#[doc = "Field `SAR_BITS_0` reader - Address of the source data for DMA channel 0"]
pub type SarBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address of the source data for DMA channel 0"]
    #[inline(always)]
    pub fn sar_bits_0(&self) -> SarBits0R {
        SarBits0R::new(self.bits)
    }
}
#[doc = "Source Address Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SarSpec;
impl crate::RegisterSpec for SarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar::R`](R) reader structure"]
impl crate::Readable for SarSpec {}
#[doc = "`reset()` method sets SAR%s to value 0"]
impl crate::Resettable for SarSpec {
    const RESET_VALUE: u32 = 0;
}
