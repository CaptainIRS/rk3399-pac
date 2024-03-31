#[doc = "Register `SOC_CON25` reader"]
pub type R = crate::R<SocCon25Spec>;
#[doc = "Register `SOC_CON25` writer"]
pub type W = crate::W<SocCon25Spec>;
#[doc = "Field `DPHY_RX0_TSETDIN` reader - dphy_rx0_tsetdin bit control"]
pub type DphyRx0TsetdinR = crate::FieldReader;
#[doc = "Field `DPHY_RX0_TSETDIN` writer - dphy_rx0_tsetdin bit control"]
pub type DphyRx0TsetdinW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `DPHY_RX0_TSETEN` reader - dphy_rx0_tseten bit control"]
pub type DphyRx0TsetenR = crate::BitReader;
#[doc = "Field `DPHY_RX0_TSETEN` writer - dphy_rx0_tseten bit control"]
pub type DphyRx0TsetenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_RX0_TSETCLK` reader - dphy_rx0_tsetclk bit control"]
pub type DphyRx0TsetclkR = crate::BitReader;
#[doc = "Field `DPHY_RX0_TSETCLK` writer - dphy_rx0_tsetclk bit control"]
pub type DphyRx0TsetclkW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DPHY_RX0_TSETCLR` reader - dphy_rx0_tsetclr bit control"]
pub type DphyRx0TsetclrR = crate::BitReader;
#[doc = "Field `DPHY_RX0_TSETCLR` writer - dphy_rx0_tsetclr bit control"]
pub type DphyRx0TsetclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDP_REF_CLK_SEL` reader - edp_ref_clk_sel bit control"]
pub type EdpRefClkSelR = crate::BitReader;
#[doc = "Field `EDP_REF_CLK_SEL` writer - edp_ref_clk_sel bit control"]
pub type EdpRefClkSelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EDP_TX_BSCAN_DATA` reader - edp_tx_bscan_data bit control"]
pub type EdpTxBscanDataR = crate::FieldReader;
#[doc = "Field `EDP_TX_BSCAN_DATA` writer - edp_tx_bscan_data bit control"]
pub type EdpTxBscanDataW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `WRITE_ENABLE` reader - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableR = crate::FieldReader<u16>;
#[doc = "Field `WRITE_ENABLE` writer - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
pub type WriteEnableW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:7 - dphy_rx0_tsetdin bit control"]
    #[inline(always)]
    pub fn dphy_rx0_tsetdin(&self) -> DphyRx0TsetdinR {
        DphyRx0TsetdinR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - dphy_rx0_tseten bit control"]
    #[inline(always)]
    pub fn dphy_rx0_tseten(&self) -> DphyRx0TsetenR {
        DphyRx0TsetenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - dphy_rx0_tsetclk bit control"]
    #[inline(always)]
    pub fn dphy_rx0_tsetclk(&self) -> DphyRx0TsetclkR {
        DphyRx0TsetclkR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - dphy_rx0_tsetclr bit control"]
    #[inline(always)]
    pub fn dphy_rx0_tsetclr(&self) -> DphyRx0TsetclrR {
        DphyRx0TsetclrR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - edp_ref_clk_sel bit control"]
    #[inline(always)]
    pub fn edp_ref_clk_sel(&self) -> EdpRefClkSelR {
        EdpRefClkSelR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - edp_tx_bscan_data bit control"]
    #[inline(always)]
    pub fn edp_tx_bscan_data(&self) -> EdpTxBscanDataR {
        EdpTxBscanDataR::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    pub fn write_enable(&self) -> WriteEnableR {
        WriteEnableR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:7 - dphy_rx0_tsetdin bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rx0_tsetdin(&mut self) -> DphyRx0TsetdinW<SocCon25Spec> {
        DphyRx0TsetdinW::new(self, 0)
    }
    #[doc = "Bit 8 - dphy_rx0_tseten bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rx0_tseten(&mut self) -> DphyRx0TsetenW<SocCon25Spec> {
        DphyRx0TsetenW::new(self, 8)
    }
    #[doc = "Bit 9 - dphy_rx0_tsetclk bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rx0_tsetclk(&mut self) -> DphyRx0TsetclkW<SocCon25Spec> {
        DphyRx0TsetclkW::new(self, 9)
    }
    #[doc = "Bit 10 - dphy_rx0_tsetclr bit control"]
    #[inline(always)]
    #[must_use]
    pub fn dphy_rx0_tsetclr(&mut self) -> DphyRx0TsetclrW<SocCon25Spec> {
        DphyRx0TsetclrW::new(self, 10)
    }
    #[doc = "Bit 11 - edp_ref_clk_sel bit control"]
    #[inline(always)]
    #[must_use]
    pub fn edp_ref_clk_sel(&mut self) -> EdpRefClkSelW<SocCon25Spec> {
        EdpRefClkSelW::new(self, 11)
    }
    #[doc = "Bits 12:15 - edp_tx_bscan_data bit control"]
    #[inline(always)]
    #[must_use]
    pub fn edp_tx_bscan_data(&mut self) -> EdpTxBscanDataW<SocCon25Spec> {
        EdpTxBscanDataW::new(self, 12)
    }
    #[doc = "Bits 16:31 - bit0~15 write enable\n\nWhen bit 16=1, bit 0 can be written by\n\nsoftware .\n\nWhen bit 16=0, bit 0 cannot be written by\n\nsoftware;\n\nWhen bit 17=1, bit 1 can be written by\n\nsoftware .\n\nWhen bit 17=0, bit 1 cannot be written by\n\nsoftware;\n\n......\n\nWhen bit 31=1, bit 15 can be written by\n\nsoftware .\n\nWhen bit 31=0, bit 15 cannot be written by\n\nsoftware;"]
    #[inline(always)]
    #[must_use]
    pub fn write_enable(&mut self) -> WriteEnableW<SocCon25Spec> {
        WriteEnableW::new(self, 16)
    }
}
#[doc = "SoC control register 25\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`soc_con25::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`soc_con25::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SocCon25Spec;
impl crate::RegisterSpec for SocCon25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`soc_con25::R`](R) reader structure"]
impl crate::Readable for SocCon25Spec {}
#[doc = "`write(|w| ..)` method takes [`soc_con25::W`](W) writer structure"]
impl crate::Writable for SocCon25Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SOC_CON25 to value 0xd45b"]
impl crate::Resettable for SocCon25Spec {
    const RESET_VALUE: u32 = 0xd45b;
}
