#[doc = "Register `PMA_LANE_CFG` reader"]
pub type R = crate::R<PmaLaneCfgSpec>;
#[doc = "Register `PMA_LANE_CFG` writer"]
pub type W = crate::W<PmaLaneCfgSpec>;
#[doc = "Field `FIELD11` reader - PMA lane 0 interface select - selects between PIPE PCS and PHY \n\nDP to drive PMA lane 0. 0 = PIPE PCS; 1 = PHY DP I/F. (Note: \n\nPHY DP I/F refers to PHY PMA transmit data interface, i.e. \n\nphy_pma_tx_data_ln_&lt;>/phy_pma_tx_data_clk_in, and the \n\ninternal DP configuration and control registers."]
pub type Field11R = crate::BitReader;
#[doc = "Field `FIELD11` writer - PMA lane 0 interface select - selects between PIPE PCS and PHY \n\nDP to drive PMA lane 0. 0 = PIPE PCS; 1 = PHY DP I/F. (Note: \n\nPHY DP I/F refers to PHY PMA transmit data interface, i.e. \n\nphy_pma_tx_data_ln_&lt;>/phy_pma_tx_data_clk_in, and the \n\ninternal DP configuration and control registers."]
pub type Field11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD10` reader - Reserved"]
pub type Field10R = crate::BitReader;
#[doc = "Field `FIELD9` reader - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 0, \n\nthis field selects which PHY DP lane drives PMA lane 0. (Only valid \n\nwith bit \\[0\\]
== 1.) When configured for DP, the same PHY DP lane \n\ncan be mapped to only 1 PMA lane. This mapping is applicable for a \n\nnormal connector orientation. The logic automatically adjusts the \n\nlane mapping for a flipped connector orientation."]
pub type Field9R = crate::FieldReader;
#[doc = "Field `FIELD9` writer - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 0, \n\nthis field selects which PHY DP lane drives PMA lane 0. (Only valid \n\nwith bit \\[0\\]
== 1.) When configured for DP, the same PHY DP lane \n\ncan be mapped to only 1 PMA lane. This mapping is applicable for a \n\nnormal connector orientation. The logic automatically adjusts the \n\nlane mapping for a flipped connector orientation."]
pub type Field9W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIELD8` reader - PMA lane 1 interface select - selects between PIPE PCS and PHY DP \n\ninterface to drive PMA lane 1. 0 = PIPE PCS; 1 = PHY DP I/F."]
pub type Field8R = crate::BitReader;
#[doc = "Field `FIELD8` writer - PMA lane 1 interface select - selects between PIPE PCS and PHY DP \n\ninterface to drive PMA lane 1. 0 = PIPE PCS; 1 = PHY DP I/F."]
pub type Field8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD7` reader - Reserved"]
pub type Field7R = crate::BitReader;
#[doc = "Field `FIELD6` reader - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 1, \n\nthis field selects which PHY DP lane drives PMA lane 1. (Only valid \n\nwith bit \\[4\\]
== 1.) When configured for DP, the same PHY DP lane \n\ncan be mapped to only 1 PMA lane. This mapping is applicable for a \n\nnormal connector orientation. The logic automatically adjusts the \n\nlane mapping for a flipped connector orientation."]
pub type Field6R = crate::FieldReader;
#[doc = "Field `FIELD6` writer - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 1, \n\nthis field selects which PHY DP lane drives PMA lane 1. (Only valid \n\nwith bit \\[4\\]
== 1.) When configured for DP, the same PHY DP lane \n\ncan be mapped to only 1 PMA lane. This mapping is applicable for a \n\nnormal connector orientation. The logic automatically adjusts the \n\nlane mapping for a flipped connector orientation."]
pub type Field6W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIELD5` reader - PMA lane 2 interface select - selects between PIPE PCS and PHY \n\nDP to drive PMA lane 2. 0 = PIPE PCS; 1 = PHY DP I/F."]
pub type Field5R = crate::BitReader;
#[doc = "Field `FIELD5` writer - PMA lane 2 interface select - selects between PIPE PCS and PHY \n\nDP to drive PMA lane 2. 0 = PIPE PCS; 1 = PHY DP I/F."]
pub type Field5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD4` reader - Reserved"]
pub type Field4R = crate::BitReader;
#[doc = "Field `FIELD3` reader - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 2, \n\nthis field selects which PHY DP lane drives PMA lane 2. (Only valid \n\nwith bit \\[8\\]
== 1.) When configured for DP, the same PHY DP lane \n\ncan be mapped to only 1 PMA lane. This mapping is applicable for a \n\nnormal connector orientation. The logic automatically adjusts the \n\nlane mapping for a flipped connector orientation."]
pub type Field3R = crate::FieldReader;
#[doc = "Field `FIELD3` writer - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 2, \n\nthis field selects which PHY DP lane drives PMA lane 2. (Only valid \n\nwith bit \\[8\\]
== 1.) When configured for DP, the same PHY DP lane \n\ncan be mapped to only 1 PMA lane. This mapping is applicable for a \n\nnormal connector orientation. The logic automatically adjusts the \n\nlane mapping for a flipped connector orientation."]
pub type Field3W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FIELD2` reader - PMA lane 3 interface select - selects between PIPE PCS and PHY \n\nDP to drive PMA lane 3. 0 = PIPE PCS; 1 = PHY DP I/F."]
pub type Field2R = crate::BitReader;
#[doc = "Field `FIELD2` writer - PMA lane 3 interface select - selects between PIPE PCS and PHY \n\nDP to drive PMA lane 3. 0 = PIPE PCS; 1 = PHY DP I/F."]
pub type Field2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD1` reader - Reserved"]
pub type Field1R = crate::BitReader;
#[doc = "Field `FIELD0` reader - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 3, \n\nthis field selects which PHY DP lane drives PMA lane 3. (Only valid \n\nwith bit \\[12\\]
== 1.) When configured for PHY DP, the same PHY DP \n\nlane can be mapped to only 1 PMA lane. This mapping is applicable \n\nfor a normal connector orientation. The logic automatically adjusts \n\nthe lane mapping for a flipped connector orientation."]
pub type Field0R = crate::FieldReader;
#[doc = "Field `FIELD0` writer - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 3, \n\nthis field selects which PHY DP lane drives PMA lane 3. (Only valid \n\nwith bit \\[12\\]
== 1.) When configured for PHY DP, the same PHY DP \n\nlane can be mapped to only 1 PMA lane. This mapping is applicable \n\nfor a normal connector orientation. The logic automatically adjusts \n\nthe lane mapping for a flipped connector orientation."]
pub type Field0W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - PMA lane 0 interface select - selects between PIPE PCS and PHY \n\nDP to drive PMA lane 0. 0 = PIPE PCS; 1 = PHY DP I/F. (Note: \n\nPHY DP I/F refers to PHY PMA transmit data interface, i.e. \n\nphy_pma_tx_data_ln_&lt;>/phy_pma_tx_data_clk_in, and the \n\ninternal DP configuration and control registers."]
    #[inline(always)]
    pub fn field11(&self) -> Field11R {
        Field11R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved"]
    #[inline(always)]
    pub fn field10(&self) -> Field10R {
        Field10R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 0, \n\nthis field selects which PHY DP lane drives PMA lane 0. (Only valid \n\nwith bit \\[0\\]
== 1.) When configured for DP, the same PHY DP lane \n\ncan be mapped to only 1 PMA lane. This mapping is applicable for a \n\nnormal connector orientation. The logic automatically adjusts the \n\nlane mapping for a flipped connector orientation."]
    #[inline(always)]
    pub fn field9(&self) -> Field9R {
        Field9R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - PMA lane 1 interface select - selects between PIPE PCS and PHY DP \n\ninterface to drive PMA lane 1. 0 = PIPE PCS; 1 = PHY DP I/F."]
    #[inline(always)]
    pub fn field8(&self) -> Field8R {
        Field8R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reserved"]
    #[inline(always)]
    pub fn field7(&self) -> Field7R {
        Field7R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 1, \n\nthis field selects which PHY DP lane drives PMA lane 1. (Only valid \n\nwith bit \\[4\\]
== 1.) When configured for DP, the same PHY DP lane \n\ncan be mapped to only 1 PMA lane. This mapping is applicable for a \n\nnormal connector orientation. The logic automatically adjusts the \n\nlane mapping for a flipped connector orientation."]
    #[inline(always)]
    pub fn field6(&self) -> Field6R {
        Field6R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - PMA lane 2 interface select - selects between PIPE PCS and PHY \n\nDP to drive PMA lane 2. 0 = PIPE PCS; 1 = PHY DP I/F."]
    #[inline(always)]
    pub fn field5(&self) -> Field5R {
        Field5R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Reserved"]
    #[inline(always)]
    pub fn field4(&self) -> Field4R {
        Field4R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 2, \n\nthis field selects which PHY DP lane drives PMA lane 2. (Only valid \n\nwith bit \\[8\\]
== 1.) When configured for DP, the same PHY DP lane \n\ncan be mapped to only 1 PMA lane. This mapping is applicable for a \n\nnormal connector orientation. The logic automatically adjusts the \n\nlane mapping for a flipped connector orientation."]
    #[inline(always)]
    pub fn field3(&self) -> Field3R {
        Field3R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - PMA lane 3 interface select - selects between PIPE PCS and PHY \n\nDP to drive PMA lane 3. 0 = PIPE PCS; 1 = PHY DP I/F."]
    #[inline(always)]
    pub fn field2(&self) -> Field2R {
        Field2R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 3, \n\nthis field selects which PHY DP lane drives PMA lane 3. (Only valid \n\nwith bit \\[12\\]
== 1.) When configured for PHY DP, the same PHY DP \n\nlane can be mapped to only 1 PMA lane. This mapping is applicable \n\nfor a normal connector orientation. The logic automatically adjusts \n\nthe lane mapping for a flipped connector orientation."]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - PMA lane 0 interface select - selects between PIPE PCS and PHY \n\nDP to drive PMA lane 0. 0 = PIPE PCS; 1 = PHY DP I/F. (Note: \n\nPHY DP I/F refers to PHY PMA transmit data interface, i.e. \n\nphy_pma_tx_data_ln_&lt;>/phy_pma_tx_data_clk_in, and the \n\ninternal DP configuration and control registers."]
    #[inline(always)]
    #[must_use]
    pub fn field11(&mut self) -> Field11W<PmaLaneCfgSpec> {
        Field11W::new(self, 0)
    }
    #[doc = "Bits 2:3 - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 0, \n\nthis field selects which PHY DP lane drives PMA lane 0. (Only valid \n\nwith bit \\[0\\]
== 1.) When configured for DP, the same PHY DP lane \n\ncan be mapped to only 1 PMA lane. This mapping is applicable for a \n\nnormal connector orientation. The logic automatically adjusts the \n\nlane mapping for a flipped connector orientation."]
    #[inline(always)]
    #[must_use]
    pub fn field9(&mut self) -> Field9W<PmaLaneCfgSpec> {
        Field9W::new(self, 2)
    }
    #[doc = "Bit 4 - PMA lane 1 interface select - selects between PIPE PCS and PHY DP \n\ninterface to drive PMA lane 1. 0 = PIPE PCS; 1 = PHY DP I/F."]
    #[inline(always)]
    #[must_use]
    pub fn field8(&mut self) -> Field8W<PmaLaneCfgSpec> {
        Field8W::new(self, 4)
    }
    #[doc = "Bits 6:7 - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 1, \n\nthis field selects which PHY DP lane drives PMA lane 1. (Only valid \n\nwith bit \\[4\\]
== 1.) When configured for DP, the same PHY DP lane \n\ncan be mapped to only 1 PMA lane. This mapping is applicable for a \n\nnormal connector orientation. The logic automatically adjusts the \n\nlane mapping for a flipped connector orientation."]
    #[inline(always)]
    #[must_use]
    pub fn field6(&mut self) -> Field6W<PmaLaneCfgSpec> {
        Field6W::new(self, 6)
    }
    #[doc = "Bit 8 - PMA lane 2 interface select - selects between PIPE PCS and PHY \n\nDP to drive PMA lane 2. 0 = PIPE PCS; 1 = PHY DP I/F."]
    #[inline(always)]
    #[must_use]
    pub fn field5(&mut self) -> Field5W<PmaLaneCfgSpec> {
        Field5W::new(self, 8)
    }
    #[doc = "Bits 10:11 - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 2, \n\nthis field selects which PHY DP lane drives PMA lane 2. (Only valid \n\nwith bit \\[8\\]
== 1.) When configured for DP, the same PHY DP lane \n\ncan be mapped to only 1 PMA lane. This mapping is applicable for a \n\nnormal connector orientation. The logic automatically adjusts the \n\nlane mapping for a flipped connector orientation."]
    #[inline(always)]
    #[must_use]
    pub fn field3(&mut self) -> Field3W<PmaLaneCfgSpec> {
        Field3W::new(self, 10)
    }
    #[doc = "Bit 12 - PMA lane 3 interface select - selects between PIPE PCS and PHY \n\nDP to drive PMA lane 3. 0 = PIPE PCS; 1 = PHY DP I/F."]
    #[inline(always)]
    #[must_use]
    pub fn field2(&mut self) -> Field2W<PmaLaneCfgSpec> {
        Field2W::new(self, 12)
    }
    #[doc = "Bits 14:15 - PHY DP lane selection - when PHY DP I/F is selected for PMA lane 3, \n\nthis field selects which PHY DP lane drives PMA lane 3. (Only valid \n\nwith bit \\[12\\]
== 1.) When configured for PHY DP, the same PHY DP \n\nlane can be mapped to only 1 PMA lane. This mapping is applicable \n\nfor a normal connector orientation. The logic automatically adjusts \n\nthe lane mapping for a flipped connector orientation."]
    #[inline(always)]
    #[must_use]
    pub fn field0(&mut self) -> Field0W<PmaLaneCfgSpec> {
        Field0W::new(self, 14)
    }
}
#[doc = "PMA lane configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pma_lane_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pma_lane_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmaLaneCfgSpec;
impl crate::RegisterSpec for PmaLaneCfgSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pma_lane_cfg::R`](R) reader structure"]
impl crate::Readable for PmaLaneCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`pma_lane_cfg::W`](W) writer structure"]
impl crate::Writable for PmaLaneCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets PMA_LANE_CFG to value 0x5100"]
impl crate::Resettable for PmaLaneCfgSpec {
    const RESET_VALUE: u16 = 0x5100;
}
