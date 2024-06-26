#[doc = "Register `TX_POLL_DEMAND` reader"]
pub type R = crate::R<TxPollDemandSpec>;
#[doc = "Field `TPD` reader - Transmit Poll Demand\n\nWhen these bits are written with any value, the DMA reads the\n\ncurrent descriptor pointed to by Register\n\nGMAC_CUR_HOST_TX_DESC. If that descriptor is not available\n\n(owned by Host), transmission returns to the Suspend state and\n\nDMA Register GMAC_STATUS\\[2\\]
is asserted. If the descriptor is\n\navailable, transmission resumes."]
pub type TpdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit Poll Demand\n\nWhen these bits are written with any value, the DMA reads the\n\ncurrent descriptor pointed to by Register\n\nGMAC_CUR_HOST_TX_DESC. If that descriptor is not available\n\n(owned by Host), transmission returns to the Suspend state and\n\nDMA Register GMAC_STATUS\\[2\\]
is asserted. If the descriptor is\n\navailable, transmission resumes."]
    #[inline(always)]
    pub fn tpd(&self) -> TpdR {
        TpdR::new(self.bits)
    }
}
#[doc = "Transmit Poll Demand Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`tx_poll_demand::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxPollDemandSpec;
impl crate::RegisterSpec for TxPollDemandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_poll_demand::R`](R) reader structure"]
impl crate::Readable for TxPollDemandSpec {}
#[doc = "`reset()` method sets TX_POLL_DEMAND to value 0"]
impl crate::Resettable for TxPollDemandSpec {
    const RESET_VALUE: u32 = 0;
}
