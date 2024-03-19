#[doc = "Register `CRU_CLKGATE_CON29` reader"]
pub type R = crate::R<CruClkgateCon29Spec>;
#[doc = "Register `CRU_CLKGATE_CON29` writer"]
pub type W = crate::W<CruClkgateCon29Spec>;
#[doc = "Field `ACLK_VIO_NOC_EN` reader - aclk_vio_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkVioNocEnR = crate::BitReader;
#[doc = "Field `ACLK_VIO_NOC_EN` writer - aclk_vio_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkVioNocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_MIPI_DSI0_EN` reader - pclk_mipi_dsi0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkMipiDsi0EnR = crate::BitReader;
#[doc = "Field `PCLK_MIPI_DSI0_EN` writer - pclk_mipi_dsi0 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkMipiDsi0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_MIPI_DSI1_EN` reader - pclk_mipi_dsi1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkMipiDsi1EnR = crate::BitReader;
#[doc = "Field `PCLK_MIPI_DSI1_EN` writer - pclk_mipi_dsi1 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkMipiDsi1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_HDCPNOC_EN` reader - pclk_hdcpnoc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type PclkHdcpnocEnR = crate::BitReader;
#[doc = "Field `PCLK_HDCPNOC_EN` writer - pclk_hdcpnoc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type PclkHdcpnocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_HDCPNOC_EN` reader - aclk_hdcpnoc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkHdcpnocEnR = crate::BitReader;
#[doc = "Field `ACLK_HDCPNOC_EN` writer - aclk_hdcpnoc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type AclkHdcpnocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_HDCPNOC_EN` reader - hclk_hdcpnoc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type HclkHdcpnocEnR = crate::BitReader;
#[doc = "Field `HCLK_HDCPNOC_EN` writer - hclk_hdcpnoc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type HclkHdcpnocEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_HDMI_CTRL_EN` reader - pclk_hdmi_ctrl clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkHdmiCtrlEnR = crate::BitReader;
#[doc = "Field `PCLK_HDMI_CTRL_EN` writer - pclk_hdmi_ctrl clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkHdmiCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_DP_CTRL_EN` reader - pclk_dp_ctrl clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkDpCtrlEnR = crate::BitReader;
#[doc = "Field `PCLK_DP_CTRL_EN` writer - pclk_dp_ctrl clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkDpCtrlEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_HDCP22_EN` reader - pclk_hdcp22 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkHdcp22EnR = crate::BitReader;
#[doc = "Field `PCLK_HDCP22_EN` writer - pclk_hdcp22 clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkHdcp22EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_HDCP22_EN` reader - hclk_hdcp22 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkHdcp22EnR = crate::BitReader;
#[doc = "Field `HCLK_HDCP22_EN` writer - hclk_hdcp22 clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkHdcp22EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_HDCP22_EN` reader - aclk_hdcp22 clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkHdcp22EnR = crate::BitReader;
#[doc = "Field `ACLK_HDCP22_EN` writer - aclk_hdcp22 clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkHdcp22EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_GASKET_EN` reader - pclk_gasket clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkGasketEnR = crate::BitReader;
#[doc = "Field `PCLK_GASKET_EN` writer - pclk_gasket clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkGasketEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_VIO_GRF_EN` reader - pclk_vio_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type PclkVioGrfEnR = crate::BitReader;
#[doc = "Field `PCLK_VIO_GRF_EN` writer - pclk_vio_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
pub type PclkVioGrfEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_vio_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn aclk_vio_noc_en(&self) -> AclkVioNocEnR {
        AclkVioNocEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - pclk_mipi_dsi0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_mipi_dsi0_en(&self) -> PclkMipiDsi0EnR {
        PclkMipiDsi0EnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - pclk_mipi_dsi1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_mipi_dsi1_en(&self) -> PclkMipiDsi1EnR {
        PclkMipiDsi1EnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - pclk_hdcpnoc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn pclk_hdcpnoc_en(&self) -> PclkHdcpnocEnR {
        PclkHdcpnocEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - aclk_hdcpnoc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn aclk_hdcpnoc_en(&self) -> AclkHdcpnocEnR {
        AclkHdcpnocEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - hclk_hdcpnoc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn hclk_hdcpnoc_en(&self) -> HclkHdcpnocEnR {
        HclkHdcpnocEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - pclk_hdmi_ctrl clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_hdmi_ctrl_en(&self) -> PclkHdmiCtrlEnR {
        PclkHdmiCtrlEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - pclk_dp_ctrl clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_dp_ctrl_en(&self) -> PclkDpCtrlEnR {
        PclkDpCtrlEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - pclk_hdcp22 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_hdcp22_en(&self) -> PclkHdcp22EnR {
        PclkHdcp22EnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - hclk_hdcp22 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_hdcp22_en(&self) -> HclkHdcp22EnR {
        HclkHdcp22EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - aclk_hdcp22 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_hdcp22_en(&self) -> AclkHdcp22EnR {
        AclkHdcp22EnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - pclk_gasket clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_gasket_en(&self) -> PclkGasketEnR {
        PclkGasketEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - pclk_vio_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    pub fn pclk_vio_grf_en(&self) -> PclkVioGrfEnR {
        PclkVioGrfEnR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_vio_noc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_vio_noc_en(&mut self) -> AclkVioNocEnW<CruClkgateCon29Spec> {
        AclkVioNocEnW::new(self, 0)
    }
    #[doc = "Bit 1 - pclk_mipi_dsi0 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_mipi_dsi0_en(&mut self) -> PclkMipiDsi0EnW<CruClkgateCon29Spec> {
        PclkMipiDsi0EnW::new(self, 1)
    }
    #[doc = "Bit 2 - pclk_mipi_dsi1 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_mipi_dsi1_en(&mut self) -> PclkMipiDsi1EnW<CruClkgateCon29Spec> {
        PclkMipiDsi1EnW::new(self, 2)
    }
    #[doc = "Bit 3 - pclk_hdcpnoc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_hdcpnoc_en(&mut self) -> PclkHdcpnocEnW<CruClkgateCon29Spec> {
        PclkHdcpnocEnW::new(self, 3)
    }
    #[doc = "Bit 4 - aclk_hdcpnoc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_hdcpnoc_en(&mut self) -> AclkHdcpnocEnW<CruClkgateCon29Spec> {
        AclkHdcpnocEnW::new(self, 4)
    }
    #[doc = "Bit 5 - hclk_hdcpnoc clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_hdcpnoc_en(&mut self) -> HclkHdcpnocEnW<CruClkgateCon29Spec> {
        HclkHdcpnocEnW::new(self, 5)
    }
    #[doc = "Bit 6 - pclk_hdmi_ctrl clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_hdmi_ctrl_en(&mut self) -> PclkHdmiCtrlEnW<CruClkgateCon29Spec> {
        PclkHdmiCtrlEnW::new(self, 6)
    }
    #[doc = "Bit 7 - pclk_dp_ctrl clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_dp_ctrl_en(&mut self) -> PclkDpCtrlEnW<CruClkgateCon29Spec> {
        PclkDpCtrlEnW::new(self, 7)
    }
    #[doc = "Bit 8 - pclk_hdcp22 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_hdcp22_en(&mut self) -> PclkHdcp22EnW<CruClkgateCon29Spec> {
        PclkHdcp22EnW::new(self, 8)
    }
    #[doc = "Bit 9 - hclk_hdcp22 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_hdcp22_en(&mut self) -> HclkHdcp22EnW<CruClkgateCon29Spec> {
        HclkHdcp22EnW::new(self, 9)
    }
    #[doc = "Bit 10 - aclk_hdcp22 clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_hdcp22_en(&mut self) -> AclkHdcp22EnW<CruClkgateCon29Spec> {
        AclkHdcp22EnW::new(self, 10)
    }
    #[doc = "Bit 11 - pclk_gasket clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_gasket_en(&mut self) -> PclkGasketEnW<CruClkgateCon29Spec> {
        PclkGasketEnW::new(self, 11)
    }
    #[doc = "Bit 12 - pclk_vio_grf clock disable bit\n\nWhen HIGH, disable clock\n\nSuggest always on"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_vio_grf_en(&mut self) -> PclkVioGrfEnW<CruClkgateCon29Spec> {
        PclkVioGrfEnW::new(self, 12)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon29Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register29\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con29::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con29::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon29Spec;
impl crate::RegisterSpec for CruClkgateCon29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con29::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon29Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con29::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon29Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON29 to value 0"]
impl crate::Resettable for CruClkgateCon29Spec {
    const RESET_VALUE: u32 = 0;
}
