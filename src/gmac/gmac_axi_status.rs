#[doc = "Register `GMAC_AXI_STATUS` reader"]
pub type R = crate::R<GmacAxiStatusSpec>;
#[doc = "Field `WR_CH_STA` reader - When high, it indicates that AXI Master's write channel is active and transferring data."]
pub type WrChStaR = crate::BitReader;
#[doc = "Field `RD_CH_STA` reader - When high, it indicates that AXI Master's read channel is active and transferring data."]
pub type RdChStaR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - When high, it indicates that AXI Master's write channel is active and transferring data."]
    #[inline(always)]
    pub fn wr_ch_sta(&self) -> WrChStaR {
        WrChStaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When high, it indicates that AXI Master's read channel is active and transferring data."]
    #[inline(always)]
    pub fn rd_ch_sta(&self) -> RdChStaR {
        RdChStaR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "AXI Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`gmac_axi_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GmacAxiStatusSpec;
impl crate::RegisterSpec for GmacAxiStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gmac_axi_status::R`](R) reader structure"]
impl crate::Readable for GmacAxiStatusSpec {}
#[doc = "`reset()` method sets GMAC_AXI_STATUS to value 0"]
impl crate::Resettable for GmacAxiStatusSpec {
    const RESET_VALUE: u32 = 0;
}
