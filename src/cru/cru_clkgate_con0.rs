#[doc = "Register `CRU_CLKGATE_CON0` reader"]
pub type R = crate::R<CruClkgateCon0Spec>;
#[doc = "Register `CRU_CLKGATE_CON0` writer"]
pub type W = crate::W<CruClkgateCon0Spec>;
#[doc = "Field `CLK_CORE_L_LPLL_SRC_EN` reader - clk_core_l_lpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkCoreLLpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_CORE_L_LPLL_SRC_EN` writer - clk_core_l_lpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkCoreLLpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CORE_L_BPLL_SRC_EN` reader - clk_core_l_bpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkCoreLBpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_CORE_L_BPLL_SRC_EN` writer - clk_core_l_bpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkCoreLBpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CORE_L_DPLL_SRC_EN` reader - clk_core_l_dpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkCoreLDpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_CORE_L_DPLL_SRC_EN` writer - clk_core_l_dpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkCoreLDpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CORE_L_GPLL_SRC_EN` reader - clk_core_l_gpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkCoreLGpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_CORE_L_GPLL_SRC_EN` writer - clk_core_l_gpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkCoreLGpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLKM_CORE_L_EN` reader - aclkm_core_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkmCoreLEnR = crate::BitReader;
#[doc = "Field `ACLKM_CORE_L_EN` writer - aclkm_core_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkmCoreLEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATCLK_CORE_L_EN` reader - atclk_core_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type AtclkCoreLEnR = crate::BitReader;
#[doc = "Field `ATCLK_CORE_L_EN` writer - atclk_core_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type AtclkCoreLEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_COREDBG_L_EN` reader - pclk_coredbg_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkCoredbgLEnR = crate::BitReader;
#[doc = "Field `PCLK_COREDBG_L_EN` writer - pclk_coredbg_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkCoredbgLEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PVTM_CORE_L_EN` reader - clk_pvtm_core_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkPvtmCoreLEnR = crate::BitReader;
#[doc = "Field `CLK_PVTM_CORE_L_EN` writer - clk_pvtm_core_l clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkPvtmCoreLEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - clk_core_l_lpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_core_l_lpll_src_en(&self) -> ClkCoreLLpllSrcEnR {
        ClkCoreLLpllSrcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clk_core_l_bpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_core_l_bpll_src_en(&self) -> ClkCoreLBpllSrcEnR {
        ClkCoreLBpllSrcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clk_core_l_dpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_core_l_dpll_src_en(&self) -> ClkCoreLDpllSrcEnR {
        ClkCoreLDpllSrcEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - clk_core_l_gpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_core_l_gpll_src_en(&self) -> ClkCoreLGpllSrcEnR {
        ClkCoreLGpllSrcEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - aclkm_core_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclkm_core_l_en(&self) -> AclkmCoreLEnR {
        AclkmCoreLEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - atclk_core_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn atclk_core_l_en(&self) -> AtclkCoreLEnR {
        AtclkCoreLEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pclk_coredbg_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_coredbg_l_en(&self) -> PclkCoredbgLEnR {
        PclkCoredbgLEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - clk_pvtm_core_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_pvtm_core_l_en(&self) -> ClkPvtmCoreLEnR {
        ClkPvtmCoreLEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clk_core_l_lpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_core_l_lpll_src_en(&mut self) -> ClkCoreLLpllSrcEnW<CruClkgateCon0Spec> {
        ClkCoreLLpllSrcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - clk_core_l_bpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_core_l_bpll_src_en(&mut self) -> ClkCoreLBpllSrcEnW<CruClkgateCon0Spec> {
        ClkCoreLBpllSrcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - clk_core_l_dpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_core_l_dpll_src_en(&mut self) -> ClkCoreLDpllSrcEnW<CruClkgateCon0Spec> {
        ClkCoreLDpllSrcEnW::new(self, 2)
    }
    #[doc = "Bit 3 - clk_core_l_gpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_core_l_gpll_src_en(&mut self) -> ClkCoreLGpllSrcEnW<CruClkgateCon0Spec> {
        ClkCoreLGpllSrcEnW::new(self, 3)
    }
    #[doc = "Bit 4 - aclkm_core_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclkm_core_l_en(&mut self) -> AclkmCoreLEnW<CruClkgateCon0Spec> {
        AclkmCoreLEnW::new(self, 4)
    }
    #[doc = "Bit 5 - atclk_core_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn atclk_core_l_en(&mut self) -> AtclkCoreLEnW<CruClkgateCon0Spec> {
        AtclkCoreLEnW::new(self, 5)
    }
    #[doc = "Bit 6 - pclk_coredbg_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_coredbg_l_en(&mut self) -> PclkCoredbgLEnW<CruClkgateCon0Spec> {
        PclkCoredbgLEnW::new(self, 6)
    }
    #[doc = "Bit 7 - clk_pvtm_core_l clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pvtm_core_l_en(&mut self) -> ClkPvtmCoreLEnW<CruClkgateCon0Spec> {
        ClkPvtmCoreLEnW::new(self, 7)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon0Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon0Spec;
impl crate::RegisterSpec for CruClkgateCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con0::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con0::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON0 to value 0"]
impl crate::Resettable for CruClkgateCon0Spec {
    const RESET_VALUE: u32 = 0;
}
