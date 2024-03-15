#[doc = "Register `CRU_CLKGATE_CON21` reader"]
pub type R = crate::R<CruClkgateCon21Spec>;
#[doc = "Register `CRU_CLKGATE_CON21` writer"]
pub type W = crate::W<CruClkgateCon21Spec>;
#[doc = "Field `DPHY_PLLCLK_EN` reader - dphy_pll clock disable bit When HIGH, disable clock"]
pub type DphyPllclkEnR = crate::BitReader;
#[doc = "Field `DPHY_PLLCLK_EN` writer - dphy_pll clock disable bit When HIGH, disable clock"]
pub type DphyPllclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_TX0_CFGCLK_EN` reader - dphy_tx0_cfg clock disable bit When HIGH, disable clock"]
pub type DphyTx0CfgclkEnR = crate::BitReader;
#[doc = "Field `DPHY_TX0_CFGCLK_EN` writer - dphy_tx0_cfg clock disable bit When HIGH, disable clock"]
pub type DphyTx0CfgclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_TX1RX1_CFGCLK_EN` reader - dphy_tx1rx1_cfg clock disable bit When HIGH, disable clock"]
pub type DphyTx1rx1CfgclkEnR = crate::BitReader;
#[doc = "Field `DPHY_TX1RX1_CFGCLK_EN` writer - dphy_tx1rx1_cfg clock disable bit When HIGH, disable clock"]
pub type DphyTx1rx1CfgclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_RX0_CFGCLK_EN` reader - dphy_rx0_cfg clock disable bit When HIGH, disable clock"]
pub type DphyRx0CfgclkEnR = crate::BitReader;
#[doc = "Field `DPHY_RX0_CFGCLK_EN` writer - dphy_rx0_cfg clock disable bit When HIGH, disable clock"]
pub type DphyRx0CfgclkEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPHY_PCLK_MUX_GATE_EN` reader - uphy_pclk_mux clock disable bit When HIGH, disable clock"]
pub type UphyPclkMuxGateEnR = crate::BitReader;
#[doc = "Field `UPHY_PCLK_MUX_GATE_EN` writer - uphy_pclk_mux clock disable bit When HIGH, disable clock"]
pub type UphyPclkMuxGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPHY0_PCLK_TCPHY_GATE_EN` reader - uphy0_pclk_tcphy clock disable bit When HIGH, disable clock"]
pub type Uphy0PclkTcphyGateEnR = crate::BitReader;
#[doc = "Field `UPHY0_PCLK_TCPHY_GATE_EN` writer - uphy0_pclk_tcphy clock disable bit When HIGH, disable clock"]
pub type Uphy0PclkTcphyGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPHY0_PCLK_TCPD_GATE_EN` reader - uphy0_pclk_tcpd clock disable bit When HIGH, disable clock"]
pub type Uphy0PclkTcpdGateEnR = crate::BitReader;
#[doc = "Field `UPHY0_PCLK_TCPD_GATE_EN` writer - uphy0_pclk_tcpd clock disable bit When HIGH, disable clock"]
pub type Uphy0PclkTcpdGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPHY1_PCLK_TCPHY_GATE_EN` reader - uphy1_pclk_tcphy clock disable bit When HIGH, disable clock"]
pub type Uphy1PclkTcphyGateEnR = crate::BitReader;
#[doc = "Field `UPHY1_PCLK_TCPHY_GATE_EN` writer - uphy1_pclk_tcphy clock disable bit When HIGH, disable clock"]
pub type Uphy1PclkTcphyGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UPHY1_PCLK_TCPD_GATE_EN` reader - uphy1_pclk_tcpd disable bit When HIGH, disable clock"]
pub type Uphy1PclkTcpdGateEnR = crate::BitReader;
#[doc = "Field `UPHY1_PCLK_TCPD_GATE_EN` writer - uphy1_pclk_tcpd disable bit When HIGH, disable clock"]
pub type Uphy1PclkTcpdGateEnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WRITE_MASK` writer - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
pub type WriteMaskW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - dphy_pll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn dphy_pllclk_en(&self) -> DphyPllclkEnR {
        DphyPllclkEnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - dphy_tx0_cfg clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn dphy_tx0_cfgclk_en(&self) -> DphyTx0CfgclkEnR {
        DphyTx0CfgclkEnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - dphy_tx1rx1_cfg clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn dphy_tx1rx1_cfgclk_en(&self) -> DphyTx1rx1CfgclkEnR {
        DphyTx1rx1CfgclkEnR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - dphy_rx0_cfg clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn dphy_rx0_cfgclk_en(&self) -> DphyRx0CfgclkEnR {
        DphyRx0CfgclkEnR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - uphy_pclk_mux clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn uphy_pclk_mux_gate_en(&self) -> UphyPclkMuxGateEnR {
        UphyPclkMuxGateEnR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - uphy0_pclk_tcphy clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn uphy0_pclk_tcphy_gate_en(&self) -> Uphy0PclkTcphyGateEnR {
        Uphy0PclkTcphyGateEnR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - uphy0_pclk_tcpd clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn uphy0_pclk_tcpd_gate_en(&self) -> Uphy0PclkTcpdGateEnR {
        Uphy0PclkTcpdGateEnR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - uphy1_pclk_tcphy clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn uphy1_pclk_tcphy_gate_en(&self) -> Uphy1PclkTcphyGateEnR {
        Uphy1PclkTcphyGateEnR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - uphy1_pclk_tcpd disable bit When HIGH, disable clock"]
    #[inline(always)]
    pub fn uphy1_pclk_tcpd_gate_en(&self) -> Uphy1PclkTcpdGateEnR {
        Uphy1PclkTcpdGateEnR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - dphy_pll clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_pllclk_en(&mut self) -> DphyPllclkEnW<CruClkgateCon21Spec> {
        DphyPllclkEnW::new(self, 0)
    }
    #[doc = "Bit 1 - dphy_tx0_cfg clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_tx0_cfgclk_en(&mut self) -> DphyTx0CfgclkEnW<CruClkgateCon21Spec> {
        DphyTx0CfgclkEnW::new(self, 1)
    }
    #[doc = "Bit 2 - dphy_tx1rx1_cfg clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_tx1rx1_cfgclk_en(&mut self) -> DphyTx1rx1CfgclkEnW<CruClkgateCon21Spec> {
        DphyTx1rx1CfgclkEnW::new(self, 2)
    }
    #[doc = "Bit 3 - dphy_rx0_cfg clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rx0_cfgclk_en(&mut self) -> DphyRx0CfgclkEnW<CruClkgateCon21Spec> {
        DphyRx0CfgclkEnW::new(self, 3)
    }
    #[doc = "Bit 4 - uphy_pclk_mux clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn uphy_pclk_mux_gate_en(&mut self) -> UphyPclkMuxGateEnW<CruClkgateCon21Spec> {
        UphyPclkMuxGateEnW::new(self, 4)
    }
    #[doc = "Bit 5 - uphy0_pclk_tcphy clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn uphy0_pclk_tcphy_gate_en(&mut self) -> Uphy0PclkTcphyGateEnW<CruClkgateCon21Spec> {
        Uphy0PclkTcphyGateEnW::new(self, 5)
    }
    #[doc = "Bit 6 - uphy0_pclk_tcpd clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn uphy0_pclk_tcpd_gate_en(&mut self) -> Uphy0PclkTcpdGateEnW<CruClkgateCon21Spec> {
        Uphy0PclkTcpdGateEnW::new(self, 6)
    }
    #[doc = "Bit 8 - uphy1_pclk_tcphy clock disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn uphy1_pclk_tcphy_gate_en(&mut self) -> Uphy1PclkTcphyGateEnW<CruClkgateCon21Spec> {
        Uphy1PclkTcphyGateEnW::new(self, 8)
    }
    #[doc = "Bit 9 - uphy1_pclk_tcpd disable bit When HIGH, disable clock"]
    #[inline(always)]
    #[must_use]
    pub fn uphy1_pclk_tcpd_gate_en(&mut self) -> Uphy1PclkTcpdGateEnW<CruClkgateCon21Spec> {
        Uphy1PclkTcpdGateEnW::new(self, 9)
    }
    #[doc = "Bits 16:31 - write mask bits When every bit HIGH, enable the writing corresponding bit When every bit LOW, don't care the writing corresponding bit"]
    #[inline(always)]
    #[must_use]
    pub fn write_mask(&mut self) -> WriteMaskW<CruClkgateCon21Spec> {
        WriteMaskW::new(self, 16)
    }
}
#[doc = "Internal clock gating register21\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cru_clkgate_con21::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cru_clkgate_con21::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CruClkgateCon21Spec;
impl crate::RegisterSpec for CruClkgateCon21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cru_clkgate_con21::R`](R) reader structure"]
impl crate::Readable for CruClkgateCon21Spec {}
#[doc = "`write(|w| ..)` method takes [`cru_clkgate_con21::W`](W) writer structure"]
impl crate::Writable for CruClkgateCon21Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CRU_CLKGATE_CON21 to value 0"]
impl crate::Resettable for CruClkgateCon21Spec {
    const RESET_VALUE: u32 = 0;
}
