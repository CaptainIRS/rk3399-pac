#[doc = "Register `GMAC_CUR_HOST_TX_BUF_ADDR` reader"]
pub type R = crate::R<GmacCurHostTxBufAddrSpec>;
#[doc = "Field `HTBAP` reader - Host Transmit Buffer Address Pointer Cleared on Reset. Pointer updated by DMA during operation."]
pub type HtbapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Buffer Address Pointer Cleared on Reset. Pointer updated by DMA during operation."]
    #[inline(always)]
    pub fn htbap(&self) -> HtbapR {
        HtbapR::new(self.bits)
    }
}
#[doc = "Current Host Transmit Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_cur_host_tx_buf_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacCurHostTxBufAddrSpec;
impl crate::RegisterSpec for GmacCurHostTxBufAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_cur_host_tx_buf_addr::R`](R) reader structure"]
impl crate::Readable for GmacCurHostTxBufAddrSpec {}
#[doc = "`reset()` method sets GMAC_CUR_HOST_TX_BUF_ADDR to value 0"]
impl crate::Resettable for GmacCurHostTxBufAddrSpec {
    const RESET_VALUE: u32 = 0;
}
