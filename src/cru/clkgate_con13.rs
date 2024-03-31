#[doc = "Register `CLKGATE_CON13` reader"]
pub type R = crate::R<ClkgateCon13Spec>;
#[doc = "Register `CLKGATE_CON13` writer"]
pub type W = crate::W<ClkgateCon13Spec>;
#[doc = "Field `ACLK_GPU_PLL_SRC_EN` reader - aclk_gpu_pll_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkGpuPllSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_GPU_PLL_SRC_EN` writer - aclk_gpu_pll_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkGpuPllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_PVTM_GPU_EN` reader - clk_pvtm_gpu clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkPvtmGpuEnR = crate::BitReader;
#[doc = "Field `CLK_PVTM_GPU_EN` writer - clk_pvtm_gpu clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkPvtmGpuEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_UPHY0_TCPDPHYREF_EN` reader - clk_uphy0_tcpdphyref clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUphy0TcpdphyrefEnR = crate::BitReader;
#[doc = "Field `CLK_UPHY0_TCPDPHYREF_EN` writer - clk_uphy0_tcpdphyref clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUphy0TcpdphyrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_UPHY0_TCPDCORE_EN` reader - clk_uphy0_tcpdcore clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUphy0TcpdcoreEnR = crate::BitReader;
#[doc = "Field `CLK_UPHY0_TCPDCORE_EN` writer - clk_uphy0_tcpdcore clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUphy0TcpdcoreEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_UPHY1_TCPDPHYREF_EN` reader - clk_uphy1_tcpdphyref clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUphy1TcpdphyrefEnR = crate::BitReader;
#[doc = "Field `CLK_UPHY1_TCPDPHYREF_EN` writer - clk_uphy1_tcpdphyref clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUphy1TcpdphyrefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_UPHY1_TCPDCORE_EN` reader - clk_uphy1_tcpdcore clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUphy1TcpdcoreEnR = crate::BitReader;
#[doc = "Field `CLK_UPHY1_TCPDCORE_EN` writer - clk_uphy1_tcpdcore clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUphy1TcpdcoreEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_WIFI_EN` reader - clk_wifi clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkWifiEnR = crate::BitReader;
#[doc = "Field `CLK_WIFI_EN` writer - clk_wifi clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkWifiEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TESTCLK_EN` reader - testclk clock disable bit\n\nWhen HIGH, disable clock"]
pub type TestclkEnR = crate::BitReader;
#[doc = "Field `TESTCLK_EN` writer - testclk clock disable bit\n\nWhen HIGH, disable clock"]
pub type TestclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_USB480M_EN` reader - clk_usb480m clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUsb480mEnR = crate::BitReader;
#[doc = "Field `CLK_USB480M_EN` writer - clk_usb480m clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkUsb480mEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_SPI5_SRC_EN` reader - clk_spi5_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpi5SrcEnR = crate::BitReader;
#[doc = "Field `CLK_SPI5_SRC_EN` writer - clk_spi5_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkSpi5SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TESTOUT1_SRC_EN` reader - clk_testout1_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTestout1SrcEnR = crate::BitReader;
#[doc = "Field `CLK_TESTOUT1_SRC_EN` writer - clk_testout1_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTestout1SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_TESTOUT2_SRC_EN` reader - clk_testout2_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTestout2SrcEnR = crate::BitReader;
#[doc = "Field `CLK_TESTOUT2_SRC_EN` writer - clk_testout2_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkTestout2SrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_gpu_pll_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_gpu_pll_src_en(&self) -> AclkGpuPllSrcEnR {
        AclkGpuPllSrcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - clk_pvtm_gpu clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_pvtm_gpu_en(&self) -> ClkPvtmGpuEnR {
        ClkPvtmGpuEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - clk_uphy0_tcpdphyref clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_uphy0_tcpdphyref_en(&self) -> ClkUphy0TcpdphyrefEnR {
        ClkUphy0TcpdphyrefEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_uphy0_tcpdcore clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_uphy0_tcpdcore_en(&self) -> ClkUphy0TcpdcoreEnR {
        ClkUphy0TcpdcoreEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_uphy1_tcpdphyref clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_uphy1_tcpdphyref_en(&self) -> ClkUphy1TcpdphyrefEnR {
        ClkUphy1TcpdphyrefEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - clk_uphy1_tcpdcore clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_uphy1_tcpdcore_en(&self) -> ClkUphy1TcpdcoreEnR {
        ClkUphy1TcpdcoreEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - clk_wifi clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_wifi_en(&self) -> ClkWifiEnR {
        ClkWifiEnR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - testclk clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn testclk_en(&self) -> TestclkEnR {
        TestclkEnR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - clk_usb480m clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_usb480m_en(&self) -> ClkUsb480mEnR {
        ClkUsb480mEnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - clk_spi5_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_spi5_src_en(&self) -> ClkSpi5SrcEnR {
        ClkSpi5SrcEnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - clk_testout1_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_testout1_src_en(&self) -> ClkTestout1SrcEnR {
        ClkTestout1SrcEnR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - clk_testout2_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_testout2_src_en(&self) -> ClkTestout2SrcEnR {
        ClkTestout2SrcEnR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_gpu_pll_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_gpu_pll_src_en(&mut self) -> AclkGpuPllSrcEnW<ClkgateCon13Spec> {
        AclkGpuPllSrcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - clk_pvtm_gpu clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_pvtm_gpu_en(&mut self) -> ClkPvtmGpuEnW<ClkgateCon13Spec> {
        ClkPvtmGpuEnW::new(self, 1)
    }
    #[doc = "Bit 4 - clk_uphy0_tcpdphyref clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uphy0_tcpdphyref_en(&mut self) -> ClkUphy0TcpdphyrefEnW<ClkgateCon13Spec> {
        ClkUphy0TcpdphyrefEnW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_uphy0_tcpdcore clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uphy0_tcpdcore_en(&mut self) -> ClkUphy0TcpdcoreEnW<ClkgateCon13Spec> {
        ClkUphy0TcpdcoreEnW::new(self, 5)
    }
    #[doc = "Bit 6 - clk_uphy1_tcpdphyref clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uphy1_tcpdphyref_en(&mut self) -> ClkUphy1TcpdphyrefEnW<ClkgateCon13Spec> {
        ClkUphy1TcpdphyrefEnW::new(self, 6)
    }
    #[doc = "Bit 7 - clk_uphy1_tcpdcore clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_uphy1_tcpdcore_en(&mut self) -> ClkUphy1TcpdcoreEnW<ClkgateCon13Spec> {
        ClkUphy1TcpdcoreEnW::new(self, 7)
    }
    #[doc = "Bit 9 - clk_wifi clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_wifi_en(&mut self) -> ClkWifiEnW<ClkgateCon13Spec> {
        ClkWifiEnW::new(self, 9)
    }
    #[doc = "Bit 11 - testclk clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn testclk_en(&mut self) -> TestclkEnW<ClkgateCon13Spec> {
        TestclkEnW::new(self, 11)
    }
    #[doc = "Bit 12 - clk_usb480m clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_usb480m_en(&mut self) -> ClkUsb480mEnW<ClkgateCon13Spec> {
        ClkUsb480mEnW::new(self, 12)
    }
    #[doc = "Bit 13 - clk_spi5_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_spi5_src_en(&mut self) -> ClkSpi5SrcEnW<ClkgateCon13Spec> {
        ClkSpi5SrcEnW::new(self, 13)
    }
    #[doc = "Bit 14 - clk_testout1_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_testout1_src_en(&mut self) -> ClkTestout1SrcEnW<ClkgateCon13Spec> {
        ClkTestout1SrcEnW::new(self, 14)
    }
    #[doc = "Bit 15 - clk_testout2_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_testout2_src_en(&mut self) -> ClkTestout2SrcEnW<ClkgateCon13Spec> {
        ClkTestout2SrcEnW::new(self, 15)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkgateCon13Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register13\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con13::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con13::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkgateCon13Spec;
impl crate::RegisterSpec for ClkgateCon13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgate_con13::R`](R) reader structure"]
impl crate::Readable for ClkgateCon13Spec {}
#[doc = "`write(|w| ..)` method takes [`clkgate_con13::W`](W) writer structure"]
impl crate::Writable for ClkgateCon13Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKGATE_CON13 to value 0"]
impl crate::Resettable for ClkgateCon13Spec {
    const RESET_VALUE: u32 = 0;
}
