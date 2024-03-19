#[doc = "Register `PCIE_PF_L1_PM_SUBSTATES_CAPABILITIES` reader"]
pub type R = crate::R<PciePfL1PmSubstatesCapabilitiesSpec>;
#[doc = "Field `L1PML12SUPP` reader - PML1.2 Supported \\[L1PML12SUPP\\]
(no description)"]
pub type L1pml12suppR = crate::BitReader;
#[doc = "Field `L1PML11SUPP` reader - PML1.1 Supported \\[L1PML11SUPP\\]
(no description)"]
pub type L1pml11suppR = crate::BitReader;
#[doc = "Field `L1ASPML12SUPP` reader - ASPML1.2 Supported \\[L1ASPML12SUPP\\]
(no description)"]
pub type L1aspml12suppR = crate::BitReader;
#[doc = "Field `L1ASPML11SUPP` reader - ASPML1.1 Supported \\[L1ASPML11SUPP\\]
(no description)"]
pub type L1aspml11suppR = crate::BitReader;
#[doc = "Field `L1PMSUPP` reader - L1 PML Supported \\[L1PMSUPP\\]
(no description)"]
pub type L1pmsuppR = crate::BitReader;
#[doc = "Field `L1PrtCmMdRetrTime` reader - Port Common Mode Restore Time \\[L1PrtCmMdRetrTime\\]
(no description)"]
pub type L1prtCmMdRetrTimeR = crate::FieldReader;
#[doc = "Field `L1PrtPvrOnScale` reader - Port Power-On Time Scale \\[L1PrtPvrOnScale\\]
(no description)"]
pub type L1prtPvrOnScaleR = crate::FieldReader;
#[doc = "Field `R0` reader - Port Power- On Time Value \\[R0\\]
(no description)"]
pub type R0R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - PML1.2 Supported \\[L1PML12SUPP\\]
(no description)"]
    #[inline(always)]
    pub fn l1pml12supp(&self) -> L1pml12suppR {
        L1pml12suppR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PML1.1 Supported \\[L1PML11SUPP\\]
(no description)"]
    #[inline(always)]
    pub fn l1pml11supp(&self) -> L1pml11suppR {
        L1pml11suppR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ASPML1.2 Supported \\[L1ASPML12SUPP\\]
(no description)"]
    #[inline(always)]
    pub fn l1aspml12supp(&self) -> L1aspml12suppR {
        L1aspml12suppR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ASPML1.1 Supported \\[L1ASPML11SUPP\\]
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
    #[doc = "Bits 8:15 - Port Common Mode Restore Time \\[L1PrtCmMdRetrTime\\]
(no description)"]
    #[inline(always)]
    pub fn l1prt_cm_md_retr_time(&self) -> L1prtCmMdRetrTimeR {
        L1prtCmMdRetrTimeR::new(((self.bits >> 8) & 0xff) as u8)
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
#[doc = "L1 PM Substates Capabilities Register RSVD\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_l1_pm_substates_capabilities::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfL1PmSubstatesCapabilitiesSpec;
impl crate::RegisterSpec for PciePfL1PmSubstatesCapabilitiesSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_l1_pm_substates_capabilities::R`](R) reader structure"]
impl crate::Readable for PciePfL1PmSubstatesCapabilitiesSpec {}
#[doc = "`reset()` method sets PCIE_PF_L1_PM_SUBSTATES_CAPABILITIES to value 0x0028_ff1f"]
impl crate::Resettable for PciePfL1PmSubstatesCapabilitiesSpec {
    const RESET_VALUE: u32 = 0x0028_ff1f;
}
