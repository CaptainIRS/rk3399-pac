#[doc = "Register `PCIE_RC_LINK_CONTROL_AND_STATUS` reader"]
pub type R = crate::R<PcieRcLinkControlAndStatusSpec>;
#[doc = "Register `PCIE_RC_LINK_CONTROL_AND_STATUS` writer"]
pub type W = crate::W<PcieRcLinkControlAndStatusSpec>;
#[doc = "Field `ASPMC` reader - Active State Power Management Control \\[ASPMC\\]\n\nControls the level of ASPM support\n\non the PCI Express link associated\n\nwith the function. The valid setting\n\nare 00: ASPM disabled 01: L0s entry\n\nenabled, L1 disabled 10: L1 entry\n\nenabled, L0s disabled 11: Both L0s\n\nand L1 enabled. Note that these\n\nControl bits can be enabled only if\n\nthe corresponding ASPM Support bit\n\nis 1."]
pub type AspmcR = crate::FieldReader;
#[doc = "Field `ASPMC` writer - Active State Power Management Control \\[ASPMC\\]\n\nControls the level of ASPM support\n\non the PCI Express link associated\n\nwith the function. The valid setting\n\nare 00: ASPM disabled 01: L0s entry\n\nenabled, L1 disabled 10: L1 entry\n\nenabled, L0s disabled 11: Both L0s\n\nand L1 enabled. Note that these\n\nControl bits can be enabled only if\n\nthe corresponding ASPM Support bit\n\nis 1."]
pub type AspmcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `R10` reader - Reserved \\[R10\\]\n\nReserved"]
pub type R10R = crate::BitReader;
#[doc = "Field `RCB` reader - Read Completion Boundary \\[RCB\\]\n\nIndicates the Read Completion\n\nBoundary of the Root Port (0 = 64\n\nbytes, 1 = 128 bytes). This field can\n\nbe written from the APB bus by\n\nsetting \\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
pub type RcbR = crate::BitReader;
#[doc = "Field `LD` reader - Link Disable \\[LD\\]\n\nWriting a 1 to this bit position\n\ncauses the LTSSM to go to the\n\nDisable Link state. The LTSSM stays\n\nin the Disable Link state while this\n\nbit is set."]
pub type LdR = crate::BitReader;
#[doc = "Field `LD` writer - Link Disable \\[LD\\]\n\nWriting a 1 to this bit position\n\ncauses the LTSSM to go to the\n\nDisable Link state. The LTSSM stays\n\nin the Disable Link state while this\n\nbit is set."]
pub type LdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RL` reader - Retrain Link \\[RL\\]\n\nSetting this bit to 1 causes the\n\nLTSSM to initiate link training. This\n\nbit always reads as 0."]
pub type RlR = crate::BitReader;
#[doc = "Field `CCC` reader - Common Clock Configuration \\[CCC\\]\n\nA value of 0 indicates that the\n\nreference clock of this device is\n\nasynchronous to that of the\n\nupstream device. A value of 1\n\nindicates that the reference clock is\n\ncommon."]
pub type CccR = crate::BitReader;
#[doc = "Field `CCC` writer - Common Clock Configuration \\[CCC\\]\n\nA value of 0 indicates that the\n\nreference clock of this device is\n\nasynchronous to that of the\n\nupstream device. A value of 1\n\nindicates that the reference clock is\n\ncommon."]
pub type CccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ES` reader - Extended Synch \\[ES\\]\n\nSet to 1 to extend the sequence of\n\nordered sets transmitted while\n\nexiting from the L0S state."]
pub type EsR = crate::BitReader;
#[doc = "Field `ES` writer - Extended Synch \\[ES\\]\n\nSet to 1 to extend the sequence of\n\nordered sets transmitted while\n\nexiting from the L0S state."]
pub type EsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECPM` reader - Enable Clock Power Management \\[ECPM\\]\n\nThis field is hardwired to 0 when the\n\ncore is in the RC mode."]
pub type EcpmR = crate::BitReader;
#[doc = "Field `R9` reader - Reserved \\[R9\\]\n\nReserved"]
pub type R9R = crate::BitReader;
#[doc = "Field `LBMIE` reader - Link Bandwidth Management Interrupt Enable \\[LBMIE\\]\n\nWhen Set, this bit enables the\n\ngeneration of an interrupt to\n\nindicate that the Link Bandwidth\n\nManagement Status bit has been\n\nSet. This enables an interrupt to be\n\ngenerated through\n\nPHY_INTERRUPT_OUT if triggered.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
pub type LbmieR = crate::BitReader;
#[doc = "Field `LBMIE` writer - Link Bandwidth Management Interrupt Enable \\[LBMIE\\]\n\nWhen Set, this bit enables the\n\ngeneration of an interrupt to\n\nindicate that the Link Bandwidth\n\nManagement Status bit has been\n\nSet. This enables an interrupt to be\n\ngenerated through\n\nPHY_INTERRUPT_OUT if triggered.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
pub type LbmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LABIE` reader - Link Autonomous Bandwidth Interrupt Enable \\[LABIE\\]\n\nWhen Set, this bit enables the\n\ngeneration of an interrupt to\n\nindicate that the Link Autonomous\n\nBandwidth Status bit has been Set.\n\nThis enables an interrupt to be\n\ngenerated through\n\nPHY_INTERRUPT_OUT if triggered.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
pub type LabieR = crate::BitReader;
#[doc = "Field `LABIE` writer - Link Autonomous Bandwidth Interrupt Enable \\[LABIE\\]\n\nWhen Set, this bit enables the\n\ngeneration of an interrupt to\n\nindicate that the Link Autonomous\n\nBandwidth Status bit has been Set.\n\nThis enables an interrupt to be\n\ngenerated through\n\nPHY_INTERRUPT_OUT if triggered.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
pub type LabieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R11` reader - Reserved \\[R11\\]\n\nReserved"]
pub type R11R = crate::FieldReader;
#[doc = "Field `NLS` reader - Negotiated Link Speed \\[NLS\\]\n\nNegotiated link speed of the device.\n\nThe only supported speed ids are\n\n2.5 GT/s per lane (0001),5 GT/s per\n\nlane (0010) ."]
pub type NlsR = crate::FieldReader;
#[doc = "Field `NLW` reader - Negotiated Link Width \\[NLW\\]\n\nSet at the end of link training to the\n\nactual link width negotiated between\n\nthe two sides."]
pub type NlwR = crate::FieldReader;
#[doc = "Field `R12` reader - Reserved \\[R12\\]\n\nReserved"]
pub type R12R = crate::BitReader;
#[doc = "Field `LTS` reader - Link Training Status \\[LTS\\]\n\nThis bit is set to 1 when the LTSSM\n\nis in the Recovery or Configuration\n\nstates, or if a 1 has been written to\n\nthe Retrain Link bit but the link\n\ntraining has not yet begun."]
pub type LtsR = crate::BitReader;
#[doc = "Field `SCC` reader - Slot Clock Configuration \\[SCC\\]\n\nIndicates that the device uses the\n\nreference clock provided by the\n\nconnector. This field can be written\n\nfrom the APB bus by setting \\[21\\]
bit\n\nhigh of the pcie_mgmt_APB_ADDR\n\nduring a local management register\n\nwrite."]
pub type SccR = crate::BitReader;
#[doc = "Field `DA` reader - Data Link Layer Active \\[DA\\]\n\nIndicates the status of the Data Link\n\nLayer. Set to 1 when the DL\n\nControl and Management State\n\nMachine has reached the DL_Active\n\nstate. This bit is hardwired to 0 in\n\nthis version of the core."]
pub type DaR = crate::BitReader;
#[doc = "Field `LBMS` reader - Link Bandwidth Management Status \\[LBMS\\]\n\nThis bit is Set by hardware to\n\nindicate that either link training has\n\ncompleted following write to retrain\n\nlink bit, or when HW has changed\n\nlink speed or width to attempt to\n\ncorrect unreliable link operation.\n\nThis triggers an interrupt to be\n\ngenerated through\n\nPHY_INTERRUPT_OUT if enabled.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
pub type LbmsR = crate::BitReader;
#[doc = "Field `LBMS` writer - Link Bandwidth Management Status \\[LBMS\\]\n\nThis bit is Set by hardware to\n\nindicate that either link training has\n\ncompleted following write to retrain\n\nlink bit, or when HW has changed\n\nlink speed or width to attempt to\n\ncorrect unreliable link operation.\n\nThis triggers an interrupt to be\n\ngenerated through\n\nPHY_INTERRUPT_OUT if enabled.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
pub type LbmsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LABS` reader - Link Autonomous Bandwidth Status \\[LABS\\]\n\nThis bit is Set by hardware to\n\nindicate that hardware has\n\nautonomously changed Link speed\n\nor width, without the Port\n\ntransitioning through DL_Down\n\nstatus, for reasons other than to\n\nattempt to correct unreliable Link\n\noperation. This triggers an interrupt\n\nto be generated through\n\nPHY_INTERRUPT_OUT if enabled.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
pub type LabsR = crate::BitReader;
#[doc = "Field `LABS` writer - Link Autonomous Bandwidth Status \\[LABS\\]\n\nThis bit is Set by hardware to\n\nindicate that hardware has\n\nautonomously changed Link speed\n\nor width, without the Port\n\ntransitioning through DL_Down\n\nstatus, for reasons other than to\n\nattempt to correct unreliable Link\n\noperation. This triggers an interrupt\n\nto be generated through\n\nPHY_INTERRUPT_OUT if enabled.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
pub type LabsW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Active State Power Management Control \\[ASPMC\\]\n\nControls the level of ASPM support\n\non the PCI Express link associated\n\nwith the function. The valid setting\n\nare 00: ASPM disabled 01: L0s entry\n\nenabled, L1 disabled 10: L1 entry\n\nenabled, L0s disabled 11: Both L0s\n\nand L1 enabled. Note that these\n\nControl bits can be enabled only if\n\nthe corresponding ASPM Support bit\n\nis 1."]
    #[inline(always)]
    pub fn aspmc(&self) -> AspmcR {
        AspmcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved \\[R10\\]\n\nReserved"]
    #[inline(always)]
    pub fn r10(&self) -> R10R {
        R10R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read Completion Boundary \\[RCB\\]\n\nIndicates the Read Completion\n\nBoundary of the Root Port (0 = 64\n\nbytes, 1 = 128 bytes). This field can\n\nbe written from the APB bus by\n\nsetting \\[21\\]
bit high of the\n\npcie_mgmt_APB_ADDR during a\n\nlocal management register write."]
    #[inline(always)]
    pub fn rcb(&self) -> RcbR {
        RcbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Link Disable \\[LD\\]\n\nWriting a 1 to this bit position\n\ncauses the LTSSM to go to the\n\nDisable Link state. The LTSSM stays\n\nin the Disable Link state while this\n\nbit is set."]
    #[inline(always)]
    pub fn ld(&self) -> LdR {
        LdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Retrain Link \\[RL\\]\n\nSetting this bit to 1 causes the\n\nLTSSM to initiate link training. This\n\nbit always reads as 0."]
    #[inline(always)]
    pub fn rl(&self) -> RlR {
        RlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Common Clock Configuration \\[CCC\\]\n\nA value of 0 indicates that the\n\nreference clock of this device is\n\nasynchronous to that of the\n\nupstream device. A value of 1\n\nindicates that the reference clock is\n\ncommon."]
    #[inline(always)]
    pub fn ccc(&self) -> CccR {
        CccR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Extended Synch \\[ES\\]\n\nSet to 1 to extend the sequence of\n\nordered sets transmitted while\n\nexiting from the L0S state."]
    #[inline(always)]
    pub fn es(&self) -> EsR {
        EsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Clock Power Management \\[ECPM\\]\n\nThis field is hardwired to 0 when the\n\ncore is in the RC mode."]
    #[inline(always)]
    pub fn ecpm(&self) -> EcpmR {
        EcpmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved \\[R9\\]\n\nReserved"]
    #[inline(always)]
    pub fn r9(&self) -> R9R {
        R9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Link Bandwidth Management Interrupt Enable \\[LBMIE\\]\n\nWhen Set, this bit enables the\n\ngeneration of an interrupt to\n\nindicate that the Link Bandwidth\n\nManagement Status bit has been\n\nSet. This enables an interrupt to be\n\ngenerated through\n\nPHY_INTERRUPT_OUT if triggered.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
    #[inline(always)]
    pub fn lbmie(&self) -> LbmieR {
        LbmieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Link Autonomous Bandwidth Interrupt Enable \\[LABIE\\]\n\nWhen Set, this bit enables the\n\ngeneration of an interrupt to\n\nindicate that the Link Autonomous\n\nBandwidth Status bit has been Set.\n\nThis enables an interrupt to be\n\ngenerated through\n\nPHY_INTERRUPT_OUT if triggered.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
    #[inline(always)]
    pub fn labie(&self) -> LabieR {
        LabieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Reserved \\[R11\\]\n\nReserved"]
    #[inline(always)]
    pub fn r11(&self) -> R11R {
        R11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Negotiated Link Speed \\[NLS\\]\n\nNegotiated link speed of the device.\n\nThe only supported speed ids are\n\n2.5 GT/s per lane (0001),5 GT/s per\n\nlane (0010) ."]
    #[inline(always)]
    pub fn nls(&self) -> NlsR {
        NlsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:25 - Negotiated Link Width \\[NLW\\]\n\nSet at the end of link training to the\n\nactual link width negotiated between\n\nthe two sides."]
    #[inline(always)]
    pub fn nlw(&self) -> NlwR {
        NlwR::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - Reserved \\[R12\\]\n\nReserved"]
    #[inline(always)]
    pub fn r12(&self) -> R12R {
        R12R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Link Training Status \\[LTS\\]\n\nThis bit is set to 1 when the LTSSM\n\nis in the Recovery or Configuration\n\nstates, or if a 1 has been written to\n\nthe Retrain Link bit but the link\n\ntraining has not yet begun."]
    #[inline(always)]
    pub fn lts(&self) -> LtsR {
        LtsR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Slot Clock Configuration \\[SCC\\]\n\nIndicates that the device uses the\n\nreference clock provided by the\n\nconnector. This field can be written\n\nfrom the APB bus by setting \\[21\\]
bit\n\nhigh of the pcie_mgmt_APB_ADDR\n\nduring a local management register\n\nwrite."]
    #[inline(always)]
    pub fn scc(&self) -> SccR {
        SccR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Data Link Layer Active \\[DA\\]\n\nIndicates the status of the Data Link\n\nLayer. Set to 1 when the DL\n\nControl and Management State\n\nMachine has reached the DL_Active\n\nstate. This bit is hardwired to 0 in\n\nthis version of the core."]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Link Bandwidth Management Status \\[LBMS\\]\n\nThis bit is Set by hardware to\n\nindicate that either link training has\n\ncompleted following write to retrain\n\nlink bit, or when HW has changed\n\nlink speed or width to attempt to\n\ncorrect unreliable link operation.\n\nThis triggers an interrupt to be\n\ngenerated through\n\nPHY_INTERRUPT_OUT if enabled.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
    #[inline(always)]
    pub fn lbms(&self) -> LbmsR {
        LbmsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Link Autonomous Bandwidth Status \\[LABS\\]\n\nThis bit is Set by hardware to\n\nindicate that hardware has\n\nautonomously changed Link speed\n\nor width, without the Port\n\ntransitioning through DL_Down\n\nstatus, for reasons other than to\n\nattempt to correct unreliable Link\n\noperation. This triggers an interrupt\n\nto be generated through\n\nPHY_INTERRUPT_OUT if enabled.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
    #[inline(always)]
    pub fn labs(&self) -> LabsR {
        LabsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Active State Power Management Control \\[ASPMC\\]\n\nControls the level of ASPM support\n\non the PCI Express link associated\n\nwith the function. The valid setting\n\nare 00: ASPM disabled 01: L0s entry\n\nenabled, L1 disabled 10: L1 entry\n\nenabled, L0s disabled 11: Both L0s\n\nand L1 enabled. Note that these\n\nControl bits can be enabled only if\n\nthe corresponding ASPM Support bit\n\nis 1."]
    #[inline(always)]
    #[must_use]
    pub fn aspmc(&mut self) -> AspmcW<PcieRcLinkControlAndStatusSpec> {
        AspmcW::new(self, 0)
    }
    #[doc = "Bit 4 - Link Disable \\[LD\\]\n\nWriting a 1 to this bit position\n\ncauses the LTSSM to go to the\n\nDisable Link state. The LTSSM stays\n\nin the Disable Link state while this\n\nbit is set."]
    #[inline(always)]
    #[must_use]
    pub fn ld(&mut self) -> LdW<PcieRcLinkControlAndStatusSpec> {
        LdW::new(self, 4)
    }
    #[doc = "Bit 6 - Common Clock Configuration \\[CCC\\]\n\nA value of 0 indicates that the\n\nreference clock of this device is\n\nasynchronous to that of the\n\nupstream device. A value of 1\n\nindicates that the reference clock is\n\ncommon."]
    #[inline(always)]
    #[must_use]
    pub fn ccc(&mut self) -> CccW<PcieRcLinkControlAndStatusSpec> {
        CccW::new(self, 6)
    }
    #[doc = "Bit 7 - Extended Synch \\[ES\\]\n\nSet to 1 to extend the sequence of\n\nordered sets transmitted while\n\nexiting from the L0S state."]
    #[inline(always)]
    #[must_use]
    pub fn es(&mut self) -> EsW<PcieRcLinkControlAndStatusSpec> {
        EsW::new(self, 7)
    }
    #[doc = "Bit 10 - Link Bandwidth Management Interrupt Enable \\[LBMIE\\]\n\nWhen Set, this bit enables the\n\ngeneration of an interrupt to\n\nindicate that the Link Bandwidth\n\nManagement Status bit has been\n\nSet. This enables an interrupt to be\n\ngenerated through\n\nPHY_INTERRUPT_OUT if triggered.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lbmie(&mut self) -> LbmieW<PcieRcLinkControlAndStatusSpec> {
        LbmieW::new(self, 10)
    }
    #[doc = "Bit 11 - Link Autonomous Bandwidth Interrupt Enable \\[LABIE\\]\n\nWhen Set, this bit enables the\n\ngeneration of an interrupt to\n\nindicate that the Link Autonomous\n\nBandwidth Status bit has been Set.\n\nThis enables an interrupt to be\n\ngenerated through\n\nPHY_INTERRUPT_OUT if triggered.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
    #[inline(always)]
    #[must_use]
    pub fn labie(&mut self) -> LabieW<PcieRcLinkControlAndStatusSpec> {
        LabieW::new(self, 11)
    }
    #[doc = "Bit 30 - Link Bandwidth Management Status \\[LBMS\\]\n\nThis bit is Set by hardware to\n\nindicate that either link training has\n\ncompleted following write to retrain\n\nlink bit, or when HW has changed\n\nlink speed or width to attempt to\n\ncorrect unreliable link operation.\n\nThis triggers an interrupt to be\n\ngenerated through\n\nPHY_INTERRUPT_OUT if enabled.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lbms(&mut self) -> LbmsW<PcieRcLinkControlAndStatusSpec> {
        LbmsW::new(self, 30)
    }
    #[doc = "Bit 31 - Link Autonomous Bandwidth Status \\[LABS\\]\n\nThis bit is Set by hardware to\n\nindicate that hardware has\n\nautonomously changed Link speed\n\nor width, without the Port\n\ntransitioning through DL_Down\n\nstatus, for reasons other than to\n\nattempt to correct unreliable Link\n\noperation. This triggers an interrupt\n\nto be generated through\n\nPHY_INTERRUPT_OUT if enabled.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0."]
    #[inline(always)]
    #[must_use]
    pub fn labs(&mut self) -> LabsW<PcieRcLinkControlAndStatusSpec> {
        LabsW::new(self, 31)
    }
}
#[doc = "Link Control and Status Register\n\nThis bit is Set by hardware to\n\nindicate that hardware has\n\nautonomously changed Link speed\n\nor width, without the Port\n\ntransitioning through DL_Down\n\nstatus, for reasons other than to\n\nattempt to correct unreliable Link\n\noperation. This triggers an interrupt\n\nto be generated through\n\nPHY_INTERRUPT_OUT if enabled.\n\nHardwired to 0 if Link Bandwidth\n\nNotification Capability is 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_link_control_and_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_link_control_and_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcLinkControlAndStatusSpec;
impl crate::RegisterSpec for PcieRcLinkControlAndStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_link_control_and_status::R`](R) reader structure"]
impl crate::Readable for PcieRcLinkControlAndStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_link_control_and_status::W`](W) writer structure"]
impl crate::Writable for PcieRcLinkControlAndStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xc000_0000;
}
#[doc = "`reset()` method sets PCIE_RC_LINK_CONTROL_AND_STATUS to value 0x0041_0000"]
impl crate::Resettable for PcieRcLinkControlAndStatusSpec {
    const RESET_VALUE: u32 = 0x0041_0000;
}
