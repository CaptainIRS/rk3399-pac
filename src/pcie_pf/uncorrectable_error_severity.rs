#[doc = "Register `UNCORRECTABLE_ERROR_SEVERITY` reader"]
pub type R = crate::R<UncorrectableErrorSeveritySpec>;
#[doc = "Register `UNCORRECTABLE_ERROR_SEVERITY` writer"]
pub type W = crate::W<UncorrectableErrorSeveritySpec>;
#[doc = "Field `R8` reader - Reserved \\[R8\\]
Reserved"]
pub type R8R = crate::FieldReader;
#[doc = "Field `DLPER` reader - Data Link Protocol Error Severity \\[DLPER\\]
Severity of Data Link Protocol Errors (0 = Non-Fatal, 1 = Fatal). STICKY."]
pub type DlperR = crate::BitReader;
#[doc = "Field `DLPER` writer - Data Link Protocol Error Severity \\[DLPER\\]
Severity of Data Link Protocol Errors (0 = Non-Fatal, 1 = Fatal). STICKY."]
pub type DlperW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDES` reader - Surprise Down Error Severity \\[SDES\\]
hard coded to 1"]
pub type SdesR = crate::BitReader;
#[doc = "Field `R10` reader - Reserved \\[R10\\]
Reserved"]
pub type R10R = crate::FieldReader;
#[doc = "Field `PTS` reader - Poisoned TLP Severity \\[PTS\\]
Severity of a Poisoned TLP error (0 = Non-Fatal, 1= Fatal). STICKY."]
pub type PtsR = crate::BitReader;
#[doc = "Field `PTS` writer - Poisoned TLP Severity \\[PTS\\]
Severity of a Poisoned TLP error (0 = Non-Fatal, 1= Fatal). STICKY."]
pub type PtsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCPES` reader - Flow Control Protocol Error Severity \\[FCPES\\]
Severity of Flow Control Protocol Errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type FcpesR = crate::BitReader;
#[doc = "Field `FCPES` writer - Flow Control Protocol Error Severity \\[FCPES\\]
Severity of Flow Control Protocol Errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
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
#[doc = "Field `URES` reader - Unsupported Requeset Error Severity \\[URES\\]
Severity of unexpected requests received from the link (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type UresR = crate::BitReader;
#[doc = "Field `URES` writer - Unsupported Requeset Error Severity \\[URES\\]
Severity of unexpected requests received from the link (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type UresW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R11` reader - Reserved \\[R11\\]
Reserved"]
pub type R11R = crate::BitReader;
#[doc = "Field `UIES` reader - Uncorrectable Internal Error Severity \\[UIES\\]
Severity of internal errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type UiesR = crate::BitReader;
#[doc = "Field `UIES` writer - Uncorrectable Internal Error Severity \\[UIES\\]
Severity of internal errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
pub type UiesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R12` reader - Reserved \\[R12\\]
Reserved"]
pub type R12R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Reserved \\[R8\\]
Reserved"]
    #[inline(always)]
    pub fn r8(&self) -> R8R {
        R8R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Data Link Protocol Error Severity \\[DLPER\\]
Severity of Data Link Protocol Errors (0 = Non-Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    pub fn dlper(&self) -> DlperR {
        DlperR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Surprise Down Error Severity \\[SDES\\]
hard coded to 1"]
    #[inline(always)]
    pub fn sdes(&self) -> SdesR {
        SdesR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:11 - Reserved \\[R10\\]
Reserved"]
    #[inline(always)]
    pub fn r10(&self) -> R10R {
        R10R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bit 12 - Poisoned TLP Severity \\[PTS\\]
Severity of a Poisoned TLP error (0 = Non-Fatal, 1= Fatal). STICKY."]
    #[inline(always)]
    pub fn pts(&self) -> PtsR {
        PtsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flow Control Protocol Error Severity \\[FCPES\\]
Severity of Flow Control Protocol Errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
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
    #[doc = "Bit 20 - Unsupported Requeset Error Severity \\[URES\\]
Severity of unexpected requests received from the link (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    pub fn ures(&self) -> UresR {
        UresR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved \\[R11\\]
Reserved"]
    #[inline(always)]
    pub fn r11(&self) -> R11R {
        R11R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Uncorrectable Internal Error Severity \\[UIES\\]
Severity of internal errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    pub fn uies(&self) -> UiesR {
        UiesR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - Reserved \\[R12\\]
Reserved"]
    #[inline(always)]
    pub fn r12(&self) -> R12R {
        R12R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 4 - Data Link Protocol Error Severity \\[DLPER\\]
Severity of Data Link Protocol Errors (0 = Non-Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn dlper(&mut self) -> DlperW<UncorrectableErrorSeveritySpec> {
        DlperW::new(self, 4)
    }
    #[doc = "Bit 12 - Poisoned TLP Severity \\[PTS\\]
Severity of a Poisoned TLP error (0 = Non-Fatal, 1= Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn pts(&mut self) -> PtsW<UncorrectableErrorSeveritySpec> {
        PtsW::new(self, 12)
    }
    #[doc = "Bit 13 - Flow Control Protocol Error Severity \\[FCPES\\]
Severity of Flow Control Protocol Errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn fcpes(&mut self) -> FcpesW<UncorrectableErrorSeveritySpec> {
        FcpesW::new(self, 13)
    }
    #[doc = "Bit 14 - Completion Timeout Severity \\[CTS\\]
Severity of Completion Timeouts (0 = Non-Fatal, 1= Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CtsW<UncorrectableErrorSeveritySpec> {
        CtsW::new(self, 14)
    }
    #[doc = "Bit 15 - Completer Abort Severity \\[CAS\\]
Severity of sending a Completer Abort (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn cas(&mut self) -> CasW<UncorrectableErrorSeveritySpec> {
        CasW::new(self, 15)
    }
    #[doc = "Bit 16 - Unexpected Completion Severity \\[UCS\\]
Severity of unexpected Completions received by the core (0 = Non-Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ucs(&mut self) -> UcsW<UncorrectableErrorSeveritySpec> {
        UcsW::new(self, 16)
    }
    #[doc = "Bit 17 - Receiver Overflow Severity \\[ROS\\]
Severity of receive credit violations (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ros(&mut self) -> RosW<UncorrectableErrorSeveritySpec> {
        RosW::new(self, 17)
    }
    #[doc = "Bit 18 - Malformed TLP Severity \\[MTS\\]
Severity of malformed TLPs received from the link (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn mts(&mut self) -> MtsW<UncorrectableErrorSeveritySpec> {
        MtsW::new(self, 18)
    }
    #[doc = "Bit 19 - ECRC Error Severity \\[EES\\]
Severity of ECRC errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ees(&mut self) -> EesW<UncorrectableErrorSeveritySpec> {
        EesW::new(self, 19)
    }
    #[doc = "Bit 20 - Unsupported Requeset Error Severity \\[URES\\]
Severity of unexpected requests received from the link (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ures(&mut self) -> UresW<UncorrectableErrorSeveritySpec> {
        UresW::new(self, 20)
    }
    #[doc = "Bit 22 - Uncorrectable Internal Error Severity \\[UIES\\]
Severity of internal errors (0 = Non- Fatal, 1 = Fatal). STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn uies(&mut self) -> UiesW<UncorrectableErrorSeveritySpec> {
        UiesW::new(self, 22)
    }
}
#[doc = "Uncorrectable Error Severity Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uncorrectable_error_severity::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uncorrectable_error_severity::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UncorrectableErrorSeveritySpec;
impl crate::RegisterSpec for UncorrectableErrorSeveritySpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uncorrectable_error_severity::R`](R) reader structure"]
impl crate::Readable for UncorrectableErrorSeveritySpec {}
#[doc = "`write(|w| ..)` method takes [`uncorrectable_error_severity::W`](W) writer structure"]
impl crate::Writable for UncorrectableErrorSeveritySpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UNCORRECTABLE_ERROR_SEVERITY to value 0x0046_2030"]
impl crate::Resettable for UncorrectableErrorSeveritySpec {
    const RESET_VALUE: u32 = 0x0046_2030;
}
