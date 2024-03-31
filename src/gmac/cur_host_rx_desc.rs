#[doc = "Register `CUR_HOST_RX_DESC` reader"]
pub type R = crate::R<CurHostRxDescSpec>;
#[doc = "Field `HRDAP` reader - Host Receive Descriptor Address Pointer\n\nCleared on Reset. Pointer updated by DMA during operation."]
pub type HrdapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Descriptor Address Pointer\n\nCleared on Reset. Pointer updated by DMA during operation."]
    #[inline(always)]
    pub fn hrdap(&self) -> HrdapR {
        HrdapR::new(self.bits)
    }
}
#[doc = "Current Host Receive Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cur_host_rx_desc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurHostRxDescSpec;
impl crate::RegisterSpec for CurHostRxDescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cur_host_rx_desc::R`](R) reader structure"]
impl crate::Readable for CurHostRxDescSpec {}
#[doc = "`reset()` method sets CUR_HOST_RX_DESC to value 0"]
impl crate::Resettable for CurHostRxDescSpec {
    const RESET_VALUE: u32 = 0;
}
