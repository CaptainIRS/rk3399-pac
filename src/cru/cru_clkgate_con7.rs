#[doc = "Register `CRU_CLKGATE_CON7` reader"]
pub type R = crate::R<CruClkgateCon7Spec>;
#[doc = "Register `CRU_CLKGATE_CON7` writer"]
pub type W = crate::W<CruClkgateCon7Spec>;
#[doc = "Field `ACLK_PERILP0_GPLL_SRC_EN` reader - aclk_perilp0_gpll clock disable bit When HIGH, disable clock"]
pub type AclkPerilp0GpllSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_PERILP0_GPLL_SRC_EN` writer - aclk_perilp0_gpll clock disable bit When HIGH, disable clock"]
pub type AclkPerilp0GpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_PERILP0_CPLL_SRC_EN` reader - aclk_perilp0_cpll clock disable bit When HIGH, disable clock"]
pub type AclkPerilp0CpllSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_PERILP0_CPLL_SRC_EN` writer - aclk_perilp0_cpll clock disable bit When HIGH, disable clock"]
pub type AclkPerilp0CpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_PERILP0_EN` reader - aclk_perilp0 clock disable bit When HIGH, disable clock"]
pub type AclkPerilp0EnR = crate::BitReader;
#[doc = "Field `ACLK_PERILP0_EN` writer - aclk_perilp0 clock disable bit When HIGH, disable clock"]
pub type AclkPerilp0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_PERILP0_EN` reader - hclk_perilp0 clock disable bit When HIGH, disable clock"]
pub type HclkPerilp0EnR = crate::BitReader;
#[doc = "Field `HCLK_PERILP0_EN` writer - hclk_perilp0 clock disable bit When HIGH, disable clock"]
pub type HclkPerilp0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_PERILP0_EN` reader - pclk_perilp0 clock disable bit When HIGH, disable clock"]
pub type PclkPerilp0EnR = crate::BitReader;
#[doc = "Field `PCLK_PERILP0_EN` writer - pclk_perilp0 clock disable bit When HIGH, disable clock"]
pub type PclkPerilp0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CM0S_GPLL_SRC_EN` reader - clk_cm0s_gpll clock disable bit When HIGH, disable clock"]
pub type ClkCm0sGpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_CM0S_GPLL_SRC_EN` writer - clk_cm0s_gpll clock disable bit When HIGH, disable clock"]
pub type ClkCm0sGpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CM0S_CPLL_SRC_EN` reader - clk_cm0s_cpll clock disable bit When HIGH, disable clock"]
pub type ClkCm0sCpllSrcEnR = crate::BitReader;
#[doc = "Field `CLK_CM0S_CPLL_SRC_EN` writer - clk_cm0s_cpll clock disable bit When HIGH, disable clock"]
pub type ClkCm0sCpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CRYPTO0_EN` reader - clk_crypto0 clock disable bit When HIGH, disable clock"]
pub type ClkCrypto0EnR = crate::BitReader;
#[doc = "Field `CLK_CRYPTO0_EN` writer - clk_crypto0 clock disable bit When HIGH, disable clock"]
pub type ClkCrypto0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CRYPTO1_EN` reader - clk_crypto1 clock disable bit When HIGH, disable clock"]
pub type ClkCrypto1EnR = crate::BitReader;
#[doc = "Field `CLK_CRYPTO1_EN` writer - clk_crypto1 clock disable bit When HIGH, disable clock"]
pub type ClkCrypto1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCLK_CM0S_EN` reader - fclk_cm0s clock disable bit When HIGH, disable clock"]
pub type FclkCm0sEnR = crate::BitReader;
#[doc = "Field `FCLK_CM0S_EN` writer - fclk_cm0s clock disable bit When HIGH, disable clock"]
pub type FclkCm0sEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_perilp0_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_perilp0_gpll_src_en(&self) -> AclkPerilp0GpllSrcEnR {
        AclkPerilp0GpllSrcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aclk_perilp0_cpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_perilp0_cpll_src_en(&self) -> AclkPerilp0CpllSrcEnR {
        AclkPerilp0CpllSrcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - aclk_perilp0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_perilp0_en(&self) -> AclkPerilp0EnR {
        AclkPerilp0EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hclk_perilp0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_perilp0_en(&self) -> HclkPerilp0EnR {
        HclkPerilp0EnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pclk_perilp0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_perilp0_en(&self) -> PclkPerilp0EnR {
        PclkPerilp0EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_cm0s_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_cm0s_gpll_src_en(&self) -> ClkCm0sGpllSrcEnR {
        ClkCm0sGpllSrcEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_cm0s_cpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_cm0s_cpll_src_en(&self) -> ClkCm0sCpllSrcEnR {
        ClkCm0sCpllSrcEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - clk_crypto0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_crypto0_en(&self) -> ClkCrypto0EnR {
        ClkCrypto0EnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - clk_crypto1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_crypto1_en(&self) -> ClkCrypto1EnR {
        ClkCrypto1EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - fclk_cm0s clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn fclk_cm0s_en(&self) -> FclkCm0sEnR {
        FclkCm0sEnR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_perilp0_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perilp0_gpll_src_en(&mut self) -> AclkPerilp0GpllSrcEnW<CruClkgateCon7Spec> {
        AclkPerilp0GpllSrcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - aclk_perilp0_cpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perilp0_cpll_src_en(&mut self) -> AclkPerilp0CpllSrcEnW<CruClkgateCon7Spec> {
        AclkPerilp0CpllSrcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - aclk_perilp0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perilp0_en(&mut self) -> AclkPerilp0EnW<CruClkgateCon7Spec> {
        AclkPerilp0EnW::new(self, 2)
    }
    #[doc = "Bit 3 - hclk_perilp0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_perilp0_en(&mut self) -> HclkPerilp0EnW<CruClkgateCon7Spec> {
        HclkPerilp0EnW::new(self, 3)
    }
    #[doc = "Bit 4 - pclk_perilp0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_perilp0_en(&mut self) -> PclkPerilp0EnW<CruClkgateCon7Spec> {
        PclkPerilp0EnW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_cm0s_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cm0s_gpll_src_en(&mut self) -> ClkCm0sGpllSrcEnW<CruClkgateCon7Spec> {
        ClkCm0sGpllSrcEnW::new(self, 5)
    }
    #[doc = "Bit 6 - clk_cm0s_cpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cm0s_cpll_src_en(&mut self) -> ClkCm0sCpllSrcEnW<CruClkgateCon7Spec> {
        ClkCm0sCpllSrcEnW::new(self, 6)
    }
    #[doc = "Bit 7 - clk_crypto0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_crypto0_en(&mut self) -> ClkCrypto0EnW<CruClkgateCon7Spec> {
        ClkCrypto0EnW::new(self, 7)
    }
    #[doc = "Bit 8 - clk_crypto1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_crypto1_en(&mut self) -> ClkCrypto1EnW<CruClkgateCon7Spec> {
        ClkCrypto1EnW::new(self, 8)
    }
    #[doc = "Bit 9 - fclk_cm0s clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn fclk_cm0s_en(&mut self) -> FclkCm0sEnW<CruClkgateCon7Spec> {
        FclkCm0sEnW::new(self, 9)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon7Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register7\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con7::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con7::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon7Spec;
impl crate::RegisterSpec for CruClkgateCon7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con7::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon7Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con7::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon7Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON7 to value 0"]
impl crate::Resettable for CruClkgateCon7Spec {
    const RESET_VALUE: u32 = 0;
}
