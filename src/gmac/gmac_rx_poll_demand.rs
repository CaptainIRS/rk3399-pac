#[doc = "Register `GMAC_RX_POLL_DEMAND` reader"]
pub type R = crate::R<GmacRxPollDemandSpec>;
#[doc = "Field `RPD` reader - Receive Poll Demand When these bits are written with any value, the DMA reads the current descriptor pointed to by Register GMAC_CUR_HOST_RX_DESC. If that descriptor is not available (owned by Host), reception returns to the Suspended state and Register GMAC_STATUS\\[7\\]
is not asserted. If the descriptor is available, the Receive DMA returns to active state."]
pub type RpdR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Poll Demand When these bits are written with any value, the DMA reads the current descriptor pointed to by Register GMAC_CUR_HOST_RX_DESC. If that descriptor is not available (owned by Host), reception returns to the Suspended state and Register GMAC_STATUS\\[7\\]
is not asserted. If the descriptor is available, the Receive DMA returns to active state."]
    #[inline(always)]
    pub fn rpd(&self) -> RpdR {
        RpdR::new(self.bits)
    }
}
#[doc = "Receive Poll Demand Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_rx_poll_demand::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacRxPollDemandSpec;
impl crate::RegisterSpec for GmacRxPollDemandSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_rx_poll_demand::R`](R) reader structure"]
impl crate::Readable for GmacRxPollDemandSpec {}
#[doc = "`reset()` method sets GMAC_RX_POLL_DEMAND to value 0"]
impl crate::Resettable for GmacRxPollDemandSpec {
    const RESET_VALUE: u32 = 0;
}
