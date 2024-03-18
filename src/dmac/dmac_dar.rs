#[doc = "Register `DMAC_DAR%s` reader"]
pub type R = crate::R<DmacDarSpec>;
#[doc = "Field `DMAC_DAR_BITS_0` reader - Address of the Destination data for DMA channel 0"]
pub type DmacDarBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address of the Destination data for DMA channel 0"]
    #[inline(always)]
    pub fn dmac_dar_bits_0(&self) -> DmacDarBits0R {
        DmacDarBits0R::new(self.bits)
    }
}
#[doc = "DestinationAddress Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_dar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacDarSpec;
impl crate::RegisterSpec for DmacDarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_dar::R`](R) reader structure"]
impl crate::Readable for DmacDarSpec {}
#[doc = "`reset()` method sets DMAC_DAR%s to value 0"]
impl crate::Resettable for DmacDarSpec {
    const RESET_VALUE: u32 = 0;
}
