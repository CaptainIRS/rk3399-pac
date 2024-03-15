#[doc = "Register `CRU_CLKGATE_CON10` reader"]
pub type R = crate::R<CruClkgateCon10Spec>;
#[doc = "Register `CRU_CLKGATE_CON10` writer"]
pub type W = crate::W<CruClkgateCon10Spec>;
#[doc = "Field `CLK_I2C1_SRC_EN` reader - clk_i2c1_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c1SrcEnR = crate::BitReader;
#[doc = "Field `CLK_I2C1_SRC_EN` writer - clk_i2c1_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c1SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2C5_SRC_EN` reader - clk_i2c5_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c5SrcEnR = crate::BitReader;
#[doc = "Field `CLK_I2C5_SRC_EN` writer - clk_i2c5_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c5SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2C2_SRC_EN` reader - clk_i2c2_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c2SrcEnR = crate::BitReader;
#[doc = "Field `CLK_I2C2_SRC_EN` writer - clk_i2c2_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c2SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2C6_SRC_EN` reader - clk_i2c6_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c6SrcEnR = crate::BitReader;
#[doc = "Field `CLK_I2C6_SRC_EN` writer - clk_i2c6_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c6SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2C3_SRC_EN` reader - clk_i2c3_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c3SrcEnR = crate::BitReader;
#[doc = "Field `CLK_I2C3_SRC_EN` writer - clk_i2c3_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c3SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_I2C7_SRC_EN` reader - clk_i2c7_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c7SrcEnR = crate::BitReader;
#[doc = "Field `CLK_I2C7_SRC_EN` writer - clk_i2c7_src clock disable bit When HIGH, disable clock"]
pub type ClkI2c7SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DPTX_SPDIF_REC_SRC_EN` reader - clk_dptx_spdif_rec_src clock disable bit When HIGH, disable clock"]
pub type ClkDptxSpdifRecSrcEnR = crate::BitReader;
#[doc = "Field `CLK_DPTX_SPDIF_REC_SRC_EN` writer - clk_dptx_spdif_rec_src clock disable bit When HIGH, disable clock"]
pub type ClkDptxSpdifRecSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CIF_OUT_SRC_EN` reader - clk_cif_out_src clock disable bit When HIGH, disable clock"]
pub type ClkCifOutSrcEnR = crate::BitReader;
#[doc = "Field `CLK_CIF_OUT_SRC_EN` writer - clk_cif_out_src clock disable bit When HIGH, disable clock"]
pub type ClkCifOutSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_VOP0_PRE_SRC_EN` reader - aclk_vop0_pre_src clock disable bit When HIGH, disable clock"]
pub type AclkVop0PreSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_VOP0_PRE_SRC_EN` writer - aclk_vop0_pre_src clock disable bit When HIGH, disable clock"]
pub type AclkVop0PreSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_VOP0_PRE_EN` reader - hclk_vop0_pre clock disable bit When HIGH, disable clock"]
pub type HclkVop0PreEnR = crate::BitReader;
#[doc = "Field `HCLK_VOP0_PRE_EN` writer - hclk_vop0_pre clock disable bit When HIGH, disable clock"]
pub type HclkVop0PreEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_VOP1_PRE_SRC_EN` reader - aclk_vop1_pre_src clock disable bit When HIGH, disable clock"]
pub type AclkVop1PreSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_VOP1_PRE_SRC_EN` writer - aclk_vop1_pre_src clock disable bit When HIGH, disable clock"]
pub type AclkVop1PreSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_VOP1_PRE_EN` reader - hclk_vop1_pre clock disable bit When HIGH, disable clock"]
pub type HclkVop1PreEnR = crate::BitReader;
#[doc = "Field `HCLK_VOP1_PRE_EN` writer - hclk_vop1_pre clock disable bit When HIGH, disable clock"]
pub type HclkVop1PreEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCLK_VOP0_SRC_EN` reader - dclk_vop0_src clock disable bit When HIGH, disable clock"]
pub type DclkVop0SrcEnR = crate::BitReader;
#[doc = "Field `DCLK_VOP0_SRC_EN` writer - dclk_vop0_src clock disable bit When HIGH, disable clock"]
pub type DclkVop0SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCLK_VOP1_SRC_EN` reader - dclk_vop1_src clock disable bit When HIGH, disable clock"]
pub type DclkVop1SrcEnR = crate::BitReader;
#[doc = "Field `DCLK_VOP1_SRC_EN` writer - dclk_vop1_src clock disable bit When HIGH, disable clock"]
pub type DclkVop1SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_VOP0_PWM_EN` reader - clk_vop0_pwm clock disable bit When HIGH, disable clock"]
pub type ClkVop0PwmEnR = crate::BitReader;
#[doc = "Field `CLK_VOP0_PWM_EN` writer - clk_vop0_pwm clock disable bit When HIGH, disable clock"]
pub type ClkVop0PwmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_VOP1_PWM_EN` reader - clk_vop1_pwm clock disable bit When HIGH, disable clock"]
pub type ClkVop1PwmEnR = crate::BitReader;
#[doc = "Field `CLK_VOP1_PWM_EN` writer - clk_vop1_pwm clock disable bit When HIGH, disable clock"]
pub type ClkVop1PwmEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - clk_i2c1_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2c1_src_en(&self) -> ClkI2c1SrcEnR {
        ClkI2c1SrcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clk_i2c5_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2c5_src_en(&self) -> ClkI2c5SrcEnR {
        ClkI2c5SrcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clk_i2c2_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2c2_src_en(&self) -> ClkI2c2SrcEnR {
        ClkI2c2SrcEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - clk_i2c6_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2c6_src_en(&self) -> ClkI2c6SrcEnR {
        ClkI2c6SrcEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - clk_i2c3_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2c3_src_en(&self) -> ClkI2c3SrcEnR {
        ClkI2c3SrcEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_i2c7_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_i2c7_src_en(&self) -> ClkI2c7SrcEnR {
        ClkI2c7SrcEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_dptx_spdif_rec_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_dptx_spdif_rec_src_en(&self) -> ClkDptxSpdifRecSrcEnR {
        ClkDptxSpdifRecSrcEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - clk_cif_out_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_cif_out_src_en(&self) -> ClkCifOutSrcEnR {
        ClkCifOutSrcEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - aclk_vop0_pre_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_vop0_pre_src_en(&self) -> AclkVop0PreSrcEnR {
        AclkVop0PreSrcEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - hclk_vop0_pre clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_vop0_pre_en(&self) -> HclkVop0PreEnR {
        HclkVop0PreEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - aclk_vop1_pre_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_vop1_pre_src_en(&self) -> AclkVop1PreSrcEnR {
        AclkVop1PreSrcEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - hclk_vop1_pre clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_vop1_pre_en(&self) -> HclkVop1PreEnR {
        HclkVop1PreEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - dclk_vop0_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn dclk_vop0_src_en(&self) -> DclkVop0SrcEnR {
        DclkVop0SrcEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - dclk_vop1_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn dclk_vop1_src_en(&self) -> DclkVop1SrcEnR {
        DclkVop1SrcEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - clk_vop0_pwm clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_vop0_pwm_en(&self) -> ClkVop0PwmEnR {
        ClkVop0PwmEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - clk_vop1_pwm clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_vop1_pwm_en(&self) -> ClkVop1PwmEnR {
        ClkVop1PwmEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clk_i2c1_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c1_src_en(&mut self) -> ClkI2c1SrcEnW<CruClkgateCon10Spec> {
        ClkI2c1SrcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - clk_i2c5_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c5_src_en(&mut self) -> ClkI2c5SrcEnW<CruClkgateCon10Spec> {
        ClkI2c5SrcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - clk_i2c2_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c2_src_en(&mut self) -> ClkI2c2SrcEnW<CruClkgateCon10Spec> {
        ClkI2c2SrcEnW::new(self, 2)
    }
    #[doc = "Bit 3 - clk_i2c6_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c6_src_en(&mut self) -> ClkI2c6SrcEnW<CruClkgateCon10Spec> {
        ClkI2c6SrcEnW::new(self, 3)
    }
    #[doc = "Bit 4 - clk_i2c3_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c3_src_en(&mut self) -> ClkI2c3SrcEnW<CruClkgateCon10Spec> {
        ClkI2c3SrcEnW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_i2c7_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_i2c7_src_en(&mut self) -> ClkI2c7SrcEnW<CruClkgateCon10Spec> {
        ClkI2c7SrcEnW::new(self, 5)
    }
    #[doc = "Bit 6 - clk_dptx_spdif_rec_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dptx_spdif_rec_src_en(&mut self) -> ClkDptxSpdifRecSrcEnW<CruClkgateCon10Spec> {
        ClkDptxSpdifRecSrcEnW::new(self, 6)
    }
    #[doc = "Bit 7 - clk_cif_out_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_cif_out_src_en(&mut self) -> ClkCifOutSrcEnW<CruClkgateCon10Spec> {
        ClkCifOutSrcEnW::new(self, 7)
    }
    #[doc = "Bit 8 - aclk_vop0_pre_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vop0_pre_src_en(&mut self) -> AclkVop0PreSrcEnW<CruClkgateCon10Spec> {
        AclkVop0PreSrcEnW::new(self, 8)
    }
    #[doc = "Bit 9 - hclk_vop0_pre clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_vop0_pre_en(&mut self) -> HclkVop0PreEnW<CruClkgateCon10Spec> {
        HclkVop0PreEnW::new(self, 9)
    }
    #[doc = "Bit 10 - aclk_vop1_pre_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vop1_pre_src_en(&mut self) -> AclkVop1PreSrcEnW<CruClkgateCon10Spec> {
        AclkVop1PreSrcEnW::new(self, 10)
    }
    #[doc = "Bit 11 - hclk_vop1_pre clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_vop1_pre_en(&mut self) -> HclkVop1PreEnW<CruClkgateCon10Spec> {
        HclkVop1PreEnW::new(self, 11)
    }
    #[doc = "Bit 12 - dclk_vop0_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn dclk_vop0_src_en(&mut self) -> DclkVop0SrcEnW<CruClkgateCon10Spec> {
        DclkVop0SrcEnW::new(self, 12)
    }
    #[doc = "Bit 13 - dclk_vop1_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn dclk_vop1_src_en(&mut self) -> DclkVop1SrcEnW<CruClkgateCon10Spec> {
        DclkVop1SrcEnW::new(self, 13)
    }
    #[doc = "Bit 14 - clk_vop0_pwm clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vop0_pwm_en(&mut self) -> ClkVop0PwmEnW<CruClkgateCon10Spec> {
        ClkVop0PwmEnW::new(self, 14)
    }
    #[doc = "Bit 15 - clk_vop1_pwm clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vop1_pwm_en(&mut self) -> ClkVop1PwmEnW<CruClkgateCon10Spec> {
        ClkVop1PwmEnW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon10Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register10\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con10::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con10::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon10Spec;
impl crate::RegisterSpec for CruClkgateCon10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con10::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon10Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con10::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon10Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON10 to value 0"]
impl crate::Resettable for CruClkgateCon10Spec {
    const RESET_VALUE: u32 = 0;
}
