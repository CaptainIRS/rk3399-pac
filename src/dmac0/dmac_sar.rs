#[doc = "Register `DMAC_SAR%s` reader"]
pub type R = crate::R<DmacSarSpec>;
#[doc = "Field `DMAC_SAR_BITS_0` reader - Address of the source data for DMA channel 0"]
pub type DmacSarBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address of the source data for DMA channel 0"]
    #[inline(always)]
    pub fn dmac_sar_bits_0(&self) -> DmacSarBits0R {
        DmacSarBits0R::new(self.bits)
    }
}
#[doc = "Source Address Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_sar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacSarSpec;
impl crate::RegisterSpec for DmacSarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_sar::R`](R) reader structure"]
impl crate::Readable for DmacSarSpec {}
#[doc = "`reset()` method sets DMAC_SAR%s to value 0"]
impl crate::Resettable for DmacSarSpec {
    const RESET_VALUE: u32 = 0;
}
