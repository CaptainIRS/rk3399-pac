#[doc = "Register `CRU_CLKGATE_CON33` reader"]
pub type R = crate::R<CruClkgateCon33Spec>;
#[doc = "Register `CRU_CLKGATE_CON33` writer"]
pub type W = crate::W<CruClkgateCon33Spec>;
#[doc = "Field `ACLK_GIC_EN` reader - aclk_gic clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkGicEnR = crate::BitReader;
#[doc = "Field `ACLK_GIC_EN` writer - aclk_gic clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkGicEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_GIC_NOC_EN` reader - aclk_gic_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkGicNocEnR = crate::BitReader;
#[doc = "Field `ACLK_GIC_NOC_EN` writer - aclk_gic_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkGicNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_GIC_ADB400_CORE_L_2_GIC_EN` reader - aclk_gic_adb400_core_l_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkGicAdb400CoreL2GicEnR = crate::BitReader;
#[doc = "Field `ACLK_GIC_ADB400_CORE_L_2_GIC_EN` writer - aclk_gic_adb400_core_l_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkGicAdb400CoreL2GicEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_GIC_ADB400_CORE_B_2_GIC_EN` reader - aclk_gic_adb400_core_b_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkGicAdb400CoreB2GicEnR = crate::BitReader;
#[doc = "Field `ACLK_GIC_ADB400_CORE_B_2_GIC_EN` writer - aclk_gic_adb400_core_b_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkGicAdb400CoreB2GicEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_GIC_ADB400_GIC_2_CORE_L_EN` reader - aclk_gic_adb400_gic_2_core_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkGicAdb400Gic2CoreLEnR = crate::BitReader;
#[doc = "Field `ACLK_GIC_ADB400_GIC_2_CORE_L_EN` writer - aclk_gic_adb400_gic_2_core_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkGicAdb400Gic2CoreLEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_GIC_ADB400_GIC_2_CORE_B_EN` reader - aclk_gic_adb400_gic_2_core_b clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkGicAdb400Gic2CoreBEnR = crate::BitReader;
#[doc = "Field `ACLK_GIC_ADB400_GIC_2_CORE_B_EN` writer - aclk_gic_adb400_gic_2_core_b clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkGicAdb400Gic2CoreBEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_SDMMC_EN` reader - hclk_sdmmc clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkSdmmcEnR = crate::BitReader;
#[doc = "Field `HCLK_SDMMC_EN` writer - hclk_sdmmc clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkSdmmcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_SD_NOC_EN` reader - hclk_sd_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type HclkSdNocEnR = crate::BitReader;
#[doc = "Field `HCLK_SD_NOC_EN` writer - hclk_sd_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type HclkSdNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_gic clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_gic_en(&self) -> AclkGicEnR {
        AclkGicEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aclk_gic_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn aclk_gic_noc_en(&self) -> AclkGicNocEnR {
        AclkGicNocEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - aclk_gic_adb400_core_l_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_gic_adb400_core_l_2_gic_en(&self) -> AclkGicAdb400CoreL2GicEnR {
        AclkGicAdb400CoreL2GicEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - aclk_gic_adb400_core_b_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_gic_adb400_core_b_2_gic_en(&self) -> AclkGicAdb400CoreB2GicEnR {
        AclkGicAdb400CoreB2GicEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - aclk_gic_adb400_gic_2_core_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_gic_adb400_gic_2_core_l_en(&self) -> AclkGicAdb400Gic2CoreLEnR {
        AclkGicAdb400Gic2CoreLEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - aclk_gic_adb400_gic_2_core_b clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_gic_adb400_gic_2_core_b_en(&self) -> AclkGicAdb400Gic2CoreBEnR {
        AclkGicAdb400Gic2CoreBEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - hclk_sdmmc clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_sdmmc_en(&self) -> HclkSdmmcEnR {
        HclkSdmmcEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - hclk_sd_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn hclk_sd_noc_en(&self) -> HclkSdNocEnR {
        HclkSdNocEnR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_gic clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gic_en(&mut self) -> AclkGicEnW<CruClkgateCon33Spec> {
        AclkGicEnW::new(self, 0)
    }
    #[doc = "Bit 1 - aclk_gic_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gic_noc_en(&mut self) -> AclkGicNocEnW<CruClkgateCon33Spec> {
        AclkGicNocEnW::new(self, 1)
    }
    #[doc = "Bit 2 - aclk_gic_adb400_core_l_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gic_adb400_core_l_2_gic_en(
        &mut self,
    ) -> AclkGicAdb400CoreL2GicEnW<CruClkgateCon33Spec> {
        AclkGicAdb400CoreL2GicEnW::new(self, 2)
    }
    #[doc = "Bit 3 - aclk_gic_adb400_core_b_2_gic clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gic_adb400_core_b_2_gic_en(
        &mut self,
    ) -> AclkGicAdb400CoreB2GicEnW<CruClkgateCon33Spec> {
        AclkGicAdb400CoreB2GicEnW::new(self, 3)
    }
    #[doc = "Bit 4 - aclk_gic_adb400_gic_2_core_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gic_adb400_gic_2_core_l_en(
        &mut self,
    ) -> AclkGicAdb400Gic2CoreLEnW<CruClkgateCon33Spec> {
        AclkGicAdb400Gic2CoreLEnW::new(self, 4)
    }
    #[doc = "Bit 5 - aclk_gic_adb400_gic_2_core_b clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gic_adb400_gic_2_core_b_en(
        &mut self,
    ) -> AclkGicAdb400Gic2CoreBEnW<CruClkgateCon33Spec> {
        AclkGicAdb400Gic2CoreBEnW::new(self, 5)
    }
    #[doc = "Bit 8 - hclk_sdmmc clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_sdmmc_en(&mut self) -> HclkSdmmcEnW<CruClkgateCon33Spec> {
        HclkSdmmcEnW::new(self, 8)
    }
    #[doc = "Bit 9 - hclk_sd_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_sd_noc_en(&mut self) -> HclkSdNocEnW<CruClkgateCon33Spec> {
        HclkSdNocEnW::new(self, 9)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon33Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register33\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con33::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con33::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon33Spec;
impl crate::RegisterSpec for CruClkgateCon33Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con33::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon33Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con33::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon33Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON33 to value 0"]
impl crate::Resettable for CruClkgateCon33Spec {
    const RESET_VALUE: u32 = 0;
}
