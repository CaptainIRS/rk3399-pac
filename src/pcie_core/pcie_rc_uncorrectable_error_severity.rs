#[doc = "Register `PCIE_RC_UNCORRECTABLE_ERROR_SEVERITY` reader"]
pub type R = crate::R<PcieRcUncorrectableErrorSeveritySpec>;
#[doc = "Register `PCIE_RC_UNCORRECTABLE_ERROR_SEVERITY` writer"]
pub type W = crate::W<PcieRcUncorrectableErrorSeveritySpec>;
#[doc = "Field `R33` reader - Reserved \\[R33\\]
Reserved"]
pub type R33R = crate::FieldReader;
#[doc = "Field `DLPES` reader - Data Link Protocol Error Severity \\[DLPES\\]
Severity of Data Link Protocol Errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type DlpesR = crate::BitReader;
#[doc = "Field `DLPES` writer - Data Link Protocol Error Severity \\[DLPES\\]
Severity of Data Link Protocol Errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type DlpesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDES` reader - surprise down error severity \\[SDES\\]
surprise down error severity. This field is hard coded to 1."]
pub type SdesR = crate::BitReader;
#[doc = "Field `R35` reader - Reserved \\[R35\\]
(no description)"]
pub type R35R = crate::FieldReader;
#[doc = "Field `PTS` reader - Poisoned TLP Severity \\[PTS\\]
Severity of a Poisoned TLP error (0 = Non-Fatal, 1= Fatal). STICKY."]
pub type PtsR = crate::BitReader;
#[doc = "Field `PTS` writer - Poisoned TLP Severity \\[PTS\\]
Severity of a Poisoned TLP error (0 = Non-Fatal, 1= Fatal). STICKY."]
pub type PtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCPES` reader - Flow Control Protocol Error Severity \\[FCPES\\]
Severity of a Flow Control Protocol Error (0 = Non-Fatal, 1 = Fatal). STICKY."]
pub type FcpesR = crate::BitReader;
#[doc = "Field `FCPES` writer - Flow Control Protocol Error Severity \\[FCPES\\]
Severity of a Flow Control Protocol Error (0 = Non-Fatal, 1 = Fatal). STICKY."]
pub type FcpesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTS` reader - Completion Timeout Severity \\[CTS\\]
Severity of Completion Timeouts (0 = Non-Fatal, 1= Fatal). STICKY."]
pub type CtsR = crate::BitReader;
#[doc = "Field `CTS` writer - Completion Timeout Severity \\[CTS\\]
Severity of Completion Timeouts (0 = Non-Fatal, 1= Fatal). STICKY."]
pub type CtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAS` reader - Completer Abort Severity \\[CAS\\]
Severity of sending a Completer Abort (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type CasR = crate::BitReader;
#[doc = "Field `CAS` writer - Completer Abort Severity \\[CAS\\]
Severity of sending a Completer Abort (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type CasW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UCS` reader - Unexpected Completion Severity \\[UCS\\]
Severity of unexpected Completions received by the core (0 = Non-Fatal, 1 = Fatal). STICKY."]
pub type UcsR = crate::BitReader;
#[doc = "Field `UCS` writer - Unexpected Completion Severity \\[UCS\\]
Severity of unexpected Completions received by the core (0 = Non-Fatal, 1 = Fatal). STICKY."]
pub type UcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROS` reader - Receiver Overflow Severity \\[ROS\\]
Severity of receive credit violations (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type RosR = crate::BitReader;
#[doc = "Field `ROS` writer - Receiver Overflow Severity \\[ROS\\]
Severity of receive credit violations (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type RosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MTS` reader - Malformed TLP Severity \\[MTS\\]
Severity of malformed TLPs received from the link (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type MtsR = crate::BitReader;
#[doc = "Field `MTS` writer - Malformed TLP Severity \\[MTS\\]
Severity of malformed TLPs received from the link (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type MtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EES` reader - ECRC Error Severity \\[EES\\]
Severity of ECRC errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type EesR = crate::BitReader;
#[doc = "Field `EES` writer - ECRC Error Severity \\[EES\\]
Severity of ECRC errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type EesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URES` reader - Unsupported Request Error Severity \\[URES\\]
Severity of unexpected requests received from the link (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type UresR = crate::BitReader;
#[doc = "Field `URES` writer - Unsupported Request Error Severity \\[URES\\]
Severity of unexpected requests received from the link (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type UresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R36` reader - Reserved \\[R36\\]
Reserved"]
pub type R36R = crate::BitReader;
#[doc = "Field `Uncorr_Intrnal_Err_Svrty` reader - Uncorrectable Internal Error Severity \\[Uncorr_Intrnal_ Err_Svrty\\]
Severity of internal errors (0 = Non-Fatal, 1 =Fatal)."]
pub type UncorrIntrnalErrSvrtyR = crate::BitReader;
#[doc = "Field `Uncorr_Intrnal_Err_Svrty` writer - Uncorrectable Internal Error Severity \\[Uncorr_Intrnal_ Err_Svrty\\]
Severity of internal errors (0 = Non-Fatal, 1 =Fatal)."]
pub type UncorrIntrnalErrSvrtyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R37` reader - Reserved \\[R37\\]
(no description)"]
pub type R37R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Reserved \\[R33\\]
Reserved"]
    #[inline(always)]
    pub fn r33(&self) -> R33R {
        R33R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Data Link Protocol Error Severity \\[DLPES\\]
Severity of Data Link Protocol Errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    pub fn dlpes(&self) -> DlpesR {
        DlpesR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - surprise down error severity \\[SDES\\]
surprise down error severity. This field is hard coded to 1."]
    #[inline(always)]
    pub fn sdes(&self) -> SdesR {
        SdesR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:11 - Reserved \\[R35\\]
(no description)"]
    #[inline(always)]
    pub fn r35(&self) -> R35R {
        R35R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Poisoned TLP Severity \\[PTS\\]
Severity of a Poisoned TLP error (0 = Non-Fatal, 1= Fatal). STICKY."]
    #[inline(always)]
    pub fn pts(&self) -> PtsR {
        PtsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flow Control Protocol Error Severity \\[FCPES\\]
Severity of a Flow Control Protocol Error (0 = Non-Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    pub fn fcpes(&self) -> FcpesR {
        FcpesR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Completion Timeout Severity \\[CTS\\]
Severity of Completion Timeouts (0 = Non-Fatal, 1= Fatal). STICKY."]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Completer Abort Severity \\[CAS\\]
Severity of sending a Completer Abort (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    pub fn cas(&self) -> CasR {
        CasR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Unexpected Completion Severity \\[UCS\\]
Severity of unexpected Completions received by the core (0 = Non-Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    pub fn ucs(&self) -> UcsR {
        UcsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receiver Overflow Severity \\[ROS\\]
Severity of receive credit violations (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    pub fn ros(&self) -> RosR {
        RosR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Malformed TLP Severity \\[MTS\\]
Severity of malformed TLPs received from the link (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    pub fn mts(&self) -> MtsR {
        MtsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ECRC Error Severity \\[EES\\]
Severity of ECRC errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    pub fn ees(&self) -> EesR {
        EesR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Unsupported Request Error Severity \\[URES\\]
Severity of unexpected requests received from the link (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    pub fn ures(&self) -> UresR {
        UresR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved \\[R36\\]
Reserved"]
    #[inline(always)]
    pub fn r36(&self) -> R36R {
        R36R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Uncorrectable Internal Error Severity \\[Uncorr_Intrnal_ Err_Svrty\\]
Severity of internal errors (0 = Non-Fatal, 1 =Fatal)."]
    #[inline(always)]
    pub fn uncorr_intrnal_err_svrty(&self) -> UncorrIntrnalErrSvrtyR {
        UncorrIntrnalErrSvrtyR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - Reserved \\[R37\\]
(no description)"]
    #[inline(always)]
    pub fn r37(&self) -> R37R {
        R37R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 4 - Data Link Protocol Error Severity \\[DLPES\\]
Severity of Data Link Protocol Errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn dlpes(&mut self) -> DlpesW<PcieRcUncorrectableErrorSeveritySpec> {
        DlpesW::new(self, 4)
    }
    #[doc = "Bit 12 - Poisoned TLP Severity \\[PTS\\]
Severity of a Poisoned TLP error (0 = Non-Fatal, 1= Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn pts(&mut self) -> PtsW<PcieRcUncorrectableErrorSeveritySpec> {
        PtsW::new(self, 12)
    }
    #[doc = "Bit 13 - Flow Control Protocol Error Severity \\[FCPES\\]
Severity of a Flow Control Protocol Error (0 = Non-Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn fcpes(&mut self) -> FcpesW<PcieRcUncorrectableErrorSeveritySpec> {
        FcpesW::new(self, 13)
    }
    #[doc = "Bit 14 - Completion Timeout Severity \\[CTS\\]
Severity of Completion Timeouts (0 = Non-Fatal, 1= Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CtsW<PcieRcUncorrectableErrorSeveritySpec> {
        CtsW::new(self, 14)
    }
    #[doc = "Bit 15 - Completer Abort Severity \\[CAS\\]
Severity of sending a Completer Abort (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn cas(&mut self) -> CasW<PcieRcUncorrectableErrorSeveritySpec> {
        CasW::new(self, 15)
    }
    #[doc = "Bit 16 - Unexpected Completion Severity \\[UCS\\]
Severity of unexpected Completions received by the core (0 = Non-Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ucs(&mut self) -> UcsW<PcieRcUncorrectableErrorSeveritySpec> {
        UcsW::new(self, 16)
    }
    #[doc = "Bit 17 - Receiver Overflow Severity \\[ROS\\]
Severity of receive credit violations (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ros(&mut self) -> RosW<PcieRcUncorrectableErrorSeveritySpec> {
        RosW::new(self, 17)
    }
    #[doc = "Bit 18 - Malformed TLP Severity \\[MTS\\]
Severity of malformed TLPs received from the link (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn mts(&mut self) -> MtsW<PcieRcUncorrectableErrorSeveritySpec> {
        MtsW::new(self, 18)
    }
    #[doc = "Bit 19 - ECRC Error Severity \\[EES\\]
Severity of ECRC errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ees(&mut self) -> EesW<PcieRcUncorrectableErrorSeveritySpec> {
        EesW::new(self, 19)
    }
    #[doc = "Bit 20 - Unsupported Request Error Severity \\[URES\\]
Severity of unexpected requests received from the link (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ures(&mut self) -> UresW<PcieRcUncorrectableErrorSeveritySpec> {
        UresW::new(self, 20)
    }
    #[doc = "Bit 22 - Uncorrectable Internal Error Severity \\[Uncorr_Intrnal_ Err_Svrty\\]
Severity of internal errors (0 = Non-Fatal, 1 =Fatal)."]
    #[inline(always)]
    #[must_use]
    pub fn uncorr_intrnal_err_svrty(
        &mut self,
    ) -> UncorrIntrnalErrSvrtyW<PcieRcUncorrectableErrorSeveritySpec> {
        UncorrIntrnalErrSvrtyW::new(self, 22)
    }
}
#[doc = "Uncorrectable Error Severity Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_uncorrectable_error_severity::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_uncorrectable_error_severity::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcUncorrectableErrorSeveritySpec;
impl crate::RegisterSpec for PcieRcUncorrectableErrorSeveritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_uncorrectable_error_severity::R`](R) reader structure"]
impl crate::Readable for PcieRcUncorrectableErrorSeveritySpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_uncorrectable_error_severity::W`](W) writer structure"]
impl crate::Writable for PcieRcUncorrectableErrorSeveritySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_RC_UNCORRECTABLE_ERROR_SEVERITY to value 0x0046_2030"]
impl crate::Resettable for PcieRcUncorrectableErrorSeveritySpec {
    const RESET_VALUE: u32 = 0x0046_2030;
}
