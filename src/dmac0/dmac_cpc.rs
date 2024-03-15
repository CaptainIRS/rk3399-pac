#[doc = "Register `DMAC_CPC%s` reader"]
pub type R = crate::R<DmacCpcSpec>;
#[doc = "Field `DMAC_CPC_BITS_0` reader - Program counter for the DMA channel 0 thread"]
pub type DmacCpcBits0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Program counter for the DMA channel 0 thread"]
    #[inline(always)]
    pub fn dmac_cpc_bits_0(&self) -> DmacCpcBits0R {
        DmacCpcBits0R::new(self.bits)
    }
}
#[doc = "Channel Program Counter Registers\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmac_cpc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacCpcSpec;
impl crate::RegisterSpec for DmacCpcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmac_cpc::R`](R) reader structure"]
impl crate::Readable for DmacCpcSpec {}
#[doc = "`reset()` method sets DMAC_CPC%s to value 0"]
impl crate::Resettable for DmacCpcSpec {
    const RESET_VALUE: u32 = 0;
}
