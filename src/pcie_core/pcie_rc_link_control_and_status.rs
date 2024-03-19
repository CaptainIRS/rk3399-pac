#[doc = "Register `PCIE_RC_LINK_CONTROL_AND_STATUS` reader"]
pub type R = crate::R<PcieRcLinkControlAndStatusSpec>;
#[doc = "Register `PCIE_RC_LINK_CONTROL_AND_STATUS` writer"]
pub type W = crate::W<PcieRcLinkControlAndStatusSpec>;
#[doc = "Field `ASPMC` reader - Active State Power Management Control \\[ASPMC\\]
Controls the level of ASPM support on the PCI Express link associated with the function. The valid setting are 00: ASPM disabled 01: L0s entry enabled, L1 disabled 10: L1 entry enabled, L0s disabled 11: Both L0s and L1 enabled. Note that these Control bits can be enabled only if the corresponding ASPM Support bit is 1."]
pub type AspmcR = crate::FieldReader;
#[doc = "Field `ASPMC` writer - Active State Power Management Control \\[ASPMC\\]
Controls the level of ASPM support on the PCI Express link associated with the function. The valid setting are 00: ASPM disabled 01: L0s entry enabled, L1 disabled 10: L1 entry enabled, L0s disabled 11: Both L0s and L1 enabled. Note that these Control bits can be enabled only if the corresponding ASPM Support bit is 1."]
pub type AspmcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `R10` reader - Reserved \\[R10\\]
Reserved"]
pub type R10R = crate::BitReader;
#[doc = "Field `RCB` reader - Read Completion Boundary \\[RCB\\]
Indicates the Read Completion Boundary of the Root Port (0 = 64 bytes, 1 = 128 bytes). This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type RcbR = crate::BitReader;
#[doc = "Field `LD` reader - Link Disable \\[LD\\]
Writing a 1 to this bit position causes the LTSSM to go to the Disable Link state. The LTSSM stays in the Disable Link state while this bit is set."]
pub type LdR = crate::BitReader;
#[doc = "Field `LD` writer - Link Disable \\[LD\\]
Writing a 1 to this bit position causes the LTSSM to go to the Disable Link state. The LTSSM stays in the Disable Link state while this bit is set."]
pub type LdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RL` reader - Retrain Link \\[RL\\]
Setting this bit to 1 causes the LTSSM to initiate link training. This bit always reads as 0."]
pub type RlR = crate::BitReader;
#[doc = "Field `CCC` reader - Common Clock Configuration \\[CCC\\]
A value of 0 indicates that the reference clock of this device is asynchronous to that of the upstream device. A value of 1 indicates that the reference clock is common."]
pub type CccR = crate::BitReader;
#[doc = "Field `CCC` writer - Common Clock Configuration \\[CCC\\]
A value of 0 indicates that the reference clock of this device is asynchronous to that of the upstream device. A value of 1 indicates that the reference clock is common."]
pub type CccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ES` reader - Extended Synch \\[ES\\]
Set to 1 to extend the sequence of ordered sets transmitted while exiting from the L0S state."]
pub type EsR = crate::BitReader;
#[doc = "Field `ES` writer - Extended Synch \\[ES\\]
Set to 1 to extend the sequence of ordered sets transmitted while exiting from the L0S state."]
pub type EsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECPM` reader - Enable Clock Power Management \\[ECPM\\]
This field is hardwired to 0 when the core is in the RC mode."]
pub type EcpmR = crate::BitReader;
#[doc = "Field `R9` reader - Reserved \\[R9\\]
Reserved"]
pub type R9R = crate::BitReader;
#[doc = "Field `LBMIE` reader - Link Bandwidth Management Interrupt Enable \\[LBMIE\\]
When Set, this bit enables the generation of an interrupt to indicate that the Link Bandwidth Management Status bit has been Set. This enables an interrupt to be generated through PHY_INTERRUPT_OUT if triggered. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
pub type LbmieR = crate::BitReader;
#[doc = "Field `LBMIE` writer - Link Bandwidth Management Interrupt Enable \\[LBMIE\\]
When Set, this bit enables the generation of an interrupt to indicate that the Link Bandwidth Management Status bit has been Set. This enables an interrupt to be generated through PHY_INTERRUPT_OUT if triggered. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
pub type LbmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LABIE` reader - Link Autonomous Bandwidth Interrupt Enable \\[LABIE\\]
When Set, this bit enables the generation of an interrupt to indicate that the Link Autonomous Bandwidth Status bit has been Set. This enables an interrupt to be generated through PHY_INTERRUPT_OUT if triggered. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
pub type LabieR = crate::BitReader;
#[doc = "Field `LABIE` writer - Link Autonomous Bandwidth Interrupt Enable \\[LABIE\\]
When Set, this bit enables the generation of an interrupt to indicate that the Link Autonomous Bandwidth Status bit has been Set. This enables an interrupt to be generated through PHY_INTERRUPT_OUT if triggered. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
pub type LabieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R11` reader - Reserved \\[R11\\]
Reserved"]
pub type R11R = crate::FieldReader;
#[doc = "Field `NLS` reader - Negotiated Link Speed \\[NLS\\]
Negotiated link speed of the device. The only supported speed ids are 2.5 GT/s per lane (0001),5 GT/s per lane (0010) ."]
pub type NlsR = crate::FieldReader;
#[doc = "Field `NLW` reader - Negotiated Link Width \\[NLW\\]
Set at the end of link training to the actual link width negotiated between the two sides."]
pub type NlwR = crate::FieldReader;
#[doc = "Field `R12` reader - Reserved \\[R12\\]
Reserved"]
pub type R12R = crate::BitReader;
#[doc = "Field `LTS` reader - Link Training Status \\[LTS\\]
This bit is set to 1 when the LTSSM is in the Recovery or Configuration states, or if a 1 has been written to the Retrain Link bit but the link training has not yet begun."]
pub type LtsR = crate::BitReader;
#[doc = "Field `SCC` reader - Slot Clock Configuration \\[SCC\\]
Indicates that the device uses the reference clock provided by the connector. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type SccR = crate::BitReader;
#[doc = "Field `DA` reader - Data Link Layer Active \\[DA\\]
Indicates the status of the Data Link Layer. Set to 1 when the DL Control and Management State Machine has reached the DL_Active state. This bit is hardwired to 0 in this version of the core."]
pub type DaR = crate::BitReader;
#[doc = "Field `LBMS` reader - Link Bandwidth Management Status \\[LBMS\\]
This bit is Set by hardware to indicate that either link training has completed following write to retrain link bit, or when HW has changed link speed or width to attempt to correct unreliable link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
pub type LbmsR = crate::BitReader;
#[doc = "Field `LBMS` writer - Link Bandwidth Management Status \\[LBMS\\]
This bit is Set by hardware to indicate that either link training has completed following write to retrain link bit, or when HW has changed link speed or width to attempt to correct unreliable link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
pub type LbmsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LABS` reader - Link Autonomous Bandwidth Status \\[LABS\\]
This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
pub type LabsR = crate::BitReader;
#[doc = "Field `LABS` writer - Link Autonomous Bandwidth Status \\[LABS\\]
This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
pub type LabsW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Active State Power Management Control \\[ASPMC\\]
Controls the level of ASPM support on the PCI Express link associated with the function. The valid setting are 00: ASPM disabled 01: L0s entry enabled, L1 disabled 10: L1 entry enabled, L0s disabled 11: Both L0s and L1 enabled. Note that these Control bits can be enabled only if the corresponding ASPM Support bit is 1."]
    #[inline(always)]
    pub fn aspmc(&self) -> AspmcR {
        AspmcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved \\[R10\\]
Reserved"]
    #[inline(always)]
    pub fn r10(&self) -> R10R {
        R10R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read Completion Boundary \\[RCB\\]
Indicates the Read Completion Boundary of the Root Port (0 = 64 bytes, 1 = 128 bytes). This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn rcb(&self) -> RcbR {
        RcbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Link Disable \\[LD\\]
Writing a 1 to this bit position causes the LTSSM to go to the Disable Link state. The LTSSM stays in the Disable Link state while this bit is set."]
    #[inline(always)]
    pub fn ld(&self) -> LdR {
        LdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Retrain Link \\[RL\\]
Setting this bit to 1 causes the LTSSM to initiate link training. This bit always reads as 0."]
    #[inline(always)]
    pub fn rl(&self) -> RlR {
        RlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Common Clock Configuration \\[CCC\\]
A value of 0 indicates that the reference clock of this device is asynchronous to that of the upstream device. A value of 1 indicates that the reference clock is common."]
    #[inline(always)]
    pub fn ccc(&self) -> CccR {
        CccR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Extended Synch \\[ES\\]
Set to 1 to extend the sequence of ordered sets transmitted while exiting from the L0S state."]
    #[inline(always)]
    pub fn es(&self) -> EsR {
        EsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable Clock Power Management \\[ECPM\\]
This field is hardwired to 0 when the core is in the RC mode."]
    #[inline(always)]
    pub fn ecpm(&self) -> EcpmR {
        EcpmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved \\[R9\\]
Reserved"]
    #[inline(always)]
    pub fn r9(&self) -> R9R {
        R9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Link Bandwidth Management Interrupt Enable \\[LBMIE\\]
When Set, this bit enables the generation of an interrupt to indicate that the Link Bandwidth Management Status bit has been Set. This enables an interrupt to be generated through PHY_INTERRUPT_OUT if triggered. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
    #[inline(always)]
    pub fn lbmie(&self) -> LbmieR {
        LbmieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Link Autonomous Bandwidth Interrupt Enable \\[LABIE\\]
When Set, this bit enables the generation of an interrupt to indicate that the Link Autonomous Bandwidth Status bit has been Set. This enables an interrupt to be generated through PHY_INTERRUPT_OUT if triggered. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
    #[inline(always)]
    pub fn labie(&self) -> LabieR {
        LabieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Reserved \\[R11\\]
Reserved"]
    #[inline(always)]
    pub fn r11(&self) -> R11R {
        R11R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Negotiated Link Speed \\[NLS\\]
Negotiated link speed of the device. The only supported speed ids are 2.5 GT/s per lane (0001),5 GT/s per lane (0010) ."]
    #[inline(always)]
    pub fn nls(&self) -> NlsR {
        NlsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:25 - Negotiated Link Width \\[NLW\\]
Set at the end of link training to the actual link width negotiated between the two sides."]
    #[inline(always)]
    pub fn nlw(&self) -> NlwR {
        NlwR::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - Reserved \\[R12\\]
Reserved"]
    #[inline(always)]
    pub fn r12(&self) -> R12R {
        R12R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Link Training Status \\[LTS\\]
This bit is set to 1 when the LTSSM is in the Recovery or Configuration states, or if a 1 has been written to the Retrain Link bit but the link training has not yet begun."]
    #[inline(always)]
    pub fn lts(&self) -> LtsR {
        LtsR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Slot Clock Configuration \\[SCC\\]
Indicates that the device uses the reference clock provided by the connector. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn scc(&self) -> SccR {
        SccR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Data Link Layer Active \\[DA\\]
Indicates the status of the Data Link Layer. Set to 1 when the DL Control and Management State Machine has reached the DL_Active state. This bit is hardwired to 0 in this version of the core."]
    #[inline(always)]
    pub fn da(&self) -> DaR {
        DaR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Link Bandwidth Management Status \\[LBMS\\]
This bit is Set by hardware to indicate that either link training has completed following write to retrain link bit, or when HW has changed link speed or width to attempt to correct unreliable link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
    #[inline(always)]
    pub fn lbms(&self) -> LbmsR {
        LbmsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Link Autonomous Bandwidth Status \\[LABS\\]
This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
    #[inline(always)]
    pub fn labs(&self) -> LabsR {
        LabsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Active State Power Management Control \\[ASPMC\\]
Controls the level of ASPM support on the PCI Express link associated with the function. The valid setting are 00: ASPM disabled 01: L0s entry enabled, L1 disabled 10: L1 entry enabled, L0s disabled 11: Both L0s and L1 enabled. Note that these Control bits can be enabled only if the corresponding ASPM Support bit is 1."]
    #[inline(always)]
    #[must_use]
    pub fn aspmc(&mut self) -> AspmcW<PcieRcLinkControlAndStatusSpec> {
        AspmcW::new(self, 0)
    }
    #[doc = "Bit 4 - Link Disable \\[LD\\]
Writing a 1 to this bit position causes the LTSSM to go to the Disable Link state. The LTSSM stays in the Disable Link state while this bit is set."]
    #[inline(always)]
    #[must_use]
    pub fn ld(&mut self) -> LdW<PcieRcLinkControlAndStatusSpec> {
        LdW::new(self, 4)
    }
    #[doc = "Bit 6 - Common Clock Configuration \\[CCC\\]
A value of 0 indicates that the reference clock of this device is asynchronous to that of the upstream device. A value of 1 indicates that the reference clock is common."]
    #[inline(always)]
    #[must_use]
    pub fn ccc(&mut self) -> CccW<PcieRcLinkControlAndStatusSpec> {
        CccW::new(self, 6)
    }
    #[doc = "Bit 7 - Extended Synch \\[ES\\]
Set to 1 to extend the sequence of ordered sets transmitted while exiting from the L0S state."]
    #[inline(always)]
    #[must_use]
    pub fn es(&mut self) -> EsW<PcieRcLinkControlAndStatusSpec> {
        EsW::new(self, 7)
    }
    #[doc = "Bit 10 - Link Bandwidth Management Interrupt Enable \\[LBMIE\\]
When Set, this bit enables the generation of an interrupt to indicate that the Link Bandwidth Management Status bit has been Set. This enables an interrupt to be generated through PHY_INTERRUPT_OUT if triggered. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lbmie(&mut self) -> LbmieW<PcieRcLinkControlAndStatusSpec> {
        LbmieW::new(self, 10)
    }
    #[doc = "Bit 11 - Link Autonomous Bandwidth Interrupt Enable \\[LABIE\\]
When Set, this bit enables the generation of an interrupt to indicate that the Link Autonomous Bandwidth Status bit has been Set. This enables an interrupt to be generated through PHY_INTERRUPT_OUT if triggered. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
    #[inline(always)]
    #[must_use]
    pub fn labie(&mut self) -> LabieW<PcieRcLinkControlAndStatusSpec> {
        LabieW::new(self, 11)
    }
    #[doc = "Bit 30 - Link Bandwidth Management Status \\[LBMS\\]
This bit is Set by hardware to indicate that either link training has completed following write to retrain link bit, or when HW has changed link speed or width to attempt to correct unreliable link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
    #[inline(always)]
    #[must_use]
    pub fn lbms(&mut self) -> LbmsW<PcieRcLinkControlAndStatusSpec> {
        LbmsW::new(self, 30)
    }
    #[doc = "Bit 31 - Link Autonomous Bandwidth Status \\[LABS\\]
This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0."]
    #[inline(always)]
    #[must_use]
    pub fn labs(&mut self) -> LabsW<PcieRcLinkControlAndStatusSpec> {
        LabsW::new(self, 31)
    }
}
#[doc = "Link Control and Status Register This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_link_control_and_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_link_control_and_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
