#[doc = "Register `CUR_HOST_TX_DESC` reader"]
pub type R = crate::R<CurHostTxDescSpec>;
#[doc = "Field `HTDAP` reader - Host Transmit Descriptor Address Pointer\n\nCleared on Reset. Pointer updated by DMA during operation."]
pub type HtdapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Descriptor Address Pointer\n\nCleared on Reset. Pointer updated by DMA during operation."]
    #[inline(always)]
    pub fn htdap(&self) -> HtdapR {
        HtdapR::new(self.bits)
    }
}
#[doc = "Current Host Transmit Descriptor Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cur_host_tx_desc::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurHostTxDescSpec;
impl crate::RegisterSpec for CurHostTxDescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cur_host_tx_desc::R`](R) reader structure"]
impl crate::Readable for CurHostTxDescSpec {}
#[doc = "`reset()` method sets CUR_HOST_TX_DESC to value 0"]
impl crate::Resettable for CurHostTxDescSpec {
    const RESET_VALUE: u32 = 0;
}
