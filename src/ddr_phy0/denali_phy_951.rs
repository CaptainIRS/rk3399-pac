#[doc = "Register `DENALI_PHY_951` reader"]
pub type R = crate::R<DenaliPhy951Spec>;
#[doc = "Register `DENALI_PHY_951` writer"]
pub type W = crate::W<DenaliPhy951Spec>;
#[doc = "Field `PHY_AC_LPBK_OBS_SELECT` reader - Select value to map an individual address/control slice observation register to the global observation register."]
pub type PhyAcLpbkObsSelectR = crate::FieldReader;
#[doc = "Field `PHY_AC_LPBK_OBS_SELECT` writer - Select value to map an individual address/control slice observation register to the global observation register."]
pub type PhyAcLpbkObsSelectW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_AC_LPBK_ENABLE` reader - Loopback enable for the address/ control slices."]
pub type PhyAcLpbkEnableR = crate::FieldReader;
#[doc = "Field `PHY_AC_LPBK_ENABLE` writer - Loopback enable for the address/ control slices."]
pub type PhyAcLpbkEnableW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `PHY_AC_LPBK_CONTROL` reader - Address/control slice loopback control setting."]
pub type PhyAcLpbkControlR = crate::FieldReader;
#[doc = "Field `PHY_AC_LPBK_CONTROL` writer - Address/control slice loopback control setting."]
pub type PhyAcLpbkControlW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - Select value to map an individual address/control slice observation register to the global observation register."]
    #[inline(always)]
    pub fn phy_ac_lpbk_obs_select(&self) -> PhyAcLpbkObsSelectR {
        PhyAcLpbkObsSelectR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:11 - Loopback enable for the address/ control slices."]
    #[inline(always)]
    pub fn phy_ac_lpbk_enable(&self) -> PhyAcLpbkEnableR {
        PhyAcLpbkEnableR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 16:23 - Address/control slice loopback control setting."]
    #[inline(always)]
    pub fn phy_ac_lpbk_control(&self) -> PhyAcLpbkControlR {
        PhyAcLpbkControlR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Select value to map an individual address/control slice observation register to the global observation register."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_lpbk_obs_select(&mut self) -> PhyAcLpbkObsSelectW<DenaliPhy951Spec> {
        PhyAcLpbkObsSelectW::new(self, 0)
    }
    #[doc = "Bits 8:11 - Loopback enable for the address/ control slices."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_lpbk_enable(&mut self) -> PhyAcLpbkEnableW<DenaliPhy951Spec> {
        PhyAcLpbkEnableW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Address/control slice loopback control setting."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ac_lpbk_control(&mut self) -> PhyAcLpbkControlW<DenaliPhy951Spec> {
        PhyAcLpbkControlW::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_951::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_951::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy951Spec;
impl crate::RegisterSpec for DenaliPhy951Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_951::R`](R) reader structure"]
impl crate::Readable for DenaliPhy951Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_951::W`](W) writer structure"]
impl crate::Writable for DenaliPhy951Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_951 to value 0"]
impl crate::Resettable for DenaliPhy951Spec {
    const RESET_VALUE: u32 = 0;
}
