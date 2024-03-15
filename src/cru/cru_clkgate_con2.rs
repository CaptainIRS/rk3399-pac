#[doc = "Register `CRU_CLKGATE_CON2` reader"]
pub type R = crate::R<CruClkgateCon2Spec>;
#[doc = "Register `CRU_CLKGATE_CON2` writer"]
pub type W = crate::W<CruClkgateCon2Spec>;
#[doc = "Field `ACLK_CCI_CPLL_SRC_EN` reader - aclk_cci_cpll clock disable bit When HIGH, disable clock"]
pub type AclkCciCpllSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_CCI_CPLL_SRC_EN` writer - aclk_cci_cpll clock disable bit When HIGH, disable clock"]
pub type AclkCciCpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CCI_GPLL_SRC_EN` reader - aclk_cci_gpll clock disable bit When HIGH, disable clock"]
pub type AclkCciGpllSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_CCI_GPLL_SRC_EN` writer - aclk_cci_gpll clock disable bit When HIGH, disable clock"]
pub type AclkCciGpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CCI_NPLL_SRC_EN` reader - aclk_cci_npll clock disable bit When HIGH, disable clock"]
pub type AclkCciNpllSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_CCI_NPLL_SRC_EN` writer - aclk_cci_npll clock disable bit When HIGH, disable clock"]
pub type AclkCciNpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CCI_VPLL_SRC_EN` reader - aclk_cci_vpll clock disable bit When HIGH, disable clock"]
pub type AclkCciVpllSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_CCI_VPLL_SRC_EN` writer - aclk_cci_vpll clock disable bit When HIGH, disable clock"]
pub type AclkCciVpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_CCI_SRC_EN` reader - aclk_cci_src clock disable bit When HIGH, disable clock"]
pub type AclkCciSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_CCI_SRC_EN` writer - aclk_cci_src clock disable bit When HIGH, disable clock"]
pub type AclkCciSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CCI_TRACE_CPLL_SRC_EN` reader - clk_cci_trace_cpll clock disable bit When HIGH, disable clock"]
pub type ClkCciTraceCpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_CCI_TRACE_CPLL_SRC_EN` writer - clk_cci_trace_cpll clock disable bit When HIGH, disable clock"]
pub type ClkCciTraceCpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CCI_TRACE_GPLL_SRC_EN` reader - clk_cci_trace_gpll clock disable bit When HIGH, disable clock"]
pub type ClkCciTraceGpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_CCI_TRACE_GPLL_SRC_EN` writer - clk_cci_trace_gpll clock disable bit When HIGH, disable clock"]
pub type ClkCciTraceGpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CCI_TRACE_EN` reader - clk_cci_trace clock disable bit When HIGH, disable clock"]
pub type ClkCciTraceEnR = crate::BitReader;
#[doc = "Field `CLK_CCI_TRACE_EN` writer - clk_cci_trace clock disable bit When HIGH, disable clock"]
pub type ClkCciTraceEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_CPLL_CLK_EN` reader - cs_cpll_clk clock disable bit When HIGH, disable clock"]
pub type CsCpllClkEnR = crate::BitReader;
#[doc = "Field `CS_CPLL_CLK_EN` writer - cs_cpll_clk clock disable bit When HIGH, disable clock"]
pub type CsCpllClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_GPLL_CLK_EN` reader - cs_gpll_clk clock disable bit When HIGH, disable clock"]
pub type CsGpllClkEnR = crate::BitReader;
#[doc = "Field `CS_GPLL_CLK_EN` writer - cs_gpll_clk clock disable bit When HIGH, disable clock"]
pub type CsGpllClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_NPLL_CLK_EN` reader - cs_npll_clk clock disable bit When HIGH, disable clock"]
pub type CsNpllClkEnR = crate::BitReader;
#[doc = "Field `CS_NPLL_CLK_EN` writer - cs_npll_clk clock disable bit When HIGH, disable clock"]
pub type CsNpllClkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_cci_cpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_cci_cpll_src_en(&self) -> AclkCciCpllSrcEnR {
        AclkCciCpllSrcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aclk_cci_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_cci_gpll_src_en(&self) -> AclkCciGpllSrcEnR {
        AclkCciGpllSrcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - aclk_cci_npll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_cci_npll_src_en(&self) -> AclkCciNpllSrcEnR {
        AclkCciNpllSrcEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - aclk_cci_vpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_cci_vpll_src_en(&self) -> AclkCciVpllSrcEnR {
        AclkCciVpllSrcEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - aclk_cci_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_cci_src_en(&self) -> AclkCciSrcEnR {
        AclkCciSrcEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_cci_trace_cpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_cci_trace_cpll_src_en(&self) -> ClkCciTraceCpllSrcEnR {
        ClkCciTraceCpllSrcEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_cci_trace_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_cci_trace_gpll_src_en(&self) -> ClkCciTraceGpllSrcEnR {
        ClkCciTraceGpllSrcEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - clk_cci_trace clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_cci_trace_en(&self) -> ClkCciTraceEnR {
        ClkCciTraceEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - cs_cpll_clk clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn cs_cpll_clk_en(&self) -> CsCpllClkEnR {
        CsCpllClkEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - cs_gpll_clk clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn cs_gpll_clk_en(&self) -> CsGpllClkEnR {
        CsGpllClkEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - cs_npll_clk clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn cs_npll_clk_en(&self) -> CsNpllClkEnR {
        CsNpllClkEnR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_cci_cpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_cci_cpll_src_en(&mut self) -> AclkCciCpllSrcEnW<CruClkgateCon2Spec> {
        AclkCciCpllSrcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - aclk_cci_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_cci_gpll_src_en(&mut self) -> AclkCciGpllSrcEnW<CruClkgateCon2Spec> {
        AclkCciGpllSrcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - aclk_cci_npll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_cci_npll_src_en(&mut self) -> AclkCciNpllSrcEnW<CruClkgateCon2Spec> {
        AclkCciNpllSrcEnW::new(self, 2)
    }
    #[doc = "Bit 3 - aclk_cci_vpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_cci_vpll_src_en(&mut self) -> AclkCciVpllSrcEnW<CruClkgateCon2Spec> {
        AclkCciVpllSrcEnW::new(self, 3)
    }
    #[doc = "Bit 4 - aclk_cci_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_cci_src_en(&mut self) -> AclkCciSrcEnW<CruClkgateCon2Spec> {
        AclkCciSrcEnW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_cci_trace_cpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cci_trace_cpll_src_en(&mut self) -> ClkCciTraceCpllSrcEnW<CruClkgateCon2Spec> {
        ClkCciTraceCpllSrcEnW::new(self, 5)
    }
    #[doc = "Bit 6 - clk_cci_trace_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cci_trace_gpll_src_en(&mut self) -> ClkCciTraceGpllSrcEnW<CruClkgateCon2Spec> {
        ClkCciTraceGpllSrcEnW::new(self, 6)
    }
    #[doc = "Bit 7 - clk_cci_trace clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cci_trace_en(&mut self) -> ClkCciTraceEnW<CruClkgateCon2Spec> {
        ClkCciTraceEnW::new(self, 7)
    }
    #[doc = "Bit 8 - cs_cpll_clk clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn cs_cpll_clk_en(&mut self) -> CsCpllClkEnW<CruClkgateCon2Spec> {
        CsCpllClkEnW::new(self, 8)
    }
    #[doc = "Bit 9 - cs_gpll_clk clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn cs_gpll_clk_en(&mut self) -> CsGpllClkEnW<CruClkgateCon2Spec> {
        CsGpllClkEnW::new(self, 9)
    }
    #[doc = "Bit 10 - cs_npll_clk clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn cs_npll_clk_en(&mut self) -> CsNpllClkEnW<CruClkgateCon2Spec> {
        CsNpllClkEnW::new(self, 10)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon2Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon2Spec;
impl crate::RegisterSpec for CruClkgateCon2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con2::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon2Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con2::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON2 to value 0"]
impl crate::Resettable for CruClkgateCon2Spec {
    const RESET_VALUE: u32 = 0;
}
