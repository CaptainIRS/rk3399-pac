#[doc = "Register `SOFTRST_CON15` reader"]
pub type R = crate::R<SoftrstCon15Spec>;
#[doc = "Register `SOFTRST_CON15` writer"]
pub type W = crate::W<SoftrstCon15Spec>;
#[doc = "Field `ARESETN_VIO_NOC_REQ` reader - aresetn_vio_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVioNocReqR = crate::BitReader;
#[doc = "Field `ARESETN_VIO_NOC_REQ` writer - aresetn_vio_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnVioNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_HDCP_NOC_REQ` reader - aresetn_hdcp_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnHdcpNocReqR = crate::BitReader;
#[doc = "Field `ARESETN_HDCP_NOC_REQ` writer - aresetn_hdcp_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnHdcpNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_HDCP_REQ` reader - aresetn_hdcp request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnHdcpReqR = crate::BitReader;
#[doc = "Field `ARESETN_HDCP_REQ` writer - aresetn_hdcp request bit\n\nWhen HIGH, reset relative logic"]
pub type AresetnHdcpReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_HDCP_NOC_REQ` reader - hresetn_hdcp_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHdcpNocReqR = crate::BitReader;
#[doc = "Field `HRESETN_HDCP_NOC_REQ` writer - hresetn_hdcp_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHdcpNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_HDCP_REQ` reader - hresetn_hdcp request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHdcpReqR = crate::BitReader;
#[doc = "Field `HRESETN_HDCP_REQ` writer - hresetn_hdcp request bit\n\nWhen HIGH, reset relative logic"]
pub type HresetnHdcpReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_HDCP_NOC_REQ` reader - presetn_hdcp_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnHdcpNocReqR = crate::BitReader;
#[doc = "Field `PRESETN_HDCP_NOC_REQ` writer - presetn_hdcp_noc request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnHdcpNocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_HDCP_REQ` reader - presetn_hdcp request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnHdcpReqR = crate::BitReader;
#[doc = "Field `PRESETN_HDCP_REQ` writer - presetn_hdcp request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnHdcpReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_HDMI_CTRL_REQ` reader - presetn_hdmi_ctrl request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnHdmiCtrlReqR = crate::BitReader;
#[doc = "Field `PRESETN_HDMI_CTRL_REQ` writer - presetn_hdmi_ctrl request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnHdmiCtrlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_DP_CTRL_REQ` reader - presetn_dp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnDpCtrlReqR = crate::BitReader;
#[doc = "Field `PRESETN_DP_CTRL_REQ` writer - presetn_dp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnDpCtrlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRESETN_DP_CTRL_REQ` reader - sresetn_dp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
pub type SresetnDpCtrlReqR = crate::BitReader;
#[doc = "Field `SRESETN_DP_CTRL_REQ` writer - sresetn_dp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
pub type SresetnDpCtrlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRESETN_DP_CTRL_REQ` reader - cresetn_dp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
pub type CresetnDpCtrlReqR = crate::BitReader;
#[doc = "Field `CRESETN_DP_CTRL_REQ` writer - cresetn_dp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
pub type CresetnDpCtrlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_MIPI_DSI0_REQ` reader - presetn_mipi_dsi0 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnMipiDsi0ReqR = crate::BitReader;
#[doc = "Field `PRESETN_MIPI_DSI0_REQ` writer - presetn_mipi_dsi0 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnMipiDsi0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_MIPI_DSI1_REQ` reader - presetn_mipi_dsi1 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnMipiDsi1ReqR = crate::BitReader;
#[doc = "Field `PRESETN_MIPI_DSI1_REQ` writer - presetn_mipi_dsi1 request bit\n\nWhen HIGH, reset relative logic"]
pub type PresetnMipiDsi1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_DP_CORE_REQ` reader - resetn_dp_core request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDpCoreReqR = crate::BitReader;
#[doc = "Field `RESETN_DP_CORE_REQ` writer - resetn_dp_core request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDpCoreReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_DP_I2S_REQ` reader - resetn_dp_i2s request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDpI2sReqR = crate::BitReader;
#[doc = "Field `RESETN_DP_I2S_REQ` writer - resetn_dp_i2s request bit\n\nWhen HIGH, reset relative logic"]
pub type ResetnDpI2sReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aresetn_vio_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_vio_noc_req(&self) -> AresetnVioNocReqR {
        AresetnVioNocReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aresetn_hdcp_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_hdcp_noc_req(&self) -> AresetnHdcpNocReqR {
        AresetnHdcpNocReqR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - aresetn_hdcp request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_hdcp_req(&self) -> AresetnHdcpReqR {
        AresetnHdcpReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hresetn_hdcp_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_hdcp_noc_req(&self) -> HresetnHdcpNocReqR {
        HresetnHdcpNocReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - hresetn_hdcp request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_hdcp_req(&self) -> HresetnHdcpReqR {
        HresetnHdcpReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - presetn_hdcp_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_hdcp_noc_req(&self) -> PresetnHdcpNocReqR {
        PresetnHdcpNocReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - presetn_hdcp request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_hdcp_req(&self) -> PresetnHdcpReqR {
        PresetnHdcpReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - presetn_hdmi_ctrl request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_hdmi_ctrl_req(&self) -> PresetnHdmiCtrlReqR {
        PresetnHdmiCtrlReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - presetn_dp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_dp_ctrl_req(&self) -> PresetnDpCtrlReqR {
        PresetnDpCtrlReqR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - sresetn_dp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn sresetn_dp_ctrl_req(&self) -> SresetnDpCtrlReqR {
        SresetnDpCtrlReqR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - cresetn_dp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn cresetn_dp_ctrl_req(&self) -> CresetnDpCtrlReqR {
        CresetnDpCtrlReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - presetn_mipi_dsi0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_mipi_dsi0_req(&self) -> PresetnMipiDsi0ReqR {
        PresetnMipiDsi0ReqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - presetn_mipi_dsi1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_mipi_dsi1_req(&self) -> PresetnMipiDsi1ReqR {
        PresetnMipiDsi1ReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - resetn_dp_core request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_dp_core_req(&self) -> ResetnDpCoreReqR {
        ResetnDpCoreReqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - resetn_dp_i2s request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_dp_i2s_req(&self) -> ResetnDpI2sReqR {
        ResetnDpI2sReqR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aresetn_vio_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_vio_noc_req(&mut self) -> AresetnVioNocReqW<SoftrstCon15Spec> {
        AresetnVioNocReqW::new(self, 0)
    }
    #[doc = "Bit 1 - aresetn_hdcp_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_hdcp_noc_req(&mut self) -> AresetnHdcpNocReqW<SoftrstCon15Spec> {
        AresetnHdcpNocReqW::new(self, 1)
    }
    #[doc = "Bit 2 - aresetn_hdcp request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_hdcp_req(&mut self) -> AresetnHdcpReqW<SoftrstCon15Spec> {
        AresetnHdcpReqW::new(self, 2)
    }
    #[doc = "Bit 3 - hresetn_hdcp_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_hdcp_noc_req(&mut self) -> HresetnHdcpNocReqW<SoftrstCon15Spec> {
        HresetnHdcpNocReqW::new(self, 3)
    }
    #[doc = "Bit 4 - hresetn_hdcp request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_hdcp_req(&mut self) -> HresetnHdcpReqW<SoftrstCon15Spec> {
        HresetnHdcpReqW::new(self, 4)
    }
    #[doc = "Bit 5 - presetn_hdcp_noc request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_hdcp_noc_req(&mut self) -> PresetnHdcpNocReqW<SoftrstCon15Spec> {
        PresetnHdcpNocReqW::new(self, 5)
    }
    #[doc = "Bit 6 - presetn_hdcp request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_hdcp_req(&mut self) -> PresetnHdcpReqW<SoftrstCon15Spec> {
        PresetnHdcpReqW::new(self, 6)
    }
    #[doc = "Bit 7 - presetn_hdmi_ctrl request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_hdmi_ctrl_req(&mut self) -> PresetnHdmiCtrlReqW<SoftrstCon15Spec> {
        PresetnHdmiCtrlReqW::new(self, 7)
    }
    #[doc = "Bit 8 - presetn_dp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_dp_ctrl_req(&mut self) -> PresetnDpCtrlReqW<SoftrstCon15Spec> {
        PresetnDpCtrlReqW::new(self, 8)
    }
    #[doc = "Bit 9 - sresetn_dp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn sresetn_dp_ctrl_req(&mut self) -> SresetnDpCtrlReqW<SoftrstCon15Spec> {
        SresetnDpCtrlReqW::new(self, 9)
    }
    #[doc = "Bit 10 - cresetn_dp_ctrl request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn cresetn_dp_ctrl_req(&mut self) -> CresetnDpCtrlReqW<SoftrstCon15Spec> {
        CresetnDpCtrlReqW::new(self, 10)
    }
    #[doc = "Bit 11 - presetn_mipi_dsi0 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_mipi_dsi0_req(&mut self) -> PresetnMipiDsi0ReqW<SoftrstCon15Spec> {
        PresetnMipiDsi0ReqW::new(self, 11)
    }
    #[doc = "Bit 12 - presetn_mipi_dsi1 request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_mipi_dsi1_req(&mut self) -> PresetnMipiDsi1ReqW<SoftrstCon15Spec> {
        PresetnMipiDsi1ReqW::new(self, 12)
    }
    #[doc = "Bit 13 - resetn_dp_core request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_dp_core_req(&mut self) -> ResetnDpCoreReqW<SoftrstCon15Spec> {
        ResetnDpCoreReqW::new(self, 13)
    }
    #[doc = "Bit 14 - resetn_dp_i2s request bit\n\nWhen HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_dp_i2s_req(&mut self) -> ResetnDpI2sReqW<SoftrstCon15Spec> {
        ResetnDpI2sReqW::new(self, 14)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<SoftrstCon15Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register15\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`softrst_con15::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`softrst_con15::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SoftrstCon15Spec;
impl crate::RegisterSpec for SoftrstCon15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`softrst_con15::R`](R) reader structure"]
impl crate::Readable for SoftrstCon15Spec {}
#[doc = "`write(|w| ..)` method takes [`softrst_con15::W`](W) writer structure"]
impl crate::Writable for SoftrstCon15Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOFTRST_CON15 to value 0"]
impl crate::Resettable for SoftrstCon15Spec {
    const RESET_VALUE: u32 = 0;
}
