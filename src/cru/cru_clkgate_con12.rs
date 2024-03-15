#[doc = "Register `CRU_CLKGATE_CON12` reader"]
pub type R = crate::R<CruClkgateCon12Spec>;
#[doc = "Register `CRU_CLKGATE_CON12` writer"]
pub type W = crate::W<CruClkgateCon12Spec>;
#[doc = "Field `ACLK_USB3_SRC_EN` reader - aclk_usb3_src clock disable bit When HIGH, disable clock"]
pub type AclkUsb3SrcEnR = crate::BitReader;
#[doc = "Field `ACLK_USB3_SRC_EN` writer - aclk_usb3_src clock disable bit When HIGH, disable clock"]
pub type AclkUsb3SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_USB3_OTG0_REF_EN` reader - clk_usb3_otg0_ref clock disable bit When HIGH, disable clock"]
pub type ClkUsb3Otg0RefEnR = crate::BitReader;
#[doc = "Field `CLK_USB3_OTG0_REF_EN` writer - clk_usb3_otg0_ref clock disable bit When HIGH, disable clock"]
pub type ClkUsb3Otg0RefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_USB3_OTG1_REF_EN` reader - clk_usb3_otg1_ref clock disable bit When HIGH, disable clock"]
pub type ClkUsb3Otg1RefEnR = crate::BitReader;
#[doc = "Field `CLK_USB3_OTG1_REF_EN` writer - clk_usb3_otg1_ref clock disable bit When HIGH, disable clock"]
pub type ClkUsb3Otg1RefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_USB3_OTG0_SUSPEND_EN` reader - clk_usb3_otg0_suspend clock disable bit When HIGH, disable clock"]
pub type ClkUsb3Otg0SuspendEnR = crate::BitReader;
#[doc = "Field `CLK_USB3_OTG0_SUSPEND_EN` writer - clk_usb3_otg0_suspend clock disable bit When HIGH, disable clock"]
pub type ClkUsb3Otg0SuspendEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_USB3_OTG1_SUSPEND_EN` reader - clk_usb3_otg1_suspend clock disable bit When HIGH, disable clock"]
pub type ClkUsb3Otg1SuspendEnR = crate::BitReader;
#[doc = "Field `CLK_USB3_OTG1_SUSPEND_EN` writer - clk_usb3_otg1_suspend clock disable bit When HIGH, disable clock"]
pub type ClkUsb3Otg1SuspendEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PCIEPHY_REF100M_EN` reader - clk_pciephy_ref100m clock disable bit When HIGH, disable clock"]
pub type ClkPciephyRef100mEnR = crate::BitReader;
#[doc = "Field `CLK_PCIEPHY_REF100M_EN` writer - clk_pciephy_ref100m clock disable bit When HIGH, disable clock"]
pub type ClkPciephyRef100mEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_ISP0_SRC_EN` reader - aclk_isp0_src clock disable bit When HIGH, disable clock"]
pub type AclkIsp0SrcEnR = crate::BitReader;
#[doc = "Field `ACLK_ISP0_SRC_EN` writer - aclk_isp0_src clock disable bit When HIGH, disable clock"]
pub type AclkIsp0SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_ISP0_EN` reader - hclk_isp0 clock disable bit When HIGH, disable clock"]
pub type HclkIsp0EnR = crate::BitReader;
#[doc = "Field `HCLK_ISP0_EN` writer - hclk_isp0 clock disable bit When HIGH, disable clock"]
pub type HclkIsp0EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_ISP1_SRC_EN` reader - aclk_isp1_src clock disable bit When HIGH, disable clock"]
pub type AclkIsp1SrcEnR = crate::BitReader;
#[doc = "Field `ACLK_ISP1_SRC_EN` writer - aclk_isp1_src clock disable bit When HIGH, disable clock"]
pub type AclkIsp1SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_ISP1_EN` reader - hclk_isp1 clock disable bit When HIGH, disable clock"]
pub type HclkIsp1EnR = crate::BitReader;
#[doc = "Field `HCLK_ISP1_EN` writer - hclk_isp1 clock disable bit When HIGH, disable clock"]
pub type HclkIsp1EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_GIC_SRC_EN` reader - aclk_gic_src clock disable bit When HIGH, disable clock"]
pub type AclkGicSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_GIC_SRC_EN` writer - aclk_gic_src clock disable bit When HIGH, disable clock"]
pub type AclkGicSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_SD_SRC_EN` reader - hclk_sd_src clock disable bit When HIGH, disable clock"]
pub type HclkSdSrcEnR = crate::BitReader;
#[doc = "Field `HCLK_SD_SRC_EN` writer - hclk_sd_src clock disable bit When HIGH, disable clock"]
pub type HclkSdSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_usb3_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_usb3_src_en(&self) -> AclkUsb3SrcEnR {
        AclkUsb3SrcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clk_usb3_otg0_ref clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_usb3_otg0_ref_en(&self) -> ClkUsb3Otg0RefEnR {
        ClkUsb3Otg0RefEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - clk_usb3_otg1_ref clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_usb3_otg1_ref_en(&self) -> ClkUsb3Otg1RefEnR {
        ClkUsb3Otg1RefEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - clk_usb3_otg0_suspend clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_usb3_otg0_suspend_en(&self) -> ClkUsb3Otg0SuspendEnR {
        ClkUsb3Otg0SuspendEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - clk_usb3_otg1_suspend clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_usb3_otg1_suspend_en(&self) -> ClkUsb3Otg1SuspendEnR {
        ClkUsb3Otg1SuspendEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_pciephy_ref100m clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_pciephy_ref100m_en(&self) -> ClkPciephyRef100mEnR {
        ClkPciephyRef100mEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - aclk_isp0_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_isp0_src_en(&self) -> AclkIsp0SrcEnR {
        AclkIsp0SrcEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - hclk_isp0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_isp0_en(&self) -> HclkIsp0EnR {
        HclkIsp0EnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - aclk_isp1_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_isp1_src_en(&self) -> AclkIsp1SrcEnR {
        AclkIsp1SrcEnR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - hclk_isp1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_isp1_en(&self) -> HclkIsp1EnR {
        HclkIsp1EnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - aclk_gic_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_gic_src_en(&self) -> AclkGicSrcEnR {
        AclkGicSrcEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - hclk_sd_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_sd_src_en(&self) -> HclkSdSrcEnR {
        HclkSdSrcEnR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_usb3_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_usb3_src_en(&mut self) -> AclkUsb3SrcEnW<CruClkgateCon12Spec> {
        AclkUsb3SrcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - clk_usb3_otg0_ref clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usb3_otg0_ref_en(&mut self) -> ClkUsb3Otg0RefEnW<CruClkgateCon12Spec> {
        ClkUsb3Otg0RefEnW::new(self, 1)
    }
    #[doc = "Bit 2 - clk_usb3_otg1_ref clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usb3_otg1_ref_en(&mut self) -> ClkUsb3Otg1RefEnW<CruClkgateCon12Spec> {
        ClkUsb3Otg1RefEnW::new(self, 2)
    }
    #[doc = "Bit 3 - clk_usb3_otg0_suspend clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usb3_otg0_suspend_en(&mut self) -> ClkUsb3Otg0SuspendEnW<CruClkgateCon12Spec> {
        ClkUsb3Otg0SuspendEnW::new(self, 3)
    }
    #[doc = "Bit 4 - clk_usb3_otg1_suspend clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usb3_otg1_suspend_en(&mut self) -> ClkUsb3Otg1SuspendEnW<CruClkgateCon12Spec> {
        ClkUsb3Otg1SuspendEnW::new(self, 4)
    }
    #[doc = "Bit 6 - clk_pciephy_ref100m clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pciephy_ref100m_en(&mut self) -> ClkPciephyRef100mEnW<CruClkgateCon12Spec> {
        ClkPciephyRef100mEnW::new(self, 6)
    }
    #[doc = "Bit 8 - aclk_isp0_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_isp0_src_en(&mut self) -> AclkIsp0SrcEnW<CruClkgateCon12Spec> {
        AclkIsp0SrcEnW::new(self, 8)
    }
    #[doc = "Bit 9 - hclk_isp0 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_isp0_en(&mut self) -> HclkIsp0EnW<CruClkgateCon12Spec> {
        HclkIsp0EnW::new(self, 9)
    }
    #[doc = "Bit 10 - aclk_isp1_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_isp1_src_en(&mut self) -> AclkIsp1SrcEnW<CruClkgateCon12Spec> {
        AclkIsp1SrcEnW::new(self, 10)
    }
    #[doc = "Bit 11 - hclk_isp1 clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_isp1_en(&mut self) -> HclkIsp1EnW<CruClkgateCon12Spec> {
        HclkIsp1EnW::new(self, 11)
    }
    #[doc = "Bit 12 - aclk_gic_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gic_src_en(&mut self) -> AclkGicSrcEnW<CruClkgateCon12Spec> {
        AclkGicSrcEnW::new(self, 12)
    }
    #[doc = "Bit 13 - hclk_sd_src clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_sd_src_en(&mut self) -> HclkSdSrcEnW<CruClkgateCon12Spec> {
        HclkSdSrcEnW::new(self, 13)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon12Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register12\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con12::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con12::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon12Spec;
impl crate::RegisterSpec for CruClkgateCon12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con12::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon12Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con12::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon12Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON12 to value 0"]
impl crate::Resettable for CruClkgateCon12Spec {
    const RESET_VALUE: u32 = 0;
}
