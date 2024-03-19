#[doc = "Register `PCIE_PF_UNCORRECTABLE_ERROR_STATUS` reader"]
pub type R = crate::R<PciePfUncorrectableErrorStatusSpec>;
#[doc = "Register `PCIE_PF_UNCORRECTABLE_ERROR_STATUS` writer"]
pub type W = crate::W<PciePfUncorrectableErrorStatusSpec>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]\n\nReserved"]
pub type R0R = crate::FieldReader;
#[doc = "Field `DLPES` reader - Data Link Protocol Error Status \\[DLPES\\]\n\nThis bit is set when the core\n\nreceives an Ack or Nak DLLP whose\n\nsequence number does not\n\ncorrespond to that of an\n\nunacknowledged TLP or that of the\n\nlast acknowledged TLP (for details,\n\nrefer to PCI Express Base\n\nSpecification 1.1, Section 3.5.2).\n\nThis error is not Function-specific,\n\nand is reported by Function 0.\n\nSTICKY."]
pub type DlpesR = crate::BitReader;
#[doc = "Field `DLPES` writer - Data Link Protocol Error Status \\[DLPES\\]\n\nThis bit is set when the core\n\nreceives an Ack or Nak DLLP whose\n\nsequence number does not\n\ncorrespond to that of an\n\nunacknowledged TLP or that of the\n\nlast acknowledged TLP (for details,\n\nrefer to PCI Express Base\n\nSpecification 1.1, Section 3.5.2).\n\nThis error is not Function-specific,\n\nand is reported by Function 0.\n\nSTICKY."]
pub type DlpesW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R1` reader - Reserved \\[R1\\]\n\nReserved"]
pub type R1R = crate::FieldReader;
#[doc = "Field `PTS` reader - Poisoned TLP Status \\[PTS\\]\n\nThis bit is set when the core\n\nreceives a poisoned TLP from the\n\nlink. This error is Function-specific.\n\nThis error is considered non-fatal by\n\ndefault. The error is reported by\n\nsending an ERR_NONFATAL\n\nmessage. The header of the\n\nreceived TLP with error is logged in\n\nthe Header Log Registers.\n\nSTICKY."]
pub type PtsR = crate::BitReader;
#[doc = "Field `PTS` writer - Poisoned TLP Status \\[PTS\\]\n\nThis bit is set when the core\n\nreceives a poisoned TLP from the\n\nlink. This error is Function-specific.\n\nThis error is considered non-fatal by\n\ndefault. The error is reported by\n\nsending an ERR_NONFATAL\n\nmessage. The header of the\n\nreceived TLP with error is logged in\n\nthe Header Log Registers.\n\nSTICKY."]
pub type PtsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FCPES` reader - Flow Control Protocol Error Status \\[FCPES\\]\n\nThis bit is set when certain\n\nviolations of the flow control\n\nprotocol are detected by the core.\n\nSee Section 2.6.1 for details. This\n\nerror is not Function- specific\n\nSTICKY."]
pub type FcpesR = crate::BitReader;
#[doc = "Field `FCPES` writer - Flow Control Protocol Error Status \\[FCPES\\]\n\nThis bit is set when certain\n\nviolations of the flow control\n\nprotocol are detected by the core.\n\nSee Section 2.6.1 for details. This\n\nerror is not Function- specific\n\nSTICKY."]
pub type FcpesW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CTS` reader - Completion Timeout Status \\[CTS\\]\n\nThis bit is set when the completion\n\ntimer associated with an\n\noutstanding request times out. This\n\nerror is Function-specific. This error\n\nis considered non-fatal by default.\n\nSTICKY."]
pub type CtsR = crate::BitReader;
#[doc = "Field `CTS` writer - Completion Timeout Status \\[CTS\\]\n\nThis bit is set when the completion\n\ntimer associated with an\n\noutstanding request times out. This\n\nerror is Function-specific. This error\n\nis considered non-fatal by default.\n\nSTICKY."]
pub type CtsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CAS` reader - Completer Abort Status \\[CAS\\]\n\nThis bit is set when the core has\n\nreturned the Completer Abort (CA)\n\nstatus to a request received from\n\nthe link. This error is Function-\n\nspecific. The header of the received\n\nrequest that caused the error is\n\nlogged in the Header Log Registers.\n\nSTICKY."]
pub type CasR = crate::BitReader;
#[doc = "Field `CAS` writer - Completer Abort Status \\[CAS\\]\n\nThis bit is set when the core has\n\nreturned the Completer Abort (CA)\n\nstatus to a request received from\n\nthe link. This error is Function-\n\nspecific. The header of the received\n\nrequest that caused the error is\n\nlogged in the Header Log Registers.\n\nSTICKY."]
pub type CasW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `UCS` reader - Unexpected Completion Status \\[UCS\\]\n\nThis bit is set when the core has\n\nreceived an unexpected Completion\n\npacket from the link. This error is\n\nnot Function-specific. STICKY."]
pub type UcsR = crate::BitReader;
#[doc = "Field `UCS` writer - Unexpected Completion Status \\[UCS\\]\n\nThis bit is set when the core has\n\nreceived an unexpected Completion\n\npacket from the link. This error is\n\nnot Function-specific. STICKY."]
pub type UcsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `ROS` reader - Receiver Overflow Status \\[ROS\\]\n\nThis bit is set when the core\n\nreceives a TLP in violation of the\n\nreceive credit currently available.\n\nThis error is not Function-specific.\n\nSTICKY."]
pub type RosR = crate::BitReader;
#[doc = "Field `ROS` writer - Receiver Overflow Status \\[ROS\\]\n\nThis bit is set when the core\n\nreceives a TLP in violation of the\n\nreceive credit currently available.\n\nThis error is not Function-specific.\n\nSTICKY."]
pub type RosW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MTS` reader - Malformed TLP Status \\[MTS\\]\n\nThis bit is set when the core\n\nreceives a malformed TLP from the\n\nlink. This error is not Function-\n\nspecific. This error is considered\n\nfatal by default, and is reported by\n\nsending an ERR_FATAL message.\n\nThe header of the received TLP with\n\nerror is logged in the Header Log\n\nRegisters.\n\nSTICKY."]
pub type MtsR = crate::BitReader;
#[doc = "Field `MTS` writer - Malformed TLP Status \\[MTS\\]\n\nThis bit is set when the core\n\nreceives a malformed TLP from the\n\nlink. This error is not Function-\n\nspecific. This error is considered\n\nfatal by default, and is reported by\n\nsending an ERR_FATAL message.\n\nThe header of the received TLP with\n\nerror is logged in the Header Log\n\nRegisters.\n\nSTICKY."]
pub type MtsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EES` reader - ECRC Error Status \\[EES\\]\n\nThis bit is set when the core has\n\ndetected an ECRC error in a\n\nreceived\n\nTLP. This error is not Function-\n\nspecific. The header of the received\n\nTLP with error is logged in the\n\nHeader Log Registers. STICKY."]
pub type EesR = crate::BitReader;
#[doc = "Field `EES` writer - ECRC Error Status \\[EES\\]\n\nThis bit is set when the core has\n\ndetected an ECRC error in a\n\nreceived\n\nTLP. This error is not Function-\n\nspecific. The header of the received\n\nTLP with error is logged in the\n\nHeader Log Registers. STICKY."]
pub type EesW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `URES` reader - Unsupported Request Error Status \\[URES\\]\n\nThis bit is set when the core has\n\nreceived a request from the link that\n\nit does not support. This error is not\n\nFunction-specific. This error is\n\nconsidered non-fatal by default. In\n\nthe special case described in\n\nSections 6.2.3.2.4.1 of the PCI\n\nExpress Specifications, the error is\n\nreported by sending an ERR_COR\n\nmessage. In all other cases, the\n\nerror is reported by sending an\n\nERR_NONFATAL message. The\n\nheader of the received request that\n\ncaused the error is logged in the\n\nHeader Log Registers. STICKY."]
pub type UresR = crate::BitReader;
#[doc = "Field `URES` writer - Unsupported Request Error Status \\[URES\\]\n\nThis bit is set when the core has\n\nreceived a request from the link that\n\nit does not support. This error is not\n\nFunction-specific. This error is\n\nconsidered non-fatal by default. In\n\nthe special case described in\n\nSections 6.2.3.2.4.1 of the PCI\n\nExpress Specifications, the error is\n\nreported by sending an ERR_COR\n\nmessage. In all other cases, the\n\nerror is reported by sending an\n\nERR_NONFATAL message. The\n\nheader of the received request that\n\ncaused the error is logged in the\n\nHeader Log Registers. STICKY."]
pub type UresW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R2` reader - Reserved \\[R2\\]\n\nReserved"]
pub type R2R = crate::BitReader;
#[doc = "Field `UIES` reader - Uncorrectable Internal Error Status \\[UIES\\]\n\nThis bit is set when the core has\n\ndetected an internal uncorrectable\n\nerror (HAL parity error or an\n\nuncorrectable ECC error while\n\nreading from any of the RAMs). This\n\nbit is also set in response to the\n\nclient signaling an internal error\n\nthrough the input\n\nUNCORRECTABLE_ERROR_IN. This\n\nerror is not Function-specific. This\n\nerror is considered fatal by default,\n\nand is reported by sending an\n\nERR_FATAL message. STICKY."]
pub type UiesR = crate::BitReader;
#[doc = "Field `UIES` writer - Uncorrectable Internal Error Status \\[UIES\\]\n\nThis bit is set when the core has\n\ndetected an internal uncorrectable\n\nerror (HAL parity error or an\n\nuncorrectable ECC error while\n\nreading from any of the RAMs). This\n\nbit is also set in response to the\n\nclient signaling an internal error\n\nthrough the input\n\nUNCORRECTABLE_ERROR_IN. This\n\nerror is not Function-specific. This\n\nerror is considered fatal by default,\n\nand is reported by sending an\n\nERR_FATAL message. STICKY."]
pub type UiesW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R3` reader - Reserved \\[R3\\]\n\n(no description)"]
pub type R3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Reserved \\[R0\\]\n\nReserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Data Link Protocol Error Status \\[DLPES\\]\n\nThis bit is set when the core\n\nreceives an Ack or Nak DLLP whose\n\nsequence number does not\n\ncorrespond to that of an\n\nunacknowledged TLP or that of the\n\nlast acknowledged TLP (for details,\n\nrefer to PCI Express Base\n\nSpecification 1.1, Section 3.5.2).\n\nThis error is not Function-specific,\n\nand is reported by Function 0.\n\nSTICKY."]
    #[inline(always)]
    pub fn dlpes(&self) -> DlpesR {
        DlpesR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:11 - Reserved \\[R1\\]\n\nReserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 5) & 0x7f) as u8)
    }
    #[doc = "Bit 12 - Poisoned TLP Status \\[PTS\\]\n\nThis bit is set when the core\n\nreceives a poisoned TLP from the\n\nlink. This error is Function-specific.\n\nThis error is considered non-fatal by\n\ndefault. The error is reported by\n\nsending an ERR_NONFATAL\n\nmessage. The header of the\n\nreceived TLP with error is logged in\n\nthe Header Log Registers.\n\nSTICKY."]
    #[inline(always)]
    pub fn pts(&self) -> PtsR {
        PtsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flow Control Protocol Error Status \\[FCPES\\]\n\nThis bit is set when certain\n\nviolations of the flow control\n\nprotocol are detected by the core.\n\nSee Section 2.6.1 for details. This\n\nerror is not Function- specific\n\nSTICKY."]
    #[inline(always)]
    pub fn fcpes(&self) -> FcpesR {
        FcpesR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Completion Timeout Status \\[CTS\\]\n\nThis bit is set when the completion\n\ntimer associated with an\n\noutstanding request times out. This\n\nerror is Function-specific. This error\n\nis considered non-fatal by default.\n\nSTICKY."]
    #[inline(always)]
    pub fn cts(&self) -> CtsR {
        CtsR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Completer Abort Status \\[CAS\\]\n\nThis bit is set when the core has\n\nreturned the Completer Abort (CA)\n\nstatus to a request received from\n\nthe link. This error is Function-\n\nspecific. The header of the received\n\nrequest that caused the error is\n\nlogged in the Header Log Registers.\n\nSTICKY."]
    #[inline(always)]
    pub fn cas(&self) -> CasR {
        CasR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Unexpected Completion Status \\[UCS\\]\n\nThis bit is set when the core has\n\nreceived an unexpected Completion\n\npacket from the link. This error is\n\nnot Function-specific. STICKY."]
    #[inline(always)]
    pub fn ucs(&self) -> UcsR {
        UcsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receiver Overflow Status \\[ROS\\]\n\nThis bit is set when the core\n\nreceives a TLP in violation of the\n\nreceive credit currently available.\n\nThis error is not Function-specific.\n\nSTICKY."]
    #[inline(always)]
    pub fn ros(&self) -> RosR {
        RosR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Malformed TLP Status \\[MTS\\]\n\nThis bit is set when the core\n\nreceives a malformed TLP from the\n\nlink. This error is not Function-\n\nspecific. This error is considered\n\nfatal by default, and is reported by\n\nsending an ERR_FATAL message.\n\nThe header of the received TLP with\n\nerror is logged in the Header Log\n\nRegisters.\n\nSTICKY."]
    #[inline(always)]
    pub fn mts(&self) -> MtsR {
        MtsR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ECRC Error Status \\[EES\\]\n\nThis bit is set when the core has\n\ndetected an ECRC error in a\n\nreceived\n\nTLP. This error is not Function-\n\nspecific. The header of the received\n\nTLP with error is logged in the\n\nHeader Log Registers. STICKY."]
    #[inline(always)]
    pub fn ees(&self) -> EesR {
        EesR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Unsupported Request Error Status \\[URES\\]\n\nThis bit is set when the core has\n\nreceived a request from the link that\n\nit does not support. This error is not\n\nFunction-specific. This error is\n\nconsidered non-fatal by default. In\n\nthe special case described in\n\nSections 6.2.3.2.4.1 of the PCI\n\nExpress Specifications, the error is\n\nreported by sending an ERR_COR\n\nmessage. In all other cases, the\n\nerror is reported by sending an\n\nERR_NONFATAL message. The\n\nheader of the received request that\n\ncaused the error is logged in the\n\nHeader Log Registers. STICKY."]
    #[inline(always)]
    pub fn ures(&self) -> UresR {
        UresR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved \\[R2\\]\n\nReserved"]
    #[inline(always)]
    pub fn r2(&self) -> R2R {
        R2R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Uncorrectable Internal Error Status \\[UIES\\]\n\nThis bit is set when the core has\n\ndetected an internal uncorrectable\n\nerror (HAL parity error or an\n\nuncorrectable ECC error while\n\nreading from any of the RAMs). This\n\nbit is also set in response to the\n\nclient signaling an internal error\n\nthrough the input\n\nUNCORRECTABLE_ERROR_IN. This\n\nerror is not Function-specific. This\n\nerror is considered fatal by default,\n\nand is reported by sending an\n\nERR_FATAL message. STICKY."]
    #[inline(always)]
    pub fn uies(&self) -> UiesR {
        UiesR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - Reserved \\[R3\\]\n\n(no description)"]
    #[inline(always)]
    pub fn r3(&self) -> R3R {
        R3R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 4 - Data Link Protocol Error Status \\[DLPES\\]\n\nThis bit is set when the core\n\nreceives an Ack or Nak DLLP whose\n\nsequence number does not\n\ncorrespond to that of an\n\nunacknowledged TLP or that of the\n\nlast acknowledged TLP (for details,\n\nrefer to PCI Express Base\n\nSpecification 1.1, Section 3.5.2).\n\nThis error is not Function-specific,\n\nand is reported by Function 0.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn dlpes(&mut self) -> DlpesW<PciePfUncorrectableErrorStatusSpec> {
        DlpesW::new(self, 4)
    }
    #[doc = "Bit 12 - Poisoned TLP Status \\[PTS\\]\n\nThis bit is set when the core\n\nreceives a poisoned TLP from the\n\nlink. This error is Function-specific.\n\nThis error is considered non-fatal by\n\ndefault. The error is reported by\n\nsending an ERR_NONFATAL\n\nmessage. The header of the\n\nreceived TLP with error is logged in\n\nthe Header Log Registers.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn pts(&mut self) -> PtsW<PciePfUncorrectableErrorStatusSpec> {
        PtsW::new(self, 12)
    }
    #[doc = "Bit 13 - Flow Control Protocol Error Status \\[FCPES\\]\n\nThis bit is set when certain\n\nviolations of the flow control\n\nprotocol are detected by the core.\n\nSee Section 2.6.1 for details. This\n\nerror is not Function- specific\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn fcpes(&mut self) -> FcpesW<PciePfUncorrectableErrorStatusSpec> {
        FcpesW::new(self, 13)
    }
    #[doc = "Bit 14 - Completion Timeout Status \\[CTS\\]\n\nThis bit is set when the completion\n\ntimer associated with an\n\noutstanding request times out. This\n\nerror is Function-specific. This error\n\nis considered non-fatal by default.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn cts(&mut self) -> CtsW<PciePfUncorrectableErrorStatusSpec> {
        CtsW::new(self, 14)
    }
    #[doc = "Bit 15 - Completer Abort Status \\[CAS\\]\n\nThis bit is set when the core has\n\nreturned the Completer Abort (CA)\n\nstatus to a request received from\n\nthe link. This error is Function-\n\nspecific. The header of the received\n\nrequest that caused the error is\n\nlogged in the Header Log Registers.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn cas(&mut self) -> CasW<PciePfUncorrectableErrorStatusSpec> {
        CasW::new(self, 15)
    }
    #[doc = "Bit 16 - Unexpected Completion Status \\[UCS\\]\n\nThis bit is set when the core has\n\nreceived an unexpected Completion\n\npacket from the link. This error is\n\nnot Function-specific. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ucs(&mut self) -> UcsW<PciePfUncorrectableErrorStatusSpec> {
        UcsW::new(self, 16)
    }
    #[doc = "Bit 17 - Receiver Overflow Status \\[ROS\\]\n\nThis bit is set when the core\n\nreceives a TLP in violation of the\n\nreceive credit currently available.\n\nThis error is not Function-specific.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ros(&mut self) -> RosW<PciePfUncorrectableErrorStatusSpec> {
        RosW::new(self, 17)
    }
    #[doc = "Bit 18 - Malformed TLP Status \\[MTS\\]\n\nThis bit is set when the core\n\nreceives a malformed TLP from the\n\nlink. This error is not Function-\n\nspecific. This error is considered\n\nfatal by default, and is reported by\n\nsending an ERR_FATAL message.\n\nThe header of the received TLP with\n\nerror is logged in the Header Log\n\nRegisters.\n\nSTICKY."]
    #[inline(always)]
    #[must_use]
    pub fn mts(&mut self) -> MtsW<PciePfUncorrectableErrorStatusSpec> {
        MtsW::new(self, 18)
    }
    #[doc = "Bit 19 - ECRC Error Status \\[EES\\]\n\nThis bit is set when the core has\n\ndetected an ECRC error in a\n\nreceived\n\nTLP. This error is not Function-\n\nspecific. The header of the received\n\nTLP with error is logged in the\n\nHeader Log Registers. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ees(&mut self) -> EesW<PciePfUncorrectableErrorStatusSpec> {
        EesW::new(self, 19)
    }
    #[doc = "Bit 20 - Unsupported Request Error Status \\[URES\\]\n\nThis bit is set when the core has\n\nreceived a request from the link that\n\nit does not support. This error is not\n\nFunction-specific. This error is\n\nconsidered non-fatal by default. In\n\nthe special case described in\n\nSections 6.2.3.2.4.1 of the PCI\n\nExpress Specifications, the error is\n\nreported by sending an ERR_COR\n\nmessage. In all other cases, the\n\nerror is reported by sending an\n\nERR_NONFATAL message. The\n\nheader of the received request that\n\ncaused the error is logged in the\n\nHeader Log Registers. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn ures(&mut self) -> UresW<PciePfUncorrectableErrorStatusSpec> {
        UresW::new(self, 20)
    }
    #[doc = "Bit 22 - Uncorrectable Internal Error Status \\[UIES\\]\n\nThis bit is set when the core has\n\ndetected an internal uncorrectable\n\nerror (HAL parity error or an\n\nuncorrectable ECC error while\n\nreading from any of the RAMs). This\n\nbit is also set in response to the\n\nclient signaling an internal error\n\nthrough the input\n\nUNCORRECTABLE_ERROR_IN. This\n\nerror is not Function-specific. This\n\nerror is considered fatal by default,\n\nand is reported by sending an\n\nERR_FATAL message. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn uies(&mut self) -> UiesW<PciePfUncorrectableErrorStatusSpec> {
        UiesW::new(self, 22)
    }
}
#[doc = "Uncorrectable Error Status Register\n\n(no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_uncorrectable_error_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_uncorrectable_error_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfUncorrectableErrorStatusSpec;
impl crate::RegisterSpec for PciePfUncorrectableErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_uncorrectable_error_status::R`](R) reader structure"]
impl crate::Readable for PciePfUncorrectableErrorStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_uncorrectable_error_status::W`](W) writer structure"]
impl crate::Writable for PciePfUncorrectableErrorStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x005f_f010;
}
#[doc = "`reset()` method sets PCIE_PF_UNCORRECTABLE_ERROR_STATUS to value 0"]
impl crate::Resettable for PciePfUncorrectableErrorStatusSpec {
    const RESET_VALUE: u32 = 0;
}
