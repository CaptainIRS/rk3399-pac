#[doc = "Register `CRU_CLKGATE_CON6` reader"]
pub type R = crate::R<CruClkgateCon6Spec>;
#[doc = "Register `CRU_CLKGATE_CON6` writer"]
pub type W = crate::W<CruClkgateCon6Spec>;
#[doc = "Field `CLK_SDIO_SRC_EN` reader - clk_sdio_src clock disable bit When HIGH, disable clock"]
pub type ClkSdioSrcEnR = crate::BitReader;
#[doc = "Field `CLK_SDIO_SRC_EN` writer - clk_sdio_src clock disable bit When HIGH, disable clock"]
pub type ClkSdioSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SDMMC_SRC_EN` reader - clk_sdmmc_src clock disable bit When HIGH, disable clock"]
pub type ClkSdmmcSrcEnR = crate::BitReader;
#[doc = "Field `CLK_SDMMC_SRC_EN` writer - clk_sdmmc_src clock disable bit When HIGH, disable clock"]
pub type ClkSdmmcSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PCIE_PM_SRC_EN` reader - clk_pcie_pm_src clock disable bit When HIGH, disable clock"]
pub type ClkPciePmSrcEnR = crate::BitReader;
#[doc = "Field `CLK_PCIE_PM_SRC_EN` writer - clk_pcie_pm_src clock disable bit When HIGH, disable clock"]
pub type ClkPciePmSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PCIE_CORE_SRC_EN` reader - clk_pcie_core_src clock disable bit When HIGH, disable clock"]
pub type ClkPcieCoreSrcEnR = crate::BitReader;
#[doc = "Field `CLK_PCIE_CORE_SRC_EN` writer - clk_pcie_core_src clock disable bit When HIGH, disable clock"]
pub type ClkPcieCoreSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_HSICPHY_EN` reader - clk_hsicphy clock disable bit When HIGH, disable clock"]
pub type ClkHsicphyEnR = crate::BitReader;
#[doc = "Field `CLK_HSICPHY_EN` writer - clk_hsicphy clock disable bit When HIGH, disable clock"]
pub type ClkHsicphyEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_USB2PHY0_REF_EN` reader - clk_usb2phy0_ref clock disable bit When HIGH, disable clock"]
pub type ClkUsb2phy0RefEnR = crate::BitReader;
#[doc = "Field `CLK_USB2PHY0_REF_EN` writer - clk_usb2phy0_ref clock disable bit When HIGH, disable clock"]
pub type ClkUsb2phy0RefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_USB2PHY1_REF_EN` reader - clk_usb2phy1_ref clock disable bit When HIGH, disable clock"]
pub type ClkUsb2phy1RefEnR = crate::BitReader;
#[doc = "Field `CLK_USB2PHY1_REF_EN` writer - clk_usb2phy1_ref clock disable bit When HIGH, disable clock"]
pub type ClkUsb2phy1RefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_GMAC_GPLL_SRC_EN` reader - aclk_gmac_gpll clock disable bit When HIGH, disable clock"]
pub type AclkGmacGpllSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_GMAC_GPLL_SRC_EN` writer - aclk_gmac_gpll clock disable bit When HIGH, disable clock"]
pub type AclkGmacGpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_GMAC_CPLL_SRC_EN` reader - aclk_gmac_cpll clock disable bit When HIGH, disable clock"]
pub type AclkGmacCpllSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_GMAC_CPLL_SRC_EN` writer - aclk_gmac_cpll clock disable bit When HIGH, disable clock"]
pub type AclkGmacCpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_GMAC_EN` reader - aclk_gmac clock disable bit When HIGH, disable clock"]
pub type AclkGmacEnR = crate::BitReader;
#[doc = "Field `ACLK_GMAC_EN` writer - aclk_gmac clock disable bit When HIGH, disable clock"]
pub type AclkGmacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_GMAC_EN` reader - pclk_gmac clock disable bit When HIGH, disable clock"]
pub type PclkGmacEnR = crate::BitReader;
#[doc = "Field `PCLK_GMAC_EN` writer - pclk_gmac clock disable bit When HIGH, disable clock"]
pub type PclkGmacEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_EMMC_GPLL_SRC_EN` reader - aclk_emmc_gpll clock disable bit When HIGH, disable clock"]
pub type AclkEmmcGpllSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_EMMC_GPLL_SRC_EN` writer - aclk_emmc_gpll clock disable bit When HIGH, disable clock"]
pub type AclkEmmcGpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_EMMC_CPLL_SRC_EN` reader - aclk_emmc_cpll clock disable bit When HIGH, disable clock"]
pub type AclkEmmcCpllSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_EMMC_CPLL_SRC_EN` writer - aclk_emmc_cpll clock disable bit When HIGH, disable clock"]
pub type AclkEmmcCpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_EMMC_SRC_EN` reader - clk_emmc_src clock disable bit When HIGH, disable clock"]
pub type ClkEmmcSrcEnR = crate::BitReader;
#[doc = "Field `CLK_EMMC_SRC_EN` writer - clk_emmc_src clock disable bit When HIGH, disable clock"]
pub type ClkEmmcSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - clk_sdio_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_sdio_src_en(&self) -> ClkSdioSrcEnR {
        ClkSdioSrcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clk_sdmmc_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_sdmmc_src_en(&self) -> ClkSdmmcSrcEnR {
        ClkSdmmcSrcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clk_pcie_pm_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_pcie_pm_src_en(&self) -> ClkPciePmSrcEnR {
        ClkPciePmSrcEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - clk_pcie_core_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_pcie_core_src_en(&self) -> ClkPcieCoreSrcEnR {
        ClkPcieCoreSrcEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - clk_hsicphy clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_hsicphy_en(&self) -> ClkHsicphyEnR {
        ClkHsicphyEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_usb2phy0_ref clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_usb2phy0_ref_en(&self) -> ClkUsb2phy0RefEnR {
        ClkUsb2phy0RefEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_usb2phy1_ref clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_usb2phy1_ref_en(&self) -> ClkUsb2phy1RefEnR {
        ClkUsb2phy1RefEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - aclk_gmac_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_gmac_gpll_src_en(&self) -> AclkGmacGpllSrcEnR {
        AclkGmacGpllSrcEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - aclk_gmac_cpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_gmac_cpll_src_en(&self) -> AclkGmacCpllSrcEnR {
        AclkGmacCpllSrcEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - aclk_gmac clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_gmac_en(&self) -> AclkGmacEnR {
        AclkGmacEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - pclk_gmac clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_gmac_en(&self) -> PclkGmacEnR {
        PclkGmacEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - aclk_emmc_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_emmc_gpll_src_en(&self) -> AclkEmmcGpllSrcEnR {
        AclkEmmcGpllSrcEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - aclk_emmc_cpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_emmc_cpll_src_en(&self) -> AclkEmmcCpllSrcEnR {
        AclkEmmcCpllSrcEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - clk_emmc_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_emmc_src_en(&self) -> ClkEmmcSrcEnR {
        ClkEmmcSrcEnR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - clk_sdio_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sdio_src_en(&mut self) -> ClkSdioSrcEnW<CruClkgateCon6Spec> {
        ClkSdioSrcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - clk_sdmmc_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_sdmmc_src_en(&mut self) -> ClkSdmmcSrcEnW<CruClkgateCon6Spec> {
        ClkSdmmcSrcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - clk_pcie_pm_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pcie_pm_src_en(&mut self) -> ClkPciePmSrcEnW<CruClkgateCon6Spec> {
        ClkPciePmSrcEnW::new(self, 2)
    }
    #[doc = "Bit 3 - clk_pcie_core_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pcie_core_src_en(&mut self) -> ClkPcieCoreSrcEnW<CruClkgateCon6Spec> {
        ClkPcieCoreSrcEnW::new(self, 3)
    }
    #[doc = "Bit 4 - clk_hsicphy clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_hsicphy_en(&mut self) -> ClkHsicphyEnW<CruClkgateCon6Spec> {
        ClkHsicphyEnW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_usb2phy0_ref clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usb2phy0_ref_en(&mut self) -> ClkUsb2phy0RefEnW<CruClkgateCon6Spec> {
        ClkUsb2phy0RefEnW::new(self, 5)
    }
    #[doc = "Bit 6 - clk_usb2phy1_ref clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usb2phy1_ref_en(&mut self) -> ClkUsb2phy1RefEnW<CruClkgateCon6Spec> {
        ClkUsb2phy1RefEnW::new(self, 6)
    }
    #[doc = "Bit 8 - aclk_gmac_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gmac_gpll_src_en(&mut self) -> AclkGmacGpllSrcEnW<CruClkgateCon6Spec> {
        AclkGmacGpllSrcEnW::new(self, 8)
    }
    #[doc = "Bit 9 - aclk_gmac_cpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gmac_cpll_src_en(&mut self) -> AclkGmacCpllSrcEnW<CruClkgateCon6Spec> {
        AclkGmacCpllSrcEnW::new(self, 9)
    }
    #[doc = "Bit 10 - aclk_gmac clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gmac_en(&mut self) -> AclkGmacEnW<CruClkgateCon6Spec> {
        AclkGmacEnW::new(self, 10)
    }
    #[doc = "Bit 11 - pclk_gmac clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_gmac_en(&mut self) -> PclkGmacEnW<CruClkgateCon6Spec> {
        PclkGmacEnW::new(self, 11)
    }
    #[doc = "Bit 12 - aclk_emmc_gpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_emmc_gpll_src_en(&mut self) -> AclkEmmcGpllSrcEnW<CruClkgateCon6Spec> {
        AclkEmmcGpllSrcEnW::new(self, 12)
    }
    #[doc = "Bit 13 - aclk_emmc_cpll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_emmc_cpll_src_en(&mut self) -> AclkEmmcCpllSrcEnW<CruClkgateCon6Spec> {
        AclkEmmcCpllSrcEnW::new(self, 13)
    }
    #[doc = "Bit 14 - clk_emmc_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_emmc_src_en(&mut self) -> ClkEmmcSrcEnW<CruClkgateCon6Spec> {
        ClkEmmcSrcEnW::new(self, 14)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon6Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon6Spec;
impl crate::RegisterSpec for CruClkgateCon6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con6::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon6Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con6::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON6 to value 0"]
impl crate::Resettable for CruClkgateCon6Spec {
    const RESET_VALUE: u32 = 0;
}
