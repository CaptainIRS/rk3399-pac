#[doc = "Register `PCIE_RC_UNCORRECTABLE_ERROR_STATUS` reader"]
pub type R = crate::R<PcieRcUncorrectableErrorStatusSpec>;
#[doc = "Register `PCIE_RC_UNCORRECTABLE_ERROR_STATUS` writer"]
pub type W = crate::W<PcieRcUncorrectableErrorStatusSpec>;
#[doc = "Field `R25` reader - Reserved \\[R25\\]\n\n(no description)"]
pub type R25R = crate::FieldReader;
#[doc = "Field `DLPE` reader - Data Link Protocol Error Status \\[DLPE\\]\n\nThis bit is set when the core\n\nreceives an Ack or Nak DLLP whose\n\nsequence does not correspond to\n\nthat of an unacknowledged TLP or\n\nthat of the last acknowledged TLP\n\n(for details, refer to the PCI Express\n\nBase Specifications)."]
pub type DlpeR = crate::BitReader;
#[doc = "Field `DLPE` writer - Data Link Protocol Error Status \\[DLPE\\]\n\nThis bit is set when the core\n\nreceives an Ack or Nak DLLP whose\n\nsequence does not correspond to\n\nthat of an unacknowledged TLP or\n\nthat of the last acknowledged TLP\n\n(for details, refer to the PCI Express\n\nBase Specifications)."]
pub type DlpeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R26` reader - Reserved \\[R26\\]\n\nReserved"]
pub type R26R = crate::FieldReader;
#[doc = "Field `PT` reader - Poisoned TLP Status \\[PT\\]\n\nThis bit is set when the core\n\nreceives a poisoned TLP from the\n\nlink. This error is considered non-\n\nfatal by default. The header of the\n\nreceived TLP with error is logged in\n\nthe Header Log Registers."]
pub type PtR = crate::BitReader;
#[doc = "Field `PT` writer - Poisoned TLP Status \\[PT\\]\n\nThis bit is set when the core\n\nreceives a poisoned TLP from the\n\nlink. This error is considered non-\n\nfatal by default. The header of the\n\nreceived TLP with error is logged in\n\nthe Header Log Registers."]
pub type PtW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FCPE` reader - Flow Control Protocol Error Status \\[FCPE\\]\n\nThis bit is set when certain\n\nviolations of the flow control\n\nprotocol are detected by the core."]
pub type FcpeR = crate::BitReader;
#[doc = "Field `FCPE` writer - Flow Control Protocol Error Status \\[FCPE\\]\n\nThis bit is set when certain\n\nviolations of the flow control\n\nprotocol are detected by the core."]
pub type FcpeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CT` reader - Completion Timeout Status \\[CT\\]\n\nThis bit is set when the completion\n\ntimer associated with an\n\noutstanding request times out. This\n\nerror is considered non-fatal by\n\ndefault."]
pub type CtR = crate::BitReader;
#[doc = "Field `CT` writer - Completion Timeout Status \\[CT\\]\n\nThis bit is set when the completion\n\ntimer associated with an\n\noutstanding request times out. This\n\nerror is considered non-fatal by\n\ndefault."]
pub type CtW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CA` reader - Completer Abort Status \\[CA\\]\n\nThis bit is set when the core has\n\nreturned the Completer Abort (CA)\n\nstatus to a request received from\n\nthe link. This error is considered\n\nnon-fatal by default, except for the\n\nspecial cases outlined in PCI Express\n\nBase Specification 2.0. The header\n\nof the received request that caused\n\nthe error is logged in the Header\n\nLog Registers."]
pub type CaR = crate::BitReader;
#[doc = "Field `CA` writer - Completer Abort Status \\[CA\\]\n\nThis bit is set when the core has\n\nreturned the Completer Abort (CA)\n\nstatus to a request received from\n\nthe link. This error is considered\n\nnon-fatal by default, except for the\n\nspecial cases outlined in PCI Express\n\nBase Specification 2.0. The header\n\nof the received request that caused\n\nthe error is logged in the Header\n\nLog Registers."]
pub type CaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `UC` reader - Unexpected Completion Status \\[UC\\]\n\nThis bit is set when the core has\n\nreceived an unexpected Completion\n\npacket from the link."]
pub type UcR = crate::BitReader;
#[doc = "Field `UC` writer - Unexpected Completion Status \\[UC\\]\n\nThis bit is set when the core has\n\nreceived an unexpected Completion\n\npacket from the link."]
pub type UcW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RO` reader - Receiver Overflow Status \\[RO\\]\n\nThis bit is set when the core\n\nreceives a TLP in violation of the\n\nreceive credit currently available."]
pub type RoR = crate::BitReader;
#[doc = "Field `RO` writer - Receiver Overflow Status \\[RO\\]\n\nThis bit is set when the core\n\nreceives a TLP in violation of the\n\nreceive credit currently available."]
pub type RoW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MT` reader - Malformed TLP Status \\[MT\\]\n\nThis bit is set when the core\n\nreceives a malformed TLP from the\n\nlink. This error is considered fatal by\n\ndefault. The header of the received\n\nTLP with error is logged in the\n\nHeader Log Registers."]
pub type MtR = crate::BitReader;
#[doc = "Field `MT` writer - Malformed TLP Status \\[MT\\]\n\nThis bit is set when the core\n\nreceives a malformed TLP from the\n\nlink. This error is considered fatal by\n\ndefault. The header of the received\n\nTLP with error is logged in the\n\nHeader Log Registers."]
pub type MtW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EE` reader - ECRC Error Status \\[EE\\]\n\nThis bit is set when the core has\n\ndetected an ECRC error in a\n\nreceived TLP."]
pub type EeR = crate::BitReader;
#[doc = "Field `EE` writer - ECRC Error Status \\[EE\\]\n\nThis bit is set when the core has\n\ndetected an ECRC error in a\n\nreceived TLP."]
pub type EeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `URE` reader - Unsupported Request Error Status \\[URE\\]\n\nThis bit is set when the core has\n\nreceived a request from the link that\n\nit does not support. This error is not\n\nFunction-specific. This error is\n\nconsidered non-fatal by default,\n\nexcept for the special case outlined\n\nin PCI Express Base Specification\n\n2.0. The header of the received\n\nrequest that caused the error is\n\nlogged in the Header Log\n\nRegisters."]
pub type UreR = crate::BitReader;
#[doc = "Field `URE` writer - Unsupported Request Error Status \\[URE\\]\n\nThis bit is set when the core has\n\nreceived a request from the link that\n\nit does not support. This error is not\n\nFunction-specific. This error is\n\nconsidered non-fatal by default,\n\nexcept for the special case outlined\n\nin PCI Express Base Specification\n\n2.0. The header of the received\n\nrequest that caused the error is\n\nlogged in the Header Log\n\nRegisters."]
pub type UreW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R27` reader - Reserved \\[R27\\]\n\nReserved"]
pub type R27R = crate::BitReader;
#[doc = "Field `UIE` reader - Uncorrectable Internal Error Status \\[UIE\\]\n\nThis bit is set when the core has\n\ndetected an internal uncorrectable\n\nerror (HAL parity error or an\n\nuncorrectable ECC error while\n\nreading from any of the RAMs). This\n\nbit is also set in response to the\n\nclient signaling an internal error\n\nthrough the input\n\nUNCORRECTABLE_ERROR_IN. This\n\nerror is considered fatal by default."]
pub type UieR = crate::BitReader;
#[doc = "Field `UIE` writer - Uncorrectable Internal Error Status \\[UIE\\]\n\nThis bit is set when the core has\n\ndetected an internal uncorrectable\n\nerror (HAL parity error or an\n\nuncorrectable ECC error while\n\nreading from any of the RAMs). This\n\nbit is also set in response to the\n\nclient signaling an internal error\n\nthrough the input\n\nUNCORRECTABLE_ERROR_IN. This\n\nerror is considered fatal by default."]
pub type UieW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R28` reader - Reserved \\[R28\\]\n\nReserved"]
pub type R28R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Reserved \\[R25\\]\n\n(no description)"]
    #[inline(always)]
    pub fn r25(&self) -> R25R {
        R25R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Data Link Protocol Error Status \\[DLPE\\]\n\nThis bit is set when the core\n\nreceives an Ack or Nak DLLP whose\n\nsequence does not correspond to\n\nthat of an unacknowledged TLP or\n\nthat of the last acknowledged TLP\n\n(for details, refer to the PCI Express\n\nBase Specifications)."]
    #[inline(always)]
    pub fn dlpe(&self) -> DlpeR {
        DlpeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:11 - Reserved \\[R26\\]\n\nReserved"]
    #[inline(always)]
    pub fn r26(&self) -> R26R {
        R26R::new(((self.bits >> 5) & 0x7f) as u8)
    }
    #[doc = "Bit 12 - Poisoned TLP Status \\[PT\\]\n\nThis bit is set when the core\n\nreceives a poisoned TLP from the\n\nlink. This error is considered non-\n\nfatal by default. The header of the\n\nreceived TLP with error is logged in\n\nthe Header Log Registers."]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flow Control Protocol Error Status \\[FCPE\\]\n\nThis bit is set when certain\n\nviolations of the flow control\n\nprotocol are detected by the core."]
    #[inline(always)]
    pub fn fcpe(&self) -> FcpeR {
        FcpeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Completion Timeout Status \\[CT\\]\n\nThis bit is set when the completion\n\ntimer associated with an\n\noutstanding request times out. This\n\nerror is considered non-fatal by\n\ndefault."]
    #[inline(always)]
    pub fn ct(&self) -> CtR {
        CtR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Completer Abort Status \\[CA\\]\n\nThis bit is set when the core has\n\nreturned the Completer Abort (CA)\n\nstatus to a request received from\n\nthe link. This error is considered\n\nnon-fatal by default, except for the\n\nspecial cases outlined in PCI Express\n\nBase Specification 2.0. The header\n\nof the received request that caused\n\nthe error is logged in the Header\n\nLog Registers."]
    #[inline(always)]
    pub fn ca(&self) -> CaR {
        CaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Unexpected Completion Status \\[UC\\]\n\nThis bit is set when the core has\n\nreceived an unexpected Completion\n\npacket from the link."]
    #[inline(always)]
    pub fn uc(&self) -> UcR {
        UcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receiver Overflow Status \\[RO\\]\n\nThis bit is set when the core\n\nreceives a TLP in violation of the\n\nreceive credit currently available."]
    #[inline(always)]
    pub fn ro(&self) -> RoR {
        RoR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Malformed TLP Status \\[MT\\]\n\nThis bit is set when the core\n\nreceives a malformed TLP from the\n\nlink. This error is considered fatal by\n\ndefault. The header of the received\n\nTLP with error is logged in the\n\nHeader Log Registers."]
    #[inline(always)]
    pub fn mt(&self) -> MtR {
        MtR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ECRC Error Status \\[EE\\]\n\nThis bit is set when the core has\n\ndetected an ECRC error in a\n\nreceived TLP."]
    #[inline(always)]
    pub fn ee(&self) -> EeR {
        EeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Unsupported Request Error Status \\[URE\\]\n\nThis bit is set when the core has\n\nreceived a request from the link that\n\nit does not support. This error is not\n\nFunction-specific. This error is\n\nconsidered non-fatal by default,\n\nexcept for the special case outlined\n\nin PCI Express Base Specification\n\n2.0. The header of the received\n\nrequest that caused the error is\n\nlogged in the Header Log\n\nRegisters."]
    #[inline(always)]
    pub fn ure(&self) -> UreR {
        UreR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved \\[R27\\]\n\nReserved"]
    #[inline(always)]
    pub fn r27(&self) -> R27R {
        R27R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Uncorrectable Internal Error Status \\[UIE\\]\n\nThis bit is set when the core has\n\ndetected an internal uncorrectable\n\nerror (HAL parity error or an\n\nuncorrectable ECC error while\n\nreading from any of the RAMs). This\n\nbit is also set in response to the\n\nclient signaling an internal error\n\nthrough the input\n\nUNCORRECTABLE_ERROR_IN. This\n\nerror is considered fatal by default."]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - Reserved \\[R28\\]\n\nReserved"]
    #[inline(always)]
    pub fn r28(&self) -> R28R {
        R28R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 4 - Data Link Protocol Error Status \\[DLPE\\]\n\nThis bit is set when the core\n\nreceives an Ack or Nak DLLP whose\n\nsequence does not correspond to\n\nthat of an unacknowledged TLP or\n\nthat of the last acknowledged TLP\n\n(for details, refer to the PCI Express\n\nBase Specifications)."]
    #[inline(always)]
    #[must_use]
    pub fn dlpe(&mut self) -> DlpeW<PcieRcUncorrectableErrorStatusSpec> {
        DlpeW::new(self, 4)
    }
    #[doc = "Bit 12 - Poisoned TLP Status \\[PT\\]\n\nThis bit is set when the core\n\nreceives a poisoned TLP from the\n\nlink. This error is considered non-\n\nfatal by default. The header of the\n\nreceived TLP with error is logged in\n\nthe Header Log Registers."]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PtW<PcieRcUncorrectableErrorStatusSpec> {
        PtW::new(self, 12)
    }
    #[doc = "Bit 13 - Flow Control Protocol Error Status \\[FCPE\\]\n\nThis bit is set when certain\n\nviolations of the flow control\n\nprotocol are detected by the core."]
    #[inline(always)]
    #[must_use]
    pub fn fcpe(&mut self) -> FcpeW<PcieRcUncorrectableErrorStatusSpec> {
        FcpeW::new(self, 13)
    }
    #[doc = "Bit 14 - Completion Timeout Status \\[CT\\]\n\nThis bit is set when the completion\n\ntimer associated with an\n\noutstanding request times out. This\n\nerror is considered non-fatal by\n\ndefault."]
    #[inline(always)]
    #[must_use]
    pub fn ct(&mut self) -> CtW<PcieRcUncorrectableErrorStatusSpec> {
        CtW::new(self, 14)
    }
    #[doc = "Bit 15 - Completer Abort Status \\[CA\\]\n\nThis bit is set when the core has\n\nreturned the Completer Abort (CA)\n\nstatus to a request received from\n\nthe link. This error is considered\n\nnon-fatal by default, except for the\n\nspecial cases outlined in PCI Express\n\nBase Specification 2.0. The header\n\nof the received request that caused\n\nthe error is logged in the Header\n\nLog Registers."]
    #[inline(always)]
    #[must_use]
    pub fn ca(&mut self) -> CaW<PcieRcUncorrectableErrorStatusSpec> {
        CaW::new(self, 15)
    }
    #[doc = "Bit 16 - Unexpected Completion Status \\[UC\\]\n\nThis bit is set when the core has\n\nreceived an unexpected Completion\n\npacket from the link."]
    #[inline(always)]
    #[must_use]
    pub fn uc(&mut self) -> UcW<PcieRcUncorrectableErrorStatusSpec> {
        UcW::new(self, 16)
    }
    #[doc = "Bit 17 - Receiver Overflow Status \\[RO\\]\n\nThis bit is set when the core\n\nreceives a TLP in violation of the\n\nreceive credit currently available."]
    #[inline(always)]
    #[must_use]
    pub fn ro(&mut self) -> RoW<PcieRcUncorrectableErrorStatusSpec> {
        RoW::new(self, 17)
    }
    #[doc = "Bit 18 - Malformed TLP Status \\[MT\\]\n\nThis bit is set when the core\n\nreceives a malformed TLP from the\n\nlink. This error is considered fatal by\n\ndefault. The header of the received\n\nTLP with error is logged in the\n\nHeader Log Registers."]
    #[inline(always)]
    #[must_use]
    pub fn mt(&mut self) -> MtW<PcieRcUncorrectableErrorStatusSpec> {
        MtW::new(self, 18)
    }
    #[doc = "Bit 19 - ECRC Error Status \\[EE\\]\n\nThis bit is set when the core has\n\ndetected an ECRC error in a\n\nreceived TLP."]
    #[inline(always)]
    #[must_use]
    pub fn ee(&mut self) -> EeW<PcieRcUncorrectableErrorStatusSpec> {
        EeW::new(self, 19)
    }
    #[doc = "Bit 20 - Unsupported Request Error Status \\[URE\\]\n\nThis bit is set when the core has\n\nreceived a request from the link that\n\nit does not support. This error is not\n\nFunction-specific. This error is\n\nconsidered non-fatal by default,\n\nexcept for the special case outlined\n\nin PCI Express Base Specification\n\n2.0. The header of the received\n\nrequest that caused the error is\n\nlogged in the Header Log\n\nRegisters."]
    #[inline(always)]
    #[must_use]
    pub fn ure(&mut self) -> UreW<PcieRcUncorrectableErrorStatusSpec> {
        UreW::new(self, 20)
    }
    #[doc = "Bit 22 - Uncorrectable Internal Error Status \\[UIE\\]\n\nThis bit is set when the core has\n\ndetected an internal uncorrectable\n\nerror (HAL parity error or an\n\nuncorrectable ECC error while\n\nreading from any of the RAMs). This\n\nbit is also set in response to the\n\nclient signaling an internal error\n\nthrough the input\n\nUNCORRECTABLE_ERROR_IN. This\n\nerror is considered fatal by default."]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UieW<PcieRcUncorrectableErrorStatusSpec> {
        UieW::new(self, 22)
    }
}
#[doc = "Uncorrectable Error Status Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_uncorrectable_error_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_uncorrectable_error_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcUncorrectableErrorStatusSpec;
impl crate::RegisterSpec for PcieRcUncorrectableErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_uncorrectable_error_status::R`](R) reader structure"]
impl crate::Readable for PcieRcUncorrectableErrorStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_uncorrectable_error_status::W`](W) writer structure"]
impl crate::Writable for PcieRcUncorrectableErrorStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x005f_f010;
}
#[doc = "`reset()` method sets PCIE_RC_UNCORRECTABLE_ERROR_STATUS to value 0"]
impl crate::Resettable for PcieRcUncorrectableErrorStatusSpec {
    const RESET_VALUE: u32 = 0;
}
