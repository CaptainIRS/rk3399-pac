#[doc = "Register `DENALI_PHY_653` reader"]
pub type R = crate::R<DenaliPhy653Spec>;
#[doc = "Register `DENALI_PHY_653` writer"]
pub type W = crate::W<DenaliPhy653Spec>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE0_0_1` reader - CA training RD DQ bit swizzle map 0 for device 0 in address slice 1."]
pub type PhyAdrCalvlSwizzle0_0_1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE0_0_1` writer - CA training RD DQ bit swizzle map 0 for device 0 in address slice 1."]
pub type PhyAdrCalvlSwizzle0_0_1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - CA training RD DQ bit swizzle map 0 for device 0 in address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_swizzle0_0_1(&self) -> PhyAdrCalvlSwizzle0_0_1R {
        PhyAdrCalvlSwizzle0_0_1R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - CA training RD DQ bit swizzle map 0 for device 0 in address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_swizzle0_0_1(&mut self) -> PhyAdrCalvlSwizzle0_0_1W<DenaliPhy653Spec> {
        PhyAdrCalvlSwizzle0_0_1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_653::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_653::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy653Spec;
impl crate::RegisterSpec for DenaliPhy653Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_653::R`](R) reader structure"]
impl crate::Readable for DenaliPhy653Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_653::W`](W) writer structure"]
impl crate::Writable for DenaliPhy653Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_653 to value 0"]
impl crate::Resettable for DenaliPhy653Spec {
    const RESET_VALUE: u32 = 0;
}
