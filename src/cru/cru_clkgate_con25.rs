#[doc = "Register `CRU_CLKGATE_CON25` reader"]
pub type R = crate::R<CruClkgateCon25Spec>;
#[doc = "Register `CRU_CLKGATE_CON25` writer"]
pub type W = crate::W<CruClkgateCon25Spec>;
#[doc = "Field `ACLK_DMAC0_PERILP_EN` reader - aclk_dmac0_perilp clock disable bit When HIGH, disable clock"]
pub type AclkDmac0PerilpEnR = crate::BitReader;
#[doc = "Field `ACLK_DMAC0_PERILP_EN` writer - aclk_dmac0_perilp clock disable bit When HIGH, disable clock"]
pub type AclkDmac0PerilpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_DMAC1_PERILP_EN` reader - aclk_dmac1_perilp clock disable bit When HIGH, disable clock"]
pub type AclkDmac1PerilpEnR = crate::BitReader;
#[doc = "Field `ACLK_DMAC1_PERILP_EN` writer - aclk_dmac1_perilp clock disable bit When HIGH, disable clock"]
pub type AclkDmac1PerilpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_PERILP0_NOC_EN` reader - aclk_perilp0_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkPerilp0NocEnR = crate::BitReader;
#[doc = "Field `ACLK_PERILP0_NOC_EN` writer - aclk_perilp0_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkPerilp0NocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_PERILP0_NOC_EN` reader - hclk_perilp0_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkPerilp0NocEnR = crate::BitReader;
#[doc = "Field `HCLK_PERILP0_NOC_EN` writer - hclk_perilp0_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkPerilp0NocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_PERILP1_NOC_EN` reader - hclk_perilp1_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkPerilp1NocEnR = crate::BitReader;
#[doc = "Field `HCLK_PERILP1_NOC_EN` writer - hclk_perilp1_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkPerilp1NocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_PERILP1_NOC_EN` reader - pclk_perilp1_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkPerilp1NocEnR = crate::BitReader;
#[doc = "Field `PCLK_PERILP1_NOC_EN` writer - pclk_perilp1_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkPerilp1NocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_M0_PERILP_NOC_EN` reader - hclk_m0_perilp_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkM0PerilpNocEnR = crate::BitReader;
#[doc = "Field `HCLK_M0_PERILP_NOC_EN` writer - hclk_m0_perilp_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkM0PerilpNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_SDIO_NOC_EN` reader - hclk_sdio_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkSdioNocEnR = crate::BitReader;
#[doc = "Field `HCLK_SDIO_NOC_EN` writer - hclk_sdio_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type HclkSdioNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 5 - aclk_dmac0_perilp clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_dmac0_perilp_en(&self) -> AclkDmac0PerilpEnR {
        AclkDmac0PerilpEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - aclk_dmac1_perilp clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_dmac1_perilp_en(&self) -> AclkDmac1PerilpEnR {
        AclkDmac1PerilpEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - aclk_perilp0_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn aclk_perilp0_noc_en(&self) -> AclkPerilp0NocEnR {
        AclkPerilp0NocEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - hclk_perilp0_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn hclk_perilp0_noc_en(&self) -> HclkPerilp0NocEnR {
        HclkPerilp0NocEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - hclk_perilp1_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn hclk_perilp1_noc_en(&self) -> HclkPerilp1NocEnR {
        HclkPerilp1NocEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - pclk_perilp1_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn pclk_perilp1_noc_en(&self) -> PclkPerilp1NocEnR {
        PclkPerilp1NocEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - hclk_m0_perilp_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn hclk_m0_perilp_noc_en(&self) -> HclkM0PerilpNocEnR {
        HclkM0PerilpNocEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - hclk_sdio_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn hclk_sdio_noc_en(&self) -> HclkSdioNocEnR {
        HclkSdioNocEnR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - aclk_dmac0_perilp clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_dmac0_perilp_en(&mut self) -> AclkDmac0PerilpEnW<CruClkgateCon25Spec> {
        AclkDmac0PerilpEnW::new(self, 5)
    }
    #[doc = "Bit 6 - aclk_dmac1_perilp clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_dmac1_perilp_en(&mut self) -> AclkDmac1PerilpEnW<CruClkgateCon25Spec> {
        AclkDmac1PerilpEnW::new(self, 6)
    }
    #[doc = "Bit 7 - aclk_perilp0_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perilp0_noc_en(&mut self) -> AclkPerilp0NocEnW<CruClkgateCon25Spec> {
        AclkPerilp0NocEnW::new(self, 7)
    }
    #[doc = "Bit 8 - hclk_perilp0_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_perilp0_noc_en(&mut self) -> HclkPerilp0NocEnW<CruClkgateCon25Spec> {
        HclkPerilp0NocEnW::new(self, 8)
    }
    #[doc = "Bit 9 - hclk_perilp1_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_perilp1_noc_en(&mut self) -> HclkPerilp1NocEnW<CruClkgateCon25Spec> {
        HclkPerilp1NocEnW::new(self, 9)
    }
    #[doc = "Bit 10 - pclk_perilp1_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_perilp1_noc_en(&mut self) -> PclkPerilp1NocEnW<CruClkgateCon25Spec> {
        PclkPerilp1NocEnW::new(self, 10)
    }
    #[doc = "Bit 11 - hclk_m0_perilp_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_m0_perilp_noc_en(&mut self) -> HclkM0PerilpNocEnW<CruClkgateCon25Spec> {
        HclkM0PerilpNocEnW::new(self, 11)
    }
    #[doc = "Bit 12 - hclk_sdio_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_sdio_noc_en(&mut self) -> HclkSdioNocEnW<CruClkgateCon25Spec> {
        HclkSdioNocEnW::new(self, 12)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon25Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon25Spec;
impl crate::RegisterSpec for CruClkgateCon25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con25::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon25Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con25::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON25 to value 0"]
impl crate::Resettable for CruClkgateCon25Spec {
    const RESET_VALUE: u32 = 0;
}
