#[doc = "Register `CRU_CLKGATE_CON1` reader"]
pub type R = crate::R<CruClkgateCon1Spec>;
#[doc = "Register `CRU_CLKGATE_CON1` writer"]
pub type W = crate::W<CruClkgateCon1Spec>;
#[doc = "Field `CLK_CORE_B_LPLL_SRC_EN` reader - clk_core_b_lpll clock disable bit When HIGH, disable clock"]
pub type ClkCoreBLpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_CORE_B_LPLL_SRC_EN` writer - clk_core_b_lpll clock disable bit When HIGH, disable clock"]
pub type ClkCoreBLpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CORE_B_BPLL_SRC_EN` reader - clk_core_b_bpll clock disable bit When HIGH, disable clock"]
pub type ClkCoreBBpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_CORE_B_BPLL_SRC_EN` writer - clk_core_b_bpll clock disable bit When HIGH, disable clock"]
pub type ClkCoreBBpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CORE_B_DPLL_SRC_EN` reader - clk_core_b_dpll clock disable bit When HIGH, disable clock"]
pub type ClkCoreBDpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_CORE_B_DPLL_SRC_EN` writer - clk_core_b_dpll clock disable bit When HIGH, disable clock"]
pub type ClkCoreBDpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CORE_B_GPLL_SRC_EN` reader - clk_core_b_gpll clock disable bit When HIGH, disable clock"]
pub type ClkCoreBGpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_CORE_B_GPLL_SRC_EN` writer - clk_core_b_gpll clock disable bit When HIGH, disable clock"]
pub type ClkCoreBGpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLKM_CORE_B_EN` reader - aclkm_core_b clock disable bit When HIGH, disable clock"]
pub type AclkmCoreBEnR = crate::BitReader;
#[doc = "Field `ACLKM_CORE_B_EN` writer - aclkm_core_b clock disable bit When HIGH, disable clock"]
pub type AclkmCoreBEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCLK_CORE_B_EN` reader - atclk_core_b clock disable bit When HIGH, disable clock"]
pub type AtclkCoreBEnR = crate::BitReader;
#[doc = "Field `ATCLK_CORE_B_EN` writer - atclk_core_b clock disable bit When HIGH, disable clock"]
pub type AtclkCoreBEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_COREDBG_B_EN` reader - pclk_coredbg_b clock disable bit When HIGH, disable clock"]
pub type PclkCoredbgBEnR = crate::BitReader;
#[doc = "Field `PCLK_COREDBG_B_EN` writer - pclk_coredbg_b clock disable bit When HIGH, disable clock"]
pub type PclkCoredbgBEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PVTM_CORE_B_EN` reader - clk_pvtm_core_b clock disable bit When HIGH, disable clock"]
pub type ClkPvtmCoreBEnR = crate::BitReader;
#[doc = "Field `CLK_PVTM_CORE_B_EN` writer - clk_pvtm_core_b clock disable bit When HIGH, disable clock"]
pub type ClkPvtmCoreBEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - clk_core_b_lpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_core_b_lpll_src_en(&self) -> ClkCoreBLpllSrcEnR {
        ClkCoreBLpllSrcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clk_core_b_bpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_core_b_bpll_src_en(&self) -> ClkCoreBBpllSrcEnR {
        ClkCoreBBpllSrcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clk_core_b_dpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_core_b_dpll_src_en(&self) -> ClkCoreBDpllSrcEnR {
        ClkCoreBDpllSrcEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - clk_core_b_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_core_b_gpll_src_en(&self) -> ClkCoreBGpllSrcEnR {
        ClkCoreBGpllSrcEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - aclkm_core_b clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclkm_core_b_en(&self) -> AclkmCoreBEnR {
        AclkmCoreBEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - atclk_core_b clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn atclk_core_b_en(&self) -> AtclkCoreBEnR {
        AtclkCoreBEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pclk_coredbg_b clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_coredbg_b_en(&self) -> PclkCoredbgBEnR {
        PclkCoredbgBEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - clk_pvtm_core_b clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_pvtm_core_b_en(&self) -> ClkPvtmCoreBEnR {
        ClkPvtmCoreBEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clk_core_b_lpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_core_b_lpll_src_en(&mut self) -> ClkCoreBLpllSrcEnW<CruClkgateCon1Spec> {
        ClkCoreBLpllSrcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - clk_core_b_bpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_core_b_bpll_src_en(&mut self) -> ClkCoreBBpllSrcEnW<CruClkgateCon1Spec> {
        ClkCoreBBpllSrcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - clk_core_b_dpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_core_b_dpll_src_en(&mut self) -> ClkCoreBDpllSrcEnW<CruClkgateCon1Spec> {
        ClkCoreBDpllSrcEnW::new(self, 2)
    }
    #[doc = "Bit 3 - clk_core_b_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_core_b_gpll_src_en(&mut self) -> ClkCoreBGpllSrcEnW<CruClkgateCon1Spec> {
        ClkCoreBGpllSrcEnW::new(self, 3)
    }
    #[doc = "Bit 4 - aclkm_core_b clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclkm_core_b_en(&mut self) -> AclkmCoreBEnW<CruClkgateCon1Spec> {
        AclkmCoreBEnW::new(self, 4)
    }
    #[doc = "Bit 5 - atclk_core_b clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn atclk_core_b_en(&mut self) -> AtclkCoreBEnW<CruClkgateCon1Spec> {
        AtclkCoreBEnW::new(self, 5)
    }
    #[doc = "Bit 6 - pclk_coredbg_b clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_coredbg_b_en(&mut self) -> PclkCoredbgBEnW<CruClkgateCon1Spec> {
        PclkCoredbgBEnW::new(self, 6)
    }
    #[doc = "Bit 7 - clk_pvtm_core_b clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pvtm_core_b_en(&mut self) -> ClkPvtmCoreBEnW<CruClkgateCon1Spec> {
        ClkPvtmCoreBEnW::new(self, 7)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon1Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon1Spec;
impl crate::RegisterSpec for CruClkgateCon1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con1::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon1Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con1::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON1 to value 0"]
impl crate::Resettable for CruClkgateCon1Spec {
    const RESET_VALUE: u32 = 0;
}
