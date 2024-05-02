#[doc = "Register `DP_CLK_CTL` reader"]
pub type R = crate::R<DpClkCtlSpec>;
#[doc = "Register `DP_CLK_CTL` writer"]
pub type W = crate::W<DpClkCtlSpec>;
#[doc = "Field `FIELD6` reader - DP PLL enable - DP PLL's enable for DP high speed clocks. 1 = enabled, \n\n0 = disabled."]
pub type Field6R = crate::BitReader;
#[doc = "Field `FIELD6` writer - DP PLL enable - DP PLL's enable for DP high speed clocks. 1 = enabled, \n\n0 = disabled."]
pub type Field6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD5` reader - DP PLL ready - DP PLL's ready indication for DP high speed clocks. 1 = \n\nPLL ready, 0 = PLL not ready."]
pub type Field5R = crate::BitReader;
#[doc = "Field `FIELD4` reader - DP PLL clock enable - Clock enable for DP PLL's data rate and full rate \n\nclocks out of PMA. 1 = enable PLL data rate and full rate clocks, 0 = gate \n\nPLL data rate and full rate clocks"]
pub type Field4R = crate::BitReader;
#[doc = "Field `FIELD4` writer - DP PLL clock enable - Clock enable for DP PLL's data rate and full rate \n\nclocks out of PMA. 1 = enable PLL data rate and full rate clocks, 0 = gate \n\nPLL data rate and full rate clocks"]
pub type Field4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD3` reader - DP PLL clock enable acknowledge - Indicates whether DP PLL's data rate \n\nand full rate clocks are active/ enabled. 1 = clocks enabled/active, 0 \n\n= clocks disabled/gated."]
pub type Field3R = crate::BitReader;
#[doc = "Field `FIELD2` reader - Reserved"]
pub type Field2R = crate::FieldReader;
#[doc = "Field `FIELD1` reader - DP PLL data rate 0 clock divider value. Divider value for \n\nthe PLL clock to generate phy_pma_tx_data_clk_out. \n\n(HBR2 = 2, RBR/HBR = 4.)"]
pub type Field1R = crate::FieldReader;
#[doc = "Field `FIELD1` writer - DP PLL data rate 0 clock divider value. Divider value for \n\nthe PLL clock to generate phy_pma_tx_data_clk_out. \n\n(HBR2 = 2, RBR/HBR = 4.)"]
pub type Field1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `FIELD0` reader - DP PLL data rate 1 clock divider value. Divider value for \n\nthe \n\nPLL \n\nclock \n\nto \n\ngenerate \n\nphy_pma_char_clk_out. \n\n(HBR2 = 1, RBR/HBR = 2.)"]
pub type Field0R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - DP PLL enable - DP PLL's enable for DP high speed clocks. 1 = enabled, \n\n0 = disabled."]
    #[inline(always)]
    pub fn field6(&self) -> Field6R {
        Field6R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DP PLL ready - DP PLL's ready indication for DP high speed clocks. 1 = \n\nPLL ready, 0 = PLL not ready."]
    #[inline(always)]
    pub fn field5(&self) -> Field5R {
        Field5R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DP PLL clock enable - Clock enable for DP PLL's data rate and full rate \n\nclocks out of PMA. 1 = enable PLL data rate and full rate clocks, 0 = gate \n\nPLL data rate and full rate clocks"]
    #[inline(always)]
    pub fn field4(&self) -> Field4R {
        Field4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DP PLL clock enable acknowledge - Indicates whether DP PLL's data rate \n\nand full rate clocks are active/ enabled. 1 = clocks enabled/active, 0 \n\n= clocks disabled/gated."]
    #[inline(always)]
    pub fn field3(&self) -> Field3R {
        Field3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Reserved"]
    #[inline(always)]
    pub fn field2(&self) -> Field2R {
        Field2R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - DP PLL data rate 0 clock divider value. Divider value for \n\nthe PLL clock to generate phy_pma_tx_data_clk_out. \n\n(HBR2 = 2, RBR/HBR = 4.)"]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - DP PLL data rate 1 clock divider value. Divider value for \n\nthe \n\nPLL \n\nclock \n\nto \n\ngenerate \n\nphy_pma_char_clk_out. \n\n(HBR2 = 1, RBR/HBR = 2.)"]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - DP PLL enable - DP PLL's enable for DP high speed clocks. 1 = enabled, \n\n0 = disabled."]
    #[inline(always)]
    #[must_use]
    pub fn field6(&mut self) -> Field6W<DpClkCtlSpec> {
        Field6W::new(self, 0)
    }
    #[doc = "Bit 2 - DP PLL clock enable - Clock enable for DP PLL's data rate and full rate \n\nclocks out of PMA. 1 = enable PLL data rate and full rate clocks, 0 = gate \n\nPLL data rate and full rate clocks"]
    #[inline(always)]
    #[must_use]
    pub fn field4(&mut self) -> Field4W<DpClkCtlSpec> {
        Field4W::new(self, 2)
    }
    #[doc = "Bits 8:11 - DP PLL data rate 0 clock divider value. Divider value for \n\nthe PLL clock to generate phy_pma_tx_data_clk_out. \n\n(HBR2 = 2, RBR/HBR = 4.)"]
    #[inline(always)]
    #[must_use]
    pub fn field1(&mut self) -> Field1W<DpClkCtlSpec> {
        Field1W::new(self, 8)
    }
}
#[doc = "DP Clock Control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dp_clk_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dp_clk_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DpClkCtlSpec;
impl crate::RegisterSpec for DpClkCtlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`dp_clk_ctl::R`](R) reader structure"]
impl crate::Readable for DpClkCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`dp_clk_ctl::W`](W) writer structure"]
impl crate::Writable for DpClkCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets DP_CLK_CTL to value 0x1205"]
impl crate::Resettable for DpClkCtlSpec {
    const RESET_VALUE: u16 = 0x1205;
}
