#[doc = "Register `DDR_DENALI_PHY_953` reader"]
pub type R = crate::R<DdrDenaliPhy953Spec>;
#[doc = "Register `DDR_DENALI_PHY_953` writer"]
pub type W = crate::W<DdrDenaliPhy953Spec>;
#[doc = "Field `PHY_AC_CLK_LPBK_OBS_SELECT` reader - Select value to map an individual mem clk block observation register to the global observation register."]
pub type PhyAcClkLpbkObsSelectR = crate::BitReader;
#[doc = "Field `PHY_AC_CLK_LPBK_OBS_SELECT` writer - Select value to map an individual mem clk block observation register to the global observation register."]
pub type PhyAcClkLpbkObsSelectW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PHY_AC_CLK_LPBK_ENABLE` reader - Loopback enable for mem clk blocks."]
pub type PhyAcClkLpbkEnableR = crate::FieldReader;
#[doc = "Field `PHY_AC_CLK_LPBK_ENABLE` writer - Loopback enable for mem clk blocks."]
pub type PhyAcClkLpbkEnableW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_AC_CLK_LPBK_CONTROL` reader - Mem clk block loopback control setting."]
pub type PhyAcClkLpbkControlR = crate::FieldReader;
#[doc = "Field `PHY_AC_CLK_LPBK_CONTROL` writer - Mem clk block loopback control setting."]
pub type PhyAcClkLpbkControlW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bit 0 - Select value to map an individual mem clk block observation register to the global observation register."]
    #[inline(always)]
    pub fn phy_ac_clk_lpbk_obs_select(&self) -> PhyAcClkLpbkObsSelectR {
        PhyAcClkLpbkObsSelectR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 8:9 - Loopback enable for mem clk blocks."]
    #[inline(always)]
    pub fn phy_ac_clk_lpbk_enable(&self) -> PhyAcClkLpbkEnableR {
        PhyAcClkLpbkEnableR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:21 - Mem clk block loopback control setting."]
    #[inline(always)]
    pub fn phy_ac_clk_lpbk_control(&self) -> PhyAcClkLpbkControlR {
        PhyAcClkLpbkControlR::new(((self.bits >> 16) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Select value to map an individual mem clk block observation register to the global observation register."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_clk_lpbk_obs_select(&mut self) -> PhyAcClkLpbkObsSelectW<DdrDenaliPhy953Spec> {
        PhyAcClkLpbkObsSelectW::new(self, 0)
    }
    #[doc = "Bits 8:9 - Loopback enable for mem clk blocks."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_clk_lpbk_enable(&mut self) -> PhyAcClkLpbkEnableW<DdrDenaliPhy953Spec> {
        PhyAcClkLpbkEnableW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Mem clk block loopback control setting."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_clk_lpbk_control(&mut self) -> PhyAcClkLpbkControlW<DdrDenaliPhy953Spec> {
        PhyAcClkLpbkControlW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_953::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_953::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy953Spec;
impl crate::RegisterSpec for DdrDenaliPhy953Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_953::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy953Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_953::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy953Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_953 to value 0"]
impl crate::Resettable for DdrDenaliPhy953Spec {
    const RESET_VALUE: u32 = 0;
}
