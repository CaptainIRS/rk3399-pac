#[doc = "Register `GMAC_CUR_HOST_RX_DESC` reader"]
pub type R = crate::R<GmacCurHostRxDescSpec>;
#[doc = "Field `HRDAP` reader - Host Receive Descriptor Address Pointer\n\nCleared on Reset. Pointer updated by DMA during operation."]
pub type HrdapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Descriptor Address Pointer\n\nCleared on Reset. Pointer updated by DMA during operation."]
    #[inline(always)]
    pub fn hrdap(&self) -> HrdapR {
        HrdapR::new(self.bits)
    }
}
#[doc = "Current Host Receive Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_cur_host_rx_desc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacCurHostRxDescSpec;
impl crate::RegisterSpec for GmacCurHostRxDescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_cur_host_rx_desc::R`](R) reader structure"]
impl crate::Readable for GmacCurHostRxDescSpec {}
#[doc = "`reset()` method sets GMAC_CUR_HOST_RX_DESC to value 0"]
impl crate::Resettable for GmacCurHostRxDescSpec {
    const RESET_VALUE: u32 = 0;
}
