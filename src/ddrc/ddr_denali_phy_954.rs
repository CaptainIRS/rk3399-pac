#[doc = "Register `DDR_DENALI_PHY_954` reader"]
pub type R = crate::R<DdrDenaliPhy954Spec>;
#[doc = "Register `DDR_DENALI_PHY_954` writer"]
pub type W = crate::W<DdrDenaliPhy954Spec>;
#[doc = "Field `PHY_AC_CLK_LPBK_RESULT_OBS` reader - Observation register for mem clk blocks."]
pub type PhyAcClkLpbkResultObsR = crate::FieldReader<u16>;
#[doc = "Field `PHY_AC_PWR_RDC_DISABLE` reader - ac slice power reduction disable."]
pub type PhyAcPwrRdcDisableR = crate::BitReader;
#[doc = "Field `PHY_AC_PWR_RDC_DISABLE` writer - ac slice power reduction disable."]
pub type PhyAcPwrRdcDisableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_DATA_BYTE_ORDER_SEL` reader - Used to define the data slicesbyteswap.'"]
pub type PhyDataByteOrderSelR = crate::FieldReader;
#[doc = "Field `PHY_DATA_BYTE_ORDER_SEL` writer - Used to define the data slicesbyteswap.'"]
pub type PhyDataByteOrderSelW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:15 - Observation register for mem clk blocks."]
    #[inline(always)]
    pub fn phy_ac_clk_lpbk_result_obs(&self) -> PhyAcClkLpbkResultObsR {
        PhyAcClkLpbkResultObsR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - ac slice power reduction disable."]
    #[inline(always)]
    pub fn phy_ac_pwr_rdc_disable(&self) -> PhyAcPwrRdcDisableR {
        PhyAcPwrRdcDisableR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Used to define the data slicesbyteswap.'"]
    #[inline(always)]
    pub fn phy_data_byte_order_sel(&self) -> PhyDataByteOrderSelR {
        PhyDataByteOrderSelR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 16 - ac slice power reduction disable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_pwr_rdc_disable(&mut self) -> PhyAcPwrRdcDisableW<DdrDenaliPhy954Spec> {
        PhyAcPwrRdcDisableW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Used to define the data slicesbyteswap.'"]
    #[inline(always)]
    #[must_use]
    pub fn phy_data_byte_order_sel(&mut self) -> PhyDataByteOrderSelW<DdrDenaliPhy954Spec> {
        PhyDataByteOrderSelW::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_954::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_954::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy954Spec;
impl crate::RegisterSpec for DdrDenaliPhy954Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_954::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy954Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_954::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy954Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_954 to value 0"]
impl crate::Resettable for DdrDenaliPhy954Spec {
    const RESET_VALUE: u32 = 0;
}
