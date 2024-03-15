#[doc = "Register `PMUCRU_GATEDIS_CON0` reader"]
pub type R = crate::R<PmucruGatedisCon0Spec>;
#[doc = "Register `PMUCRU_GATEDIS_CON0` writer"]
pub type W = crate::W<PmucruGatedisCon0Spec>;
#[doc = "Field `CLK_PMUM0_GATING_DIS` reader - clk_pmum0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkPmum0GatingDisR = crate::BitReader;
#[doc = "Field `CLK_PMUM0_GATING_DIS` writer - clk_pmum0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkPmum0GatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CENTER1_GATING_DIS` reader - clk_center1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkCenter1GatingDisR = crate::BitReader;
#[doc = "Field `CLK_CENTER1_GATING_DIS` writer - clk_center1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkCenter1GatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EMMC_GATING_DIS` reader - clk_emmc gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkEmmcGatingDisR = crate::BitReader;
#[doc = "Field `CLK_EMMC_GATING_DIS` writer - clk_emmc gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkEmmcGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_GMAC_GATING_DIS` reader - clk_gmac gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkGmacGatingDisR = crate::BitReader;
#[doc = "Field `CLK_GMAC_GATING_DIS` writer - clk_gmac gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkGmacGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EDP_GATING_DIS` reader - clk_edp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkEdpGatingDisR = crate::BitReader;
#[doc = "Field `CLK_EDP_GATING_DIS` writer - clk_edp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkEdpGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PMU_GATING_DIS` reader - clk_pmu gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkPmuGatingDisR = crate::BitReader;
#[doc = "Field `CLK_PMU_GATING_DIS` writer - clk_pmu gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkPmuGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_ALIVE_GATING_DIS` reader - clk_alive gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkAliveGatingDisR = crate::BitReader;
#[doc = "Field `CLK_ALIVE_GATING_DIS` writer - clk_alive gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkAliveGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MSCH1_GATING_DIS` reader - clk_msch1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkMsch1GatingDisR = crate::BitReader;
#[doc = "Field `CLK_MSCH1_GATING_DIS` writer - clk_msch1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkMsch1GatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MSCH0_GATING_DIS` reader - clk_msch0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkMsch0GatingDisR = crate::BitReader;
#[doc = "Field `CLK_MSCH0_GATING_DIS` writer - clk_msch0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkMsch0GatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_VIO_GATING_DIS` reader - clk_vio gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkVioGatingDisR = crate::BitReader;
#[doc = "Field `CLK_VIO_GATING_DIS` writer - clk_vio gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkVioGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CCIM1_GATING_DIS` reader - clk_ccim1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkCcim1GatingDisR = crate::BitReader;
#[doc = "Field `CLK_CCIM1_GATING_DIS` writer - clk_ccim1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkCcim1GatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CCIM0_GATING_DIS` reader - clk_ccim0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkCcim0GatingDisR = crate::BitReader;
#[doc = "Field `CLK_CCIM0_GATING_DIS` writer - clk_ccim0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkCcim0GatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_CENTER_GATING_DIS` reader - clk_center gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkCenterGatingDisR = crate::BitReader;
#[doc = "Field `CLK_CENTER_GATING_DIS` writer - clk_center gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkCenterGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PERILPM0_GATING_DIS` reader - clk_perilpm0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkPerilpm0GatingDisR = crate::BitReader;
#[doc = "Field `CLK_PERILPM0_GATING_DIS` writer - clk_perilpm0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkPerilpm0GatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_USB3_GATING_DIS` reader - clk_usb3 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkUsb3GatingDisR = crate::BitReader;
#[doc = "Field `CLK_USB3_GATING_DIS` writer - clk_usb3 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkUsb3GatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_HDCP_GATING_DIS` reader - clk_hdcp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkHdcpGatingDisR = crate::BitReader;
#[doc = "Field `CLK_HDCP_GATING_DIS` writer - clk_hdcp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkHdcpGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_ISP1_GATING_DIS` reader - clk_isp1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkIsp1GatingDisR = crate::BitReader;
#[doc = "Field `CLK_ISP1_GATING_DIS` writer - clk_isp1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkIsp1GatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_ISP0_GATING_DIS` reader - clk_isp0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkIsp0GatingDisR = crate::BitReader;
#[doc = "Field `CLK_ISP0_GATING_DIS` writer - clk_isp0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkIsp0GatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_VOPL_GATING_DIS` reader - clk_vopl gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkVoplGatingDisR = crate::BitReader;
#[doc = "Field `CLK_VOPL_GATING_DIS` writer - clk_vopl gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkVoplGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_VOPB_GATING_DIS` reader - clk_vopb gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkVopbGatingDisR = crate::BitReader;
#[doc = "Field `CLK_VOPB_GATING_DIS` writer - clk_vopb gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkVopbGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_IEP_GATING_DIS` reader - clk_iep gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkIepGatingDisR = crate::BitReader;
#[doc = "Field `CLK_IEP_GATING_DIS` writer - clk_iep gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkIepGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_RGA_GATING_DIS` reader - clk_rga gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkRgaGatingDisR = crate::BitReader;
#[doc = "Field `CLK_RGA_GATING_DIS` writer - clk_rga gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkRgaGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_VDU_GATING_DIS` reader - clk_vdu gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkVduGatingDisR = crate::BitReader;
#[doc = "Field `CLK_VDU_GATING_DIS` writer - clk_vdu gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkVduGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_VCODEC_GATING_DIS` reader - clk_vcodec gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkVcodecGatingDisR = crate::BitReader;
#[doc = "Field `CLK_VCODEC_GATING_DIS` writer - clk_vcodec gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkVcodecGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PERIHP_GATING_DIS` reader - clk_perihp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkPerihpGatingDisR = crate::BitReader;
#[doc = "Field `CLK_PERIHP_GATING_DIS` writer - clk_perihp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkPerihpGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PERILP_GATING_DIS` reader - clk_perilp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkPerilpGatingDisR = crate::BitReader;
#[doc = "Field `CLK_PERILP_GATING_DIS` writer - clk_perilp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkPerilpGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_GPU_GATING_DIS` reader - clk_gpu gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkGpuGatingDisR = crate::BitReader;
#[doc = "Field `CLK_GPU_GATING_DIS` writer - clk_gpu gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkGpuGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_GIC_GATING_DIS` reader - clk_gic gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkGicGatingDisR = crate::BitReader;
#[doc = "Field `CLK_GIC_GATING_DIS` writer - clk_gic gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkGicGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SD_GATING_DIS` reader - clk_sd gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkSdGatingDisR = crate::BitReader;
#[doc = "Field `CLK_SD_GATING_DIS` writer - clk_sd gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkSdGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SDIOAUDIO_GATING_DIS` reader - clk_sdioaudio gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkSdioaudioGatingDisR = crate::BitReader;
#[doc = "Field `CLK_SDIOAUDIO_GATING_DIS` writer - clk_sdioaudio gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
pub type ClkSdioaudioGatingDisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - clk_pmum0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_pmum0_gating_dis(&self) -> ClkPmum0GatingDisR {
        ClkPmum0GatingDisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clk_center1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_center1_gating_dis(&self) -> ClkCenter1GatingDisR {
        ClkCenter1GatingDisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clk_emmc gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_emmc_gating_dis(&self) -> ClkEmmcGatingDisR {
        ClkEmmcGatingDisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - clk_gmac gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_gmac_gating_dis(&self) -> ClkGmacGatingDisR {
        ClkGmacGatingDisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - clk_edp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_edp_gating_dis(&self) -> ClkEdpGatingDisR {
        ClkEdpGatingDisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_pmu gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_pmu_gating_dis(&self) -> ClkPmuGatingDisR {
        ClkPmuGatingDisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_alive gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_alive_gating_dis(&self) -> ClkAliveGatingDisR {
        ClkAliveGatingDisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - clk_msch1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_msch1_gating_dis(&self) -> ClkMsch1GatingDisR {
        ClkMsch1GatingDisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - clk_msch0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_msch0_gating_dis(&self) -> ClkMsch0GatingDisR {
        ClkMsch0GatingDisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - clk_vio gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_vio_gating_dis(&self) -> ClkVioGatingDisR {
        ClkVioGatingDisR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - clk_ccim1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_ccim1_gating_dis(&self) -> ClkCcim1GatingDisR {
        ClkCcim1GatingDisR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - clk_ccim0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_ccim0_gating_dis(&self) -> ClkCcim0GatingDisR {
        ClkCcim0GatingDisR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - clk_center gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_center_gating_dis(&self) -> ClkCenterGatingDisR {
        ClkCenterGatingDisR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - clk_perilpm0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_perilpm0_gating_dis(&self) -> ClkPerilpm0GatingDisR {
        ClkPerilpm0GatingDisR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - clk_usb3 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_usb3_gating_dis(&self) -> ClkUsb3GatingDisR {
        ClkUsb3GatingDisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - clk_hdcp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_hdcp_gating_dis(&self) -> ClkHdcpGatingDisR {
        ClkHdcpGatingDisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - clk_isp1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_isp1_gating_dis(&self) -> ClkIsp1GatingDisR {
        ClkIsp1GatingDisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - clk_isp0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_isp0_gating_dis(&self) -> ClkIsp0GatingDisR {
        ClkIsp0GatingDisR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - clk_vopl gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_vopl_gating_dis(&self) -> ClkVoplGatingDisR {
        ClkVoplGatingDisR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - clk_vopb gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_vopb_gating_dis(&self) -> ClkVopbGatingDisR {
        ClkVopbGatingDisR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - clk_iep gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_iep_gating_dis(&self) -> ClkIepGatingDisR {
        ClkIepGatingDisR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - clk_rga gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_rga_gating_dis(&self) -> ClkRgaGatingDisR {
        ClkRgaGatingDisR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - clk_vdu gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_vdu_gating_dis(&self) -> ClkVduGatingDisR {
        ClkVduGatingDisR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - clk_vcodec gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_vcodec_gating_dis(&self) -> ClkVcodecGatingDisR {
        ClkVcodecGatingDisR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - clk_perihp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_perihp_gating_dis(&self) -> ClkPerihpGatingDisR {
        ClkPerihpGatingDisR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - clk_perilp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_perilp_gating_dis(&self) -> ClkPerilpGatingDisR {
        ClkPerilpGatingDisR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - clk_gpu gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_gpu_gating_dis(&self) -> ClkGpuGatingDisR {
        ClkGpuGatingDisR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - clk_gic gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_gic_gating_dis(&self) -> ClkGicGatingDisR {
        ClkGicGatingDisR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - clk_sd gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_sd_gating_dis(&self) -> ClkSdGatingDisR {
        ClkSdGatingDisR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - clk_sdioaudio gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    pub fn clk_sdioaudio_gating_dis(&self) -> ClkSdioaudioGatingDisR {
        ClkSdioaudioGatingDisR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clk_pmum0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pmum0_gating_dis(&mut self) -> ClkPmum0GatingDisW<PmucruGatedisCon0Spec> {
        ClkPmum0GatingDisW::new(self, 0)
    }
    #[doc = "Bit 1 - clk_center1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_center1_gating_dis(&mut self) -> ClkCenter1GatingDisW<PmucruGatedisCon0Spec> {
        ClkCenter1GatingDisW::new(self, 1)
    }
    #[doc = "Bit 2 - clk_emmc gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_emmc_gating_dis(&mut self) -> ClkEmmcGatingDisW<PmucruGatedisCon0Spec> {
        ClkEmmcGatingDisW::new(self, 2)
    }
    #[doc = "Bit 3 - clk_gmac gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gmac_gating_dis(&mut self) -> ClkGmacGatingDisW<PmucruGatedisCon0Spec> {
        ClkGmacGatingDisW::new(self, 3)
    }
    #[doc = "Bit 4 - clk_edp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_edp_gating_dis(&mut self) -> ClkEdpGatingDisW<PmucruGatedisCon0Spec> {
        ClkEdpGatingDisW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_pmu gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pmu_gating_dis(&mut self) -> ClkPmuGatingDisW<PmucruGatedisCon0Spec> {
        ClkPmuGatingDisW::new(self, 5)
    }
    #[doc = "Bit 6 - clk_alive gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_alive_gating_dis(&mut self) -> ClkAliveGatingDisW<PmucruGatedisCon0Spec> {
        ClkAliveGatingDisW::new(self, 6)
    }
    #[doc = "Bit 7 - clk_msch1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_msch1_gating_dis(&mut self) -> ClkMsch1GatingDisW<PmucruGatedisCon0Spec> {
        ClkMsch1GatingDisW::new(self, 7)
    }
    #[doc = "Bit 8 - clk_msch0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_msch0_gating_dis(&mut self) -> ClkMsch0GatingDisW<PmucruGatedisCon0Spec> {
        ClkMsch0GatingDisW::new(self, 8)
    }
    #[doc = "Bit 9 - clk_vio gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vio_gating_dis(&mut self) -> ClkVioGatingDisW<PmucruGatedisCon0Spec> {
        ClkVioGatingDisW::new(self, 9)
    }
    #[doc = "Bit 10 - clk_ccim1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ccim1_gating_dis(&mut self) -> ClkCcim1GatingDisW<PmucruGatedisCon0Spec> {
        ClkCcim1GatingDisW::new(self, 10)
    }
    #[doc = "Bit 11 - clk_ccim0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_ccim0_gating_dis(&mut self) -> ClkCcim0GatingDisW<PmucruGatedisCon0Spec> {
        ClkCcim0GatingDisW::new(self, 11)
    }
    #[doc = "Bit 12 - clk_center gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_center_gating_dis(&mut self) -> ClkCenterGatingDisW<PmucruGatedisCon0Spec> {
        ClkCenterGatingDisW::new(self, 12)
    }
    #[doc = "Bit 13 - clk_perilpm0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_perilpm0_gating_dis(&mut self) -> ClkPerilpm0GatingDisW<PmucruGatedisCon0Spec> {
        ClkPerilpm0GatingDisW::new(self, 13)
    }
    #[doc = "Bit 14 - clk_usb3 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usb3_gating_dis(&mut self) -> ClkUsb3GatingDisW<PmucruGatedisCon0Spec> {
        ClkUsb3GatingDisW::new(self, 14)
    }
    #[doc = "Bit 15 - clk_hdcp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_hdcp_gating_dis(&mut self) -> ClkHdcpGatingDisW<PmucruGatedisCon0Spec> {
        ClkHdcpGatingDisW::new(self, 15)
    }
    #[doc = "Bit 16 - clk_isp1 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_isp1_gating_dis(&mut self) -> ClkIsp1GatingDisW<PmucruGatedisCon0Spec> {
        ClkIsp1GatingDisW::new(self, 16)
    }
    #[doc = "Bit 17 - clk_isp0 gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_isp0_gating_dis(&mut self) -> ClkIsp0GatingDisW<PmucruGatedisCon0Spec> {
        ClkIsp0GatingDisW::new(self, 17)
    }
    #[doc = "Bit 18 - clk_vopl gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vopl_gating_dis(&mut self) -> ClkVoplGatingDisW<PmucruGatedisCon0Spec> {
        ClkVoplGatingDisW::new(self, 18)
    }
    #[doc = "Bit 19 - clk_vopb gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vopb_gating_dis(&mut self) -> ClkVopbGatingDisW<PmucruGatedisCon0Spec> {
        ClkVopbGatingDisW::new(self, 19)
    }
    #[doc = "Bit 20 - clk_iep gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_iep_gating_dis(&mut self) -> ClkIepGatingDisW<PmucruGatedisCon0Spec> {
        ClkIepGatingDisW::new(self, 20)
    }
    #[doc = "Bit 21 - clk_rga gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_rga_gating_dis(&mut self) -> ClkRgaGatingDisW<PmucruGatedisCon0Spec> {
        ClkRgaGatingDisW::new(self, 21)
    }
    #[doc = "Bit 22 - clk_vdu gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vdu_gating_dis(&mut self) -> ClkVduGatingDisW<PmucruGatedisCon0Spec> {
        ClkVduGatingDisW::new(self, 22)
    }
    #[doc = "Bit 23 - clk_vcodec gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_vcodec_gating_dis(&mut self) -> ClkVcodecGatingDisW<PmucruGatedisCon0Spec> {
        ClkVcodecGatingDisW::new(self, 23)
    }
    #[doc = "Bit 24 - clk_perihp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_perihp_gating_dis(&mut self) -> ClkPerihpGatingDisW<PmucruGatedisCon0Spec> {
        ClkPerihpGatingDisW::new(self, 24)
    }
    #[doc = "Bit 25 - clk_perilp gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_perilp_gating_dis(&mut self) -> ClkPerilpGatingDisW<PmucruGatedisCon0Spec> {
        ClkPerilpGatingDisW::new(self, 25)
    }
    #[doc = "Bit 26 - clk_gpu gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gpu_gating_dis(&mut self) -> ClkGpuGatingDisW<PmucruGatedisCon0Spec> {
        ClkGpuGatingDisW::new(self, 26)
    }
    #[doc = "Bit 27 - clk_gic gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gic_gating_dis(&mut self) -> ClkGicGatingDisW<PmucruGatedisCon0Spec> {
        ClkGicGatingDisW::new(self, 27)
    }
    #[doc = "Bit 28 - clk_sd gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sd_gating_dis(&mut self) -> ClkSdGatingDisW<PmucruGatedisCon0Spec> {
        ClkSdGatingDisW::new(self, 28)
    }
    #[doc = "Bit 29 - clk_sdioaudio gate disable bit When HIGH, gate disable, open all clocks power domain idle request needed"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sdioaudio_gating_dis(&mut self) -> ClkSdioaudioGatingDisW<PmucruGatedisCon0Spec> {
        ClkSdioaudioGatingDisW::new(self, 29)
    }
}
#[doc = "Internal gate disable control register0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pmucru_gatedis_con0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pmucru_gatedis_con0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmucruGatedisCon0Spec;
impl crate::RegisterSpec for PmucruGatedisCon0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmucru_gatedis_con0::R`](R) reader structure"]
impl crate::Readable for PmucruGatedisCon0Spec {}
#[doc = "`write(|w| ..)` method takes [`pmucru_gatedis_con0::W`](W) writer structure"]
impl crate::Writable for PmucruGatedisCon0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PMUCRU_GATEDIS_CON0 to value 0"]
impl crate::Resettable for PmucruGatedisCon0Spec {
    const RESET_VALUE: u32 = 0;
}
