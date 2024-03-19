#[doc = "Register `PCIE_RC_COMMAND_AND_STATUS` reader"]
pub type R = crate::R<PcieRcCommandAndStatusSpec>;
#[doc = "Register `PCIE_RC_COMMAND_AND_STATUS` writer"]
pub type W = crate::W<PcieRcCommandAndStatusSpec>;
#[doc = "Field `ISE` reader - IO-Space Enable \\[ISE\\]
Enables IO accesses through the core for this PCI Function."]
pub type IseR = crate::BitReader;
#[doc = "Field `ISE` writer - IO-Space Enable \\[ISE\\]
Enables IO accesses through the core for this PCI Function."]
pub type IseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MSE` reader - Mem-Space Enable \\[MSE\\]
Enables memory accesses through the core for this PCI Function."]
pub type MseR = crate::BitReader;
#[doc = "Field `MSE` writer - Mem-Space Enable \\[MSE\\]
Enables memory accesses through the core for this PCI Function."]
pub type MseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BE` reader - Bus-Master Enable \\[BE\\]
Enables the device to issue memory and I/O requests from this Function."]
pub type BeR = crate::BitReader;
#[doc = "Field `BE` writer - Bus-Master Enable \\[BE\\]
Enables the device to issue memory and I/O requests from this Function."]
pub type BeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Reserved"]
pub type R0R = crate::FieldReader;
#[doc = "Field `PERE` reader - Parity Error Response Enable \\[PERE\\]
When this bit is 1, the core sets the Master Data Parity Error status bit when it detects the following error conditions: (i) The core receives a poisoned completion from the link in response to a request. (ii) The core sends out a poisoned write request on the link (this may be because an underflow occurred during the packet transfer at the host interface of the core.). When this bit is 0, the Master Data Parity Error status bit is never set."]
pub type PereR = crate::BitReader;
#[doc = "Field `PERE` writer - Parity Error Response Enable \\[PERE\\]
When this bit is 1, the core sets the Master Data Parity Error status bit when it detects the following error conditions: (i) The core receives a poisoned completion from the link in response to a request. (ii) The core sends out a poisoned write request on the link (this may be because an underflow occurred during the packet transfer at the host interface of the core.). When this bit is 0, the Master Data Parity Error status bit is never set."]
pub type PereW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R1` reader - Reserved \\[R1\\]
Reserved"]
pub type R1R = crate::BitReader;
#[doc = "Field `SE` reader - SERR Enable \\[SE\\]
Enables the reporting of fatal and non-fatal errors detected by the core to the Root Complex."]
pub type SeR = crate::BitReader;
#[doc = "Field `SE` writer - SERR Enable \\[SE\\]
Enables the reporting of fatal and non-fatal errors detected by the core to the Root Complex."]
pub type SeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R2` reader - Reserved \\[R2\\]
Reserved"]
pub type R2R = crate::BitReader;
#[doc = "Field `IMD` reader - INTx Message Disabled \\[IMD\\]
Enables or disables the transmission of INTx Assert and De-assert messages from the core. The setting of this bit has no effect on the operation of the core in the RC mode."]
pub type ImdR = crate::BitReader;
#[doc = "Field `IMD` writer - INTx Message Disabled \\[IMD\\]
Enables or disables the transmission of INTx Assert and De-assert messages from the core. The setting of this bit has no effect on the operation of the core in the RC mode."]
pub type ImdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R3` reader - Reserved \\[R3\\]
Reserved"]
pub type R3R = crate::FieldReader;
#[doc = "Field `R4` reader - Reserved \\[R4\\]
Reserved"]
pub type R4R = crate::FieldReader;
#[doc = "Field `IS` reader - Interrupt Status \\[IS\\]
This bit is valid only when the core is configured to support legacy interrupts. Indicates that the core has a pending interrupt, that is, the core has sent an Assert_INTx message but has not transmitted a corresponding Deassert_INTx message."]
pub type IsR = crate::BitReader;
#[doc = "Field `CL` reader - Capabilities List \\[CL\\]
Indicates the presence of PCI Extended Capabilities registers. This bit is hardwired to 1."]
pub type ClR = crate::BitReader;
#[doc = "Field `R5` reader - Reserved \\[R5\\]
Reserved"]
pub type R5R = crate::FieldReader;
#[doc = "Field `MDPE` reader - Master Data Parity Error \\[MDPE\\]
When the Parity Error Response enable bit is 1, the core sets this bit when it detects the following error conditions: (i) The core receives a poisoned request from the link. (ii) The core has sent a Poisoned Completion downstream to the link This bit remains 0 when the Parity Error Response enable bit is 0. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type MdpeR = crate::BitReader;
#[doc = "Field `MDPE` writer - Master Data Parity Error \\[MDPE\\]
When the Parity Error Response enable bit is 1, the core sets this bit when it detects the following error conditions: (i) The core receives a poisoned request from the link. (ii) The core has sent a Poisoned Completion downstream to the link This bit remains 0 when the Parity Error Response enable bit is 0. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type MdpeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R6` reader - Reserved \\[R6\\]
Reserved"]
pub type R6R = crate::FieldReader;
#[doc = "Field `STA` reader - Signaled Target Abort \\[STA\\]
This bit is set when the core has sent a completion to the link with the Completer Abort status. This field can also be cleared from the local management APB bus by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type StaR = crate::BitReader;
#[doc = "Field `STA` writer - Signaled Target Abort \\[STA\\]
This bit is set when the core has sent a completion to the link with the Completer Abort status. This field can also be cleared from the local management APB bus by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type StaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RTA` reader - Received Target Abort \\[RTA\\]
This bit is set when the core has received a completion from the link with the Completer Abort status. This field can also be cleared from the local management APB bus by writing a 1 into this bit position. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type RtaR = crate::BitReader;
#[doc = "Field `RTA` writer - Received Target Abort \\[RTA\\]
This bit is set when the core has received a completion from the link with the Completer Abort status. This field can also be cleared from the local management APB bus by writing a 1 into this bit position. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type RtaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RMA` reader - Received Master Abort \\[RMA\\]
This bit is set when the core has received a completion from the link with the Unsupported Request status. This field can also be cleared from the local management APB bus by writing a 1 into this bit position This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type RmaR = crate::BitReader;
#[doc = "Field `RMA` writer - Received Master Abort \\[RMA\\]
This bit is set when the core has received a completion from the link with the Unsupported Request status. This field can also be cleared from the local management APB bus by writing a 1 into this bit position This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type RmaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SSE` reader - Signaled System Error \\[SSE\\]
The core sets this bit (i)On receiving an error message from the link, if SERR-Enable in PCI Command Register is 1 and SERR-Enable in the Bridge Control Register is also 1. (ii)On any internal Fatal/Non-Fatal error detected, if SERR-Enable in PCI Command Register is 1. This field can also be cleared from the local management APB bus by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type SseR = crate::BitReader;
#[doc = "Field `SSE` writer - Signaled System Error \\[SSE\\]
The core sets this bit (i)On receiving an error message from the link, if SERR-Enable in PCI Command Register is 1 and SERR-Enable in the Bridge Control Register is also 1. (ii)On any internal Fatal/Non-Fatal error detected, if SERR-Enable in PCI Command Register is 1. This field can also be cleared from the local management APB bus by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type SseW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DPE` reader - Detected Parity Error \\[DPE\\]
This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus APB by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type DpeR = crate::BitReader;
#[doc = "Field `DPE` writer - Detected Parity Error \\[DPE\\]
This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus APB by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type DpeW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO-Space Enable \\[ISE\\]
Enables IO accesses through the core for this PCI Function."]
    #[inline(always)]
    pub fn ise(&self) -> IseR {
        IseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mem-Space Enable \\[MSE\\]
Enables memory accesses through the core for this PCI Function."]
    #[inline(always)]
    pub fn mse(&self) -> MseR {
        MseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus-Master Enable \\[BE\\]
Enables the device to issue memory and I/O requests from this Function."]
    #[inline(always)]
    pub fn be(&self) -> BeR {
        BeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Reserved \\[R0\\]
Reserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Parity Error Response Enable \\[PERE\\]
When this bit is 1, the core sets the Master Data Parity Error status bit when it detects the following error conditions: (i) The core receives a poisoned completion from the link in response to a request. (ii) The core sends out a poisoned write request on the link (this may be because an underflow occurred during the packet transfer at the host interface of the core.). When this bit is 0, the Master Data Parity Error status bit is never set."]
    #[inline(always)]
    pub fn pere(&self) -> PereR {
        PereR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Reserved \\[R1\\]
Reserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SERR Enable \\[SE\\]
Enables the reporting of fatal and non-fatal errors detected by the core to the Root Complex."]
    #[inline(always)]
    pub fn se(&self) -> SeR {
        SeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved \\[R2\\]
Reserved"]
    #[inline(always)]
    pub fn r2(&self) -> R2R {
        R2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - INTx Message Disabled \\[IMD\\]
Enables or disables the transmission of INTx Assert and De-assert messages from the core. The setting of this bit has no effect on the operation of the core in the RC mode."]
    #[inline(always)]
    pub fn imd(&self) -> ImdR {
        ImdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:15 - Reserved \\[R3\\]
Reserved"]
    #[inline(always)]
    pub fn r3(&self) -> R3R {
        R3R::new(((self.bits >> 11) & 0x1f) as u8)
    }
    #[doc = "Bits 16:18 - Reserved \\[R4\\]
Reserved"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - Interrupt Status \\[IS\\]
This bit is valid only when the core is configured to support legacy interrupts. Indicates that the core has a pending interrupt, that is, the core has sent an Assert_INTx message but has not transmitted a corresponding Deassert_INTx message."]
    #[inline(always)]
    pub fn is(&self) -> IsR {
        IsR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Capabilities List \\[CL\\]
Indicates the presence of PCI Extended Capabilities registers. This bit is hardwired to 1."]
    #[inline(always)]
    pub fn cl(&self) -> ClR {
        ClR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - Reserved \\[R5\\]
Reserved"]
    #[inline(always)]
    pub fn r5(&self) -> R5R {
        R5R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Master Data Parity Error \\[MDPE\\]
When the Parity Error Response enable bit is 1, the core sets this bit when it detects the following error conditions: (i) The core receives a poisoned request from the link. (ii) The core has sent a Poisoned Completion downstream to the link This bit remains 0 when the Parity Error Response enable bit is 0. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn mdpe(&self) -> MdpeR {
        MdpeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Reserved \\[R6\\]
Reserved"]
    #[inline(always)]
    pub fn r6(&self) -> R6R {
        R6R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Signaled Target Abort \\[STA\\]
This bit is set when the core has sent a completion to the link with the Completer Abort status. This field can also be cleared from the local management APB bus by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn sta(&self) -> StaR {
        StaR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Received Target Abort \\[RTA\\]
This bit is set when the core has received a completion from the link with the Completer Abort status. This field can also be cleared from the local management APB bus by writing a 1 into this bit position. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn rta(&self) -> RtaR {
        RtaR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Received Master Abort \\[RMA\\]
This bit is set when the core has received a completion from the link with the Unsupported Request status. This field can also be cleared from the local management APB bus by writing a 1 into this bit position This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn rma(&self) -> RmaR {
        RmaR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Signaled System Error \\[SSE\\]
The core sets this bit (i)On receiving an error message from the link, if SERR-Enable in PCI Command Register is 1 and SERR-Enable in the Bridge Control Register is also 1. (ii)On any internal Fatal/Non-Fatal error detected, if SERR-Enable in PCI Command Register is 1. This field can also be cleared from the local management APB bus by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn sse(&self) -> SseR {
        SseR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Detected Parity Error \\[DPE\\]
This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus APB by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn dpe(&self) -> DpeR {
        DpeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - IO-Space Enable \\[ISE\\]
Enables IO accesses through the core for this PCI Function."]
    #[inline(always)]
    #[must_use]
    pub fn ise(&mut self) -> IseW<PcieRcCommandAndStatusSpec> {
        IseW::new(self, 0)
    }
    #[doc = "Bit 1 - Mem-Space Enable \\[MSE\\]
Enables memory accesses through the core for this PCI Function."]
    #[inline(always)]
    #[must_use]
    pub fn mse(&mut self) -> MseW<PcieRcCommandAndStatusSpec> {
        MseW::new(self, 1)
    }
    #[doc = "Bit 2 - Bus-Master Enable \\[BE\\]
Enables the device to issue memory and I/O requests from this Function."]
    #[inline(always)]
    #[must_use]
    pub fn be(&mut self) -> BeW<PcieRcCommandAndStatusSpec> {
        BeW::new(self, 2)
    }
    #[doc = "Bit 6 - Parity Error Response Enable \\[PERE\\]
When this bit is 1, the core sets the Master Data Parity Error status bit when it detects the following error conditions: (i) The core receives a poisoned completion from the link in response to a request. (ii) The core sends out a poisoned write request on the link (this may be because an underflow occurred during the packet transfer at the host interface of the core.). When this bit is 0, the Master Data Parity Error status bit is never set."]
    #[inline(always)]
    #[must_use]
    pub fn pere(&mut self) -> PereW<PcieRcCommandAndStatusSpec> {
        PereW::new(self, 6)
    }
    #[doc = "Bit 8 - SERR Enable \\[SE\\]
Enables the reporting of fatal and non-fatal errors detected by the core to the Root Complex."]
    #[inline(always)]
    #[must_use]
    pub fn se(&mut self) -> SeW<PcieRcCommandAndStatusSpec> {
        SeW::new(self, 8)
    }
    #[doc = "Bit 10 - INTx Message Disabled \\[IMD\\]
Enables or disables the transmission of INTx Assert and De-assert messages from the core. The setting of this bit has no effect on the operation of the core in the RC mode."]
    #[inline(always)]
    #[must_use]
    pub fn imd(&mut self) -> ImdW<PcieRcCommandAndStatusSpec> {
        ImdW::new(self, 10)
    }
    #[doc = "Bit 24 - Master Data Parity Error \\[MDPE\\]
When the Parity Error Response enable bit is 1, the core sets this bit when it detects the following error conditions: (i) The core receives a poisoned request from the link. (ii) The core has sent a Poisoned Completion downstream to the link This bit remains 0 when the Parity Error Response enable bit is 0. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    #[must_use]
    pub fn mdpe(&mut self) -> MdpeW<PcieRcCommandAndStatusSpec> {
        MdpeW::new(self, 24)
    }
    #[doc = "Bit 27 - Signaled Target Abort \\[STA\\]
This bit is set when the core has sent a completion to the link with the Completer Abort status. This field can also be cleared from the local management APB bus by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    #[must_use]
    pub fn sta(&mut self) -> StaW<PcieRcCommandAndStatusSpec> {
        StaW::new(self, 27)
    }
    #[doc = "Bit 28 - Received Target Abort \\[RTA\\]
This bit is set when the core has received a completion from the link with the Completer Abort status. This field can also be cleared from the local management APB bus by writing a 1 into this bit position. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    #[must_use]
    pub fn rta(&mut self) -> RtaW<PcieRcCommandAndStatusSpec> {
        RtaW::new(self, 28)
    }
    #[doc = "Bit 29 - Received Master Abort \\[RMA\\]
This bit is set when the core has received a completion from the link with the Unsupported Request status. This field can also be cleared from the local management APB bus by writing a 1 into this bit position This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    #[must_use]
    pub fn rma(&mut self) -> RmaW<PcieRcCommandAndStatusSpec> {
        RmaW::new(self, 29)
    }
    #[doc = "Bit 30 - Signaled System Error \\[SSE\\]
The core sets this bit (i)On receiving an error message from the link, if SERR-Enable in PCI Command Register is 1 and SERR-Enable in the Bridge Control Register is also 1. (ii)On any internal Fatal/Non-Fatal error detected, if SERR-Enable in PCI Command Register is 1. This field can also be cleared from the local management APB bus by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    #[must_use]
    pub fn sse(&mut self) -> SseW<PcieRcCommandAndStatusSpec> {
        SseW::new(self, 30)
    }
    #[doc = "Bit 31 - Detected Parity Error \\[DPE\\]
This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus APB by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    #[must_use]
    pub fn dpe(&mut self) -> DpeW<PcieRcCommandAndStatusSpec> {
        DpeW::new(self, 31)
    }
}
#[doc = "Command and Status Register This bit is set when the core has received a poisoned TLP. The Parity Error Response enable bit (bit 6) has no effect on the setting of this bit. This field can also be cleared from the local management bus APB by writing a 1 into this bit position. This field can be forced to 1 from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_command_and_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_command_and_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcCommandAndStatusSpec;
impl crate::RegisterSpec for PcieRcCommandAndStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_command_and_status::R`](R) reader structure"]
impl crate::Readable for PcieRcCommandAndStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_command_and_status::W`](W) writer structure"]
impl crate::Writable for PcieRcCommandAndStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf900_0000;
}
#[doc = "`reset()` method sets PCIE_RC_COMMAND_AND_STATUS to value 0x0010_0000"]
impl crate::Resettable for PcieRcCommandAndStatusSpec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
