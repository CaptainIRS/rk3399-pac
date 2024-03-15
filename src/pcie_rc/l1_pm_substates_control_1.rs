#[doc = "Register `L1_PM_SUBSTATES_CONTROL_1` reader"]
pub type R = crate::R<L1PmSubstatesControl1Spec>;
#[doc = "Register `L1_PM_SUBSTATES_CONTROL_1` writer"]
pub type W = crate::W<L1PmSubstatesControl1Spec>;
#[doc = "Field `L1PML12EN` reader - PML1.2 Enable \\[L1PML12EN\\]
(no description)"]
pub type L1pml12enR = crate::BitReader;
#[doc = "Field `L1PML12EN` writer - PML1.2 Enable \\[L1PML12EN\\]
(no description)"]
pub type L1pml12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1PML11EN` reader - PML1.1 Enable \\[L1PML11EN\\]
(no description)"]
pub type L1pml11enR = crate::BitReader;
#[doc = "Field `L1PML11EN` writer - PML1.1 Enable \\[L1PML11EN\\]
(no description)"]
pub type L1pml11enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1ASPML12EN` reader - ASPML1.2 Enable \\[L1ASPML12EN\\]
(no description)"]
pub type L1aspml12enR = crate::BitReader;
#[doc = "Field `L1ASPML12EN` writer - ASPML1.2 Enable \\[L1ASPML12EN\\]
(no description)"]
pub type L1aspml12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1ASPML11EN` reader - ASPML1.1 Enable \\[L1ASPML11EN\\]
(no description)"]
pub type L1aspml11enR = crate::BitReader;
#[doc = "Field `L1ASPML11EN` writer - ASPML1.1 Enable \\[L1ASPML11EN\\]
(no description)"]
pub type L1aspml11enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `L1CmMdReStrTime` reader - Common Mode Restore Time \\[L1CmMdReStrTime\\]
(no description)"]
pub type L1cmMdReStrTimeR = crate::FieldReader;
#[doc = "Field `L1CmMdReStrTime` writer - Common Mode Restore Time \\[L1CmMdReStrTime\\]
(no description)"]
pub type L1cmMdReStrTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `L1ThrshldVal` reader - LTR L1.2 Threshold Value \\[L1ThrshldVal\\]
(no description)"]
pub type L1thrshldValR = crate::FieldReader<u16>;
#[doc = "Field `L1ThrshldVal` writer - LTR L1.2 Threshold Value \\[L1ThrshldVal\\]
(no description)"]
pub type L1thrshldValW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `L1ThrshldSc` reader - LTR L1.2 Threshold Scale \\[L1ThrshldSc\\]
(no description)"]
pub type L1thrshldScR = crate::FieldReader;
#[doc = "Field `L1ThrshldSc` writer - LTR L1.2 Threshold Scale \\[L1ThrshldSc\\]
(no description)"]
pub type L1thrshldScW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bit 0 - PML1.2 Enable \\[L1PML12EN\\]
(no description)"]
    #[inline(always)]
    pub fn l1pml12en(&self) -> L1pml12enR {
        L1pml12enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PML1.1 Enable \\[L1PML11EN\\]
(no description)"]
    #[inline(always)]
    pub fn l1pml11en(&self) -> L1pml11enR {
        L1pml11enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ASPML1.2 Enable \\[L1ASPML12EN\\]
(no description)"]
    #[inline(always)]
    pub fn l1aspml12en(&self) -> L1aspml12enR {
        L1aspml12enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ASPML1.1 Enable \\[L1ASPML11EN\\]
(no description)"]
    #[inline(always)]
    pub fn l1aspml11en(&self) -> L1aspml11enR {
        L1aspml11enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Common Mode Restore Time \\[L1CmMdReStrTime\\]
(no description)"]
    #[inline(always)]
    pub fn l1cm_md_re_str_time(&self) -> L1cmMdReStrTimeR {
        L1cmMdReStrTimeR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:25 - LTR L1.2 Threshold Value \\[L1ThrshldVal\\]
(no description)"]
    #[inline(always)]
    pub fn l1thrshld_val(&self) -> L1thrshldValR {
        L1thrshldValR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bits 29:31 - LTR L1.2 Threshold Scale \\[L1ThrshldSc\\]
(no description)"]
    #[inline(always)]
    pub fn l1thrshld_sc(&self) -> L1thrshldScR {
        L1thrshldScR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PML1.2 Enable \\[L1PML12EN\\]
(no description)"]
    #[inline(always)]
    #[must_use]
    pub fn l1pml12en(&mut self) -> L1pml12enW<L1PmSubstatesControl1Spec> {
        L1pml12enW::new(self, 0)
    }
    #[doc = "Bit 1 - PML1.1 Enable \\[L1PML11EN\\]
(no description)"]
    #[inline(always)]
    #[must_use]
    pub fn l1pml11en(&mut self) -> L1pml11enW<L1PmSubstatesControl1Spec> {
        L1pml11enW::new(self, 1)
    }
    #[doc = "Bit 2 - ASPML1.2 Enable \\[L1ASPML12EN\\]
(no description)"]
    #[inline(always)]
    #[must_use]
    pub fn l1aspml12en(&mut self) -> L1aspml12enW<L1PmSubstatesControl1Spec> {
        L1aspml12enW::new(self, 2)
    }
    #[doc = "Bit 3 - ASPML1.1 Enable \\[L1ASPML11EN\\]
(no description)"]
    #[inline(always)]
    #[must_use]
    pub fn l1aspml11en(&mut self) -> L1aspml11enW<L1PmSubstatesControl1Spec> {
        L1aspml11enW::new(self, 3)
    }
    #[doc = "Bits 8:15 - Common Mode Restore Time \\[L1CmMdReStrTime\\]
(no description)"]
    #[inline(always)]
    #[must_use]
    pub fn l1cm_md_re_str_time(&mut self) -> L1cmMdReStrTimeW<L1PmSubstatesControl1Spec> {
        L1cmMdReStrTimeW::new(self, 8)
    }
    #[doc = "Bits 16:25 - LTR L1.2 Threshold Value \\[L1ThrshldVal\\]
(no description)"]
    #[inline(always)]
    #[must_use]
    pub fn l1thrshld_val(&mut self) -> L1thrshldValW<L1PmSubstatesControl1Spec> {
        L1thrshldValW::new(self, 16)
    }
    #[doc = "Bits 29:31 - LTR L1.2 Threshold Scale \\[L1ThrshldSc\\]
(no description)"]
    #[inline(always)]
    #[must_use]
    pub fn l1thrshld_sc(&mut self) -> L1thrshldScW<L1PmSubstatesControl1Spec> {
        L1thrshldScW::new(self, 29)
    }
}
#[doc = "L1 PM Substates Control 1 Register (no description)\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`l1_pm_substates_control_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`l1_pm_substates_control_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1PmSubstatesControl1Spec;
impl crate::RegisterSpec for L1PmSubstatesControl1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1_pm_substates_control_1::R`](R) reader structure"]
impl crate::Readable for L1PmSubstatesControl1Spec {}
#[doc = "`write(|w| ..)` method takes [`l1_pm_substates_control_1::W`](W) writer structure"]
impl crate::Writable for L1PmSubstatesControl1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets L1_PM_SUBSTATES_CONTROL_1 to value 0"]
impl crate::Resettable for L1PmSubstatesControl1Spec {
    const RESET_VALUE: u32 = 0;
}
