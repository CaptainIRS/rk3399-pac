#[doc = "Register `DENALI_PHY_526` reader"]
pub type R = crate::R<DenaliPhy526Spec>;
#[doc = "Register `DENALI_PHY_526` writer"]
pub type W = crate::W<DenaliPhy526Spec>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE1_0_0` reader - CA training RD DQ bit swizzle map 1 for device 0 in address slice 0."]
pub type PhyAdrCalvlSwizzle1_0_0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE1_0_0` writer - CA training RD DQ bit swizzle map 1 for device 0 in address slice 0."]
pub type PhyAdrCalvlSwizzle1_0_0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - CA training RD DQ bit swizzle map 1 for device 0 in address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_swizzle1_0_0(&self) -> PhyAdrCalvlSwizzle1_0_0R {
        PhyAdrCalvlSwizzle1_0_0R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - CA training RD DQ bit swizzle map 1 for device 0 in address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_swizzle1_0_0(&mut self) -> PhyAdrCalvlSwizzle1_0_0W<DenaliPhy526Spec> {
        PhyAdrCalvlSwizzle1_0_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_526::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_526::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy526Spec;
impl crate::RegisterSpec for DenaliPhy526Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_526::R`](R) reader structure"]
impl crate::Readable for DenaliPhy526Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_526::W`](W) writer structure"]
impl crate::Writable for DenaliPhy526Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_526 to value 0"]
impl crate::Resettable for DenaliPhy526Spec {
    const RESET_VALUE: u32 = 0;
}
