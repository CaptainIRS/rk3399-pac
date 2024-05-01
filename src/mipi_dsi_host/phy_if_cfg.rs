#[doc = "Register `PHY_IF_CFG` reader"]
pub type R = crate::R<PhyIfCfgSpec>;
#[doc = "Register `PHY_IF_CFG` writer"]
pub type W = crate::W<PhyIfCfgSpec>;
#[doc = "Field `N_LANES` reader - n_lanes\n\nThis field configures the number of active data lanes:\n\n■00: One data lane (lane 0)\n\n■01: Two data lanes (lanes 0 and 1)\n\n■10: Three data lanes (lanes 0, 1, and 2)\n\n■11: Four data lanes (lanes 0, 1, 2, and 3)"]
pub type NLanesR = crate::FieldReader;
#[doc = "Field `N_LANES` writer - n_lanes\n\nThis field configures the number of active data lanes:\n\n■00: One data lane (lane 0)\n\n■01: Two data lanes (lanes 0 and 1)\n\n■10: Three data lanes (lanes 0, 1, and 2)\n\n■11: Four data lanes (lanes 0, 1, 2, and 3)"]
pub type NLanesW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PHY_STOP_WAIT_TIME` reader - phy_stop_wait_time\n\nThis field configures the minimum wait period to request a\n\nhigh-speed transmission after the Stop state."]
pub type PhyStopWaitTimeR = crate::FieldReader;
#[doc = "Field `PHY_STOP_WAIT_TIME` writer - phy_stop_wait_time\n\nThis field configures the minimum wait period to request a\n\nhigh-speed transmission after the Stop state."]
pub type PhyStopWaitTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - n_lanes\n\nThis field configures the number of active data lanes:\n\n■00: One data lane (lane 0)\n\n■01: Two data lanes (lanes 0 and 1)\n\n■10: Three data lanes (lanes 0, 1, and 2)\n\n■11: Four data lanes (lanes 0, 1, 2, and 3)"]
    #[inline(always)]
    pub fn n_lanes(&self) -> NLanesR {
        NLanesR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - phy_stop_wait_time\n\nThis field configures the minimum wait period to request a\n\nhigh-speed transmission after the Stop state."]
    #[inline(always)]
    pub fn phy_stop_wait_time(&self) -> PhyStopWaitTimeR {
        PhyStopWaitTimeR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - n_lanes\n\nThis field configures the number of active data lanes:\n\n■00: One data lane (lane 0)\n\n■01: Two data lanes (lanes 0 and 1)\n\n■10: Three data lanes (lanes 0, 1, and 2)\n\n■11: Four data lanes (lanes 0, 1, 2, and 3)"]
    #[inline(always)]
    #[must_use]
    pub fn n_lanes(&mut self) -> NLanesW<PhyIfCfgSpec> {
        NLanesW::new(self, 0)
    }
    #[doc = "Bits 8:15 - phy_stop_wait_time\n\nThis field configures the minimum wait period to request a\n\nhigh-speed transmission after the Stop state."]
    #[inline(always)]
    #[must_use]
    pub fn phy_stop_wait_time(&mut self) -> PhyStopWaitTimeW<PhyIfCfgSpec> {
        PhyStopWaitTimeW::new(self, 8)
    }
}
#[doc = "D-PHY Interface Configuration Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_if_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_if_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyIfCfgSpec;
impl crate::RegisterSpec for PhyIfCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`phy_if_cfg::R`](R) reader structure"]
impl crate::Readable for PhyIfCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`phy_if_cfg::W`](W) writer structure"]
impl crate::Writable for PhyIfCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PHY_IF_CFG to value 0x03"]
impl crate::Resettable for PhyIfCfgSpec {
    const RESET_VALUE: u32 = 0x03;
}
