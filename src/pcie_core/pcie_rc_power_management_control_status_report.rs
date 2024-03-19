#[doc = "Register `PCIE_RC_POWER_MANAGEMENT_CONTROL_STATUS_REPORT` reader"]
pub type R = crate::R<PcieRcPowerManagementControlStatusReportSpec>;
#[doc = "Register `PCIE_RC_POWER_MANAGEMENT_CONTROL_STATUS_REPORT` writer"]
pub type W = crate::W<PcieRcPowerManagementControlStatusReportSpec>;
#[doc = "Field `PS` reader - Power State \\[PS\\]
This field can also be read or written from the local management APBbus."]
pub type PsR = crate::FieldReader;
#[doc = "Field `PS` writer - Power State \\[PS\\]
This field can also be read or written from the local management APBbus."]
pub type PsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `R4` reader - Reserved \\[R4\\]
Reserved"]
pub type R4R = crate::BitReader;
#[doc = "Field `NSR` reader - No Soft Reset \\[NSR\\]
This bit is set to 1 by default. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type NsrR = crate::BitReader;
#[doc = "Field `R3` reader - Reserved \\[R3\\]
Reserved"]
pub type R3R = crate::FieldReader;
#[doc = "Field `PE` reader - PME Enable \\[PE\\]
This bit can be set or cleared from the local management APB bus, by writing a 1 or 0, respectively."]
pub type PeR = crate::BitReader;
#[doc = "Field `PE` writer - PME Enable \\[PE\\]
This bit can be set or cleared from the local management APB bus, by writing a 1 or 0, respectively."]
pub type PeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `R2` reader - Reserved \\[R2\\]
Reserved"]
pub type R2R = crate::FieldReader;
#[doc = "Field `PMES` reader - PME Status \\[PMES\\]
This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type PmesR = crate::BitReader;
#[doc = "Field `PMES` writer - PME Status \\[PMES\\]
This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
pub type PmesW<'a, REG> = crate::BitWriter1C<'a, REG>;
#[doc = "Field `R1` reader - Reserved \\[R1\\]
Reserved"]
pub type R1R = crate::FieldReader;
#[doc = "Field `DR` reader - Data Register \\[DR\\]
This optional register is not implemented in the PCIe core. This field is hardwired to 0."]
pub type DrR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1 - Power State \\[PS\\]
This field can also be read or written from the local management APBbus."]
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
This bit is set to 1 by default. This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
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
This bit can be set or cleared from the local management APB bus, by writing a 1 or 0, respectively."]
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
This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
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
This field can also be read or written from the local management APBbus."]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PsW<PcieRcPowerManagementControlStatusReportSpec> {
        PsW::new(self, 0)
    }
    #[doc = "Bit 8 - PME Enable \\[PE\\]
This bit can be set or cleared from the local management APB bus, by writing a 1 or 0, respectively."]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PeW<PcieRcPowerManagementControlStatusReportSpec> {
        PeW::new(self, 8)
    }
    #[doc = "Bit 15 - PME Status \\[PMES\\]
This field can be written from the APB bus by setting \\[21\\]
bit high of the pcie_mgmt_APB_ADDR during a local management register write."]
    #[inline(always)]
    #[must_use]
    pub fn pmes(&mut self) -> PmesW<PcieRcPowerManagementControlStatusReportSpec> {
        PmesW::new(self, 15)
    }
}
#[doc = "Power Management Control/Status Report This optional register is not implemented in the PCIe core. This field is hardwired to 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_power_management_control_status_report::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_rc_power_management_control_status_report::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcPowerManagementControlStatusReportSpec;
impl crate::RegisterSpec for PcieRcPowerManagementControlStatusReportSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_power_management_control_status_report::R`](R) reader structure"]
impl crate::Readable for PcieRcPowerManagementControlStatusReportSpec {}
#[doc = "`write(|w| ..)` method takes [`pcie_rc_power_management_control_status_report::W`](W) writer structure"]
impl crate::Writable for PcieRcPowerManagementControlStatusReportSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x8000;
}
#[doc = "`reset()` method sets PCIE_RC_POWER_MANAGEMENT_CONTROL_STATUS_REPORT to value 0x08"]
impl crate::Resettable for PcieRcPowerManagementControlStatusReportSpec {
    const RESET_VALUE: u32 = 0x08;
}
