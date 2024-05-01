#[doc = "Register `INT_ST0` reader"]
pub type R = crate::R<IntSt0Spec>;
#[doc = "Field `ACK_WITH_ERR_0` reader - ack_with_err_0\n\nThis bit retrieves the SoT error from the Acknowledge error report."]
pub type AckWithErr0R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_1` reader - ack_with_err_1\n\nThis bit retrieves the SoT Sync error from the Acknowledge error\n\nreport."]
pub type AckWithErr1R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_2` reader - ack_with_err_2\n\nThis bit retrieves the EoT Sync error from the Acknowledge error\n\nreport."]
pub type AckWithErr2R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_3` reader - ack_with_err_3\n\nThis bit retrieves the Escape Mode Entry Command error from the\n\nAcknowledge error report."]
pub type AckWithErr3R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_4` reader - ack_with_err_4\n\nThis bit retrieves the LP Transmit Sync error from the Acknowledge\n\nerror report."]
pub type AckWithErr4R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_5` reader - ack_with_err_5\n\nThis bit retrieves the Peripheral Timeout error from the\n\nAcknowledge Error report."]
pub type AckWithErr5R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_6` reader - ack_with_err_6\n\nThis bit retrieves the False Control error from the Acknowledge\n\nerror report."]
pub type AckWithErr6R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_7` reader - ack_with_err_7\n\nThis bit retrieves the reserved (specific to device) from the\n\nAcknowledge error report."]
pub type AckWithErr7R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_8` reader - ack_with_err_8\n\nThis bit retrieves the ECC error, single-bit (detected and corrected)\n\nfrom the Acknowledge error report."]
pub type AckWithErr8R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_9` reader - ack_with_err_9\n\nThis bit retrieves the ECC error, multi-bit (detected, not corrected)\n\nfrom the Acknowledge error report."]
pub type AckWithErr9R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_10` reader - ack_with_err_10\n\nThis bit retrieves the checksum error (long packet only) from the\n\nAcknowledge error report."]
pub type AckWithErr10R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_11` reader - ack_with_err_11\n\nThis bit retrieves the not recognized DSI data type from the\n\nAcknowledge error report."]
pub type AckWithErr11R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_12` reader - ack_with_err_12\n\nThis bit retrieves the DSI VC ID Invalid from the Acknowledge error\n\nreport."]
pub type AckWithErr12R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_13` reader - ack_with_err_13\n\nThis bit retrieves the invalid transmission length from the\n\nAcknowledge error report."]
pub type AckWithErr13R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_14` reader - ack_with_err_14\n\nThis bit retrieves the reserved (specific to device) from the\n\nAcknowledge error report."]
pub type AckWithErr14R = crate::BitReader;
#[doc = "Field `ACK_WITH_ERR_15` reader - ack_with_err_15\n\nThis bit retrieves the DSI protocol violation from the Acknowledge\n\nerror report."]
pub type AckWithErr15R = crate::BitReader;
#[doc = "Field `DPHY_ERRORS_0` reader - dphy_errors_0\n\nThis bit indicates ErrEsc escape entry error from Lane 0."]
pub type DphyErrors0R = crate::BitReader;
#[doc = "Field `DPHY_ERRORS_1` reader - dphy_errors_1\n\nThis bit indicates ErrSyncEsc low-power data transmission\n\nsynchronization error from Lane 0."]
pub type DphyErrors1R = crate::BitReader;
#[doc = "Field `DPHY_ERRORS_2` reader - dphy_errors_2\n\nThis bit indicates the ErrControl error from Lane 0."]
pub type DphyErrors2R = crate::BitReader;
#[doc = "Field `DPHY_ERRORS_3` reader - dphy_errors_3\n\nThis bit indicates the LP0 contention error ErrContentionLP0 from\n\nLane 0."]
pub type DphyErrors3R = crate::BitReader;
#[doc = "Field `DPHY_ERRORS_4` reader - dphy_errors_4\n\nThis bit indicates the LP1 contention error ErrContentionLP1 from\n\nLane 0."]
pub type DphyErrors4R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - ack_with_err_0\n\nThis bit retrieves the SoT error from the Acknowledge error report."]
    #[inline(always)]
    pub fn ack_with_err_0(&self) -> AckWithErr0R {
        AckWithErr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ack_with_err_1\n\nThis bit retrieves the SoT Sync error from the Acknowledge error\n\nreport."]
    #[inline(always)]
    pub fn ack_with_err_1(&self) -> AckWithErr1R {
        AckWithErr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ack_with_err_2\n\nThis bit retrieves the EoT Sync error from the Acknowledge error\n\nreport."]
    #[inline(always)]
    pub fn ack_with_err_2(&self) -> AckWithErr2R {
        AckWithErr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ack_with_err_3\n\nThis bit retrieves the Escape Mode Entry Command error from the\n\nAcknowledge error report."]
    #[inline(always)]
    pub fn ack_with_err_3(&self) -> AckWithErr3R {
        AckWithErr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ack_with_err_4\n\nThis bit retrieves the LP Transmit Sync error from the Acknowledge\n\nerror report."]
    #[inline(always)]
    pub fn ack_with_err_4(&self) -> AckWithErr4R {
        AckWithErr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ack_with_err_5\n\nThis bit retrieves the Peripheral Timeout error from the\n\nAcknowledge Error report."]
    #[inline(always)]
    pub fn ack_with_err_5(&self) -> AckWithErr5R {
        AckWithErr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ack_with_err_6\n\nThis bit retrieves the False Control error from the Acknowledge\n\nerror report."]
    #[inline(always)]
    pub fn ack_with_err_6(&self) -> AckWithErr6R {
        AckWithErr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ack_with_err_7\n\nThis bit retrieves the reserved (specific to device) from the\n\nAcknowledge error report."]
    #[inline(always)]
    pub fn ack_with_err_7(&self) -> AckWithErr7R {
        AckWithErr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ack_with_err_8\n\nThis bit retrieves the ECC error, single-bit (detected and corrected)\n\nfrom the Acknowledge error report."]
    #[inline(always)]
    pub fn ack_with_err_8(&self) -> AckWithErr8R {
        AckWithErr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ack_with_err_9\n\nThis bit retrieves the ECC error, multi-bit (detected, not corrected)\n\nfrom the Acknowledge error report."]
    #[inline(always)]
    pub fn ack_with_err_9(&self) -> AckWithErr9R {
        AckWithErr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ack_with_err_10\n\nThis bit retrieves the checksum error (long packet only) from the\n\nAcknowledge error report."]
    #[inline(always)]
    pub fn ack_with_err_10(&self) -> AckWithErr10R {
        AckWithErr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ack_with_err_11\n\nThis bit retrieves the not recognized DSI data type from the\n\nAcknowledge error report."]
    #[inline(always)]
    pub fn ack_with_err_11(&self) -> AckWithErr11R {
        AckWithErr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ack_with_err_12\n\nThis bit retrieves the DSI VC ID Invalid from the Acknowledge error\n\nreport."]
    #[inline(always)]
    pub fn ack_with_err_12(&self) -> AckWithErr12R {
        AckWithErr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ack_with_err_13\n\nThis bit retrieves the invalid transmission length from the\n\nAcknowledge error report."]
    #[inline(always)]
    pub fn ack_with_err_13(&self) -> AckWithErr13R {
        AckWithErr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ack_with_err_14\n\nThis bit retrieves the reserved (specific to device) from the\n\nAcknowledge error report."]
    #[inline(always)]
    pub fn ack_with_err_14(&self) -> AckWithErr14R {
        AckWithErr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ack_with_err_15\n\nThis bit retrieves the DSI protocol violation from the Acknowledge\n\nerror report."]
    #[inline(always)]
    pub fn ack_with_err_15(&self) -> AckWithErr15R {
        AckWithErr15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - dphy_errors_0\n\nThis bit indicates ErrEsc escape entry error from Lane 0."]
    #[inline(always)]
    pub fn dphy_errors_0(&self) -> DphyErrors0R {
        DphyErrors0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - dphy_errors_1\n\nThis bit indicates ErrSyncEsc low-power data transmission\n\nsynchronization error from Lane 0."]
    #[inline(always)]
    pub fn dphy_errors_1(&self) -> DphyErrors1R {
        DphyErrors1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - dphy_errors_2\n\nThis bit indicates the ErrControl error from Lane 0."]
    #[inline(always)]
    pub fn dphy_errors_2(&self) -> DphyErrors2R {
        DphyErrors2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - dphy_errors_3\n\nThis bit indicates the LP0 contention error ErrContentionLP0 from\n\nLane 0."]
    #[inline(always)]
    pub fn dphy_errors_3(&self) -> DphyErrors3R {
        DphyErrors3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - dphy_errors_4\n\nThis bit indicates the LP1 contention error ErrContentionLP1 from\n\nLane 0."]
    #[inline(always)]
    pub fn dphy_errors_4(&self) -> DphyErrors4R {
        DphyErrors4R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "Interrupt Status Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntSt0Spec;
impl crate::RegisterSpec for IntSt0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st0::R`](R) reader structure"]
impl crate::Readable for IntSt0Spec {}
#[doc = "`reset()` method sets INT_ST0 to value 0"]
impl crate::Resettable for IntSt0Spec {
    const RESET_VALUE: u32 = 0;
}
