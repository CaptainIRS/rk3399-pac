#[doc = "Register `DENALI_PHY_528` reader"]
pub type R = crate::R<DenaliPhy528Spec>;
#[doc = "Register `DENALI_PHY_528` writer"]
pub type W = crate::W<DenaliPhy528Spec>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE1_1_0` reader - CA training RD DQ bit swizzle map 1 for device 1 in address slice 0."]
pub type PhyAdrCalvlSwizzle1_1_0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE1_1_0` writer - CA training RD DQ bit swizzle map 1 for device 1 in address slice 0."]
pub type PhyAdrCalvlSwizzle1_1_0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
#[doc = "Field `PHY_ADR_CALVL_DEVICE_MAP_0` reader - Defines the CA training device map for address slice 0."]
pub type PhyAdrCalvlDeviceMap0R = crate::FieldReader;
#[doc = "Field `PHY_ADR_CALVL_DEVICE_MAP_0` writer - Defines the CA training device map for address slice 0."]
pub type PhyAdrCalvlDeviceMap0W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:23 - CA training RD DQ bit swizzle map 1 for device 1 in address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_swizzle1_1_0(&self) -> PhyAdrCalvlSwizzle1_1_0R {
        PhyAdrCalvlSwizzle1_1_0R::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bits 24:27 - Defines the CA training device map for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_device_map_0(&self) -> PhyAdrCalvlDeviceMap0R {
        PhyAdrCalvlDeviceMap0R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:23 - CA training RD DQ bit swizzle map 1 for device 1 in address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_swizzle1_1_0(&mut self) -> PhyAdrCalvlSwizzle1_1_0W<DenaliPhy528Spec> {
        PhyAdrCalvlSwizzle1_1_0W::new(self, 0)
    }
    #[doc = "Bits 24:27 - Defines the CA training device map for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_device_map_0(&mut self) -> PhyAdrCalvlDeviceMap0W<DenaliPhy528Spec> {
        PhyAdrCalvlDeviceMap0W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_528::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_528::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy528Spec;
impl crate::RegisterSpec for DenaliPhy528Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_528::R`](R) reader structure"]
impl crate::Readable for DenaliPhy528Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_528::W`](W) writer structure"]
impl crate::Writable for DenaliPhy528Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_528 to value 0"]
impl crate::Resettable for DenaliPhy528Spec {
    const RESET_VALUE: u32 = 0;
}
