#[doc = "Register `CRU_CLKGATE_CON32` reader"]
pub type R = crate::R<CruClkgateCon32Spec>;
#[doc = "Register `CRU_CLKGATE_CON32` writer"]
pub type W = crate::W<CruClkgateCon32Spec>;
#[doc = "Field `ACLK_GMAC_EN` reader - aclk_gmac clock disable bit When HIGH, disable clock"]
pub type AclkGmacEnR = crate::BitReader;
#[doc = "Field `ACLK_GMAC_EN` writer - aclk_gmac clock disable bit When HIGH, disable clock"]
pub type AclkGmacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_GMAC_NOC_EN` reader - aclk_gmac_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkGmacNocEnR = crate::BitReader;
#[doc = "Field `ACLK_GMAC_NOC_EN` writer - aclk_gmac_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkGmacNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_GMAC_EN` reader - pclk_gmac clock disable bit When HIGH, disable clock"]
pub type PclkGmacEnR = crate::BitReader;
#[doc = "Field `PCLK_GMAC_EN` writer - pclk_gmac clock disable bit When HIGH, disable clock"]
pub type PclkGmacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_GMAC_NOC_EN` reader - pclk_gmac_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkGmacNocEnR = crate::BitReader;
#[doc = "Field `PCLK_GMAC_NOC_EN` writer - pclk_gmac_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkGmacNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_PERF_GMAC_EN` reader - aclk_perf_gmac clock disable bit When HIGH, disable clock"]
pub type AclkPerfGmacEnR = crate::BitReader;
#[doc = "Field `ACLK_PERF_GMAC_EN` writer - aclk_perf_gmac clock disable bit When HIGH, disable clock"]
pub type AclkPerfGmacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_EMMC_CORE_EN` reader - aclk_emmc_core clock disable bit When HIGH, disable clock"]
pub type AclkEmmcCoreEnR = crate::BitReader;
#[doc = "Field `ACLK_EMMC_CORE_EN` writer - aclk_emmc_core clock disable bit When HIGH, disable clock"]
pub type AclkEmmcCoreEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_EMMC_NOC_EN` reader - aclk_emmc_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkEmmcNocEnR = crate::BitReader;
#[doc = "Field `ACLK_EMMC_NOC_EN` writer - aclk_emmc_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkEmmcNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_EMMC_GRF_EN` reader - aclk_emmc_grf clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkEmmcGrfEnR = crate::BitReader;
#[doc = "Field `ACLK_EMMC_GRF_EN` writer - aclk_emmc_grf clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkEmmcGrfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_EDP_NOC_EN` reader - pclk_edp_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkEdpNocEnR = crate::BitReader;
#[doc = "Field `PCLK_EDP_NOC_EN` writer - pclk_edp_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type PclkEdpNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_EDP_CTRL_EN` reader - pclk_edp_ctrl clock disable bit When HIGH, disable clock"]
pub type PclkEdpCtrlEnR = crate::BitReader;
#[doc = "Field `PCLK_EDP_CTRL_EN` writer - pclk_edp_ctrl clock disable bit When HIGH, disable clock"]
pub type PclkEdpCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_gmac clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_gmac_en(&self) -> AclkGmacEnR {
        AclkGmacEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aclk_gmac_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn aclk_gmac_noc_en(&self) -> AclkGmacNocEnR {
        AclkGmacNocEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pclk_gmac clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_gmac_en(&self) -> PclkGmacEnR {
        PclkGmacEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pclk_gmac_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn pclk_gmac_noc_en(&self) -> PclkGmacNocEnR {
        PclkGmacNocEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - aclk_perf_gmac clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_perf_gmac_en(&self) -> AclkPerfGmacEnR {
        AclkPerfGmacEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - aclk_emmc_core clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_emmc_core_en(&self) -> AclkEmmcCoreEnR {
        AclkEmmcCoreEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - aclk_emmc_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn aclk_emmc_noc_en(&self) -> AclkEmmcNocEnR {
        AclkEmmcNocEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - aclk_emmc_grf clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn aclk_emmc_grf_en(&self) -> AclkEmmcGrfEnR {
        AclkEmmcGrfEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 12 - pclk_edp_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn pclk_edp_noc_en(&self) -> PclkEdpNocEnR {
        PclkEdpNocEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - pclk_edp_ctrl clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_edp_ctrl_en(&self) -> PclkEdpCtrlEnR {
        PclkEdpCtrlEnR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_gmac clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gmac_en(&mut self) -> AclkGmacEnW<CruClkgateCon32Spec> {
        AclkGmacEnW::new(self, 0)
    }
    #[doc = "Bit 1 - aclk_gmac_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gmac_noc_en(&mut self) -> AclkGmacNocEnW<CruClkgateCon32Spec> {
        AclkGmacNocEnW::new(self, 1)
    }
    #[doc = "Bit 2 - pclk_gmac clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_gmac_en(&mut self) -> PclkGmacEnW<CruClkgateCon32Spec> {
        PclkGmacEnW::new(self, 2)
    }
    #[doc = "Bit 3 - pclk_gmac_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_gmac_noc_en(&mut self) -> PclkGmacNocEnW<CruClkgateCon32Spec> {
        PclkGmacNocEnW::new(self, 3)
    }
    #[doc = "Bit 4 - aclk_perf_gmac clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perf_gmac_en(&mut self) -> AclkPerfGmacEnW<CruClkgateCon32Spec> {
        AclkPerfGmacEnW::new(self, 4)
    }
    #[doc = "Bit 8 - aclk_emmc_core clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_emmc_core_en(&mut self) -> AclkEmmcCoreEnW<CruClkgateCon32Spec> {
        AclkEmmcCoreEnW::new(self, 8)
    }
    #[doc = "Bit 9 - aclk_emmc_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_emmc_noc_en(&mut self) -> AclkEmmcNocEnW<CruClkgateCon32Spec> {
        AclkEmmcNocEnW::new(self, 9)
    }
    #[doc = "Bit 10 - aclk_emmc_grf clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_emmc_grf_en(&mut self) -> AclkEmmcGrfEnW<CruClkgateCon32Spec> {
        AclkEmmcGrfEnW::new(self, 10)
    }
    #[doc = "Bit 12 - pclk_edp_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_edp_noc_en(&mut self) -> PclkEdpNocEnW<CruClkgateCon32Spec> {
        PclkEdpNocEnW::new(self, 12)
    }
    #[doc = "Bit 13 - pclk_edp_ctrl clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_edp_ctrl_en(&mut self) -> PclkEdpCtrlEnW<CruClkgateCon32Spec> {
        PclkEdpCtrlEnW::new(self, 13)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon32Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register32\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con32::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con32::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon32Spec;
impl crate::RegisterSpec for CruClkgateCon32Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con32::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon32Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con32::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon32Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON32 to value 0"]
impl crate::Resettable for CruClkgateCon32Spec {
    const RESET_VALUE: u32 = 0;
}
