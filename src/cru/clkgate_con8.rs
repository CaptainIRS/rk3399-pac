#[doc = "Register `CLKGATE_CON8` reader"]
pub type R = crate::R<ClkgateCon8Spec>;
#[doc = "Register `CLKGATE_CON8` writer"]
pub type W = crate::W<ClkgateCon8Spec>;
#[doc = "Field `HCLK_PERILP1_GPLL_SRC_EN` reader - hclk_perilp1_gpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkPerilp1GpllSrcEnR = crate::BitReader;
#[doc = "Field `HCLK_PERILP1_GPLL_SRC_EN` writer - hclk_perilp1_gpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkPerilp1GpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_PERILP1_CPLL_SRC_EN` reader - hclk_perilp1_cpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkPerilp1CpllSrcEnR = crate::BitReader;
#[doc = "Field `HCLK_PERILP1_CPLL_SRC_EN` writer - hclk_perilp1_cpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkPerilp1CpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_PERILP1_EN` reader - pclk_perilp1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkPerilp1EnR = crate::BitReader;
#[doc = "Field `PCLK_PERILP1_EN` writer - pclk_perilp1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkPerilp1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2S0_SRC_EN` reader - clk_i2s0_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s0SrcEnR = crate::BitReader;
#[doc = "Field `CLK_I2S0_SRC_EN` writer - clk_i2s0_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s0SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2S0_FRAC_SRC_EN` reader - clk_i2s0_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s0FracSrcEnR = crate::BitReader;
#[doc = "Field `CLK_I2S0_FRAC_SRC_EN` writer - clk_i2s0_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s0FracSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2S0_EN` reader - clk_i2s0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s0EnR = crate::BitReader;
#[doc = "Field `CLK_I2S0_EN` writer - clk_i2s0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2S1_SRC_EN` reader - clk_i2s1_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s1SrcEnR = crate::BitReader;
#[doc = "Field `CLK_I2S1_SRC_EN` writer - clk_i2s1_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s1SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2S1_FRAC_SRC_EN` reader - clk_i2s1_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s1FracSrcEnR = crate::BitReader;
#[doc = "Field `CLK_I2S1_FRAC_SRC_EN` writer - clk_i2s1_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s1FracSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2S1_EN` reader - clk_i2s1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s1EnR = crate::BitReader;
#[doc = "Field `CLK_I2S1_EN` writer - clk_i2s1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2S2_SRC_EN` reader - clk_i2s2_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s2SrcEnR = crate::BitReader;
#[doc = "Field `CLK_I2S2_SRC_EN` writer - clk_i2s2_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s2SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2S2_FRAC_SRC_EN` reader - clk_i2s2_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s2FracSrcEnR = crate::BitReader;
#[doc = "Field `CLK_I2S2_FRAC_SRC_EN` writer - clk_i2s2_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s2FracSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2S2_EN` reader - clk_i2s2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s2EnR = crate::BitReader;
#[doc = "Field `CLK_I2S2_EN` writer - clk_i2s2 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2s2EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2S_OUT_EN` reader - clk_i2s_out clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2sOutEnR = crate::BitReader;
#[doc = "Field `CLK_I2S_OUT_EN` writer - clk_i2s_out clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkI2sOutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SPDIF_8CH_SRC_EN` reader - clk_spdif_8ch_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpdif8chSrcEnR = crate::BitReader;
#[doc = "Field `CLK_SPDIF_8CH_SRC_EN` writer - clk_spdif_8ch_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpdif8chSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SPDIF_8CH_FRAC_SRC_EN` reader - clk_spdif_8ch_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpdif8chFracSrcEnR = crate::BitReader;
#[doc = "Field `CLK_SPDIF_8CH_FRAC_SRC_EN` writer - clk_spdif_8ch_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpdif8chFracSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SPDIF_8CH_EN` reader - clk_spdif_8ch clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpdif8chEnR = crate::BitReader;
#[doc = "Field `CLK_SPDIF_8CH_EN` writer - clk_spdif_8ch clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpdif8chEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - hclk_perilp1_gpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_perilp1_gpll_src_en(&self) -> HclkPerilp1GpllSrcEnR {
        HclkPerilp1GpllSrcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - hclk_perilp1_cpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_perilp1_cpll_src_en(&self) -> HclkPerilp1CpllSrcEnR {
        HclkPerilp1CpllSrcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pclk_perilp1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_perilp1_en(&self) -> PclkPerilp1EnR {
        PclkPerilp1EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - clk_i2s0_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2s0_src_en(&self) -> ClkI2s0SrcEnR {
        ClkI2s0SrcEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - clk_i2s0_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2s0_frac_src_en(&self) -> ClkI2s0FracSrcEnR {
        ClkI2s0FracSrcEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_i2s0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2s0_en(&self) -> ClkI2s0EnR {
        ClkI2s0EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_i2s1_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2s1_src_en(&self) -> ClkI2s1SrcEnR {
        ClkI2s1SrcEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - clk_i2s1_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2s1_frac_src_en(&self) -> ClkI2s1FracSrcEnR {
        ClkI2s1FracSrcEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - clk_i2s1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2s1_en(&self) -> ClkI2s1EnR {
        ClkI2s1EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - clk_i2s2_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2s2_src_en(&self) -> ClkI2s2SrcEnR {
        ClkI2s2SrcEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - clk_i2s2_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2s2_frac_src_en(&self) -> ClkI2s2FracSrcEnR {
        ClkI2s2FracSrcEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - clk_i2s2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2s2_en(&self) -> ClkI2s2EnR {
        ClkI2s2EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - clk_i2s_out clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2s_out_en(&self) -> ClkI2sOutEnR {
        ClkI2sOutEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - clk_spdif_8ch_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_spdif_8ch_src_en(&self) -> ClkSpdif8chSrcEnR {
        ClkSpdif8chSrcEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - clk_spdif_8ch_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_spdif_8ch_frac_src_en(&self) -> ClkSpdif8chFracSrcEnR {
        ClkSpdif8chFracSrcEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - clk_spdif_8ch clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_spdif_8ch_en(&self) -> ClkSpdif8chEnR {
        ClkSpdif8chEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - hclk_perilp1_gpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_perilp1_gpll_src_en(&mut self) -> HclkPerilp1GpllSrcEnW<ClkgateCon8Spec> {
        HclkPerilp1GpllSrcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - hclk_perilp1_cpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_perilp1_cpll_src_en(&mut self) -> HclkPerilp1CpllSrcEnW<ClkgateCon8Spec> {
        HclkPerilp1CpllSrcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - pclk_perilp1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_perilp1_en(&mut self) -> PclkPerilp1EnW<ClkgateCon8Spec> {
        PclkPerilp1EnW::new(self, 2)
    }
    #[doc = "Bit 3 - clk_i2s0_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s0_src_en(&mut self) -> ClkI2s0SrcEnW<ClkgateCon8Spec> {
        ClkI2s0SrcEnW::new(self, 3)
    }
    #[doc = "Bit 4 - clk_i2s0_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s0_frac_src_en(&mut self) -> ClkI2s0FracSrcEnW<ClkgateCon8Spec> {
        ClkI2s0FracSrcEnW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_i2s0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s0_en(&mut self) -> ClkI2s0EnW<ClkgateCon8Spec> {
        ClkI2s0EnW::new(self, 5)
    }
    #[doc = "Bit 6 - clk_i2s1_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s1_src_en(&mut self) -> ClkI2s1SrcEnW<ClkgateCon8Spec> {
        ClkI2s1SrcEnW::new(self, 6)
    }
    #[doc = "Bit 7 - clk_i2s1_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s1_frac_src_en(&mut self) -> ClkI2s1FracSrcEnW<ClkgateCon8Spec> {
        ClkI2s1FracSrcEnW::new(self, 7)
    }
    #[doc = "Bit 8 - clk_i2s1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s1_en(&mut self) -> ClkI2s1EnW<ClkgateCon8Spec> {
        ClkI2s1EnW::new(self, 8)
    }
    #[doc = "Bit 9 - clk_i2s2_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s2_src_en(&mut self) -> ClkI2s2SrcEnW<ClkgateCon8Spec> {
        ClkI2s2SrcEnW::new(self, 9)
    }
    #[doc = "Bit 10 - clk_i2s2_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s2_frac_src_en(&mut self) -> ClkI2s2FracSrcEnW<ClkgateCon8Spec> {
        ClkI2s2FracSrcEnW::new(self, 10)
    }
    #[doc = "Bit 11 - clk_i2s2 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s2_en(&mut self) -> ClkI2s2EnW<ClkgateCon8Spec> {
        ClkI2s2EnW::new(self, 11)
    }
    #[doc = "Bit 12 - clk_i2s_out clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2s_out_en(&mut self) -> ClkI2sOutEnW<ClkgateCon8Spec> {
        ClkI2sOutEnW::new(self, 12)
    }
    #[doc = "Bit 13 - clk_spdif_8ch_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spdif_8ch_src_en(&mut self) -> ClkSpdif8chSrcEnW<ClkgateCon8Spec> {
        ClkSpdif8chSrcEnW::new(self, 13)
    }
    #[doc = "Bit 14 - clk_spdif_8ch_frac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spdif_8ch_frac_src_en(&mut self) -> ClkSpdif8chFracSrcEnW<ClkgateCon8Spec> {
        ClkSpdif8chFracSrcEnW::new(self, 14)
    }
    #[doc = "Bit 15 - clk_spdif_8ch clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spdif_8ch_en(&mut self) -> ClkSpdif8chEnW<ClkgateCon8Spec> {
        ClkSpdif8chEnW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkgateCon8Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register8\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con8::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con8::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkgateCon8Spec;
impl crate::RegisterSpec for ClkgateCon8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgate_con8::R`](R) reader structure"]
impl crate::Readable for ClkgateCon8Spec {}
#[doc = "`write(|w| ..)` method takes [`clkgate_con8::W`](W) writer structure"]
impl crate::Writable for ClkgateCon8Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKGATE_CON8 to value 0"]
impl crate::Resettable for ClkgateCon8Spec {
    const RESET_VALUE: u32 = 0;
}
