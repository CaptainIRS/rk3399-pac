#[doc = "Register `DDR_DENALI_PHY_656` reader"]
pub type R = crate::R<DdrDenaliPhy656Spec>;
#[doc = "Register `DDR_DENALI_PHY_656` writer"]
pub type W = crate::W<DdrDenaliPhy656Spec>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE1_1_1` reader - CA training RD DQ bit swizzle map 1 for device 1 in address slice 1."]
pub type PhyAdrCalvlSwizzle1_1_1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE1_1_1` writer - CA training RD DQ bit swizzle map 1 for device 1 in address slice 1."]
pub type PhyAdrCalvlSwizzle1_1_1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `PHY_ADR_CALVL_DEVICE_MAP_1` reader - Defines the CA training device map for address slice 1."]
pub type PhyAdrCalvlDeviceMap1R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_DEVICE_MAP_1` writer - Defines the CA training device map for address slice 1."]
pub type PhyAdrCalvlDeviceMap1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:23 - CA training RD DQ bit swizzle map 1 for device 1 in address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_swizzle1_1_1(&self) -> PhyAdrCalvlSwizzle1_1_1R {
        PhyAdrCalvlSwizzle1_1_1R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:27 - Defines the CA training device map for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_device_map_1(&self) -> PhyAdrCalvlDeviceMap1R {
        PhyAdrCalvlDeviceMap1R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - CA training RD DQ bit swizzle map 1 for device 1 in address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_swizzle1_1_1(&mut self) -> PhyAdrCalvlSwizzle1_1_1W<DdrDenaliPhy656Spec> {
        PhyAdrCalvlSwizzle1_1_1W::new(self, 0)
    }
    #[doc = "Bits 24:27 - Defines the CA training device map for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_device_map_1(&mut self) -> PhyAdrCalvlDeviceMap1W<DdrDenaliPhy656Spec> {
        PhyAdrCalvlDeviceMap1W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_656::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_656::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy656Spec;
impl crate::RegisterSpec for DdrDenaliPhy656Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_656::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy656Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_656::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy656Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_656 to value 0"]
impl crate::Resettable for DdrDenaliPhy656Spec {
    const RESET_VALUE: u32 = 0;
}
