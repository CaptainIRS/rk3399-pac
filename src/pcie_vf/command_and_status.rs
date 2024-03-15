#[doc = "Register `COMMAND_AND_STATUS` reader"]
pub type R = crate::R<CommandAndStatusSpec>;
#[doc = "Register `COMMAND_AND_STATUS` writer"]
pub type W = crate::W<CommandAndStatusSpec>;
#[doc = "Field `IOSE` reader - IO-Space Enable \\[IOSE\\]
Reserved"]
pub type IoseR = crate::BitReader;
#[doc = "Field `MSE` reader - Mem- Space Enable \\[MSE\\]
Reserved"]
pub type MseR = crate::BitReader;
#[doc = "Field `BME` reader - Bus-Master Enable \\[BME\\]
Enables the device to issue memory requests from this Function. This field can be written from the local management bus."]
pub type BmeR = crate::BitReader;
#[doc = "Field `BME` writer - Bus-Master Enable \\[BME\\]
Enables the device to issue memory requests from this Function. This field can be written from the local management bus."]
pub type BmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R0` reader - Reserved \\[R0\\]
Reserved"]
pub type R0R = crate::FieldReader;
#[doc = "Field `PERE` reader - Parity Error Response Enable \\[PERE\\]
Reserved"]
pub type PereR = crate::BitReader;
#[doc = "Field `R1` reader - Reserved \\[R1\\]
Reserved"]
pub type R1R = crate::BitReader;
#[doc = "Field `SE` reader - SERR Enable \\[SE\\]
Reserved"]
pub type SeR = crate::BitReader;
#[doc = "Field `R2` reader - Reserved \\[R2\\]
Reserved"]
pub type R2R = crate::BitReader;
#[doc = "Field `IMD` reader - INTx Message Disable \\[IMD\\]
Reserved"]
pub type ImdR = crate::BitReader;
#[doc = "Field `R3` reader - Reserved \\[R3\\]
Reserved"]
pub type R3R = crate::FieldReader;
#[doc = "Field `IS` reader - Interrupt Status \\[IS\\]
Reserved"]
pub type IsR = crate::BitReader;
#[doc = "Field `CL` reader - Capabilities List \\[CL\\]
Indicates the presence of PCI Extended Capabilities registers. This bit is hardwired to 1."]
pub type ClR = crate::BitReader;
#[doc = "Field `R4` reader - Reserved \\[R4\\]
Reserved"]
pub type R4R = crate::FieldReader;
#[doc = "Field `MDPE` reader - Master Data Parity Error \\[MDPE\\]
When the Parity Error Response enable bit in the PCI Command Register of the associated Physical Function is set, the core sets this bit when it detects the following error conditions: (i) The core receives a Poisoned Completion TLP from the link in response to a request from this VF. (ii)The core sends out a poisoned write request on the link from this VF. (This bit remains 0 when the Parity Error Response enable bit in the PCI Command Register of the associated Physical Function is 0). This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
pub type MdpeR = crate::BitReader;
#[doc = "Field `MDPE` writer - Master Data Parity Error \\[MDPE\\]
When the Parity Error Response enable bit in the PCI Command Register of the associated Physical Function is set, the core sets this bit when it detects the following error conditions: (i) The core receives a Poisoned Completion TLP from the link in response to a request from this VF. (ii)The core sends out a poisoned write request on the link from this VF. (This bit remains 0 when the Parity Error Response enable bit in the PCI Command Register of the associated Physical Function is 0). This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
pub type MdpeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R5` reader - Reserved \\[R5\\]
Reserved"]
pub type R5R = crate::FieldReader;
#[doc = "Field `STA` reader - Signaled Target Abort \\[STA\\]
This bit is set when the core has sent a completion from this VF to the link with the Completer Abort status. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
pub type StaR = crate::BitReader;
#[doc = "Field `STA` writer - Signaled Target Abort \\[STA\\]
This bit is set when the core has sent a completion from this VF to the link with the Completer Abort status. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
pub type StaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RTA` reader - Received Target Abort \\[RTA\\]
This bit is set when this Virtual Function has received a completion from the link with the Completer Abort status. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
pub type RtaR = crate::BitReader;
#[doc = "Field `RTA` writer - Received Target Abort \\[RTA\\]
This bit is set when this Virtual Function has received a completion from the link with the Completer Abort status. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
pub type RtaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RMA` reader - Received Master Abort \\[RMA\\]
This bit is set when this VF has received a completion from the link with the Unsupported Request status. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
pub type RmaR = crate::BitReader;
#[doc = "Field `RMA` writer - Received Master Abort \\[RMA\\]
This bit is set when this VF has received a completion from the link with the Unsupported Request status. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
pub type RmaW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `SSE` reader - Signaled System Error \\[SSE\\]
If the SERR enable bit in the PCI Command Register of the associated Physical Function is 1, this bit is set when this VF has sent out a fatal or non-fatal error message on the link to the Root Complex. If the SERR enable bit is 0, this bit remains 0. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
pub type SseR = crate::BitReader;
#[doc = "Field `SSE` writer - Signaled System Error \\[SSE\\]
If the SERR enable bit in the PCI Command Register of the associated Physical Function is 1, this bit is set when this VF has sent out a fatal or non-fatal error message on the link to the Root Complex. If the SERR enable bit is 0, this bit remains 0. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
pub type SseW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `DPE` reader - Detected Parity Error \\[DPE\\]
This bit is set when the core has received a Poisoned TLP targeted at this VF. The Parity Error Response enable bit (bit 6) in the PCI Command Register of the associated PF has no effect on the setting of this bit. STICKY."]
pub type DpeR = crate::BitReader;
#[doc = "Field `DPE` writer - Detected Parity Error \\[DPE\\]
This bit is set when the core has received a Poisoned TLP targeted at this VF. The Parity Error Response enable bit (bit 6) in the PCI Command Register of the associated PF has no effect on the setting of this bit. STICKY."]
pub type DpeW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bit 0 - IO-Space Enable \\[IOSE\\]
Reserved"]
    #[inline(always)]
    pub fn iose(&self) -> IoseR {
        IoseR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Mem- Space Enable \\[MSE\\]
Reserved"]
    #[inline(always)]
    pub fn mse(&self) -> MseR {
        MseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Bus-Master Enable \\[BME\\]
Enables the device to issue memory requests from this Function. This field can be written from the local management bus."]
    #[inline(always)]
    pub fn bme(&self) -> BmeR {
        BmeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Reserved \\[R0\\]
Reserved"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - Parity Error Response Enable \\[PERE\\]
Reserved"]
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
Reserved"]
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
    #[doc = "Bit 10 - INTx Message Disable \\[IMD\\]
Reserved"]
    #[inline(always)]
    pub fn imd(&self) -> ImdR {
        ImdR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:18 - Reserved \\[R3\\]
Reserved"]
    #[inline(always)]
    pub fn r3(&self) -> R3R {
        R3R::new(((self.bits >> 11) & 0xff) as u8)
    }
    #[doc = "Bit 19 - Interrupt Status \\[IS\\]
Reserved"]
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
    #[doc = "Bits 21:23 - Reserved \\[R4\\]
Reserved"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Master Data Parity Error \\[MDPE\\]
When the Parity Error Response enable bit in the PCI Command Register of the associated Physical Function is set, the core sets this bit when it detects the following error conditions: (i) The core receives a Poisoned Completion TLP from the link in response to a request from this VF. (ii)The core sends out a poisoned write request on the link from this VF. (This bit remains 0 when the Parity Error Response enable bit in the PCI Command Register of the associated Physical Function is 0). This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
    #[inline(always)]
    pub fn mdpe(&self) -> MdpeR {
        MdpeR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Reserved \\[R5\\]
Reserved"]
    #[inline(always)]
    pub fn r5(&self) -> R5R {
        R5R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bit 27 - Signaled Target Abort \\[STA\\]
This bit is set when the core has sent a completion from this VF to the link with the Completer Abort status. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
    #[inline(always)]
    pub fn sta(&self) -> StaR {
        StaR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Received Target Abort \\[RTA\\]
This bit is set when this Virtual Function has received a completion from the link with the Completer Abort status. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
    #[inline(always)]
    pub fn rta(&self) -> RtaR {
        RtaR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Received Master Abort \\[RMA\\]
This bit is set when this VF has received a completion from the link with the Unsupported Request status. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
    #[inline(always)]
    pub fn rma(&self) -> RmaR {
        RmaR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Signaled System Error \\[SSE\\]
If the SERR enable bit in the PCI Command Register of the associated Physical Function is 1, this bit is set when this VF has sent out a fatal or non-fatal error message on the link to the Root Complex. If the SERR enable bit is 0, this bit remains 0. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
    #[inline(always)]
    pub fn sse(&self) -> SseR {
        SseR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Detected Parity Error \\[DPE\\]
This bit is set when the core has received a Poisoned TLP targeted at this VF. The Parity Error Response enable bit (bit 6) in the PCI Command Register of the associated PF has no effect on the setting of this bit. STICKY."]
    #[inline(always)]
    pub fn dpe(&self) -> DpeR {
        DpeR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Bus-Master Enable \\[BME\\]
Enables the device to issue memory requests from this Function. This field can be written from the local management bus."]
    #[inline(always)]
    #[must_use]
    pub fn bme(&mut self) -> BmeW<CommandAndStatusSpec> {
        BmeW::new(self, 2)
    }
    #[doc = "Bit 24 - Master Data Parity Error \\[MDPE\\]
When the Parity Error Response enable bit in the PCI Command Register of the associated Physical Function is set, the core sets this bit when it detects the following error conditions: (i) The core receives a Poisoned Completion TLP from the link in response to a request from this VF. (ii)The core sends out a poisoned write request on the link from this VF. (This bit remains 0 when the Parity Error Response enable bit in the PCI Command Register of the associated Physical Function is 0). This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn mdpe(&mut self) -> MdpeW<CommandAndStatusSpec> {
        MdpeW::new(self, 24)
    }
    #[doc = "Bit 27 - Signaled Target Abort \\[STA\\]
This bit is set when the core has sent a completion from this VF to the link with the Completer Abort status. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn sta(&mut self) -> StaW<CommandAndStatusSpec> {
        StaW::new(self, 27)
    }
    #[doc = "Bit 28 - Received Target Abort \\[RTA\\]
This bit is set when this Virtual Function has received a completion from the link with the Completer Abort status. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn rta(&mut self) -> RtaW<CommandAndStatusSpec> {
        RtaW::new(self, 28)
    }
    #[doc = "Bit 29 - Received Master Abort \\[RMA\\]
This bit is set when this VF has received a completion from the link with the Unsupported Request status. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn rma(&mut self) -> RmaW<CommandAndStatusSpec> {
        RmaW::new(self, 29)
    }
    #[doc = "Bit 30 - Signaled System Error \\[SSE\\]
If the SERR enable bit in the PCI Command Register of the associated Physical Function is 1, this bit is set when this VF has sent out a fatal or non-fatal error message on the link to the Root Complex. If the SERR enable bit is 0, this bit remains 0. This field can also be cleared from the local management bus by writing a 1 into this bit position. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn sse(&mut self) -> SseW<CommandAndStatusSpec> {
        SseW::new(self, 30)
    }
    #[doc = "Bit 31 - Detected Parity Error \\[DPE\\]
This bit is set when the core has received a Poisoned TLP targeted at this VF. The Parity Error Response enable bit (bit 6) in the PCI Command Register of the associated PF has no effect on the setting of this bit. STICKY."]
    #[inline(always)]
    #[must_use]
    pub fn dpe(&mut self) -> DpeW<CommandAndStatusSpec> {
        DpeW::new(self, 31)
    }
}
#[doc = "Command and Status Register This bit is set when the core has received a Poisoned TLP targeted at this VF. The Parity Error Response enable bit (bit 6) in the PCI Command Register of the associated PF has no effect on the setting of this bit. STICKY.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`command_and_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`command_and_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CommandAndStatusSpec;
impl crate::RegisterSpec for CommandAndStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`command_and_status::R`](R) reader structure"]
impl crate::Readable for CommandAndStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`command_and_status::W`](W) writer structure"]
impl crate::Writable for CommandAndStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xf900_0000;
}
#[doc = "`reset()` method sets COMMAND_AND_STATUS to value 0x0010_0000"]
impl crate::Resettable for CommandAndStatusSpec {
    const RESET_VALUE: u32 = 0x0010_0000;
}
