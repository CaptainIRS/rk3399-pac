#[doc = "Register `PCIE_LM_LOCAL_ERROR_AND_STATUS` reader"]
pub type R = crate::R<PcieLmLocalErrorAndStatusSpec>;
#[doc = "Register `PCIE_LM_LOCAL_ERROR_AND_STATUS` writer"]
pub type W = crate::W<PcieLmLocalErrorAndStatusSpec>;
#[doc = "Field `PRFPE` reader - PNP RX FIFO Parity Error \\[PRFPE\\]\n\nParity error detected while reading\n\nfrom the PNP Receive FIFO RAM."]
pub type PrfpeR = crate::BitReader;
#[doc = "Field `PRFPE` writer - PNP RX FIFO Parity Error \\[PRFPE\\]\n\nParity error detected while reading\n\nfrom the PNP Receive FIFO RAM."]
pub type PrfpeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CRFPE` reader - Completion RX FIFO Parity Error \\[CRFPE\\]\n\nParity error detected while reading\n\nfrom the Completion Receive FIFO\n\nRAM."]
pub type CrfpeR = crate::BitReader;
#[doc = "Field `CRFPE` writer - Completion RX FIFO Parity Error \\[CRFPE\\]\n\nParity error detected while reading\n\nfrom the Completion Receive FIFO\n\nRAM."]
pub type CrfpeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RRPE` reader - Replay RAM Parity Error \\[RRPE\\]\n\nParity error detected while reading\n\nfrom Replay Buffer RAM."]
pub type RrpeR = crate::BitReader;
#[doc = "Field `RRPE` writer - Replay RAM Parity Error \\[RRPE\\]\n\nParity error detected while reading\n\nfrom Replay Buffer RAM."]
pub type RrpeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PRFO` reader - PNP RX FIFO Overflow \\[PRFO\\]\n\nOverflow occurred in the PNP\n\nReceive FIFO."]
pub type PrfoR = crate::BitReader;
#[doc = "Field `PRFO` writer - PNP RX FIFO Overflow \\[PRFO\\]\n\nOverflow occurred in the PNP\n\nReceive FIFO."]
pub type PrfoW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CRFO` reader - Completion RX FIFO Overflow \\[CRFO\\]\n\nOverflow occurred in the Completion\n\nReceive FIFO."]
pub type CrfoR = crate::BitReader;
#[doc = "Field `CRFO` writer - Completion RX FIFO Overflow \\[CRFO\\]\n\nOverflow occurred in the Completion\n\nReceive FIFO."]
pub type CrfoW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RT` reader - Replay Timeout \\[RT\\]\n\nReplay timer timed out"]
pub type RtR = crate::BitReader;
#[doc = "Field `RT` writer - Replay Timeout \\[RT\\]\n\nReplay timer timed out"]
pub type RtW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `RTR` reader - Replay Timer Rollover \\[RTR\\]\n\nReplay timer rolled over after 4\n\ntransmissions of the same TLP."]
pub type RtrR = crate::BitReader;
#[doc = "Field `RTR` writer - Replay Timer Rollover \\[RTR\\]\n\nReplay timer rolled over after 4\n\ntransmissions of the same TLP."]
pub type RtrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `PE` reader - Phy Error \\[PE\\]\n\nPhy error detected on receive side."]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - Phy Error \\[PE\\]\n\nPhy error detected on receive side."]
pub type PeW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MTR` reader - Malformed TLP Received \\[MTR\\]\n\nMalformed TLP received from the\n\nlink."]
pub type MtrR = crate::BitReader;
#[doc = "Field `MTR` writer - Malformed TLP Received \\[MTR\\]\n\nMalformed TLP received from the\n\nlink."]
pub type MtrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `UCR` reader - Unexpected Completion Received \\[UCR\\]\n\nUnexpected Completion received\n\nfrom the link."]
pub type UcrR = crate::BitReader;
#[doc = "Field `UCR` writer - Unexpected Completion Received \\[UCR\\]\n\nUnexpected Completion received\n\nfrom the link."]
pub type UcrW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `FCE` reader - Flow Control Error \\[FCE\\]\n\nAn error was observed in the flow\n\ncontrol advertisements from the\n\nother side."]
pub type FceR = crate::BitReader;
#[doc = "Field `FCE` writer - Flow Control Error \\[FCE\\]\n\nAn error was observed in the flow\n\ncontrol advertisements from the\n\nother side."]
pub type FceW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `CT` reader - Completion Timeout \\[CT\\]\n\nA request timed out waiting for\n\ncompletion."]
pub type CtR = crate::BitReader;
#[doc = "Field `CT` writer - Completion Timeout \\[CT\\]\n\nA request timed out waiting for\n\ncompletion."]
pub type CtW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R12` reader - Reserved \\[R12\\]\n\nReserved"]
pub type R12R = crate::BitReader;
#[doc = "Field `R13` reader - Reserved \\[R13\\]\n\nReserved"]
pub type R13R = crate::FieldReader;
#[doc = "Field `R17` reader - Reserved \\[R17\\]\n\nReserved"]
pub type R17R = crate::BitReader;
#[doc = "Field `UTC` reader - Unmapped TC \\[UTC\\]\n\nUnmapped TC error"]
pub type UtcR = crate::BitReader;
#[doc = "Field `UTC` writer - Unmapped TC \\[UTC\\]\n\nUnmapped TC error"]
pub type UtcW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `MMVC` reader - MSI Mask Value Change \\[MMVC\\]\n\nThis status bit is set whenever the\n\nMSI mask register value in the MSI\n\ncapability register changes value in\n\nANY of the functions in the controller"]
pub type MmvcR = crate::BitReader;
#[doc = "Field `MMVC` writer - MSI Mask Value Change \\[MMVC\\]\n\nThis status bit is set whenever the\n\nMSI mask register value in the MSI\n\ncapability register changes value in\n\nANY of the functions in the controller"]
pub type MmvcW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R22` reader - Reserved \\[R22\\]\n\nReserved"]
pub type R22R = crate::BitReader;
#[doc = "Field `R9` reader - Reserved \\[R9\\]\n\nReserved"]
pub type R9R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - PNP RX FIFO Parity Error \\[PRFPE\\]\n\nParity error detected while reading\n\nfrom the PNP Receive FIFO RAM."]
    #[inline(always)]
    pub fn prfpe(&self) -> PrfpeR {
        PrfpeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Completion RX FIFO Parity Error \\[CRFPE\\]\n\nParity error detected while reading\n\nfrom the Completion Receive FIFO\n\nRAM."]
    #[inline(always)]
    pub fn crfpe(&self) -> CrfpeR {
        CrfpeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Replay RAM Parity Error \\[RRPE\\]\n\nParity error detected while reading\n\nfrom Replay Buffer RAM."]
    #[inline(always)]
    pub fn rrpe(&self) -> RrpeR {
        RrpeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PNP RX FIFO Overflow \\[PRFO\\]\n\nOverflow occurred in the PNP\n\nReceive FIFO."]
    #[inline(always)]
    pub fn prfo(&self) -> PrfoR {
        PrfoR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Completion RX FIFO Overflow \\[CRFO\\]\n\nOverflow occurred in the Completion\n\nReceive FIFO."]
    #[inline(always)]
    pub fn crfo(&self) -> CrfoR {
        CrfoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Replay Timeout \\[RT\\]\n\nReplay timer timed out"]
    #[inline(always)]
    pub fn rt(&self) -> RtR {
        RtR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Replay Timer Rollover \\[RTR\\]\n\nReplay timer rolled over after 4\n\ntransmissions of the same TLP."]
    #[inline(always)]
    pub fn rtr(&self) -> RtrR {
        RtrR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Phy Error \\[PE\\]\n\nPhy error detected on receive side."]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Malformed TLP Received \\[MTR\\]\n\nMalformed TLP received from the\n\nlink."]
    #[inline(always)]
    pub fn mtr(&self) -> MtrR {
        MtrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Unexpected Completion Received \\[UCR\\]\n\nUnexpected Completion received\n\nfrom the link."]
    #[inline(always)]
    pub fn ucr(&self) -> UcrR {
        UcrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Flow Control Error \\[FCE\\]\n\nAn error was observed in the flow\n\ncontrol advertisements from the\n\nother side."]
    #[inline(always)]
    pub fn fce(&self) -> FceR {
        FceR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Completion Timeout \\[CT\\]\n\nA request timed out waiting for\n\ncompletion."]
    #[inline(always)]
    pub fn ct(&self) -> CtR {
        CtR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Reserved \\[R12\\]\n\nReserved"]
    #[inline(always)]
    pub fn r12(&self) -> R12R {
        R12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:16 - Reserved \\[R13\\]\n\nReserved"]
    #[inline(always)]
    pub fn r13(&self) -> R13R {
        R13R::new(((self.bits >> 13) & 0x0f) as u8)
    }
    #[doc = "Bit 17 - Reserved \\[R17\\]\n\nReserved"]
    #[inline(always)]
    pub fn r17(&self) -> R17R {
        R17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Unmapped TC \\[UTC\\]\n\nUnmapped TC error"]
    #[inline(always)]
    pub fn utc(&self) -> UtcR {
        UtcR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - MSI Mask Value Change \\[MMVC\\]\n\nThis status bit is set whenever the\n\nMSI mask register value in the MSI\n\ncapability register changes value in\n\nANY of the functions in the controller"]
    #[inline(always)]
    pub fn mmvc(&self) -> MmvcR {
        MmvcR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Reserved \\[R22\\]\n\nReserved"]
    #[inline(always)]
    pub fn r22(&self) -> R22R {
        R22R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:31 - Reserved \\[R9\\]\n\nReserved"]
    #[inline(always)]
    pub fn r9(&self) -> R9R {
        R9R::new(((self.bits >> 21) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - PNP RX FIFO Parity Error \\[PRFPE\\]\n\nParity error detected while reading\n\nfrom the PNP Receive FIFO RAM."]
    #[inline(always)]
    #[must_use]
    pub fn prfpe(&mut self) -> PrfpeW<PcieLmLocalErrorAndStatusSpec> {
        PrfpeW::new(self, 0)
    }
    #[doc = "Bit 1 - Completion RX FIFO Parity Error \\[CRFPE\\]\n\nParity error detected while reading\n\nfrom the Completion Receive FIFO\n\nRAM."]
    #[inline(always)]
    #[must_use]
    pub fn crfpe(&mut self) -> CrfpeW<PcieLmLocalErrorAndStatusSpec> {
        CrfpeW::new(self, 1)
    }
    #[doc = "Bit 2 - Replay RAM Parity Error \\[RRPE\\]\n\nParity error detected while reading\n\nfrom Replay Buffer RAM."]
    #[inline(always)]
    #[must_use]
    pub fn rrpe(&mut self) -> RrpeW<PcieLmLocalErrorAndStatusSpec> {
        RrpeW::new(self, 2)
    }
    #[doc = "Bit 3 - PNP RX FIFO Overflow \\[PRFO\\]\n\nOverflow occurred in the PNP\n\nReceive FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn prfo(&mut self) -> PrfoW<PcieLmLocalErrorAndStatusSpec> {
        PrfoW::new(self, 3)
    }
    #[doc = "Bit 4 - Completion RX FIFO Overflow \\[CRFO\\]\n\nOverflow occurred in the Completion\n\nReceive FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn crfo(&mut self) -> CrfoW<PcieLmLocalErrorAndStatusSpec> {
        CrfoW::new(self, 4)
    }
    #[doc = "Bit 5 - Replay Timeout \\[RT\\]\n\nReplay timer timed out"]
    #[inline(always)]
    #[must_use]
    pub fn rt(&mut self) -> RtW<PcieLmLocalErrorAndStatusSpec> {
        RtW::new(self, 5)
    }
    #[doc = "Bit 6 - Replay Timer Rollover \\[RTR\\]\n\nReplay timer rolled over after 4\n\ntransmissions of the same TLP."]
    #[inline(always)]
    #[must_use]
    pub fn rtr(&mut self) -> RtrW<PcieLmLocalErrorAndStatusSpec> {
        RtrW::new(self, 6)
    }
    #[doc = "Bit 7 - Phy Error \\[PE\\]\n\nPhy error detected on receive side."]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<PcieLmLocalErrorAndStatusSpec> {
        PeW::new(self, 7)
    }
    #[doc = "Bit 8 - Malformed TLP Received \\[MTR\\]\n\nMalformed TLP received from the\n\nlink."]
    #[inline(always)]
    #[must_use]
    pub fn mtr(&mut self) -> MtrW<PcieLmLocalErrorAndStatusSpec> {
        MtrW::new(self, 8)
    }
    #[doc = "Bit 9 - Unexpected Completion Received \\[UCR\\]\n\nUnexpected Completion received\n\nfrom the link."]
    #[inline(always)]
    #[must_use]
    pub fn ucr(&mut self) -> UcrW<PcieLmLocalErrorAndStatusSpec> {
        UcrW::new(self, 9)
    }
    #[doc = "Bit 10 - Flow Control Error \\[FCE\\]\n\nAn error was observed in the flow\n\ncontrol advertisements from the\n\nother side."]
    #[inline(always)]
    #[must_use]
    pub fn fce(&mut self) -> FceW<PcieLmLocalErrorAndStatusSpec> {
        FceW::new(self, 10)
    }
    #[doc = "Bit 11 - Completion Timeout \\[CT\\]\n\nA request timed out waiting for\n\ncompletion."]
    #[inline(always)]
    #[must_use]
    pub fn ct(&mut self) -> CtW<PcieLmLocalErrorAndStatusSpec> {
        CtW::new(self, 11)
    }
    #[doc = "Bit 18 - Unmapped TC \\[UTC\\]\n\nUnmapped TC error"]
    #[inline(always)]
    #[must_use]
    pub fn utc(&mut self) -> UtcW<PcieLmLocalErrorAndStatusSpec> {
        UtcW::new(self, 18)
    }
    #[doc = "Bit 19 - MSI Mask Value Change \\[MMVC\\]\n\nThis status bit is set whenever the\n\nMSI mask register value in the MSI\n\ncapability register changes value in\n\nANY of the functions in the controller"]
    #[inline(always)]
    #[must_use]
    pub fn mmvc(&mut self) -> MmvcW<PcieLmLocalErrorAndStatusSpec> {
        MmvcW::new(self, 19)
    }
}
#[doc = "Local Error and Status Register\n\nReserved\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_lm_local_error_and_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_lm_local_error_and_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieLmLocalErrorAndStatusSpec;
impl crate::RegisterSpec for PcieLmLocalErrorAndStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_lm_local_error_and_status::R`](R) reader structure"]
impl crate::Readable for PcieLmLocalErrorAndStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_lm_local_error_and_status::W`](W) writer structure"]
impl crate::Writable for PcieLmLocalErrorAndStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x000c_0fff;
}
#[doc = "`reset()` method sets PCIE_LM_LOCAL_ERROR_AND_STATUS to value 0"]
impl crate::Resettable for PcieLmLocalErrorAndStatusSpec {
    const RESET_VALUE: u32 = 0;
}
