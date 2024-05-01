#[doc = "Register `INT_FORCE0` writer"]
pub type W = crate::W<IntForce0Spec>;
#[doc = "Field `ACK_WITH_ERR_0` writer - ack_with_err_0\n\nThis bit retrieves the SoT error from the Acknowledge error report."]
pub type AckWithErr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_WITH_ERR_1` writer - ack_with_err_1\n\nThis bit retrieves the SoT Sync error from the Acknowledge error\n\nreport."]
pub type AckWithErr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_WITH_ERR_2` writer - ack_with_err_2\n\nThis bit retrieves the EoT Sync error from the Acknowledge error\n\nreport."]
pub type AckWithErr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_WITH_ERR_3` writer - ack_with_err_3\n\nThis bit retrieves the Escape Mode Entry Command error from the\n\nAcknowledge error report."]
pub type AckWithErr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_WITH_ERR_4` writer - ack_with_err_4\n\nThis bit retrieves the LP Transmit Sync error from the Acknowledge\n\nerror report."]
pub type AckWithErr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_WITH_ERR_5` writer - ack_with_err_5\n\nThis bit retrieves the Peripheral Timeout error from the\n\nAcknowledge Error report."]
pub type AckWithErr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_WITH_ERR_6` writer - ack_with_err_6\n\nThis bit retrieves the False Control error from the Acknowledge\n\nerror report."]
pub type AckWithErr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_WITH_ERR_7` writer - ack_with_err_7\n\nThis bit retrieves the reserved (specific to device) from the\n\nAcknowledge error report."]
pub type AckWithErr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_WITH_ERR_8` writer - ack_with_err_8\n\nThis bit retrieves the ECC error, single-bit (detected and corrected)\n\nfrom the Acknowledge error report."]
pub type AckWithErr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_WITH_ERR_9` writer - ack_with_err_9\n\nThis bit retrieves the ECC error, multi-bit (detected, not corrected)\n\nfrom the Acknowledge error report."]
pub type AckWithErr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_WITH_ERR_10` writer - ack_with_err_10\n\nThis bit retrieves the checksum error (long packet only) from the\n\nAcknowledge error report."]
pub type AckWithErr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_WITH_ERR_11` writer - ack_with_err_11\n\nThis bit retrieves the not recognized DSI data type from the\n\nAcknowledge error report."]
pub type AckWithErr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_WITH_ERR_12` writer - ack_with_err_12\n\nThis bit retrieves the DSI VC ID Invalid from the Acknowledge error\n\nreport."]
pub type AckWithErr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_WITH_ERR_13` writer - ack_with_err_13\n\nThis bit retrieves the invalid transmission length from the\n\nAcknowledge error report."]
pub type AckWithErr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_WITH_ERR_14` writer - ack_with_err_14\n\nThis bit retrieves the reserved (specific to device) from the\n\nAcknowledge error report."]
pub type AckWithErr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACK_WITH_ERR_15` writer - ack_with_err_15\n\nThis bit retrieves the DSI protocol violation from the Acknowledge\n\nerror report."]
pub type AckWithErr15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_ERRORS_0` writer - dphy_errors_0\n\nThis bit indicates ErrEsc escape entry error from Lane 0."]
pub type DphyErrors0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_ERRORS_1` writer - dphy_errors_1\n\nThis bit indicates ErrSyncEsc low-power data transmission\n\nsynchronization error from Lane 0."]
pub type DphyErrors1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_ERRORS_2` writer - dphy_errors_2\n\nThis bit indicates the ErrControl error from Lane 0."]
pub type DphyErrors2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_ERRORS_3` writer - dphy_errors_3\n\nThis bit indicates the LP0 contention error ErrContentionLP0 from\n\nLane 0."]
pub type DphyErrors3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_ERRORS_4` writer - dphy_errors_4\n\nThis bit indicates the LP1 contention error ErrContentionLP1 from\n\nLane 0."]
pub type DphyErrors4W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - ack_with_err_0\n\nThis bit retrieves the SoT error from the Acknowledge error report."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_0(&mut self) -> AckWithErr0W<IntForce0Spec> {
        AckWithErr0W::new(self, 0)
    }
    #[doc = "Bit 1 - ack_with_err_1\n\nThis bit retrieves the SoT Sync error from the Acknowledge error\n\nreport."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_1(&mut self) -> AckWithErr1W<IntForce0Spec> {
        AckWithErr1W::new(self, 1)
    }
    #[doc = "Bit 2 - ack_with_err_2\n\nThis bit retrieves the EoT Sync error from the Acknowledge error\n\nreport."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_2(&mut self) -> AckWithErr2W<IntForce0Spec> {
        AckWithErr2W::new(self, 2)
    }
    #[doc = "Bit 3 - ack_with_err_3\n\nThis bit retrieves the Escape Mode Entry Command error from the\n\nAcknowledge error report."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_3(&mut self) -> AckWithErr3W<IntForce0Spec> {
        AckWithErr3W::new(self, 3)
    }
    #[doc = "Bit 4 - ack_with_err_4\n\nThis bit retrieves the LP Transmit Sync error from the Acknowledge\n\nerror report."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_4(&mut self) -> AckWithErr4W<IntForce0Spec> {
        AckWithErr4W::new(self, 4)
    }
    #[doc = "Bit 5 - ack_with_err_5\n\nThis bit retrieves the Peripheral Timeout error from the\n\nAcknowledge Error report."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_5(&mut self) -> AckWithErr5W<IntForce0Spec> {
        AckWithErr5W::new(self, 5)
    }
    #[doc = "Bit 6 - ack_with_err_6\n\nThis bit retrieves the False Control error from the Acknowledge\n\nerror report."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_6(&mut self) -> AckWithErr6W<IntForce0Spec> {
        AckWithErr6W::new(self, 6)
    }
    #[doc = "Bit 7 - ack_with_err_7\n\nThis bit retrieves the reserved (specific to device) from the\n\nAcknowledge error report."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_7(&mut self) -> AckWithErr7W<IntForce0Spec> {
        AckWithErr7W::new(self, 7)
    }
    #[doc = "Bit 8 - ack_with_err_8\n\nThis bit retrieves the ECC error, single-bit (detected and corrected)\n\nfrom the Acknowledge error report."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_8(&mut self) -> AckWithErr8W<IntForce0Spec> {
        AckWithErr8W::new(self, 8)
    }
    #[doc = "Bit 9 - ack_with_err_9\n\nThis bit retrieves the ECC error, multi-bit (detected, not corrected)\n\nfrom the Acknowledge error report."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_9(&mut self) -> AckWithErr9W<IntForce0Spec> {
        AckWithErr9W::new(self, 9)
    }
    #[doc = "Bit 10 - ack_with_err_10\n\nThis bit retrieves the checksum error (long packet only) from the\n\nAcknowledge error report."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_10(&mut self) -> AckWithErr10W<IntForce0Spec> {
        AckWithErr10W::new(self, 10)
    }
    #[doc = "Bit 11 - ack_with_err_11\n\nThis bit retrieves the not recognized DSI data type from the\n\nAcknowledge error report."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_11(&mut self) -> AckWithErr11W<IntForce0Spec> {
        AckWithErr11W::new(self, 11)
    }
    #[doc = "Bit 12 - ack_with_err_12\n\nThis bit retrieves the DSI VC ID Invalid from the Acknowledge error\n\nreport."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_12(&mut self) -> AckWithErr12W<IntForce0Spec> {
        AckWithErr12W::new(self, 12)
    }
    #[doc = "Bit 13 - ack_with_err_13\n\nThis bit retrieves the invalid transmission length from the\n\nAcknowledge error report."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_13(&mut self) -> AckWithErr13W<IntForce0Spec> {
        AckWithErr13W::new(self, 13)
    }
    #[doc = "Bit 14 - ack_with_err_14\n\nThis bit retrieves the reserved (specific to device) from the\n\nAcknowledge error report."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_14(&mut self) -> AckWithErr14W<IntForce0Spec> {
        AckWithErr14W::new(self, 14)
    }
    #[doc = "Bit 15 - ack_with_err_15\n\nThis bit retrieves the DSI protocol violation from the Acknowledge\n\nerror report."]
    #[inline(always)]
    #[must_use]
    pub fn ack_with_err_15(&mut self) -> AckWithErr15W<IntForce0Spec> {
        AckWithErr15W::new(self, 15)
    }
    #[doc = "Bit 16 - dphy_errors_0\n\nThis bit indicates ErrEsc escape entry error from Lane 0."]
    #[inline(always)]
    #[must_use]
    pub fn dphy_errors_0(&mut self) -> DphyErrors0W<IntForce0Spec> {
        DphyErrors0W::new(self, 16)
    }
    #[doc = "Bit 17 - dphy_errors_1\n\nThis bit indicates ErrSyncEsc low-power data transmission\n\nsynchronization error from Lane 0."]
    #[inline(always)]
    #[must_use]
    pub fn dphy_errors_1(&mut self) -> DphyErrors1W<IntForce0Spec> {
        DphyErrors1W::new(self, 17)
    }
    #[doc = "Bit 18 - dphy_errors_2\n\nThis bit indicates the ErrControl error from Lane 0."]
    #[inline(always)]
    #[must_use]
    pub fn dphy_errors_2(&mut self) -> DphyErrors2W<IntForce0Spec> {
        DphyErrors2W::new(self, 18)
    }
    #[doc = "Bit 19 - dphy_errors_3\n\nThis bit indicates the LP0 contention error ErrContentionLP0 from\n\nLane 0."]
    #[inline(always)]
    #[must_use]
    pub fn dphy_errors_3(&mut self) -> DphyErrors3W<IntForce0Spec> {
        DphyErrors3W::new(self, 19)
    }
    #[doc = "Bit 20 - dphy_errors_4\n\nThis bit indicates the LP1 contention error ErrContentionLP1 from\n\nLane 0."]
    #[inline(always)]
    #[must_use]
    pub fn dphy_errors_4(&mut self) -> DphyErrors4W<IntForce0Spec> {
        DphyErrors4W::new(self, 20)
    }
}
#[doc = "Force Interrupt Configuration Register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_force0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntForce0Spec;
impl crate::RegisterSpec for IntForce0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_force0::W`](W) writer structure"]
impl crate::Writable for IntForce0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_FORCE0 to value 0"]
impl crate::Resettable for IntForce0Spec {
    const RESET_VALUE: u32 = 0;
}
