#[doc = "Register `GMAC_TX_POLL_DEMAND` reader"]
pub type R = crate::R<GmacTxPollDemandSpec>;
#[doc = "Field `TPD` reader - Transmit Poll Demand When these bits are written with any value, the DMA reads the current descriptor pointed to by Register GMAC_CUR_HOST_TX_DESC. If that descriptor is not available (owned by Host), transmission returns to the Suspend state and DMA Register GMAC_STATUS\\[2\\]
is asserted. If the descriptor is available, transmission resumes."]
pub type TpdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit Poll Demand When these bits are written with any value, the DMA reads the current descriptor pointed to by Register GMAC_CUR_HOST_TX_DESC. If that descriptor is not available (owned by Host), transmission returns to the Suspend state and DMA Register GMAC_STATUS\\[2\\]
is asserted. If the descriptor is available, transmission resumes."]
    #[inline(always)]
    pub fn tpd(&self) -> TpdR {
        TpdR::new(self.bits)
    }
}
#[doc = "Transmit Poll Demand Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_tx_poll_demand::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacTxPollDemandSpec;
impl crate::RegisterSpec for GmacTxPollDemandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_tx_poll_demand::R`](R) reader structure"]
impl crate::Readable for GmacTxPollDemandSpec {}
#[doc = "`reset()` method sets GMAC_TX_POLL_DEMAND to value 0"]
impl crate::Resettable for GmacTxPollDemandSpec {
    const RESET_VALUE: u32 = 0;
}
