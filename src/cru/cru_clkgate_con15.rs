#[doc = "Register `CRU_CLKGATE_CON15` reader"]
pub type R = crate::R<CruClkgateCon15Spec>;
#[doc = "Register `CRU_CLKGATE_CON15` writer"]
pub type W = crate::W<CruClkgateCon15Spec>;
#[doc = "Field `ACLK_ADB400M_PD_CORE_L_EN` reader - aclk_adb400m_pd_core_l clock disable bit When HIGH, disable clock"]
pub type AclkAdb400mPdCoreLEnR = crate::BitReader;
#[doc = "Field `ACLK_ADB400M_PD_CORE_L_EN` writer - aclk_adb400m_pd_core_l clock disable bit When HIGH, disable clock"]
pub type AclkAdb400mPdCoreLEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_ADB400M_PD_CORE_B_EN` reader - aclk_adb400m_pd_core_b clock disable bit When HIGH, disable clock"]
pub type AclkAdb400mPdCoreBEnR = crate::BitReader;
#[doc = "Field `ACLK_ADB400M_PD_CORE_B_EN` writer - aclk_adb400m_pd_core_b clock disable bit When HIGH, disable clock"]
pub type AclkAdb400mPdCoreBEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CCI_EN` reader - aclk_cci clock disable bit When HIGH, disable clock"]
pub type AclkCciEnR = crate::BitReader;
#[doc = "Field `ACLK_CCI_EN` writer - aclk_cci clock disable bit When HIGH, disable clock"]
pub type AclkCciEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CCI_NOC0_EN` reader - aclk_cci_noc0 clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkCciNoc0EnR = crate::BitReader;
#[doc = "Field `ACLK_CCI_NOC0_EN` writer - aclk_cci_noc0 clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkCciNoc0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CCI_NOC1_EN` reader - aclk_cci_noc1 clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkCciNoc1EnR = crate::BitReader;
#[doc = "Field `ACLK_CCI_NOC1_EN` writer - aclk_cci_noc1 clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkCciNoc1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DBG_CXCS_EN` reader - clk_dbg_cxcs clock disable bit When HIGH, disable clock"]
pub type ClkDbgCxcsEnR = crate::BitReader;
#[doc = "Field `CLK_DBG_CXCS_EN` writer - clk_dbg_cxcs clock disable bit When HIGH, disable clock"]
pub type ClkDbgCxcsEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DBG_NOC_EN` reader - clk_dbg_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type ClkDbgNocEnR = crate::BitReader;
#[doc = "Field `CLK_DBG_NOC_EN` writer - clk_dbg_noc clock disable bit When HIGH, disable clock Suggest always on"]
pub type ClkDbgNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CCI_GRF_EN` reader - aclk_cci_grf clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkCciGrfEnR = crate::BitReader;
#[doc = "Field `ACLK_CCI_GRF_EN` writer - aclk_cci_grf clock disable bit When HIGH, disable clock Suggest always on"]
pub type AclkCciGrfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_adb400m_pd_core_l clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_adb400m_pd_core_l_en(&self) -> AclkAdb400mPdCoreLEnR {
        AclkAdb400mPdCoreLEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aclk_adb400m_pd_core_b clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_adb400m_pd_core_b_en(&self) -> AclkAdb400mPdCoreBEnR {
        AclkAdb400mPdCoreBEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - aclk_cci clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_cci_en(&self) -> AclkCciEnR {
        AclkCciEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - aclk_cci_noc0 clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn aclk_cci_noc0_en(&self) -> AclkCciNoc0EnR {
        AclkCciNoc0EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - aclk_cci_noc1 clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn aclk_cci_noc1_en(&self) -> AclkCciNoc1EnR {
        AclkCciNoc1EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_dbg_cxcs clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_dbg_cxcs_en(&self) -> ClkDbgCxcsEnR {
        ClkDbgCxcsEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_dbg_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn clk_dbg_noc_en(&self) -> ClkDbgNocEnR {
        ClkDbgNocEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - aclk_cci_grf clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    pub fn aclk_cci_grf_en(&self) -> AclkCciGrfEnR {
        AclkCciGrfEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_adb400m_pd_core_l clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_adb400m_pd_core_l_en(&mut self) -> AclkAdb400mPdCoreLEnW<CruClkgateCon15Spec> {
        AclkAdb400mPdCoreLEnW::new(self, 0)
    }
    #[doc = "Bit 1 - aclk_adb400m_pd_core_b clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_adb400m_pd_core_b_en(&mut self) -> AclkAdb400mPdCoreBEnW<CruClkgateCon15Spec> {
        AclkAdb400mPdCoreBEnW::new(self, 1)
    }
    #[doc = "Bit 2 - aclk_cci clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_cci_en(&mut self) -> AclkCciEnW<CruClkgateCon15Spec> {
        AclkCciEnW::new(self, 2)
    }
    #[doc = "Bit 3 - aclk_cci_noc0 clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_cci_noc0_en(&mut self) -> AclkCciNoc0EnW<CruClkgateCon15Spec> {
        AclkCciNoc0EnW::new(self, 3)
    }
    #[doc = "Bit 4 - aclk_cci_noc1 clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_cci_noc1_en(&mut self) -> AclkCciNoc1EnW<CruClkgateCon15Spec> {
        AclkCciNoc1EnW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_dbg_cxcs clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dbg_cxcs_en(&mut self) -> ClkDbgCxcsEnW<CruClkgateCon15Spec> {
        ClkDbgCxcsEnW::new(self, 5)
    }
    #[doc = "Bit 6 - clk_dbg_noc clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dbg_noc_en(&mut self) -> ClkDbgNocEnW<CruClkgateCon15Spec> {
        ClkDbgNocEnW::new(self, 6)
    }
    #[doc = "Bit 7 - aclk_cci_grf clock disable bit When HIGH, disable clock Suggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_cci_grf_en(&mut self) -> AclkCciGrfEnW<CruClkgateCon15Spec> {
        AclkCciGrfEnW::new(self, 7)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon15Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon15Spec;
impl crate::RegisterSpec for CruClkgateCon15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con15::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon15Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con15::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON15 to value 0"]
impl crate::Resettable for CruClkgateCon15Spec {
    const RESET_VALUE: u32 = 0;
}
