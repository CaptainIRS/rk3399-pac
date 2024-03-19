#[doc = "Register `PCIE_VF_UNCORRECTABLE_ERROR_STATUS` reader"]
pub type R = crate::R<PcieVfUncorrectableErrorStatusSpec>;
#[doc = "Register `PCIE_VF_UNCORRECTABLE_ERROR_STATUS` writer"]
pub type W = crate::W<PcieVfUncorrectableErrorStatusSpec>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Reserved"]
pub type R0R = crate::FieldReader;
#[doc = "Field `DLPER` reader - Data Link Protocol Error Status \\[DLPER\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type DlperR = crate::BitReader;
#[doc = "Field `R1` reader - Reserved \\[R1\\]
Reserved"]
pub type R1R = crate::FieldReader;
#[doc = "Field `PTS` reader - Poisoned TLP Status \\[PTS\\]
This bit is set when the core receives a poisoned TLP from the link, targeted at this VF. This error is Function-specific. This error is considered non- fatal by default. The error is reported by sending an ERR_NONFATAL message. The header of the received TLP with error is logged in the Header Log Registers associated with the VF. STICKY."]
pub type PtsR = crate::BitReader;
#[doc = "Field `PTS` writer - Poisoned TLP Status \\[PTS\\]
This bit is set when the core receives a poisoned TLP from the link, targeted at this VF. This error is Function-specific. This error is considered non- fatal by default. The error is reported by sending an ERR_NONFATAL message. The header of the received TLP with error is logged in the Header Log Registers associated with the VF. STICKY."]
pub type PtsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FCPES` reader - Flow Control Protocol Error Status \\[FCPES\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type FcpesR = crate::BitReader;
#[doc = "Field `CTS` reader - Completion Timeout Status \\[CTS\\]
This bit is set when the completion timer associated with an outstanding request times out. This error is Function- specific. This error is considered non-fatal by default. STICKY."]
pub type CtsR = crate::BitReader;
#[doc = "Field `CTS` writer - Completion Timeout Status \\[CTS\\]
This bit is set when the completion timer associated with an outstanding request times out. This error is Function- specific. This error is considered non-fatal by default. STICKY."]
pub type CtsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CAS` reader - Completer Abort Status \\[CAS\\]
This bit is set when the core has returned the Completer Abort (CA) status to a request received from the link. This error is Function-specific. The header of the received request that caused the error is logged in the Header Log Registers. STICKY."]
pub type CasR = crate::BitReader;
#[doc = "Field `CAS` writer - Completer Abort Status \\[CAS\\]
This bit is set when the core has returned the Completer Abort (CA) status to a request received from the link. This error is Function-specific. The header of the received request that caused the error is logged in the Header Log Registers. STICKY."]
pub type CasW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `UCS` reader - Unexpected Completion Status \\[UCS\\]
This bit is set when the core has received an unexpected Completion packet from the link. This error is not Function- specific. STICKY."]
pub type UcsR = crate::BitReader;
#[doc = "Field `UCS` writer - Unexpected Completion Status \\[UCS\\]
This bit is set when the core has received an unexpected Completion packet from the link. This error is not Function- specific. STICKY."]
pub type UcsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `Rcvr_Overflow_Status` reader - Receiver Overflow Status \\[Rcvr_Overflow _Status\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type RcvrOverflowStatusR = crate::BitReader;
#[doc = "Field `Malformed_TL_Status` reader - Malformed TLP Status \\[Malformed_TL_Status\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type MalformedTlStatusR = crate::BitReader;
#[doc = "Field `ECRC_Err_Status` reader - ECRC Error Status \\[ECRC_Err_Stat us\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type EcrcErrStatusR = crate::BitReader;
#[doc = "Field `URES` reader - Unsupported Request Error Status \\[URES\\]
This bit is set when the core has received a request from the link that it does not support. This error is not Function-specific. This error is considered non-fatal by default. In the special case described in Sections 6.2.3.2.4.1 of the PCI Express Specifications, the error is reported by sending an ERR_COR message. In all other cases, the error is reported by sending an ERR_NONFATAL message. The header of the received request that caused the error is logged in the Header Log Registers. STICKY."]
pub type UresR = crate::BitReader;
#[doc = "Field `URES` writer - Unsupported Request Error Status \\[URES\\]
This bit is set when the core has received a request from the link that it does not support. This error is not Function-specific. This error is considered non-fatal by default. In the special case described in Sections 6.2.3.2.4.1 of the PCI Express Specifications, the error is reported by sending an ERR_COR message. In all other cases, the error is reported by sending an ERR_NONFATAL message. The header of the received request that caused the error is logged in the Header Log Registers. STICKY."]
pub type UresW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R2` reader - Reserved \\[R2\\]
Reserved"]
pub type R2R = crate::BitReader;
#[doc = "Field `Uncorr_Int_Err_status` reader - Uncorrectable Internal Error Status \\[Uncorr_Int_Err_status\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
pub type UncorrIntErrStatusR = crate::BitReader;
#[doc = "Field `R3` reader - Reserved \\[R3\\]
Reserved"]
pub type R3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Reserved \\[R0\\]
Reserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Data Link Protocol Error Status \\[DLPER\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn dlper(&self) -> DlperR {
        DlperR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:11 - Reserved \\[R1\\]
Reserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 5) & 0x7f) as u8)
    }
    #[doc = "Bit 12 - Poisoned TLP Status \\[PTS\\]
This bit is set when the core receives a poisoned TLP from the link, targeted at this VF. This error is Function-specific. This error is considered non- fatal by default. The error is reported by sending an ERR_NONFATAL message. The header of the received TLP with error is logged in the Header Log Registers associated with the VF. STICKY."]
    #[inline(always)]
    pub fn pts(&self) -> PtsR {
        PtsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flow Control Protocol Error Status \\[FCPES\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn fcpes(&self) -> FcpesR {
        FcpesR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Completion Timeout Status \\[CTS\\]
This bit is set when the completion timer associated with an outstanding request times out. This error is Function- specific. This error is considered non-fatal by default. STICKY."]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Completer Abort Status \\[CAS\\]
This bit is set when the core has returned the Completer Abort (CA) status to a request received from the link. This error is Function-specific. The header of the received request that caused the error is logged in the Header Log Registers. STICKY."]
    #[inline(always)]
    pub fn cas(&self) -> CasR {
        CasR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Unexpected Completion Status \\[UCS\\]
This bit is set when the core has received an unexpected Completion packet from the link. This error is not Function- specific. STICKY."]
    #[inline(always)]
    pub fn ucs(&self) -> UcsR {
        UcsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receiver Overflow Status \\[Rcvr_Overflow _Status\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn rcvr_overflow_status(&self) -> RcvrOverflowStatusR {
        RcvrOverflowStatusR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Malformed TLP Status \\[Malformed_TL_Status\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn malformed_tl_status(&self) -> MalformedTlStatusR {
        MalformedTlStatusR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ECRC Error Status \\[ECRC_Err_Stat us\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn ecrc_err_status(&self) -> EcrcErrStatusR {
        EcrcErrStatusR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Unsupported Request Error Status \\[URES\\]
This bit is set when the core has received a request from the link that it does not support. This error is not Function-specific. This error is considered non-fatal by default. In the special case described in Sections 6.2.3.2.4.1 of the PCI Express Specifications, the error is reported by sending an ERR_COR message. In all other cases, the error is reported by sending an ERR_NONFATAL message. The header of the received request that caused the error is logged in the Header Log Registers. STICKY."]
    #[inline(always)]
    pub fn ures(&self) -> UresR {
        UresR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved \\[R2\\]
Reserved"]
    #[inline(always)]
    pub fn r2(&self) -> R2R {
        R2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Uncorrectable Internal Error Status \\[Uncorr_Int_Err_status\\]
This bit is not implemented for Virtual Functions. Hardwired to 0."]
    #[inline(always)]
    pub fn uncorr_int_err_status(&self) -> UncorrIntErrStatusR {
        UncorrIntErrStatusR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - Reserved \\[R3\\]
Reserved"]
    #[inline(always)]
    pub fn r3(&self) -> R3R {
        R3R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 12 - Poisoned TLP Status \\[PTS\\]
This bit is set when the core receives a poisoned TLP from the link, targeted at this VF. This error is Function-specific. This error is considered non- fatal by default. The error is reported by sending an ERR_NONFATAL message. The header of the received TLP with error is logged in the Header Log Registers associated with the VF. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn pts(&mut self) -> PtsW<PcieVfUncorrectableErrorStatusSpec> {
        PtsW::new(self, 12)
    }
    #[doc = "Bit 14 - Completion Timeout Status \\[CTS\\]
This bit is set when the completion timer associated with an outstanding request times out. This error is Function- specific. This error is considered non-fatal by default. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CtsW<PcieVfUncorrectableErrorStatusSpec> {
        CtsW::new(self, 14)
    }
    #[doc = "Bit 15 - Completer Abort Status \\[CAS\\]
This bit is set when the core has returned the Completer Abort (CA) status to a request received from the link. This error is Function-specific. The header of the received request that caused the error is logged in the Header Log Registers. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn cas(&mut self) -> CasW<PcieVfUncorrectableErrorStatusSpec> {
        CasW::new(self, 15)
    }
    #[doc = "Bit 16 - Unexpected Completion Status \\[UCS\\]
This bit is set when the core has received an unexpected Completion packet from the link. This error is not Function- specific. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ucs(&mut self) -> UcsW<PcieVfUncorrectableErrorStatusSpec> {
        UcsW::new(self, 16)
    }
    #[doc = "Bit 20 - Unsupported Request Error Status \\[URES\\]
This bit is set when the core has received a request from the link that it does not support. This error is not Function-specific. This error is considered non-fatal by default. In the special case described in Sections 6.2.3.2.4.1 of the PCI Express Specifications, the error is reported by sending an ERR_COR message. In all other cases, the error is reported by sending an ERR_NONFATAL message. The header of the received request that caused the error is logged in the Header Log Registers. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ures(&mut self) -> UresW<PcieVfUncorrectableErrorStatusSpec> {
        UresW::new(self, 20)
    }
}
#[doc = "Uncorrectable Error Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_uncorrectable_error_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_uncorrectable_error_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfUncorrectableErrorStatusSpec;
impl crate::RegisterSpec for PcieVfUncorrectableErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_uncorrectable_error_status::R`](R) reader structure"]
impl crate::Readable for PcieVfUncorrectableErrorStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_vf_uncorrectable_error_status::W`](W) writer structure"]
impl crate::Writable for PcieVfUncorrectableErrorStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x0011_d000;
}
#[doc = "`reset()` method sets PCIE_VF_UNCORRECTABLE_ERROR_STATUS to value 0"]
impl crate::Resettable for PcieVfUncorrectableErrorStatusSpec {
    const RESET_VALUE: u32 = 0;
}
