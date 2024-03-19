#[doc = "Register `PCIE_RC_L1_PM_SUBSTATES_CAPABILITIES` reader"]
pub type R = crate::R<PcieRcL1PmSubstatesCapabilitiesSpec>;
#[doc = "Field `L1ASPML12SUPP` reader - ASPML1.2Supported \\[L1ASPML12SUPP\\]
(no description)"]
pub type L1aspml12suppR = crate::BitReader;
#[doc = "Field `L1ASPML11SUPP` reader - ASPML1.1Supported \\[L1ASPML11SUPP\\]
(no description)"]
pub type L1aspml11suppR = crate::BitReader;
#[doc = "Field `L1PMSUPP` reader - L1 PML Supported \\[L1PMSUPP\\]
(no description)"]
pub type L1pmsuppR = crate::BitReader;
#[doc = "Field `L1PrtCmMdReStrTime` reader - Port Common Mode Restore Time\\[L1PrtCmMdReStrTime\\]
(no description)"]
pub type L1prtCmMdReStrTimeR = crate::FieldReader;
#[doc = "Field `L1PrtPvrOnScale` reader - Port Power-On Time Scale \\[L1PrtPvrOnScale\\]
(no description)"]
pub type L1prtPvrOnScaleR = crate::FieldReader;
#[doc = "Field `R0` reader - Port Power- On Time Value \\[R0\\]
(no description)"]
pub type R0R = crate::FieldReader;
impl R {
    #[doc = "Bit 2 - ASPML1.2Supported \\[L1ASPML12SUPP\\]
(no description)"]
    #[inline(always)]
    pub fn l1aspml12supp(&self) -> L1aspml12suppR {
        L1aspml12suppR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ASPML1.1Supported \\[L1ASPML11SUPP\\]
(no description)"]
    #[inline(always)]
    pub fn l1aspml11supp(&self) -> L1aspml11suppR {
        L1aspml11suppR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - L1 PML Supported \\[L1PMSUPP\\]
(no description)"]
    #[inline(always)]
    pub fn l1pmsupp(&self) -> L1pmsuppR {
        L1pmsuppR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Port Common Mode Restore Time\\[L1PrtCmMdReStrTime\\]
(no description)"]
    #[inline(always)]
    pub fn l1prt_cm_md_re_str_time(&self) -> L1prtCmMdReStrTimeR {
        L1prtCmMdReStrTimeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Port Power-On Time Scale \\[L1PrtPvrOnScale\\]
(no description)"]
    #[inline(always)]
    pub fn l1prt_pvr_on_scale(&self) -> L1prtPvrOnScaleR {
        L1prtPvrOnScaleR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 19:23 - Port Power- On Time Value \\[R0\\]
(no description)"]
    #[inline(always)]
    pub fn r0(&self) -> R0R {
        R0R::new(((self.bits >> 19) & 0x1f) as u8)
    }
}
#[doc = "L1 PM Substates Capabilities Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_rc_l1_pm_substates_capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PcieRcL1PmSubstatesCapabilitiesSpec;
impl crate::RegisterSpec for PcieRcL1PmSubstatesCapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_rc_l1_pm_substates_capabilities::R`](R) reader structure"]
impl crate::Readable for PcieRcL1PmSubstatesCapabilitiesSpec {}
#[doc = "`reset()` method sets PCIE_RC_L1_PM_SUBSTATES_CAPABILITIES to value 0x0028_ff1c"]
impl crate::Resettable for PcieRcL1PmSubstatesCapabilitiesSpec {
    const RESET_VALUE: u32 = 0x0028_ff1c;
}
