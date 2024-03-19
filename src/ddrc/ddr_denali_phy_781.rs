#[doc = "Register `DDR_DENALI_PHY_781` reader"]
pub type R = crate::R<DdrDenaliPhy781Spec>;
#[doc = "Register `DDR_DENALI_PHY_781` writer"]
pub type W = crate::W<DdrDenaliPhy781Spec>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE0_0_2` reader - CA training RD DQ bit swizzle map 0 for device 0 in address slice 2."]
pub type PhyAdrCalvlSwizzle0_0_2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE0_0_2` writer - CA training RD DQ bit swizzle map 0 for device 0 in address slice 2."]
pub type PhyAdrCalvlSwizzle0_0_2W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - CA training RD DQ bit swizzle map 0 for device 0 in address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_swizzle0_0_2(&self) -> PhyAdrCalvlSwizzle0_0_2R {
        PhyAdrCalvlSwizzle0_0_2R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - CA training RD DQ bit swizzle map 0 for device 0 in address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_swizzle0_0_2(&mut self) -> PhyAdrCalvlSwizzle0_0_2W<DdrDenaliPhy781Spec> {
        PhyAdrCalvlSwizzle0_0_2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_781::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_781::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy781Spec;
impl crate::RegisterSpec for DdrDenaliPhy781Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_781::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy781Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_781::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy781Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_781 to value 0"]
impl crate::Resettable for DdrDenaliPhy781Spec {
    const RESET_VALUE: u32 = 0;
}
