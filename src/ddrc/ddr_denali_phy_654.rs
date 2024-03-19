#[doc = "Register `DDR_DENALI_PHY_654` reader"]
pub type R = crate::R<DdrDenaliPhy654Spec>;
#[doc = "Register `DDR_DENALI_PHY_654` writer"]
pub type W = crate::W<DdrDenaliPhy654Spec>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE1_0_1` reader - CA training RD DQ bit swizzle map 1 for device 0 in address slice 1."]
pub type PhyAdrCalvlSwizzle1_0_1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE1_0_1` writer - CA training RD DQ bit swizzle map 1 for device 0 in address slice 1."]
pub type PhyAdrCalvlSwizzle1_0_1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - CA training RD DQ bit swizzle map 1 for device 0 in address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_swizzle1_0_1(&self) -> PhyAdrCalvlSwizzle1_0_1R {
        PhyAdrCalvlSwizzle1_0_1R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - CA training RD DQ bit swizzle map 1 for device 0 in address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_swizzle1_0_1(&mut self) -> PhyAdrCalvlSwizzle1_0_1W<DdrDenaliPhy654Spec> {
        PhyAdrCalvlSwizzle1_0_1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_654::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_654::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy654Spec;
impl crate::RegisterSpec for DdrDenaliPhy654Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_654::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy654Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_654::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy654Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_654 to value 0"]
impl crate::Resettable for DdrDenaliPhy654Spec {
    const RESET_VALUE: u32 = 0;
}
