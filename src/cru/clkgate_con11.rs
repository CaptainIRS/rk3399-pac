#[doc = "Register `CLKGATE_CON11` reader"]
pub type R = crate::R<ClkgateCon11Spec>;
#[doc = "Register `CLKGATE_CON11` writer"]
pub type W = crate::W<ClkgateCon11Spec>;
#[doc = "Field `ACLK_VIO_SRC_EN` reader - aclk_vio_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkVioSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_VIO_SRC_EN` writer - aclk_vio_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkVioSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_VIO_EN` reader - pclk_vio clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkVioEnR = crate::BitReader;
#[doc = "Field `PCLK_VIO_EN` writer - pclk_vio clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkVioEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_HDCP_SRC_EN` reader - aclk_hdcp_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkHdcpSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_HDCP_SRC_EN` writer - aclk_hdcp_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkHdcpSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_HDCP_EN` reader - hclk_hdcp clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkHdcpEnR = crate::BitReader;
#[doc = "Field `HCLK_HDCP_EN` writer - hclk_hdcp clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkHdcpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_ISP0_EN` reader - clk_isp0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIsp0EnR = crate::BitReader;
#[doc = "Field `CLK_ISP0_EN` writer - clk_isp0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIsp0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_ISP1_EN` reader - clk_isp1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIsp1EnR = crate::BitReader;
#[doc = "Field `CLK_ISP1_EN` writer - clk_isp1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkIsp1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_HDMI_SFR_EN` reader - clk_hdmi_sfr clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkHdmiSfrEnR = crate::BitReader;
#[doc = "Field `CLK_HDMI_SFR_EN` writer - clk_hdmi_sfr clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkHdmiSfrEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_HDMI_CEC_EN` reader - clk_hdmi_cec clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkHdmiCecEnR = crate::BitReader;
#[doc = "Field `CLK_HDMI_CEC_EN` writer - clk_hdmi_cec clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkHdmiCecEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_DP_CORE_SRC_EN` reader - clk_dp_core_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkDpCoreSrcEnR = crate::BitReader;
#[doc = "Field `CLK_DP_CORE_SRC_EN` writer - clk_dp_core_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkDpCoreSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_HDCP_EN` reader - pclk_hdcp clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkHdcpEnR = crate::BitReader;
#[doc = "Field `PCLK_HDCP_EN` writer - pclk_hdcp clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkHdcpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_EDP_EN` reader - pclk_edp clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkEdpEnR = crate::BitReader;
#[doc = "Field `PCLK_EDP_EN` writer - pclk_edp clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkEdpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MIPIDPHY_REF_EN` reader - clk_mipidphy_ref clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkMipidphyRefEnR = crate::BitReader;
#[doc = "Field `CLK_MIPIDPHY_REF_EN` writer - clk_mipidphy_ref clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkMipidphyRefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MIPIDPHY_CFG_EN` reader - clk_mipidphy_cfg clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkMipidphyCfgEnR = crate::BitReader;
#[doc = "Field `CLK_MIPIDPHY_CFG_EN` writer - clk_mipidphy_cfg clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkMipidphyCfgEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_vio_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_vio_src_en(&self) -> AclkVioSrcEnR {
        AclkVioSrcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pclk_vio clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_vio_en(&self) -> PclkVioEnR {
        PclkVioEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - aclk_hdcp_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_hdcp_src_en(&self) -> AclkHdcpSrcEnR {
        AclkHdcpSrcEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hclk_hdcp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_hdcp_en(&self) -> HclkHdcpEnR {
        HclkHdcpEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - clk_isp0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_isp0_en(&self) -> ClkIsp0EnR {
        ClkIsp0EnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_isp1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_isp1_en(&self) -> ClkIsp1EnR {
        ClkIsp1EnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_hdmi_sfr clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_hdmi_sfr_en(&self) -> ClkHdmiSfrEnR {
        ClkHdmiSfrEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - clk_hdmi_cec clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_hdmi_cec_en(&self) -> ClkHdmiCecEnR {
        ClkHdmiCecEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - clk_dp_core_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_dp_core_src_en(&self) -> ClkDpCoreSrcEnR {
        ClkDpCoreSrcEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 10 - pclk_hdcp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_hdcp_en(&self) -> PclkHdcpEnR {
        PclkHdcpEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - pclk_edp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_edp_en(&self) -> PclkEdpEnR {
        PclkEdpEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - clk_mipidphy_ref clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_mipidphy_ref_en(&self) -> ClkMipidphyRefEnR {
        ClkMipidphyRefEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - clk_mipidphy_cfg clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_mipidphy_cfg_en(&self) -> ClkMipidphyCfgEnR {
        ClkMipidphyCfgEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_vio_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vio_src_en(&mut self) -> AclkVioSrcEnW<ClkgateCon11Spec> {
        AclkVioSrcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - pclk_vio clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_vio_en(&mut self) -> PclkVioEnW<ClkgateCon11Spec> {
        PclkVioEnW::new(self, 1)
    }
    #[doc = "Bit 2 - aclk_hdcp_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_hdcp_src_en(&mut self) -> AclkHdcpSrcEnW<ClkgateCon11Spec> {
        AclkHdcpSrcEnW::new(self, 2)
    }
    #[doc = "Bit 3 - hclk_hdcp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_hdcp_en(&mut self) -> HclkHdcpEnW<ClkgateCon11Spec> {
        HclkHdcpEnW::new(self, 3)
    }
    #[doc = "Bit 4 - clk_isp0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_isp0_en(&mut self) -> ClkIsp0EnW<ClkgateCon11Spec> {
        ClkIsp0EnW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_isp1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_isp1_en(&mut self) -> ClkIsp1EnW<ClkgateCon11Spec> {
        ClkIsp1EnW::new(self, 5)
    }
    #[doc = "Bit 6 - clk_hdmi_sfr clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_hdmi_sfr_en(&mut self) -> ClkHdmiSfrEnW<ClkgateCon11Spec> {
        ClkHdmiSfrEnW::new(self, 6)
    }
    #[doc = "Bit 7 - clk_hdmi_cec clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_hdmi_cec_en(&mut self) -> ClkHdmiCecEnW<ClkgateCon11Spec> {
        ClkHdmiCecEnW::new(self, 7)
    }
    #[doc = "Bit 8 - clk_dp_core_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_dp_core_src_en(&mut self) -> ClkDpCoreSrcEnW<ClkgateCon11Spec> {
        ClkDpCoreSrcEnW::new(self, 8)
    }
    #[doc = "Bit 10 - pclk_hdcp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_hdcp_en(&mut self) -> PclkHdcpEnW<ClkgateCon11Spec> {
        PclkHdcpEnW::new(self, 10)
    }
    #[doc = "Bit 11 - pclk_edp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_edp_en(&mut self) -> PclkEdpEnW<ClkgateCon11Spec> {
        PclkEdpEnW::new(self, 11)
    }
    #[doc = "Bit 14 - clk_mipidphy_ref clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mipidphy_ref_en(&mut self) -> ClkMipidphyRefEnW<ClkgateCon11Spec> {
        ClkMipidphyRefEnW::new(self, 14)
    }
    #[doc = "Bit 15 - clk_mipidphy_cfg clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mipidphy_cfg_en(&mut self) -> ClkMipidphyCfgEnW<ClkgateCon11Spec> {
        ClkMipidphyCfgEnW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkgateCon11Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register11\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con11::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con11::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkgateCon11Spec;
impl crate::RegisterSpec for ClkgateCon11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgate_con11::R`](R) reader structure"]
impl crate::Readable for ClkgateCon11Spec {}
#[doc = "`write(|w| ..)` method takes [`clkgate_con11::W`](W) writer structure"]
impl crate::Writable for ClkgateCon11Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKGATE_CON11 to value 0"]
impl crate::Resettable for ClkgateCon11Spec {
    const RESET_VALUE: u32 = 0;
}
