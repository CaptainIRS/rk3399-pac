#[doc = "Register `CRU_CLKGATE_CON3` reader"]
pub type R = crate::R<CruClkgateCon3Spec>;
#[doc = "Register `CRU_CLKGATE_CON3` writer"]
pub type W = crate::W<CruClkgateCon3Spec>;
#[doc = "Field `CLK_DDRC_LPLL_SRC_EN` reader - clk_ddrc_lpll clock disable bit When HIGH, disable clock"]
pub type ClkDdrcLpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_DDRC_LPLL_SRC_EN` writer - clk_ddrc_lpll clock disable bit When HIGH, disable clock"]
pub type ClkDdrcLpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDRC_BPLL_SRC_EN` reader - clk_ddrc_bpll clock disable bit When HIGH, disable clock"]
pub type ClkDdrcBpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_DDRC_BPLL_SRC_EN` writer - clk_ddrc_bpll clock disable bit When HIGH, disable clock"]
pub type ClkDdrcBpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDRC_DPLL_SRC_EN` reader - clk_ddrc_dpll clock disable bit When HIGH, disable clock"]
pub type ClkDdrcDpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_DDRC_DPLL_SRC_EN` writer - clk_ddrc_dpll clock disable bit When HIGH, disable clock"]
pub type ClkDdrcDpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DDRC_GPLL_SRC_EN` reader - clk_ddrc_gpll clock disable bit When HIGH, disable clock"]
pub type ClkDdrcGpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_DDRC_GPLL_SRC_EN` writer - clk_ddrc_gpll clock disable bit When HIGH, disable clock"]
pub type ClkDdrcGpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_DDR_EN` reader - pclk_ddr clock disable bit When HIGH, disable clock"]
pub type PclkDdrEnR = crate::BitReader;
#[doc = "Field `PCLK_DDR_EN` writer - pclk_ddr clock disable bit When HIGH, disable clock"]
pub type PclkDdrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CENTER_SRC_EN` reader - aclk_center_src clock disable bit When HIGH, disable clock"]
pub type AclkCenterSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_CENTER_SRC_EN` writer - aclk_center_src clock disable bit When HIGH, disable clock"]
pub type AclkCenterSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - clk_ddrc_lpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddrc_lpll_src_en(&self) -> ClkDdrcLpllSrcEnR {
        ClkDdrcLpllSrcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clk_ddrc_bpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddrc_bpll_src_en(&self) -> ClkDdrcBpllSrcEnR {
        ClkDdrcBpllSrcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clk_ddrc_dpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddrc_dpll_src_en(&self) -> ClkDdrcDpllSrcEnR {
        ClkDdrcDpllSrcEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - clk_ddrc_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_ddrc_gpll_src_en(&self) -> ClkDdrcGpllSrcEnR {
        ClkDdrcGpllSrcEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pclk_ddr clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_ddr_en(&self) -> PclkDdrEnR {
        PclkDdrEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - aclk_center_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_center_src_en(&self) -> AclkCenterSrcEnR {
        AclkCenterSrcEnR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clk_ddrc_lpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrc_lpll_src_en(&mut self) -> ClkDdrcLpllSrcEnW<CruClkgateCon3Spec> {
        ClkDdrcLpllSrcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - clk_ddrc_bpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrc_bpll_src_en(&mut self) -> ClkDdrcBpllSrcEnW<CruClkgateCon3Spec> {
        ClkDdrcBpllSrcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - clk_ddrc_dpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrc_dpll_src_en(&mut self) -> ClkDdrcDpllSrcEnW<CruClkgateCon3Spec> {
        ClkDdrcDpllSrcEnW::new(self, 2)
    }
    #[doc = "Bit 3 - clk_ddrc_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ddrc_gpll_src_en(&mut self) -> ClkDdrcGpllSrcEnW<CruClkgateCon3Spec> {
        ClkDdrcGpllSrcEnW::new(self, 3)
    }
    #[doc = "Bit 4 - pclk_ddr clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_ddr_en(&mut self) -> PclkDdrEnW<CruClkgateCon3Spec> {
        PclkDdrEnW::new(self, 4)
    }
    #[doc = "Bit 7 - aclk_center_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_center_src_en(&mut self) -> AclkCenterSrcEnW<CruClkgateCon3Spec> {
        AclkCenterSrcEnW::new(self, 7)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon3Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register3\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con3::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con3::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon3Spec;
impl crate::RegisterSpec for CruClkgateCon3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con3::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon3Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con3::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON3 to value 0"]
impl crate::Resettable for CruClkgateCon3Spec {
    const RESET_VALUE: u32 = 0;
}
