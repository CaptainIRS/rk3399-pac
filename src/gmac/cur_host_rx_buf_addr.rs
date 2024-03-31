#[doc = "Register `CUR_HOST_RX_BUF_ADDR` reader"]
pub type R = crate::R<CurHostRxBufAddrSpec>;
#[doc = "Field `HRBAP` reader - Host Receive Buffer Address Pointer\n\nCleared on Reset. Pointer updated by DMA during operation."]
pub type HrbapR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Buffer Address Pointer\n\nCleared on Reset. Pointer updated by DMA during operation."]
    #[inline(always)]
    pub fn hrbap(&self) -> HrbapR {
        HrbapR::new(self.bits)
    }
}
#[doc = "Current Host Receive Buffer Address Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cur_host_rx_buf_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CurHostRxBufAddrSpec;
impl crate::RegisterSpec for CurHostRxBufAddrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cur_host_rx_buf_addr::R`](R) reader structure"]
impl crate::Readable for CurHostRxBufAddrSpec {}
#[doc = "`reset()` method sets CUR_HOST_RX_BUF_ADDR to value 0"]
impl crate::Resettable for CurHostRxBufAddrSpec {
    const RESET_VALUE: u32 = 0;
}
