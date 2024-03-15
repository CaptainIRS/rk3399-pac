#[doc = "Register `GRF_SOC_CON6` reader"]
pub type R = crate::R<GrfSocCon6Spec>;
#[doc = "Register `GRF_SOC_CON6` writer"]
pub type W = crate::W<GrfSocCon6Spec>;
#[doc = "Field `GMAC_CLK_TX_DL_CFG` reader - RGMII TX clock delayline value"]
pub type GmacClkTxDlCfgR = crate::FieldReader;
#[doc = "Field `GMAC_CLK_TX_DL_CFG` writer - RGMII TX clock delayline value"]
pub type GmacClkTxDlCfgW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "RGMII TX clock delayline enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GmacTxclkDlyEna {
    #[doc = "1: disable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<GmacTxclkDlyEna> for bool {
    #[inline(always)]
    fn from(variant: GmacTxclkDlyEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GMAC_TXCLK_DLY_ENA` reader - RGMII TX clock delayline enable"]
pub type GmacTxclkDlyEnaR = crate::BitReader<GmacTxclkDlyEna>;
impl GmacTxclkDlyEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GmacTxclkDlyEna {
        match self.bits {
            true => GmacTxclkDlyEna::B1,
            false => GmacTxclkDlyEna::B0,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GmacTxclkDlyEna::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GmacTxclkDlyEna::B0
    }
}
#[doc = "Field `GMAC_TXCLK_DLY_ENA` writer - RGMII TX clock delayline enable"]
pub type GmacTxclkDlyEnaW<'a, REG> = crate::BitWriter<'a, REG, GmacTxclkDlyEna>;
impl<'a, REG> GmacTxclkDlyEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GmacTxclkDlyEna::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GmacTxclkDlyEna::B0)
    }
}
#[doc = "Field `GMAC_CLK_RX_DL_CFG` reader - RGMII RX clock delayline value"]
pub type GmacClkRxDlCfgR = crate::FieldReader;
#[doc = "Field `GMAC_CLK_RX_DL_CFG` writer - RGMII RX clock delayline value"]
pub type GmacClkRxDlCfgW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "RGMII TX clock delayline enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GmacRxclkDlyEna {
    #[doc = "1: disable"]
    B1 = 1,
    #[doc = "0: disable"]
    B0 = 0,
}
impl From<GmacRxclkDlyEna> for bool {
    #[inline(always)]
    fn from(variant: GmacRxclkDlyEna) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `GMAC_RXCLK_DLY_ENA` reader - RGMII TX clock delayline enable"]
pub type GmacRxclkDlyEnaR = crate::BitReader<GmacRxclkDlyEna>;
impl GmacRxclkDlyEnaR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> GmacRxclkDlyEna {
        match self.bits {
            true => GmacRxclkDlyEna::B1,
            false => GmacRxclkDlyEna::B0,
        }
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b1(&self) -> bool {
        *self == GmacRxclkDlyEna::B1
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn is_b0(&self) -> bool {
        *self == GmacRxclkDlyEna::B0
    }
}
#[doc = "Field `GMAC_RXCLK_DLY_ENA` writer - RGMII TX clock delayline enable"]
pub type GmacRxclkDlyEnaW<'a, REG> = crate::BitWriter<'a, REG, GmacRxclkDlyEna>;
impl<'a, REG> GmacRxclkDlyEnaW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "disable"]
    #[inline(always)]
    pub fn b1(self) -> &'a mut crate::W<REG> {
        self.variant(GmacRxclkDlyEna::B1)
    }
    #[doc = "disable"]
    #[inline(always)]
    pub fn b0(self) -> &'a mut crate::W<REG> {
        self.variant(GmacRxclkDlyEna::B0)
    }
}
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:6 - RGMII TX clock delayline value"]
    #[inline(always)]
    pub fn gmac_clk_tx_dl_cfg(&self) -> GmacClkTxDlCfgR {
        GmacClkTxDlCfgR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 7 - RGMII TX clock delayline enable"]
    #[inline(always)]
    pub fn gmac_txclk_dly_ena(&self) -> GmacTxclkDlyEnaR {
        GmacTxclkDlyEnaR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:14 - RGMII RX clock delayline value"]
    #[inline(always)]
    pub fn gmac_clk_rx_dl_cfg(&self) -> GmacClkRxDlCfgR {
        GmacClkRxDlCfgR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - RGMII TX clock delayline enable"]
    #[inline(always)]
    pub fn gmac_rxclk_dly_ena(&self) -> GmacRxclkDlyEnaR {
        GmacRxclkDlyEnaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:6 - RGMII TX clock delayline value"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_clk_tx_dl_cfg(&mut self) -> GmacClkTxDlCfgW<GrfSocCon6Spec> {
        GmacClkTxDlCfgW::new(self, 0)
    }
    #[doc = "Bit 7 - RGMII TX clock delayline enable"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_txclk_dly_ena(&mut self) -> GmacTxclkDlyEnaW<GrfSocCon6Spec> {
        GmacTxclkDlyEnaW::new(self, 7)
    }
    #[doc = "Bits 8:14 - RGMII RX clock delayline value"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_clk_rx_dl_cfg(&mut self) -> GmacClkRxDlCfgW<GrfSocCon6Spec> {
        GmacClkRxDlCfgW::new(self, 8)
    }
    #[doc = "Bit 15 - RGMII TX clock delayline enable"]
    #[inline(always)]
    #[must_use]
    pub fn gmac_rxclk_dly_ena(&mut self) -> GmacRxclkDlyEnaW<GrfSocCon6Spec> {
        GmacRxclkDlyEnaW::new(self, 15)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable When bit 16=1, bit 0 can be written by software . When bit 16=0, bit 0 cannot be written by software; When bit 17=1, bit 1 can be written by software . When bit 17=0, bit 1 cannot be written by software; ...... When bit 31=1, bit 15 can be written by software . When bit 31=0, bit 15 cannot be written by software;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<GrfSocCon6Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 6\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`grf_soc_con6::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`grf_soc_con6::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GrfSocCon6Spec;
impl crate::RegisterSpec for GrfSocCon6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`grf_soc_con6::R`](R) reader structure"]
impl crate::Readable for GrfSocCon6Spec {}
#[doc = "`write(|w| ..)` method takes [`grf_soc_con6::W`](W) writer structure"]
impl crate::Writable for GrfSocCon6Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GRF_SOC_CON6 to value 0"]
impl crate::Resettable for GrfSocCon6Spec {
    const RESET_VALUE: u32 = 0;
}
