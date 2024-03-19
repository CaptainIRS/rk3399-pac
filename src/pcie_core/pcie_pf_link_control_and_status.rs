#[doc = "Register `PCIE_PF_LINK_CONTROL_AND_STATUS` reader"]
pub type R = crate::R<PciePfLinkControlAndStatusSpec>;
#[doc = "Register `PCIE_PF_LINK_CONTROL_AND_STATUS` writer"]
pub type W = crate::W<PciePfLinkControlAndStatusSpec>;
#[doc = "Field `ASPMC` reader - Active State Power Management Control \\[ASPMC\\]
Controls the level of ASPM support on the PCI Express link associated with this Function. The valid setting are 00: ASPM disabled 01: L0s entry enabled, L1 disabled 10: L1 entry enabled, L0s disabled 11: Both L0s and L1 enabled."]
pub type AspmcR = crate::FieldReader;
#[doc = "Field `ASPMC` writer - Active State Power Management Control \\[ASPMC\\]
Controls the level of ASPM support on the PCI Express link associated with this Function. The valid setting are 00: ASPM disabled 01: L0s entry enabled, L1 disabled 10: L1 entry enabled, L0s disabled 11: Both L0s and L1 enabled."]
pub type AspmcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `R6` reader - Reserved \\[R6\\]
Reserved"]
pub type R6R = crate::BitReader;
#[doc = "Field `RCB` reader - Read Completion Boundary \\[RCB\\]
Indicates the Read Completion Boundary of the Root Port connected to this Endpoint (0 = 64 bytes, 1 = 128 bytes). This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type RcbR = crate::BitReader;
#[doc = "Field `RCB` writer - Read Completion Boundary \\[RCB\\]
Indicates the Read Completion Boundary of the Root Port connected to this Endpoint (0 = 64 bytes, 1 = 128 bytes). This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type RcbW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LD` reader - Link Disable \\[LD\\]
Writing a 1 to this bit position causes the LTSSM to go to the Disable Link state. The LTSSM stays in the Disable Link state while this bit is set. Reserved for Endpoint mode."]
pub type LdR = crate::BitReader;
#[doc = "Field `RL` reader - Retrain Link \\[RL\\]
Setting this bit to 1 causes the LTSSM to initiate link training. Reserved for Endpoint mode. This bit always reads as 0"]
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
When this bit is set to 1, the device may use the CLKREQ# pin on the PCIe connector to power manage the Link clock. This bit is writeable only when the Clock Power Management bit in the Link Capability Register is set to 1."]
pub type EcpmR = crate::BitReader;
#[doc = "Field `R9` reader - Reserved \\[R9\\]
Reserved"]
pub type R9R = crate::BitReader;
#[doc = "Field `LBMIE` reader - Link Bandwidth Management Interrupt Enable \\[LBMIE\\]
When Set, this bit enables the generation of an interrupt to indicate that the Link Bandwidth Management Status bit has been Set. This enables an interrupt to be generated through PHY INTERRUPT_OUT if triggered. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
pub type LbmieR = crate::BitReader;
#[doc = "Field `LABIE` reader - Link Autonomous Bandwidth Interrupt Enable \\[LABIE\\]
When Set, this bit enables the generation of an interrupt to indicate that the Link Autonomous Bandwidth Status bit has been Set. This enables an interrupt to be generated through PHY_INTERRUPT_OUT if triggered. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
pub type LabieR = crate::BitReader;
#[doc = "Field `R15_12` reader - Reserved \\[R15_12\\]
Reserved"]
pub type R15_12R = crate::FieldReader;
#[doc = "Field `NLS` reader - Negotiated Link Speed \\[NLS\\]
Negotiated link speed of the device. The only supported speed ids are 2.5 GT/s per lane (0001),5 GT/s per lane (0010)."]
pub type NlsR = crate::FieldReader;
#[doc = "Field `NLW` reader - Negotiated Link Width \\[NLW\\]
Set at the end of link training to the actual link width negotiated between the two sides. Value is undefined if this register is accessed before link training."]
pub type NlwR = crate::FieldReader;
#[doc = "Field `R8` reader - Reserved \\[R8\\]
Reserved"]
pub type R8R = crate::BitReader;
#[doc = "Field `LTS` reader - Link Training Status \\[LTS\\]
This read-only bit indicates that the Physical Layer LTSSM is in the Configuration or Recovery state, or that 1b was written to the Retrain Link bit but Link training has not yet begun. Hardware clears this bit when the LTSSM exits the Configuration/ Recovery state. Not applicable to Endpoints where field is hardwired to 0."]
pub type LtsR = crate::BitReader;
#[doc = "Field `SCC` reader - Slot Clock Configuration \\[SCC\\]
Indicates that the device uses the reference clock provided by the connector."]
pub type SccR = crate::BitReader;
#[doc = "Field `DLLA` reader - Data Link Layer Active \\[DLLA\\]
Indicates the status of the Data Link Layer. Set to 1 when the DL Control and Management State Machine has reached the DL_Active state. This bit is hardwired to 0 in this version of the core."]
pub type DllaR = crate::BitReader;
#[doc = "Field `LBMS` reader - Link Bandwidth Management Status \\[LBMS\\]
This bit is Set by hardware to indicate that either link training has completed following write to retrain link bit, or when HW has changed link speed or width to attempt to correct unreliable link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
pub type LbmsR = crate::BitReader;
#[doc = "Field `LBMS` writer - Link Bandwidth Management Status \\[LBMS\\]
This bit is Set by hardware to indicate that either link training has completed following write to retrain link bit, or when HW has changed link speed or width to attempt to correct unreliable link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
pub type LbmsW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `LABS` reader - Link Autonomous Bandwidth Status \\[LABS\\]
This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
pub type LabsR = crate::BitReader;
#[doc = "Field `LABS` writer - Link Autonomous Bandwidth Status \\[LABS\\]
This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
pub type LabsW<'a, REG> = crate::BitWriter1C<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Active State Power Management Control \\[ASPMC\\]
Controls the level of ASPM support on the PCI Express link associated with this Function. The valid setting are 00: ASPM disabled 01: L0s entry enabled, L1 disabled 10: L1 entry enabled, L0s disabled 11: Both L0s and L1 enabled."]
    #[inline(always)]
    pub fn aspmc(&self) -> AspmcR {
        AspmcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved \\[R6\\]
Reserved"]
    #[inline(always)]
    pub fn r6(&self) -> R6R {
        R6R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Read Completion Boundary \\[RCB\\]
Indicates the Read Completion Boundary of the Root Port connected to this Endpoint (0 = 64 bytes, 1 = 128 bytes). This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    pub fn rcb(&self) -> RcbR {
        RcbR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Link Disable \\[LD\\]
Writing a 1 to this bit position causes the LTSSM to go to the Disable Link state. The LTSSM stays in the Disable Link state while this bit is set. Reserved for Endpoint mode."]
    #[inline(always)]
    pub fn ld(&self) -> LdR {
        LdR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Retrain Link \\[RL\\]
Setting this bit to 1 causes the LTSSM to initiate link training. Reserved for Endpoint mode. This bit always reads as 0"]
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
When this bit is set to 1, the device may use the CLKREQ# pin on the PCIe connector to power manage the Link clock. This bit is writeable only when the Clock Power Management bit in the Link Capability Register is set to 1."]
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
When Set, this bit enables the generation of an interrupt to indicate that the Link Bandwidth Management Status bit has been Set. This enables an interrupt to be generated through PHY INTERRUPT_OUT if triggered. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
    #[inline(always)]
    pub fn lbmie(&self) -> LbmieR {
        LbmieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Link Autonomous Bandwidth Interrupt Enable \\[LABIE\\]
When Set, this bit enables the generation of an interrupt to indicate that the Link Autonomous Bandwidth Status bit has been Set. This enables an interrupt to be generated through PHY_INTERRUPT_OUT if triggered. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
    #[inline(always)]
    pub fn labie(&self) -> LabieR {
        LabieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Reserved \\[R15_12\\]
Reserved"]
    #[inline(always)]
    pub fn r15_12(&self) -> R15_12R {
        R15_12R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Negotiated Link Speed \\[NLS\\]
Negotiated link speed of the device. The only supported speed ids are 2.5 GT/s per lane (0001),5 GT/s per lane (0010)."]
    #[inline(always)]
    pub fn nls(&self) -> NlsR {
        NlsR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:25 - Negotiated Link Width \\[NLW\\]
Set at the end of link training to the actual link width negotiated between the two sides. Value is undefined if this register is accessed before link training."]
    #[inline(always)]
    pub fn nlw(&self) -> NlwR {
        NlwR::new(((self.bits >> 20) & 0x3f) as u8)
    }
    #[doc = "Bit 26 - Reserved \\[R8\\]
Reserved"]
    #[inline(always)]
    pub fn r8(&self) -> R8R {
        R8R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Link Training Status \\[LTS\\]
This read-only bit indicates that the Physical Layer LTSSM is in the Configuration or Recovery state, or that 1b was written to the Retrain Link bit but Link training has not yet begun. Hardware clears this bit when the LTSSM exits the Configuration/ Recovery state. Not applicable to Endpoints where field is hardwired to 0."]
    #[inline(always)]
    pub fn lts(&self) -> LtsR {
        LtsR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Slot Clock Configuration \\[SCC\\]
Indicates that the device uses the reference clock provided by the connector."]
    #[inline(always)]
    pub fn scc(&self) -> SccR {
        SccR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Data Link Layer Active \\[DLLA\\]
Indicates the status of the Data Link Layer. Set to 1 when the DL Control and Management State Machine has reached the DL_Active state. This bit is hardwired to 0 in this version of the core."]
    #[inline(always)]
    pub fn dlla(&self) -> DllaR {
        DllaR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Link Bandwidth Management Status \\[LBMS\\]
This bit is Set by hardware to indicate that either link training has completed following write to retrain link bit, or when HW has changed link speed or width to attempt to correct unreliable link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
    #[inline(always)]
    pub fn lbms(&self) -> LbmsR {
        LbmsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Link Autonomous Bandwidth Status \\[LABS\\]
This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
    #[inline(always)]
    pub fn labs(&self) -> LabsR {
        LabsR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Active State Power Management Control \\[ASPMC\\]
Controls the level of ASPM support on the PCI Express link associated with this Function. The valid setting are 00: ASPM disabled 01: L0s entry enabled, L1 disabled 10: L1 entry enabled, L0s disabled 11: Both L0s and L1 enabled."]
    #[inline(always)]
    #[must_use]
    pub fn aspmc(&mut self) -> AspmcW<PciePfLinkControlAndStatusSpec> {
        AspmcW::new(self, 0)
    }
    #[doc = "Bit 3 - Read Completion Boundary \\[RCB\\]
Indicates the Read Completion Boundary of the Root Port connected to this Endpoint (0 = 64 bytes, 1 = 128 bytes). This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    #[must_use]
    pub fn rcb(&mut self) -> RcbW<PciePfLinkControlAndStatusSpec> {
        RcbW::new(self, 3)
    }
    #[doc = "Bit 6 - Common Clock Configuration \\[CCC\\]
A value of 0 indicates that the reference clock of this device is asynchronous to that of the upstream device. A value of 1 indicates that the reference clock is common."]
    #[inline(always)]
    #[must_use]
    pub fn ccc(&mut self) -> CccW<PciePfLinkControlAndStatusSpec> {
        CccW::new(self, 6)
    }
    #[doc = "Bit 7 - Extended Synch \\[ES\\]
Set to 1 to extend the sequence of ordered sets transmitted while exiting from the L0S state."]
    #[inline(always)]
    #[must_use]
    pub fn es(&mut self) -> EsW<PciePfLinkControlAndStatusSpec> {
        EsW::new(self, 7)
    }
    #[doc = "Bit 30 - Link Bandwidth Management Status \\[LBMS\\]
This bit is Set by hardware to indicate that either link training has completed following write to retrain link bit, or when HW has changed link speed or width to attempt to correct unreliable link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
    #[inline(always)]
    #[must_use]
    pub fn lbms(&mut self) -> LbmsW<PciePfLinkControlAndStatusSpec> {
        LbmsW::new(self, 30)
    }
    #[doc = "Bit 31 - Link Autonomous Bandwidth Status \\[LABS\\]
This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0."]
    #[inline(always)]
    #[must_use]
    pub fn labs(&mut self) -> LabsW<PciePfLinkControlAndStatusSpec> {
        LabsW::new(self, 31)
    }
}
#[doc = "Link Control and Status Register This bit is Set by hardware to indicate that hardware has autonomously changed Link speed or width, without the Port transitioning through DL_Down status, for reasons other than to attempt to correct unreliable Link operation. This triggers an interrupt to be generated through PHY_INTERRUPT_OUT if enabled. Hardwired to 0 if Link Bandwidth Notification Capability is 0. Not applicable to Endpoints where field is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_link_control_and_status::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_link_control_and_status::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfLinkControlAndStatusSpec;
impl crate::RegisterSpec for PciePfLinkControlAndStatusSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_link_control_and_status::R`](R) reader structure"]
impl crate::Readable for PciePfLinkControlAndStatusSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_link_control_and_status::W`](W) writer structure"]
impl crate::Writable for PciePfLinkControlAndStatusSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0xc000_0000;
}
#[doc = "`reset()` method sets PCIE_PF_LINK_CONTROL_AND_STATUS to value 0x0042_0000"]
impl crate::Resettable for PciePfLinkControlAndStatusSpec {
    const RESET_VALUE: u32 = 0x0042_0000;
}
