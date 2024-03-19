#[doc = "Register `PCIE_VF_POWER_MANAGEMENT_CONTROL_STATUS_REPORT` reader"]
pub type R = crate::R<PcieVfPowerManagementControlStatusReportSpec>;
#[doc = "Register `PCIE_VF_POWER_MANAGEMENT_CONTROL_STATUS_REPORT` writer"]
pub type W = crate::W<PcieVfPowerManagementControlStatusReportSpec>;
#[doc = "Field `PS` reader - Power State \\[PS\\]
Indicates the power state this Function is currently in. This field can be read by the software to monitor the current power state, or can be written to cause a transition to a new state. The valid settings are 00 (state D0), 01 (state D1) and 11 (state D3hot). The software should not write any other value into this field. This field can also be written from the local management bus independently for each VF Function."]
pub type PsR = crate::FieldReader;
#[doc = "Field `PS` writer - Power State \\[PS\\]
Indicates the power state this Function is currently in. This field can be read by the software to monitor the current power state, or can be written to cause a transition to a new state. The valid settings are 00 (state D0), 01 (state D1) and 11 (state D3hot). The software should not write any other value into this field. This field can also be written from the local management bus independently for each VF Function."]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `R4` reader - Reserved \\[R4\\]
Reserved"]
pub type R4R = crate::BitReader;
#[doc = "Field `NSR` reader - No Soft Reset \\[NSR\\]
When this bit is set to 1, the Function will maintain all its state in the PM state D3hot. The software is not required to re-initialize the Function registers on the transition back to D0. This bit is set to 1 by default, but can be modified independently for each VF from the local management bus."]
pub type NsrR = crate::BitReader;
#[doc = "Field `R3` reader - Reserved \\[R3\\]
Reserved"]
pub type R3R = crate::FieldReader;
#[doc = "Field `PE` reader - PME Enable \\[PE\\]
Setting this bit enables the notification of PME events from the associated Function. This bit can be set also by writing into this register from the local management bus."]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - PME Enable \\[PE\\]
Setting this bit enables the notification of PME events from the associated Function. This bit can be set also by writing into this register from the local management bus."]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R2` reader - Reserved \\[R2\\]
Reserved"]
pub type R2R = crate::FieldReader;
#[doc = "Field `PMES` reader - PME Status \\[PMES\\]
When PME notification is enabled, writing a 1 into this bit position from the local management bus sets this bit and causes the core to send a PME message from the associated Function. When the Root Complex processes this message, it will turn off this bit by writing a 1 into this bit position though a Config Write. This bit can be set or cleared from the local management bus, by writing a 1 or 0, respectively. It can only be cleared from the configuration path (by writing a 1)."]
pub type PmesR = crate::BitReader;
#[doc = "Field `PMES` writer - PME Status \\[PMES\\]
When PME notification is enabled, writing a 1 into this bit position from the local management bus sets this bit and causes the core to send a PME message from the associated Function. When the Root Complex processes this message, it will turn off this bit by writing a 1 into this bit position though a Config Write. This bit can be set or cleared from the local management bus, by writing a 1 or 0, respectively. It can only be cleared from the configuration path (by writing a 1)."]
pub type PmesW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R1` reader - Reserved \\[R1\\]
Reserved"]
pub type R1R = crate::FieldReader;
#[doc = "Field `DR` reader - Data Register \\[DR\\]
This optional register is not implemented in the PCIe core. This field is hardwired to 0."]
pub type DrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Power State \\[PS\\]
Indicates the power state this Function is currently in. This field can be read by the software to monitor the current power state, or can be written to cause a transition to a new state. The valid settings are 00 (state D0), 01 (state D1) and 11 (state D3hot). The software should not write any other value into this field. This field can also be written from the local management bus independently for each VF Function."]
    #[inline(always)]
    pub fn ps(&self) -> PsR {
        PsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Reserved \\[R4\\]
Reserved"]
    #[inline(always)]
    pub fn r4(&self) -> R4R {
        R4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - No Soft Reset \\[NSR\\]
When this bit is set to 1, the Function will maintain all its state in the PM state D3hot. The software is not required to re-initialize the Function registers on the transition back to D0. This bit is set to 1 by default, but can be modified independently for each VF from the local management bus."]
    #[inline(always)]
    pub fn nsr(&self) -> NsrR {
        NsrR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Reserved \\[R3\\]
Reserved"]
    #[inline(always)]
    pub fn r3(&self) -> R3R {
        R3R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - PME Enable \\[PE\\]
Setting this bit enables the notification of PME events from the associated Function. This bit can be set also by writing into this register from the local management bus."]
    #[inline(always)]
    pub fn pe(&self) -> PeR {
        PeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:14 - Reserved \\[R2\\]
Reserved"]
    #[inline(always)]
    pub fn r2(&self) -> R2R {
        R2R::new(((self.bits >> 9) & 0x3f) as u8)
    }
    #[doc = "Bit 15 - PME Status \\[PMES\\]
When PME notification is enabled, writing a 1 into this bit position from the local management bus sets this bit and causes the core to send a PME message from the associated Function. When the Root Complex processes this message, it will turn off this bit by writing a 1 into this bit position though a Config Write. This bit can be set or cleared from the local management bus, by writing a 1 or 0, respectively. It can only be cleared from the configuration path (by writing a 1)."]
    #[inline(always)]
    pub fn pmes(&self) -> PmesR {
        PmesR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Reserved \\[R1\\]
Reserved"]
    #[inline(always)]
    pub fn r1(&self) -> R1R {
        R1R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Data Register \\[DR\\]
This optional register is not implemented in the PCIe core. This field is hardwired to 0."]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Power State \\[PS\\]
Indicates the power state this Function is currently in. This field can be read by the software to monitor the current power state, or can be written to cause a transition to a new state. The valid settings are 00 (state D0), 01 (state D1) and 11 (state D3hot). The software should not write any other value into this field. This field can also be written from the local management bus independently for each VF Function."]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<PcieVfPowerManagementControlStatusReportSpec> {
        PsW::new(self, 0)
    }
    #[doc = "Bit 8 - PME Enable \\[PE\\]
Setting this bit enables the notification of PME events from the associated Function. This bit can be set also by writing into this register from the local management bus."]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<PcieVfPowerManagementControlStatusReportSpec> {
        PeW::new(self, 8)
    }
    #[doc = "Bit 15 - PME Status \\[PMES\\]
When PME notification is enabled, writing a 1 into this bit position from the local management bus sets this bit and causes the core to send a PME message from the associated Function. When the Root Complex processes this message, it will turn off this bit by writing a 1 into this bit position though a Config Write. This bit can be set or cleared from the local management bus, by writing a 1 or 0, respectively. It can only be cleared from the configuration path (by writing a 1)."]
    #[inline(always)]
    #[must_use]
    pub fn pmes(&mut self) -> PmesW<PcieVfPowerManagementControlStatusReportSpec> {
        PmesW::new(self, 15)
    }
}
#[doc = "Power Management Control/Status Report This optional register is not implemented in the PCIe core. This field is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_vf_power_management_control_status_report::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_vf_power_management_control_status_report::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieVfPowerManagementControlStatusReportSpec;
impl crate::RegisterSpec for PcieVfPowerManagementControlStatusReportSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_vf_power_management_control_status_report::R`](R) reader structure"]
impl crate::Readable for PcieVfPowerManagementControlStatusReportSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_vf_power_management_control_status_report::W`](W) writer structure"]
impl crate::Writable for PcieVfPowerManagementControlStatusReportSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8000;
}
#[doc = "`reset()` method sets PCIE_VF_POWER_MANAGEMENT_CONTROL_STATUS_REPORT to value 0x08"]
impl crate::Resettable for PcieVfPowerManagementControlStatusReportSpec {
    const RESET_VALUE: u32 = 0x08;
}
