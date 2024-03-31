#[doc = "Register `CPC%s` reader"]
pub type R = crate::R<CpcSpec>;
#[doc = "Field `CPC_BITS_0` reader - Program counter for the DMA channel 0 thread"]
pub type CpcBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Program counter for the DMA channel 0 thread"]
    #[inline(always)]
    pub fn cpc_bits_0(&self) -> CpcBits0R {
        CpcBits0R::new(self.bits)
    }
}
#[doc = "Channel Program Counter Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cpc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpcSpec;
impl crate::RegisterSpec for CpcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpc::R`](R) reader structure"]
impl crate::Readable for CpcSpec {}
#[doc = "`reset()` method sets CPC%s to value 0"]
impl crate::Resettable for CpcSpec {
    const RESET_VALUE: u32 = 0;
}
