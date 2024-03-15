#[doc = "Register `GMAC_CUR_HOST_RX_BUF_ADDR` reader"]
pub type R = crate::R<GmacCurHostRxBufAddrSpec>;
#[doc = "Field `HRBAP` reader - Host Receive Buffer Address Pointer Cleared on Reset. Pointer updated by DMA during operation."]
pub type HrbapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Buffer Address Pointer Cleared on Reset. Pointer updated by DMA during operation."]
    #[inline(always)]
    pub fn hrbap(&self) -> HrbapR {
        HrbapR::new(self.bits)
    }
}
#[doc = "Current Host Receive Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_cur_host_rx_buf_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacCurHostRxBufAddrSpec;
impl crate::RegisterSpec for GmacCurHostRxBufAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_cur_host_rx_buf_addr::R`](R) reader structure"]
impl crate::Readable for GmacCurHostRxBufAddrSpec {}
#[doc = "`reset()` method sets GMAC_CUR_HOST_RX_BUF_ADDR to value 0"]
impl crate::Resettable for GmacCurHostRxBufAddrSpec {
    const RESET_VALUE: u32 = 0;
}
