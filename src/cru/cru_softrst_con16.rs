#[doc = "Register `CRU_SOFTRST_CON16` reader"]
pub type R = crate::R<CruSoftrstCon16Spec>;
#[doc = "Register `CRU_SOFTRST_CON16` writer"]
pub type W = crate::W<CruSoftrstCon16Spec>;
#[doc = "Field `PRESETN_GASKET_REQ` reader - presetn_gasket request bit When HIGH, reset relative logic"]
pub type PresetnGasketReqR = crate::BitReader;
#[doc = "Field `PRESETN_GASKET_REQ` writer - presetn_gasket request bit When HIGH, reset relative logic"]
pub type PresetnGasketReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESETN_VIO_GRF_REQ` reader - presetn_vio_grf request bit When HIGH, reset relative logic"]
pub type PresetnVioGrfReqR = crate::BitReader;
#[doc = "Field `PRESETN_VIO_GRF_REQ` writer - presetn_vio_grf request bit When HIGH, reset relative logic"]
pub type PresetnVioGrfReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_DPTX_SPDIF_REC_REQ` reader - resetn_dptx_spdif_rec request bit When HIGH, reset relative logic"]
pub type ResetnDptxSpdifRecReqR = crate::BitReader;
#[doc = "Field `RESETN_DPTX_SPDIF_REC_REQ` writer - resetn_dptx_spdif_rec request bit When HIGH, reset relative logic"]
pub type ResetnDptxSpdifRecReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_HDMI_CTRL_REQ` reader - resetn_hdmi_ctrl request bit When HIGH, reset relative logic"]
pub type ResetnHdmiCtrlReqR = crate::BitReader;
#[doc = "Field `RESETN_HDMI_CTRL_REQ` writer - resetn_hdmi_ctrl request bit When HIGH, reset relative logic"]
pub type ResetnHdmiCtrlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_HDCP_CTRL_REQ` reader - resetn_hdcp_ctrl request bit When HIGH, reset relative logic"]
pub type ResetnHdcpCtrlReqR = crate::BitReader;
#[doc = "Field `RESETN_HDCP_CTRL_REQ` writer - resetn_hdcp_ctrl request bit When HIGH, reset relative logic"]
pub type ResetnHdcpCtrlReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_ISP0_NOC_REQ` reader - aresetn_isp0_noc request bit When HIGH, reset relative logic"]
pub type AresetnIsp0NocReqR = crate::BitReader;
#[doc = "Field `ARESETN_ISP0_NOC_REQ` writer - aresetn_isp0_noc request bit When HIGH, reset relative logic"]
pub type AresetnIsp0NocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARESETN_ISP1_NOC_REQ` reader - aresetn_isp1_noc request bit When HIGH, reset relative logic"]
pub type AresetnIsp1NocReqR = crate::BitReader;
#[doc = "Field `ARESETN_ISP1_NOC_REQ` writer - aresetn_isp1_noc request bit When HIGH, reset relative logic"]
pub type AresetnIsp1NocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_ISP0_NOC_REQ` reader - hresetn_isp0_noc request bit When HIGH, reset relative logic"]
pub type HresetnIsp0NocReqR = crate::BitReader;
#[doc = "Field `HRESETN_ISP0_NOC_REQ` writer - hresetn_isp0_noc request bit When HIGH, reset relative logic"]
pub type HresetnIsp0NocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_ISP1_NOC_REQ` reader - hresetn_isp1_noc request bit When HIGH, reset relative logic"]
pub type HresetnIsp1NocReqR = crate::BitReader;
#[doc = "Field `HRESETN_ISP1_NOC_REQ` writer - hresetn_isp1_noc request bit When HIGH, reset relative logic"]
pub type HresetnIsp1NocReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_ISP0_REQ` reader - hresetn_isp0 request bit When HIGH, reset relative logic"]
pub type HresetnIsp0ReqR = crate::BitReader;
#[doc = "Field `HRESETN_ISP0_REQ` writer - hresetn_isp0 request bit When HIGH, reset relative logic"]
pub type HresetnIsp0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRESETN_ISP1_REQ` reader - hresetn_isp1 request bit When HIGH, reset relative logic"]
pub type HresetnIsp1ReqR = crate::BitReader;
#[doc = "Field `HRESETN_ISP1_REQ` writer - hresetn_isp1 request bit When HIGH, reset relative logic"]
pub type HresetnIsp1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_ISP0_REQ` reader - resetn_isp0 request bit When HIGH, reset relative logic"]
pub type ResetnIsp0ReqR = crate::BitReader;
#[doc = "Field `RESETN_ISP0_REQ` writer - resetn_isp0 request bit When HIGH, reset relative logic"]
pub type ResetnIsp0ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RESETN_ISP1_REQ` reader - resetn_isp1 request bit When HIGH, reset relative logic"]
pub type ResetnIsp1ReqR = crate::BitReader;
#[doc = "Field `RESETN_ISP1_REQ` writer - resetn_isp1 request bit When HIGH, reset relative logic"]
pub type ResetnIsp1ReqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - presetn_gasket request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_gasket_req(&self) -> PresetnGasketReqR {
        PresetnGasketReqR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - presetn_vio_grf request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn presetn_vio_grf_req(&self) -> PresetnVioGrfReqR {
        PresetnVioGrfReqR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - resetn_dptx_spdif_rec request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_dptx_spdif_rec_req(&self) -> ResetnDptxSpdifRecReqR {
        ResetnDptxSpdifRecReqR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - resetn_hdmi_ctrl request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_hdmi_ctrl_req(&self) -> ResetnHdmiCtrlReqR {
        ResetnHdmiCtrlReqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - resetn_hdcp_ctrl request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_hdcp_ctrl_req(&self) -> ResetnHdcpCtrlReqR {
        ResetnHdcpCtrlReqR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - aresetn_isp0_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_isp0_noc_req(&self) -> AresetnIsp0NocReqR {
        AresetnIsp0NocReqR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - aresetn_isp1_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn aresetn_isp1_noc_req(&self) -> AresetnIsp1NocReqR {
        AresetnIsp1NocReqR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - hresetn_isp0_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_isp0_noc_req(&self) -> HresetnIsp0NocReqR {
        HresetnIsp0NocReqR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - hresetn_isp1_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_isp1_noc_req(&self) -> HresetnIsp1NocReqR {
        HresetnIsp1NocReqR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - hresetn_isp0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_isp0_req(&self) -> HresetnIsp0ReqR {
        HresetnIsp0ReqR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - hresetn_isp1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn hresetn_isp1_req(&self) -> HresetnIsp1ReqR {
        HresetnIsp1ReqR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - resetn_isp0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_isp0_req(&self) -> ResetnIsp0ReqR {
        ResetnIsp0ReqR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - resetn_isp1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    pub fn resetn_isp1_req(&self) -> ResetnIsp1ReqR {
        ResetnIsp1ReqR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - presetn_gasket request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_gasket_req(&mut self) -> PresetnGasketReqW<CruSoftrstCon16Spec> {
        PresetnGasketReqW::new(self, 0)
    }
    #[doc = "Bit 2 - presetn_vio_grf request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn presetn_vio_grf_req(&mut self) -> PresetnVioGrfReqW<CruSoftrstCon16Spec> {
        PresetnVioGrfReqW::new(self, 2)
    }
    #[doc = "Bit 3 - resetn_dptx_spdif_rec request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_dptx_spdif_rec_req(&mut self) -> ResetnDptxSpdifRecReqW<CruSoftrstCon16Spec> {
        ResetnDptxSpdifRecReqW::new(self, 3)
    }
    #[doc = "Bit 4 - resetn_hdmi_ctrl request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_hdmi_ctrl_req(&mut self) -> ResetnHdmiCtrlReqW<CruSoftrstCon16Spec> {
        ResetnHdmiCtrlReqW::new(self, 4)
    }
    #[doc = "Bit 5 - resetn_hdcp_ctrl request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_hdcp_ctrl_req(&mut self) -> ResetnHdcpCtrlReqW<CruSoftrstCon16Spec> {
        ResetnHdcpCtrlReqW::new(self, 5)
    }
    #[doc = "Bit 6 - aresetn_isp0_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_isp0_noc_req(&mut self) -> AresetnIsp0NocReqW<CruSoftrstCon16Spec> {
        AresetnIsp0NocReqW::new(self, 6)
    }
    #[doc = "Bit 7 - aresetn_isp1_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn aresetn_isp1_noc_req(&mut self) -> AresetnIsp1NocReqW<CruSoftrstCon16Spec> {
        AresetnIsp1NocReqW::new(self, 7)
    }
    #[doc = "Bit 10 - hresetn_isp0_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_isp0_noc_req(&mut self) -> HresetnIsp0NocReqW<CruSoftrstCon16Spec> {
        HresetnIsp0NocReqW::new(self, 10)
    }
    #[doc = "Bit 11 - hresetn_isp1_noc request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_isp1_noc_req(&mut self) -> HresetnIsp1NocReqW<CruSoftrstCon16Spec> {
        HresetnIsp1NocReqW::new(self, 11)
    }
    #[doc = "Bit 12 - hresetn_isp0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_isp0_req(&mut self) -> HresetnIsp0ReqW<CruSoftrstCon16Spec> {
        HresetnIsp0ReqW::new(self, 12)
    }
    #[doc = "Bit 13 - hresetn_isp1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn hresetn_isp1_req(&mut self) -> HresetnIsp1ReqW<CruSoftrstCon16Spec> {
        HresetnIsp1ReqW::new(self, 13)
    }
    #[doc = "Bit 14 - resetn_isp0 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_isp0_req(&mut self) -> ResetnIsp0ReqW<CruSoftrstCon16Spec> {
        ResetnIsp0ReqW::new(self, 14)
    }
    #[doc = "Bit 15 - resetn_isp1 request bit When HIGH, reset relative logic"]
    #[inline(always)]
    #[must_use]
    pub fn resetn_isp1_req(&mut self) -> ResetnIsp1ReqW<CruSoftrstCon16Spec> {
        ResetnIsp1ReqW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruSoftrstCon16Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal software reset control register16\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_softrst_con16::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_softrst_con16::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruSoftrstCon16Spec;
impl crate::RegisterSpec for CruSoftrstCon16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_softrst_con16::R`](R) reader structure"]
impl crate::Readable for CruSoftrstCon16Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_softrst_con16::W`](W) writer structure"]
impl crate::Writable for CruSoftrstCon16Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_SOFTRST_CON16 to value 0"]
impl crate::Resettable for CruSoftrstCon16Spec {
    const RESET_VALUE: u32 = 0;
}
