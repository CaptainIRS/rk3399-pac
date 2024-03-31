#[doc = "Register `DAR%s` reader"]
pub type R = crate::R<DarSpec>;
#[doc = "Field `DAR_BITS_0` reader - Address of the Destination data for DMA channel 0"]
pub type DarBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Address of the Destination data for DMA channel 0"]
    #[inline(always)]
    pub fn dar_bits_0(&self) -> DarBits0R {
        DarBits0R::new(self.bits)
    }
}
#[doc = "DestinationAddress Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dar::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DarSpec;
impl crate::RegisterSpec for DarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dar::R`](R) reader structure"]
impl crate::Readable for DarSpec {}
#[doc = "`reset()` method sets DAR%s to value 0"]
impl crate::Resettable for DarSpec {
    const RESET_VALUE: u32 = 0;
}
