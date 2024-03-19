#[doc = "Register `PCIE_PF_L1_PM_SUBSTATES_CONTROL_1` reader"]
pub type R = crate::R<PciePfL1PmSubstatesControl1Spec>;
#[doc = "Register `PCIE_PF_L1_PM_SUBSTATES_CONTROL_1` writer"]
pub type W = crate::W<PciePfL1PmSubstatesControl1Spec>;
#[doc = "Field `L1PML12EN` reader - PML1.2 Enable \\[L1PML12 EN\\]\n\n(no description)"]
pub type L1pml12enR = crate::BitReader;
#[doc = "Field `L1PML12EN` writer - PML1.2 Enable \\[L1PML12 EN\\]\n\n(no description)"]
pub type L1pml12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1PML11EN` reader - PML1.1 Enable \\[L1PML11 EN\\]\n\n(no description)"]
pub type L1pml11enR = crate::BitReader;
#[doc = "Field `L1PML11EN` writer - PML1.1 Enable \\[L1PML11 EN\\]\n\n(no description)"]
pub type L1pml11enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1ASPML12E` reader - ASPML1.2 Enable \\[L1ASPML 12E\\]\n\n(no description)"]
pub type L1aspml12eR = crate::BitReader;
#[doc = "Field `L1ASPML12E` writer - ASPML1.2 Enable \\[L1ASPML 12E\\]\n\n(no description)"]
pub type L1aspml12eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1ASPM11E` reader - ASPML1.1 Enable \\[L1ASPM1 1E\\]\n\n(no description)"]
pub type L1aspm11eR = crate::BitReader;
#[doc = "Field `L1ASPM11E` writer - ASPML1.1 Enable \\[L1ASPM1 1E\\]\n\n(no description)"]
pub type L1aspm11eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1CmMdReStr` reader - Common Mode Restore Time \\[L1CmMdR eStr\\]\n\nThis field is reserved for EP."]
pub type L1cmMdReStrR = crate::FieldReader;
#[doc = "Field `L1ThrshldVal` reader - LTR L1.2 Threshold Value \\[L1Thrshld Val\\]\n\n(no description)"]
pub type L1thrshldValR = crate::FieldReader<u16>;
#[doc = "Field `L1ThrshldVal` writer - LTR L1.2 Threshold Value \\[L1Thrshld Val\\]\n\n(no description)"]
pub type L1thrshldValW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `L1ThrshldSc` reader - LTR L1.2 Threshold Scale \\[L1Thrshld Sc\\]\n\n(no description)"]
pub type L1thrshldScR = crate::FieldReader;
#[doc = "Field `L1ThrshldSc` writer - LTR L1.2 Threshold Scale \\[L1Thrshld Sc\\]\n\n(no description)"]
pub type L1thrshldScW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - PML1.2 Enable \\[L1PML12 EN\\]\n\n(no description)"]
    #[inline(always)]
    pub fn l1pml12en(&self) -> L1pml12enR {
        L1pml12enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PML1.1 Enable \\[L1PML11 EN\\]\n\n(no description)"]
    #[inline(always)]
    pub fn l1pml11en(&self) -> L1pml11enR {
        L1pml11enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ASPML1.2 Enable \\[L1ASPML 12E\\]\n\n(no description)"]
    #[inline(always)]
    pub fn l1aspml12e(&self) -> L1aspml12eR {
        L1aspml12eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ASPML1.1 Enable \\[L1ASPM1 1E\\]\n\n(no description)"]
    #[inline(always)]
    pub fn l1aspm11e(&self) -> L1aspm11eR {
        L1aspm11eR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Common Mode Restore Time \\[L1CmMdR eStr\\]\n\nThis field is reserved for EP."]
    #[inline(always)]
    pub fn l1cm_md_re_str(&self) -> L1cmMdReStrR {
        L1cmMdReStrR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - LTR L1.2 Threshold Value \\[L1Thrshld Val\\]\n\n(no description)"]
    #[inline(always)]
    pub fn l1thrshld_val(&self) -> L1thrshldValR {
        L1thrshldValR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:31 - LTR L1.2 Threshold Scale \\[L1Thrshld Sc\\]\n\n(no description)"]
    #[inline(always)]
    pub fn l1thrshld_sc(&self) -> L1thrshldScR {
        L1thrshldScR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PML1.2 Enable \\[L1PML12 EN\\]\n\n(no description)"]
    #[inline(always)]
    #[must_use]
    pub fn l1pml12en(&mut self) -> L1pml12enW<PciePfL1PmSubstatesControl1Spec> {
        L1pml12enW::new(self, 0)
    }
    #[doc = "Bit 1 - PML1.1 Enable \\[L1PML11 EN\\]\n\n(no description)"]
    #[inline(always)]
    #[must_use]
    pub fn l1pml11en(&mut self) -> L1pml11enW<PciePfL1PmSubstatesControl1Spec> {
        L1pml11enW::new(self, 1)
    }
    #[doc = "Bit 2 - ASPML1.2 Enable \\[L1ASPML 12E\\]\n\n(no description)"]
    #[inline(always)]
    #[must_use]
    pub fn l1aspml12e(&mut self) -> L1aspml12eW<PciePfL1PmSubstatesControl1Spec> {
        L1aspml12eW::new(self, 2)
    }
    #[doc = "Bit 3 - ASPML1.1 Enable \\[L1ASPM1 1E\\]\n\n(no description)"]
    #[inline(always)]
    #[must_use]
    pub fn l1aspm11e(&mut self) -> L1aspm11eW<PciePfL1PmSubstatesControl1Spec> {
        L1aspm11eW::new(self, 3)
    }
    #[doc = "Bits 16:25 - LTR L1.2 Threshold Value \\[L1Thrshld Val\\]\n\n(no description)"]
    #[inline(always)]
    #[must_use]
    pub fn l1thrshld_val(&mut self) -> L1thrshldValW<PciePfL1PmSubstatesControl1Spec> {
        L1thrshldValW::new(self, 16)
    }
    #[doc = "Bits 29:31 - LTR L1.2 Threshold Scale \\[L1Thrshld Sc\\]\n\n(no description)"]
    #[inline(always)]
    #[must_use]
    pub fn l1thrshld_sc(&mut self) -> L1thrshldScW<PciePfL1PmSubstatesControl1Spec> {
        L1thrshldScW::new(self, 29)
    }
}
#[doc = "L1 PM Substates Control 1 Register\n\n(no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pcie_pf_l1_pm_substates_control_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pcie_pf_l1_pm_substates_control_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PciePfL1PmSubstatesControl1Spec;
impl crate::RegisterSpec for PciePfL1PmSubstatesControl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pcie_pf_l1_pm_substates_control_1::R`](R) reader structure"]
impl crate::Readable for PciePfL1PmSubstatesControl1Spec {}
#[doc = "`write(|w| ..)` method takes [`pcie_pf_l1_pm_substates_control_1::W`](W) writer structure"]
impl crate::Writable for PciePfL1PmSubstatesControl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PCIE_PF_L1_PM_SUBSTATES_CONTROL_1 to value 0"]
impl crate::Resettable for PciePfL1PmSubstatesControl1Spec {
    const RESET_VALUE: u32 = 0;
}
