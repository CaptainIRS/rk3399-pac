#[doc = "Register `DDR_DENALI_PHY_919` reader"]
pub type R = crate::R<DdrDenaliPhy919Spec>;
#[doc = "Register `DDR_DENALI_PHY_919` writer"]
pub type W = crate::W<DdrDenaliPhy919Spec>;
#[doc = "Field `PHY_LP4_BOOT_PLL_CTRL` reader - PHY deskew PLL controls for LPDDR4 boot frequency."]
pub type PhyLp4BootPllCtrlR = crate::FieldReader<u16>;
#[doc = "Field `PHY_LP4_BOOT_PLL_CTRL` writer - PHY deskew PLL controls for LPDDR4 boot frequency."]
pub type PhyLp4BootPllCtrlW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
#[doc = "Field `PHY_LP4_BOOT_PLL_CTRL_CA` reader - PHY deskew PLL controls for LPDDR4 boot frequency for 2X ca slice."]
pub type PhyLp4BootPllCtrlCaR = crate::FieldReader<u16>;
#[doc = "Field `PHY_LP4_BOOT_PLL_CTRL_CA` writer - PHY deskew PLL controls for LPDDR4 boot frequency for 2X ca slice."]
pub type PhyLp4BootPllCtrlCaW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - PHY deskew PLL controls for LPDDR4 boot frequency."]
    #[inline(always)]
    pub fn phy_lp4_boot_pll_ctrl(&self) -> PhyLp4BootPllCtrlR {
        PhyLp4BootPllCtrlR::new((self.bits & 0x1fff) as u16)
    }
    #[doc = "Bits 16:28 - PHY deskew PLL controls for LPDDR4 boot frequency for 2X ca slice."]
    #[inline(always)]
    pub fn phy_lp4_boot_pll_ctrl_ca(&self) -> PhyLp4BootPllCtrlCaR {
        PhyLp4BootPllCtrlCaR::new(((self.bits >> 16) & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - PHY deskew PLL controls for LPDDR4 boot frequency."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_pll_ctrl(&mut self) -> PhyLp4BootPllCtrlW<DdrDenaliPhy919Spec> {
        PhyLp4BootPllCtrlW::new(self, 0)
    }
    #[doc = "Bits 16:28 - PHY deskew PLL controls for LPDDR4 boot frequency for 2X ca slice."]
    #[inline(always)]
    #[must_use]
    pub fn phy_lp4_boot_pll_ctrl_ca(&mut self) -> PhyLp4BootPllCtrlCaW<DdrDenaliPhy919Spec> {
        PhyLp4BootPllCtrlCaW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_919::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_919::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy919Spec;
impl crate::RegisterSpec for DdrDenaliPhy919Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_919::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy919Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_919::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy919Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_919 to value 0"]
impl crate::Resettable for DdrDenaliPhy919Spec {
    const RESET_VALUE: u32 = 0;
}
