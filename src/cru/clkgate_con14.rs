#[doc = "Register `CLKGATE_CON14` reader"]
pub type R = crate::R<ClkgateCon14Spec>;
#[doc = "Register `CLKGATE_CON14` writer"]
pub type W = crate::W<ClkgateCon14Spec>;
#[doc = "Field `CLK_DBG_PD_CORE_B_EN` reader - clk_dbg_pd_core_b clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkDbgPdCoreBEnR = crate::BitReader;
#[doc = "Field `CLK_DBG_PD_CORE_B_EN` writer - clk_dbg_pd_core_b clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkDbgPdCoreBEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_DBG_CXCS_PD_CORE_B_EN` reader - pclk_dbg_cxcs_pd_core_b clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkDbgCxcsPdCoreBEnR = crate::BitReader;
#[doc = "Field `PCLK_DBG_CXCS_PD_CORE_B_EN` writer - pclk_dbg_cxcs_pd_core_b clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkDbgCxcsPdCoreBEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CORE_ADB400_GIC_2_CORE_B_EN` reader - aclk_core_adb400_gic_2_core_b clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkCoreAdb400Gic2CoreBEnR = crate::BitReader;
#[doc = "Field `ACLK_CORE_ADB400_GIC_2_CORE_B_EN` writer - aclk_core_adb400_gic_2_core_b clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkCoreAdb400Gic2CoreBEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CORE_ADB400_CORE_B_2_GIC_EN` reader - aclk_core_adb400_core_b_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkCoreAdb400CoreB2GicEnR = crate::BitReader;
#[doc = "Field `ACLK_CORE_ADB400_CORE_B_2_GIC_EN` writer - aclk_core_adb400_core_b_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkCoreAdb400CoreB2GicEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CORE_ADB400_CORE_B_2_CCI500_EN` reader - aclk_core_adb400_core_b_2_cci500 clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkCoreAdb400CoreB2Cci500EnR = crate::BitReader;
#[doc = "Field `ACLK_CORE_ADB400_CORE_B_2_CCI500_EN` writer - aclk_core_adb400_core_b_2_cci500 clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkCoreAdb400CoreB2Cci500EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_PERF_CORE_B_EN` reader - aclk_perf_core_b clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkPerfCoreBEnR = crate::BitReader;
#[doc = "Field `ACLK_PERF_CORE_B_EN` writer - aclk_perf_core_b clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkPerfCoreBEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DBG_PD_CORE_L_EN` reader - clk_dbg_pd_core_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkDbgPdCoreLEnR = crate::BitReader;
#[doc = "Field `CLK_DBG_PD_CORE_L_EN` writer - clk_dbg_pd_core_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkDbgPdCoreLEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CORE_ADB400_GIC_2_CORE_L_EN` reader - aclk_core_adb400_gic_2_core_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkCoreAdb400Gic2CoreLEnR = crate::BitReader;
#[doc = "Field `ACLK_CORE_ADB400_GIC_2_CORE_L_EN` writer - aclk_core_adb400_gic_2_core_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkCoreAdb400Gic2CoreLEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CORE_ADB400_CORE_L_2_GIC_EN` reader - aclk_core_adb400_core_l_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkCoreAdb400CoreL2GicEnR = crate::BitReader;
#[doc = "Field `ACLK_CORE_ADB400_CORE_L_2_GIC_EN` writer - aclk_core_adb400_core_l_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkCoreAdb400CoreL2GicEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CORE_ADB400_CORE_L_2_CCI500_EN` reader - aclk_core_adb400_core_l_2_cci500 clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkCoreAdb400CoreL2Cci500EnR = crate::BitReader;
#[doc = "Field `ACLK_CORE_ADB400_CORE_L_2_CCI500_EN` writer - aclk_core_adb400_core_l_2_cci500 clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkCoreAdb400CoreL2Cci500EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_PERF_CORE_L_EN` reader - aclk_perf_core_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkPerfCoreLEnR = crate::BitReader;
#[doc = "Field `ACLK_PERF_CORE_L_EN` writer - aclk_perf_core_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkPerfCoreLEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 1 - clk_dbg_pd_core_b clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_dbg_pd_core_b_en(&self) -> ClkDbgPdCoreBEnR {
        ClkDbgPdCoreBEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pclk_dbg_cxcs_pd_core_b clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_dbg_cxcs_pd_core_b_en(&self) -> PclkDbgCxcsPdCoreBEnR {
        PclkDbgCxcsPdCoreBEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - aclk_core_adb400_gic_2_core_b clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_core_adb400_gic_2_core_b_en(&self) -> AclkCoreAdb400Gic2CoreBEnR {
        AclkCoreAdb400Gic2CoreBEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - aclk_core_adb400_core_b_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_core_adb400_core_b_2_gic_en(&self) -> AclkCoreAdb400CoreB2GicEnR {
        AclkCoreAdb400CoreB2GicEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - aclk_core_adb400_core_b_2_cci500 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_core_adb400_core_b_2_cci500_en(&self) -> AclkCoreAdb400CoreB2Cci500EnR {
        AclkCoreAdb400CoreB2Cci500EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - aclk_perf_core_b clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_perf_core_b_en(&self) -> AclkPerfCoreBEnR {
        AclkPerfCoreBEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - clk_dbg_pd_core_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_dbg_pd_core_l_en(&self) -> ClkDbgPdCoreLEnR {
        ClkDbgPdCoreLEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - aclk_core_adb400_gic_2_core_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_core_adb400_gic_2_core_l_en(&self) -> AclkCoreAdb400Gic2CoreLEnR {
        AclkCoreAdb400Gic2CoreLEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - aclk_core_adb400_core_l_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_core_adb400_core_l_2_gic_en(&self) -> AclkCoreAdb400CoreL2GicEnR {
        AclkCoreAdb400CoreL2GicEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - aclk_core_adb400_core_l_2_cci500 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_core_adb400_core_l_2_cci500_en(&self) -> AclkCoreAdb400CoreL2Cci500EnR {
        AclkCoreAdb400CoreL2Cci500EnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - aclk_perf_core_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_perf_core_l_en(&self) -> AclkPerfCoreLEnR {
        AclkPerfCoreLEnR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - clk_dbg_pd_core_b clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dbg_pd_core_b_en(&mut self) -> ClkDbgPdCoreBEnW<ClkgateCon14Spec> {
        ClkDbgPdCoreBEnW::new(self, 1)
    }
    #[doc = "Bit 2 - pclk_dbg_cxcs_pd_core_b clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_dbg_cxcs_pd_core_b_en(&mut self) -> PclkDbgCxcsPdCoreBEnW<ClkgateCon14Spec> {
        PclkDbgCxcsPdCoreBEnW::new(self, 2)
    }
    #[doc = "Bit 3 - aclk_core_adb400_gic_2_core_b clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_core_adb400_gic_2_core_b_en(
        &mut self,
    ) -> AclkCoreAdb400Gic2CoreBEnW<ClkgateCon14Spec> {
        AclkCoreAdb400Gic2CoreBEnW::new(self, 3)
    }
    #[doc = "Bit 4 - aclk_core_adb400_core_b_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_core_adb400_core_b_2_gic_en(
        &mut self,
    ) -> AclkCoreAdb400CoreB2GicEnW<ClkgateCon14Spec> {
        AclkCoreAdb400CoreB2GicEnW::new(self, 4)
    }
    #[doc = "Bit 5 - aclk_core_adb400_core_b_2_cci500 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_core_adb400_core_b_2_cci500_en(
        &mut self,
    ) -> AclkCoreAdb400CoreB2Cci500EnW<ClkgateCon14Spec> {
        AclkCoreAdb400CoreB2Cci500EnW::new(self, 5)
    }
    #[doc = "Bit 6 - aclk_perf_core_b clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perf_core_b_en(&mut self) -> AclkPerfCoreBEnW<ClkgateCon14Spec> {
        AclkPerfCoreBEnW::new(self, 6)
    }
    #[doc = "Bit 9 - clk_dbg_pd_core_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dbg_pd_core_l_en(&mut self) -> ClkDbgPdCoreLEnW<ClkgateCon14Spec> {
        ClkDbgPdCoreLEnW::new(self, 9)
    }
    #[doc = "Bit 10 - aclk_core_adb400_gic_2_core_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_core_adb400_gic_2_core_l_en(
        &mut self,
    ) -> AclkCoreAdb400Gic2CoreLEnW<ClkgateCon14Spec> {
        AclkCoreAdb400Gic2CoreLEnW::new(self, 10)
    }
    #[doc = "Bit 11 - aclk_core_adb400_core_l_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_core_adb400_core_l_2_gic_en(
        &mut self,
    ) -> AclkCoreAdb400CoreL2GicEnW<ClkgateCon14Spec> {
        AclkCoreAdb400CoreL2GicEnW::new(self, 11)
    }
    #[doc = "Bit 12 - aclk_core_adb400_core_l_2_cci500 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_core_adb400_core_l_2_cci500_en(
        &mut self,
    ) -> AclkCoreAdb400CoreL2Cci500EnW<ClkgateCon14Spec> {
        AclkCoreAdb400CoreL2Cci500EnW::new(self, 12)
    }
    #[doc = "Bit 13 - aclk_perf_core_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perf_core_l_en(&mut self) -> AclkPerfCoreLEnW<ClkgateCon14Spec> {
        AclkPerfCoreLEnW::new(self, 13)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkgateCon14Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register14\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con14::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con14::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkgateCon14Spec;
impl crate::RegisterSpec for ClkgateCon14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgate_con14::R`](R) reader structure"]
impl crate::Readable for ClkgateCon14Spec {}
#[doc = "`write(|w| ..)` method takes [`clkgate_con14::W`](W) writer structure"]
impl crate::Writable for ClkgateCon14Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKGATE_CON14 to value 0"]
impl crate::Resettable for ClkgateCon14Spec {
    const RESET_VALUE: u32 = 0;
}
