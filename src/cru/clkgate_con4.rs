#[doc = "Register `CLKGATE_CON4` reader"]
pub type R = crate::R<ClkgateCon4Spec>;
#[doc = "Register `CLKGATE_CON4` writer"]
pub type W = crate::W<ClkgateCon4Spec>;
#[doc = "Field `ACLK_VCODEC_SRC_EN` reader - aclk_vcodec_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkVcodecSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_VCODEC_SRC_EN` writer - aclk_vcodec_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkVcodecSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_VCODEC_SRC_EN` reader - hclk_vcodec_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkVcodecSrcEnR = crate::BitReader;
#[doc = "Field `HCLK_VCODEC_SRC_EN` writer - hclk_vcodec_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkVcodecSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_VDU_SRC_EN` reader - aclk_vdu_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkVduSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_VDU_SRC_EN` writer - aclk_vdu_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkVduSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_VDU_SRC_EN` reader - hclk_vdu_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkVduSrcEnR = crate::BitReader;
#[doc = "Field `HCLK_VDU_SRC_EN` writer - hclk_vdu_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkVduSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_VDU_CORE_SRC_EN` reader - clk_vdu_core_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkVduCoreSrcEnR = crate::BitReader;
#[doc = "Field `CLK_VDU_CORE_SRC_EN` writer - clk_vdu_core_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkVduCoreSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_VDU_CA_SRC_EN` reader - clk_vdu_ca_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkVduCaSrcEnR = crate::BitReader;
#[doc = "Field `CLK_VDU_CA_SRC_EN` writer - clk_vdu_ca_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkVduCaSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_IEP_SRC_EN` reader - aclk_iep_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkIepSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_IEP_SRC_EN` writer - aclk_iep_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkIepSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_IEP_SRC_EN` reader - hclk_iep_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkIepSrcEnR = crate::BitReader;
#[doc = "Field `HCLK_IEP_SRC_EN` writer - hclk_iep_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkIepSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_RGA_SRC_EN` reader - aclk_rga_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkRgaSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_RGA_SRC_EN` writer - aclk_rga_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkRgaSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_RGA_SRC_EN` reader - hclk_rga_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkRgaSrcEnR = crate::BitReader;
#[doc = "Field `HCLK_RGA_SRC_EN` writer - hclk_rga_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkRgaSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_RGA_CORE_SRC_EN` reader - clk_rga_core_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkRgaCoreSrcEnR = crate::BitReader;
#[doc = "Field `CLK_RGA_CORE_SRC_EN` writer - clk_rga_core_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkRgaCoreSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PVTM_DDR_EN` reader - clk_pvtm_ddr clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkPvtmDdrEnR = crate::BitReader;
#[doc = "Field `CLK_PVTM_DDR_EN` writer - clk_pvtm_ddr clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkPvtmDdrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_vcodec_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_vcodec_src_en(&self) -> AclkVcodecSrcEnR {
        AclkVcodecSrcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - hclk_vcodec_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_vcodec_src_en(&self) -> HclkVcodecSrcEnR {
        HclkVcodecSrcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - aclk_vdu_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_vdu_src_en(&self) -> AclkVduSrcEnR {
        AclkVduSrcEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hclk_vdu_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_vdu_src_en(&self) -> HclkVduSrcEnR {
        HclkVduSrcEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - clk_vdu_core_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_vdu_core_src_en(&self) -> ClkVduCoreSrcEnR {
        ClkVduCoreSrcEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_vdu_ca_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_vdu_ca_src_en(&self) -> ClkVduCaSrcEnR {
        ClkVduCaSrcEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - aclk_iep_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_iep_src_en(&self) -> AclkIepSrcEnR {
        AclkIepSrcEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - hclk_iep_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_iep_src_en(&self) -> HclkIepSrcEnR {
        HclkIepSrcEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - aclk_rga_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_rga_src_en(&self) -> AclkRgaSrcEnR {
        AclkRgaSrcEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - hclk_rga_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_rga_src_en(&self) -> HclkRgaSrcEnR {
        HclkRgaSrcEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - clk_rga_core_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_rga_core_src_en(&self) -> ClkRgaCoreSrcEnR {
        ClkRgaCoreSrcEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - clk_pvtm_ddr clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_pvtm_ddr_en(&self) -> ClkPvtmDdrEnR {
        ClkPvtmDdrEnR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_vcodec_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vcodec_src_en(&mut self) -> AclkVcodecSrcEnW<ClkgateCon4Spec> {
        AclkVcodecSrcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - hclk_vcodec_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_vcodec_src_en(&mut self) -> HclkVcodecSrcEnW<ClkgateCon4Spec> {
        HclkVcodecSrcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - aclk_vdu_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vdu_src_en(&mut self) -> AclkVduSrcEnW<ClkgateCon4Spec> {
        AclkVduSrcEnW::new(self, 2)
    }
    #[doc = "Bit 3 - hclk_vdu_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_vdu_src_en(&mut self) -> HclkVduSrcEnW<ClkgateCon4Spec> {
        HclkVduSrcEnW::new(self, 3)
    }
    #[doc = "Bit 4 - clk_vdu_core_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vdu_core_src_en(&mut self) -> ClkVduCoreSrcEnW<ClkgateCon4Spec> {
        ClkVduCoreSrcEnW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_vdu_ca_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vdu_ca_src_en(&mut self) -> ClkVduCaSrcEnW<ClkgateCon4Spec> {
        ClkVduCaSrcEnW::new(self, 5)
    }
    #[doc = "Bit 6 - aclk_iep_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_iep_src_en(&mut self) -> AclkIepSrcEnW<ClkgateCon4Spec> {
        AclkIepSrcEnW::new(self, 6)
    }
    #[doc = "Bit 7 - hclk_iep_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_iep_src_en(&mut self) -> HclkIepSrcEnW<ClkgateCon4Spec> {
        HclkIepSrcEnW::new(self, 7)
    }
    #[doc = "Bit 8 - aclk_rga_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_rga_src_en(&mut self) -> AclkRgaSrcEnW<ClkgateCon4Spec> {
        AclkRgaSrcEnW::new(self, 8)
    }
    #[doc = "Bit 9 - hclk_rga_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_rga_src_en(&mut self) -> HclkRgaSrcEnW<ClkgateCon4Spec> {
        HclkRgaSrcEnW::new(self, 9)
    }
    #[doc = "Bit 10 - clk_rga_core_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_rga_core_src_en(&mut self) -> ClkRgaCoreSrcEnW<ClkgateCon4Spec> {
        ClkRgaCoreSrcEnW::new(self, 10)
    }
    #[doc = "Bit 11 - clk_pvtm_ddr clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pvtm_ddr_en(&mut self) -> ClkPvtmDdrEnW<ClkgateCon4Spec> {
        ClkPvtmDdrEnW::new(self, 11)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkgateCon4Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkgateCon4Spec;
impl crate::RegisterSpec for ClkgateCon4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgate_con4::R`](R) reader structure"]
impl crate::Readable for ClkgateCon4Spec {}
#[doc = "`write(|w| ..)` method takes [`clkgate_con4::W`](W) writer structure"]
impl crate::Writable for ClkgateCon4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKGATE_CON4 to value 0"]
impl crate::Resettable for ClkgateCon4Spec {
    const RESET_VALUE: u32 = 0;
}
