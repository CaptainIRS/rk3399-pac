#[doc = "Register `CRU_CLKGATE_CON20` reader"]
pub type R = crate::R<CruClkgateCon20Spec>;
#[doc = "Register `CRU_CLKGATE_CON20` writer"]
pub type W = crate::W<CruClkgateCon20Spec>;
#[doc = "Field `ACLK_PERF_PCIE_EN` reader - aclk_perf_pcie clock disable bit When HIGH, disable clock"]
pub type AclkPerfPcieEnR = crate::BitReader;
#[doc = "Field `ACLK_PERF_PCIE_EN` writer - aclk_perf_pcie clock disable bit When HIGH, disable clock"]
pub type AclkPerfPcieEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_PERIHP_GRF_EN` reader - pclk_perihp_grf clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkPerihpGrfEnR = crate::BitReader;
#[doc = "Field `PCLK_PERIHP_GRF_EN` writer - pclk_perihp_grf clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkPerihpGrfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_HOST0_EN` reader - hclk_host0 clock disable bit When HIGH, disable clock"]
pub type HclkHost0EnR = crate::BitReader;
#[doc = "Field `HCLK_HOST0_EN` writer - hclk_host0 clock disable bit When HIGH, disable clock"]
pub type HclkHost0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_HOST0_ARB_EN` reader - hclk_host0_arb clock disable bit When HIGH, disable clock"]
pub type HclkHost0ArbEnR = crate::BitReader;
#[doc = "Field `HCLK_HOST0_ARB_EN` writer - hclk_host0_arb clock disable bit When HIGH, disable clock"]
pub type HclkHost0ArbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_HOST1_EN` reader - hclk_host1 clock disable bit When HIGH, disable clock"]
pub type HclkHost1EnR = crate::BitReader;
#[doc = "Field `HCLK_HOST1_EN` writer - hclk_host1 clock disable bit When HIGH, disable clock"]
pub type HclkHost1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_HOST1_ARB_EN` reader - hclk_host1_arb clock disable bit When HIGH, disable clock"]
pub type HclkHost1ArbEnR = crate::BitReader;
#[doc = "Field `HCLK_HOST1_ARB_EN` writer - hclk_host1_arb clock disable bit When HIGH, disable clock"]
pub type HclkHost1ArbEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_HSIC_EN` reader - hclk_hsic clock disable bit When HIGH, disable clock"]
pub type HclkHsicEnR = crate::BitReader;
#[doc = "Field `HCLK_HSIC_EN` writer - hclk_hsic clock disable bit When HIGH, disable clock"]
pub type HclkHsicEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_PCIE_EN` reader - aclk_pcie clock disable bit When HIGH, disable clock"]
pub type AclkPcieEnR = crate::BitReader;
#[doc = "Field `ACLK_PCIE_EN` writer - aclk_pcie clock disable bit When HIGH, disable clock"]
pub type AclkPcieEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_PCIE_EN` reader - pclk_pcie clock disable bit When HIGH, disable clock"]
pub type PclkPcieEnR = crate::BitReader;
#[doc = "Field `PCLK_PCIE_EN` writer - pclk_pcie clock disable bit When HIGH, disable clock"]
pub type PclkPcieEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_PERIHP_NOC_EN` reader - aclk_perihp_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkPerihpNocEnR = crate::BitReader;
#[doc = "Field `ACLK_PERIHP_NOC_EN` writer - aclk_perihp_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkPerihpNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_PERIHP_NOC_EN` reader - hclk_perihp_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkPerihpNocEnR = crate::BitReader;
#[doc = "Field `HCLK_PERIHP_NOC_EN` writer - hclk_perihp_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkPerihpNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_PERIHP_NOC_EN` reader - pclk_perihp_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkPerihpNocEnR = crate::BitReader;
#[doc = "Field `PCLK_PERIHP_NOC_EN` writer - pclk_perihp_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkPerihpNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_AHB1TOM_EN` reader - hclk_ahb1tom clock disable bit When HIGH, disable clock"]
pub type HclkAhb1tomEnR = crate::BitReader;
#[doc = "Field `HCLK_AHB1TOM_EN` writer - hclk_ahb1tom clock disable bit When HIGH, disable clock"]
pub type HclkAhb1tomEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 2 - aclk_perf_pcie clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_perf_pcie_en(&self) -> AclkPerfPcieEnR {
        AclkPerfPcieEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - pclk_perihp_grf clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn pclk_perihp_grf_en(&self) -> PclkPerihpGrfEnR {
        PclkPerihpGrfEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - hclk_host0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_host0_en(&self) -> HclkHost0EnR {
        HclkHost0EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - hclk_host0_arb clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_host0_arb_en(&self) -> HclkHost0ArbEnR {
        HclkHost0ArbEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - hclk_host1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_host1_en(&self) -> HclkHost1EnR {
        HclkHost1EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - hclk_host1_arb clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_host1_arb_en(&self) -> HclkHost1ArbEnR {
        HclkHost1ArbEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - hclk_hsic clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_hsic_en(&self) -> HclkHsicEnR {
        HclkHsicEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - aclk_pcie clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_pcie_en(&self) -> AclkPcieEnR {
        AclkPcieEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - pclk_pcie clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_pcie_en(&self) -> PclkPcieEnR {
        PclkPcieEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - aclk_perihp_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn aclk_perihp_noc_en(&self) -> AclkPerihpNocEnR {
        AclkPerihpNocEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - hclk_perihp_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn hclk_perihp_noc_en(&self) -> HclkPerihpNocEnR {
        HclkPerihpNocEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - pclk_perihp_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn pclk_perihp_noc_en(&self) -> PclkPerihpNocEnR {
        PclkPerihpNocEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - hclk_ahb1tom clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_ahb1tom_en(&self) -> HclkAhb1tomEnR {
        HclkAhb1tomEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - aclk_perf_pcie clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perf_pcie_en(&mut self) -> AclkPerfPcieEnW<CruClkgateCon20Spec> {
        AclkPerfPcieEnW::new(self, 2)
    }
    #[doc = "Bit 4 - pclk_perihp_grf clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_perihp_grf_en(&mut self) -> PclkPerihpGrfEnW<CruClkgateCon20Spec> {
        PclkPerihpGrfEnW::new(self, 4)
    }
    #[doc = "Bit 5 - hclk_host0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_host0_en(&mut self) -> HclkHost0EnW<CruClkgateCon20Spec> {
        HclkHost0EnW::new(self, 5)
    }
    #[doc = "Bit 6 - hclk_host0_arb clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_host0_arb_en(&mut self) -> HclkHost0ArbEnW<CruClkgateCon20Spec> {
        HclkHost0ArbEnW::new(self, 6)
    }
    #[doc = "Bit 7 - hclk_host1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_host1_en(&mut self) -> HclkHost1EnW<CruClkgateCon20Spec> {
        HclkHost1EnW::new(self, 7)
    }
    #[doc = "Bit 8 - hclk_host1_arb clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_host1_arb_en(&mut self) -> HclkHost1ArbEnW<CruClkgateCon20Spec> {
        HclkHost1ArbEnW::new(self, 8)
    }
    #[doc = "Bit 9 - hclk_hsic clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_hsic_en(&mut self) -> HclkHsicEnW<CruClkgateCon20Spec> {
        HclkHsicEnW::new(self, 9)
    }
    #[doc = "Bit 10 - aclk_pcie clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_pcie_en(&mut self) -> AclkPcieEnW<CruClkgateCon20Spec> {
        AclkPcieEnW::new(self, 10)
    }
    #[doc = "Bit 11 - pclk_pcie clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_pcie_en(&mut self) -> PclkPcieEnW<CruClkgateCon20Spec> {
        PclkPcieEnW::new(self, 11)
    }
    #[doc = "Bit 12 - aclk_perihp_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perihp_noc_en(&mut self) -> AclkPerihpNocEnW<CruClkgateCon20Spec> {
        AclkPerihpNocEnW::new(self, 12)
    }
    #[doc = "Bit 13 - hclk_perihp_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_perihp_noc_en(&mut self) -> HclkPerihpNocEnW<CruClkgateCon20Spec> {
        HclkPerihpNocEnW::new(self, 13)
    }
    #[doc = "Bit 14 - pclk_perihp_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_perihp_noc_en(&mut self) -> PclkPerihpNocEnW<CruClkgateCon20Spec> {
        PclkPerihpNocEnW::new(self, 14)
    }
    #[doc = "Bit 15 - hclk_ahb1tom clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_ahb1tom_en(&mut self) -> HclkAhb1tomEnW<CruClkgateCon20Spec> {
        HclkAhb1tomEnW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon20Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register20\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con20::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con20::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon20Spec;
impl crate::RegisterSpec for CruClkgateCon20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con20::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon20Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con20::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon20Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON20 to value 0"]
impl crate::Resettable for CruClkgateCon20Spec {
    const RESET_VALUE: u32 = 0;
}
