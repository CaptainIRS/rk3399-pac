#[doc = "Register `CLKGATE_CON5` reader"]
pub type R = crate::R<ClkgateCon5Spec>;
#[doc = "Register `CLKGATE_CON5` writer"]
pub type W = crate::W<ClkgateCon5Spec>;
#[doc = "Field `ACLK_PERIHP_GPLL_SRC_EN` reader - aclk_perihp_gpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkPerihpGpllSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_PERIHP_GPLL_SRC_EN` writer - aclk_perihp_gpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkPerihpGpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_PERIHP_CPLL_SRC_EN` reader - aclk_perihp_cpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkPerihpCpllSrcEnR = crate::BitReader;
#[doc = "Field `ACLK_PERIHP_CPLL_SRC_EN` writer - aclk_perihp_cpll clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkPerihpCpllSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACLK_PERIHP_EN` reader - aclk_perihp clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkPerihpEnR = crate::BitReader;
#[doc = "Field `ACLK_PERIHP_EN` writer - aclk_perihp clock disable bit\n\nWhen HIGH, disable clock"]
pub type AclkPerihpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HCLK_PERIHP_EN` reader - hclk_perihp clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkPerihpEnR = crate::BitReader;
#[doc = "Field `HCLK_PERIHP_EN` writer - hclk_perihp clock disable bit\n\nWhen HIGH, disable clock"]
pub type HclkPerihpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCLK_PERIHP_EN` reader - pclk_perihp clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkPerihpEnR = crate::BitReader;
#[doc = "Field `PCLK_PERIHP_EN` writer - pclk_perihp clock disable bit\n\nWhen HIGH, disable clock"]
pub type PclkPerihpEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_GMAC_SRC_EN` reader - clk_gmac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkGmacSrcEnR = crate::BitReader;
#[doc = "Field `CLK_GMAC_SRC_EN` writer - clk_gmac_src clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkGmacSrcEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MAC_REF_EN` reader - clk_mac_ref clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkMacRefEnR = crate::BitReader;
#[doc = "Field `CLK_MAC_REF_EN` writer - clk_mac_ref clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkMacRefEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MAC_REFOUT_EN` reader - clk_mac_refout clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkMacRefoutEnR = crate::BitReader;
#[doc = "Field `CLK_MAC_REFOUT_EN` writer - clk_mac_refout clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkMacRefoutEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MAC_RX_EN` reader - clk_mac_rx clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkMacRxEnR = crate::BitReader;
#[doc = "Field `CLK_MAC_RX_EN` writer - clk_mac_rx clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkMacRxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLK_MAC_TX_EN` reader - clk_mac_tx clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkMacTxEnR = crate::BitReader;
#[doc = "Field `CLK_MAC_TX_EN` writer - clk_mac_tx clock disable bit\n\nWhen HIGH, disable clock"]
pub type ClkMacTxEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - aclk_perihp_gpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_perihp_gpll_src_en(&self) -> AclkPerihpGpllSrcEnR {
        AclkPerihpGpllSrcEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - aclk_perihp_cpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_perihp_cpll_src_en(&self) -> AclkPerihpCpllSrcEnR {
        AclkPerihpCpllSrcEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - aclk_perihp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn aclk_perihp_en(&self) -> AclkPerihpEnR {
        AclkPerihpEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - hclk_perihp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn hclk_perihp_en(&self) -> HclkPerihpEnR {
        HclkPerihpEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pclk_perihp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn pclk_perihp_en(&self) -> PclkPerihpEnR {
        PclkPerihpEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - clk_gmac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_gmac_src_en(&self) -> ClkGmacSrcEnR {
        ClkGmacSrcEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - clk_mac_ref clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_mac_ref_en(&self) -> ClkMacRefEnR {
        ClkMacRefEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - clk_mac_refout clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_mac_refout_en(&self) -> ClkMacRefoutEnR {
        ClkMacRefoutEnR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - clk_mac_rx clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_mac_rx_en(&self) -> ClkMacRxEnR {
        ClkMacRxEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - clk_mac_tx clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    pub fn clk_mac_tx_en(&self) -> ClkMacTxEnR {
        ClkMacTxEnR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - aclk_perihp_gpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perihp_gpll_src_en(&mut self) -> AclkPerihpGpllSrcEnW<ClkgateCon5Spec> {
        AclkPerihpGpllSrcEnW::new(self, 0)
    }
    #[doc = "Bit 1 - aclk_perihp_cpll clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perihp_cpll_src_en(&mut self) -> AclkPerihpCpllSrcEnW<ClkgateCon5Spec> {
        AclkPerihpCpllSrcEnW::new(self, 1)
    }
    #[doc = "Bit 2 - aclk_perihp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn aclk_perihp_en(&mut self) -> AclkPerihpEnW<ClkgateCon5Spec> {
        AclkPerihpEnW::new(self, 2)
    }
    #[doc = "Bit 3 - hclk_perihp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn hclk_perihp_en(&mut self) -> HclkPerihpEnW<ClkgateCon5Spec> {
        HclkPerihpEnW::new(self, 3)
    }
    #[doc = "Bit 4 - pclk_perihp clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn pclk_perihp_en(&mut self) -> PclkPerihpEnW<ClkgateCon5Spec> {
        PclkPerihpEnW::new(self, 4)
    }
    #[doc = "Bit 5 - clk_gmac_src clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_gmac_src_en(&mut self) -> ClkGmacSrcEnW<ClkgateCon5Spec> {
        ClkGmacSrcEnW::new(self, 5)
    }
    #[doc = "Bit 6 - clk_mac_ref clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mac_ref_en(&mut self) -> ClkMacRefEnW<ClkgateCon5Spec> {
        ClkMacRefEnW::new(self, 6)
    }
    #[doc = "Bit 7 - clk_mac_refout clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mac_refout_en(&mut self) -> ClkMacRefoutEnW<ClkgateCon5Spec> {
        ClkMacRefoutEnW::new(self, 7)
    }
    #[doc = "Bit 8 - clk_mac_rx clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mac_rx_en(&mut self) -> ClkMacRxEnW<ClkgateCon5Spec> {
        ClkMacRxEnW::new(self, 8)
    }
    #[doc = "Bit 9 - clk_mac_tx clock disable bit\n\nWhen HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn clk_mac_tx_en(&mut self) -> ClkMacTxEnW<ClkgateCon5Spec> {
        ClkMacTxEnW::new(self, 9)
    }
    #[doc = "Bits 16:31 - write mask bits\n\nWhen every bit HIGH, enable the writing corresponding bit\n\nWhen every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<ClkgateCon5Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register5\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clkgate_con5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clkgate_con5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkgateCon5Spec;
impl crate::RegisterSpec for ClkgateCon5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clkgate_con5::R`](R) reader structure"]
impl crate::Readable for ClkgateCon5Spec {}
#[doc = "`write(|w| ..)` method takes [`clkgate_con5::W`](W) writer structure"]
impl crate::Writable for ClkgateCon5Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLKGATE_CON5 to value 0"]
impl crate::Resettable for ClkgateCon5Spec {
    const RESET_VALUE: u32 = 0;
}
