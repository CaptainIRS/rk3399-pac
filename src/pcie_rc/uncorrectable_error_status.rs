#[doc = "Register `UNCORRECTABLE_ERROR_STATUS` reader"]
pub type R = crate::R<UncorrectableErrorStatusSpec>;
#[doc = "Register `UNCORRECTABLE_ERROR_STATUS` writer"]
pub type W = crate::W<UncorrectableErrorStatusSpec>;
#[doc = "Field `R25` reader - Reserved \\[R25\\]
(no description)"]
pub type R25R = crate::FieldReader;
#[doc = "Field `DLPE` reader - Data Link Protocol Error Status \\[DLPE\\]
This bit is set when the core receives an Ack or Nak DLLP whose sequence does not correspond to that of an unacknowledged TLP or that of the last acknowledged TLP (for details, refer to the PCI Express Base Specifications)."]
pub type DlpeR = crate::BitReader;
#[doc = "Field `DLPE` writer - Data Link Protocol Error Status \\[DLPE\\]
This bit is set when the core receives an Ack or Nak DLLP whose sequence does not correspond to that of an unacknowledged TLP or that of the last acknowledged TLP (for details, refer to the PCI Express Base Specifications)."]
pub type DlpeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R26` reader - Reserved \\[R26\\]
Reserved"]
pub type R26R = crate::FieldReader;
#[doc = "Field `PT` reader - Poisoned TLP Status \\[PT\\]
This bit is set when the core receives a poisoned TLP from the link. This error is considered non- fatal by default. The header of the received TLP with error is logged in the Header Log Registers."]
pub type PtR = crate::BitReader;
#[doc = "Field `PT` writer - Poisoned TLP Status \\[PT\\]
This bit is set when the core receives a poisoned TLP from the link. This error is considered non- fatal by default. The header of the received TLP with error is logged in the Header Log Registers."]
pub type PtW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FCPE` reader - Flow Control Protocol Error Status \\[FCPE\\]
This bit is set when certain violations of the flow control protocol are detected by the core."]
pub type FcpeR = crate::BitReader;
#[doc = "Field `FCPE` writer - Flow Control Protocol Error Status \\[FCPE\\]
This bit is set when certain violations of the flow control protocol are detected by the core."]
pub type FcpeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CT` reader - Completion Timeout Status \\[CT\\]
This bit is set when the completion timer associated with an outstanding request times out. This error is considered non-fatal by default."]
pub type CtR = crate::BitReader;
#[doc = "Field `CT` writer - Completion Timeout Status \\[CT\\]
This bit is set when the completion timer associated with an outstanding request times out. This error is considered non-fatal by default."]
pub type CtW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CA` reader - Completer Abort Status \\[CA\\]
This bit is set when the core has returned the Completer Abort (CA) status to a request received from the link. This error is considered non-fatal by default, except for the special cases outlined in PCI Express Base Specification 2.0. The header of the received request that caused the error is logged in the Header Log Registers."]
pub type CaR = crate::BitReader;
#[doc = "Field `CA` writer - Completer Abort Status \\[CA\\]
This bit is set when the core has returned the Completer Abort (CA) status to a request received from the link. This error is considered non-fatal by default, except for the special cases outlined in PCI Express Base Specification 2.0. The header of the received request that caused the error is logged in the Header Log Registers."]
pub type CaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `UC` reader - Unexpected Completion Status \\[UC\\]
This bit is set when the core has received an unexpected Completion packet from the link."]
pub type UcR = crate::BitReader;
#[doc = "Field `UC` writer - Unexpected Completion Status \\[UC\\]
This bit is set when the core has received an unexpected Completion packet from the link."]
pub type UcW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RO` reader - Receiver Overflow Status \\[RO\\]
This bit is set when the core receives a TLP in violation of the receive credit currently available."]
pub type RoR = crate::BitReader;
#[doc = "Field `RO` writer - Receiver Overflow Status \\[RO\\]
This bit is set when the core receives a TLP in violation of the receive credit currently available."]
pub type RoW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MT` reader - Malformed TLP Status \\[MT\\]
This bit is set when the core receives a malformed TLP from the link. This error is considered fatal by default. The header of the received TLP with error is logged in the Header Log Registers."]
pub type MtR = crate::BitReader;
#[doc = "Field `MT` writer - Malformed TLP Status \\[MT\\]
This bit is set when the core receives a malformed TLP from the link. This error is considered fatal by default. The header of the received TLP with error is logged in the Header Log Registers."]
pub type MtW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `EE` reader - ECRC Error Status \\[EE\\]
This bit is set when the core has detected an ECRC error in a received TLP."]
pub type EeR = crate::BitReader;
#[doc = "Field `EE` writer - ECRC Error Status \\[EE\\]
This bit is set when the core has detected an ECRC error in a received TLP."]
pub type EeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `URE` reader - Unsupported Request Error Status \\[URE\\]
This bit is set when the core has received a request from the link that it does not support. This error is not Function-specific. This error is considered non-fatal by default, except for the special case outlined in PCI Express Base Specification 2.0. The header of the received request that caused the error is logged in the Header Log Registers."]
pub type UreR = crate::BitReader;
#[doc = "Field `URE` writer - Unsupported Request Error Status \\[URE\\]
This bit is set when the core has received a request from the link that it does not support. This error is not Function-specific. This error is considered non-fatal by default, except for the special case outlined in PCI Express Base Specification 2.0. The header of the received request that caused the error is logged in the Header Log Registers."]
pub type UreW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R27` reader - Reserved \\[R27\\]
Reserved"]
pub type R27R = crate::BitReader;
#[doc = "Field `UIE` reader - Uncorrectable Internal Error Status \\[UIE\\]
This bit is set when the core has detected an internal uncorrectable error (HAL parity error or an uncorrectable ECC error while reading from any of the RAMs). This bit is also set in response to the client signaling an internal error through the input UNCORRECTABLE_ERROR_IN. This error is considered fatal by default."]
pub type UieR = crate::BitReader;
#[doc = "Field `UIE` writer - Uncorrectable Internal Error Status \\[UIE\\]
This bit is set when the core has detected an internal uncorrectable error (HAL parity error or an uncorrectable ECC error while reading from any of the RAMs). This bit is also set in response to the client signaling an internal error through the input UNCORRECTABLE_ERROR_IN. This error is considered fatal by default."]
pub type UieW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R28` reader - Reserved \\[R28\\]
Reserved"]
pub type R28R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Reserved \\[R25\\]
(no description)"]
    #[inline(always)]
    pub fn r25(&self) -> R25R {
        R25R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Data Link Protocol Error Status \\[DLPE\\]
This bit is set when the core receives an Ack or Nak DLLP whose sequence does not correspond to that of an unacknowledged TLP or that of the last acknowledged TLP (for details, refer to the PCI Express Base Specifications)."]
    #[inline(always)]
    pub fn dlpe(&self) -> DlpeR {
        DlpeR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:11 - Reserved \\[R26\\]
Reserved"]
    #[inline(always)]
    pub fn r26(&self) -> R26R {
        R26R::new(((self.bits >> 5) & 0x7f) as u8)
    }
    #[doc = "Bit 12 - Poisoned TLP Status \\[PT\\]
This bit is set when the core receives a poisoned TLP from the link. This error is considered non- fatal by default. The header of the received TLP with error is logged in the Header Log Registers."]
    #[inline(always)]
    pub fn pt(&self) -> PtR {
        PtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Flow Control Protocol Error Status \\[FCPE\\]
This bit is set when certain violations of the flow control protocol are detected by the core."]
    #[inline(always)]
    pub fn fcpe(&self) -> FcpeR {
        FcpeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Completion Timeout Status \\[CT\\]
This bit is set when the completion timer associated with an outstanding request times out. This error is considered non-fatal by default."]
    #[inline(always)]
    pub fn ct(&self) -> CtR {
        CtR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Completer Abort Status \\[CA\\]
This bit is set when the core has returned the Completer Abort (CA) status to a request received from the link. This error is considered non-fatal by default, except for the special cases outlined in PCI Express Base Specification 2.0. The header of the received request that caused the error is logged in the Header Log Registers."]
    #[inline(always)]
    pub fn ca(&self) -> CaR {
        CaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Unexpected Completion Status \\[UC\\]
This bit is set when the core has received an unexpected Completion packet from the link."]
    #[inline(always)]
    pub fn uc(&self) -> UcR {
        UcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Receiver Overflow Status \\[RO\\]
This bit is set when the core receives a TLP in violation of the receive credit currently available."]
    #[inline(always)]
    pub fn ro(&self) -> RoR {
        RoR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Malformed TLP Status \\[MT\\]
This bit is set when the core receives a malformed TLP from the link. This error is considered fatal by default. The header of the received TLP with error is logged in the Header Log Registers."]
    #[inline(always)]
    pub fn mt(&self) -> MtR {
        MtR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - ECRC Error Status \\[EE\\]
This bit is set when the core has detected an ECRC error in a received TLP."]
    #[inline(always)]
    pub fn ee(&self) -> EeR {
        EeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Unsupported Request Error Status \\[URE\\]
This bit is set when the core has received a request from the link that it does not support. This error is not Function-specific. This error is considered non-fatal by default, except for the special case outlined in PCI Express Base Specification 2.0. The header of the received request that caused the error is logged in the Header Log Registers."]
    #[inline(always)]
    pub fn ure(&self) -> UreR {
        UreR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Reserved \\[R27\\]
Reserved"]
    #[inline(always)]
    pub fn r27(&self) -> R27R {
        R27R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Uncorrectable Internal Error Status \\[UIE\\]
This bit is set when the core has detected an internal uncorrectable error (HAL parity error or an uncorrectable ECC error while reading from any of the RAMs). This bit is also set in response to the client signaling an internal error through the input UNCORRECTABLE_ERROR_IN. This error is considered fatal by default."]
    #[inline(always)]
    pub fn uie(&self) -> UieR {
        UieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:31 - Reserved \\[R28\\]
Reserved"]
    #[inline(always)]
    pub fn r28(&self) -> R28R {
        R28R::new(((self.bits >> 23) & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 4 - Data Link Protocol Error Status \\[DLPE\\]
This bit is set when the core receives an Ack or Nak DLLP whose sequence does not correspond to that of an unacknowledged TLP or that of the last acknowledged TLP (for details, refer to the PCI Express Base Specifications)."]
    #[inline(always)]
    #[must_use]
    pub fn dlpe(&mut self) -> DlpeW<UncorrectableErrorStatusSpec> {
        DlpeW::new(self, 4)
    }
    #[doc = "Bit 12 - Poisoned TLP Status \\[PT\\]
This bit is set when the core receives a poisoned TLP from the link. This error is considered non- fatal by default. The header of the received TLP with error is logged in the Header Log Registers."]
    #[inline(always)]
    #[must_use]
    pub fn pt(&mut self) -> PtW<UncorrectableErrorStatusSpec> {
        PtW::new(self, 12)
    }
    #[doc = "Bit 13 - Flow Control Protocol Error Status \\[FCPE\\]
This bit is set when certain violations of the flow control protocol are detected by the core."]
    #[inline(always)]
    #[must_use]
    pub fn fcpe(&mut self) -> FcpeW<UncorrectableErrorStatusSpec> {
        FcpeW::new(self, 13)
    }
    #[doc = "Bit 14 - Completion Timeout Status \\[CT\\]
This bit is set when the completion timer associated with an outstanding request times out. This error is considered non-fatal by default."]
    #[inline(always)]
    #[must_use]
    pub fn ct(&mut self) -> CtW<UncorrectableErrorStatusSpec> {
        CtW::new(self, 14)
    }
    #[doc = "Bit 15 - Completer Abort Status \\[CA\\]
This bit is set when the core has returned the Completer Abort (CA) status to a request received from the link. This error is considered non-fatal by default, except for the special cases outlined in PCI Express Base Specification 2.0. The header of the received request that caused the error is logged in the Header Log Registers."]
    #[inline(always)]
    #[must_use]
    pub fn ca(&mut self) -> CaW<UncorrectableErrorStatusSpec> {
        CaW::new(self, 15)
    }
    #[doc = "Bit 16 - Unexpected Completion Status \\[UC\\]
This bit is set when the core has received an unexpected Completion packet from the link."]
    #[inline(always)]
    #[must_use]
    pub fn uc(&mut self) -> UcW<UncorrectableErrorStatusSpec> {
        UcW::new(self, 16)
    }
    #[doc = "Bit 17 - Receiver Overflow Status \\[RO\\]
This bit is set when the core receives a TLP in violation of the receive credit currently available."]
    #[inline(always)]
    #[must_use]
    pub fn ro(&mut self) -> RoW<UncorrectableErrorStatusSpec> {
        RoW::new(self, 17)
    }
    #[doc = "Bit 18 - Malformed TLP Status \\[MT\\]
This bit is set when the core receives a malformed TLP from the link. This error is considered fatal by default. The header of the received TLP with error is logged in the Header Log Registers."]
    #[inline(always)]
    #[must_use]
    pub fn mt(&mut self) -> MtW<UncorrectableErrorStatusSpec> {
        MtW::new(self, 18)
    }
    #[doc = "Bit 19 - ECRC Error Status \\[EE\\]
This bit is set when the core has detected an ECRC error in a received TLP."]
    #[inline(always)]
    #[must_use]
    pub fn ee(&mut self) -> EeW<UncorrectableErrorStatusSpec> {
        EeW::new(self, 19)
    }
    #[doc = "Bit 20 - Unsupported Request Error Status \\[URE\\]
This bit is set when the core has received a request from the link that it does not support. This error is not Function-specific. This error is considered non-fatal by default, except for the special case outlined in PCI Express Base Specification 2.0. The header of the received request that caused the error is logged in the Header Log Registers."]
    #[inline(always)]
    #[must_use]
    pub fn ure(&mut self) -> UreW<UncorrectableErrorStatusSpec> {
        UreW::new(self, 20)
    }
    #[doc = "Bit 22 - Uncorrectable Internal Error Status \\[UIE\\]
This bit is set when the core has detected an internal uncorrectable error (HAL parity error or an uncorrectable ECC error while reading from any of the RAMs). This bit is also set in response to the client signaling an internal error through the input UNCORRECTABLE_ERROR_IN. This error is considered fatal by default."]
    #[inline(always)]
    #[must_use]
    pub fn uie(&mut self) -> UieW<UncorrectableErrorStatusSpec> {
        UieW::new(self, 22)
    }
}
#[doc = "Uncorrectable Error Status Register Reserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`uncorrectable_error_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`uncorrectable_error_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UncorrectableErrorStatusSpec;
impl crate::RegisterSpec for UncorrectableErrorStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uncorrectable_error_status::R`](R) reader structure"]
impl crate::Readable for UncorrectableErrorStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`uncorrectable_error_status::W`](W) writer structure"]
impl crate::Writable for UncorrectableErrorStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x005f_f010;
}
#[doc = "`reset()` method sets UNCORRECTABLE_ERROR_STATUS to value 0"]
impl crate::Resettable for UncorrectableErrorStatusSpec {
    const RESET_VALUE: u32 = 0;
}
