#[doc = "Register `DDR_DENALI_PHY_525` reader"]
pub type R = crate::R<DdrDenaliPhy525Spec>;
#[doc = "Register `DDR_DENALI_PHY_525` writer"]
pub type W = crate::W<DdrDenaliPhy525Spec>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE0_0_0` reader - CA training RD DQ bit swizzle map 0 for device 0 in address slice 0."]
pub type PhyAdrCalvlSwizzle0_0_0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_SWIZZLE0_0_0` writer - CA training RD DQ bit swizzle map 0 for device 0 in address slice 0."]
pub type PhyAdrCalvlSwizzle0_0_0W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:23 - CA training RD DQ bit swizzle map 0 for device 0 in address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_swizzle0_0_0(&self) -> PhyAdrCalvlSwizzle0_0_0R {
        PhyAdrCalvlSwizzle0_0_0R::new(self.bits & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:23 - CA training RD DQ bit swizzle map 0 for device 0 in address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_swizzle0_0_0(&mut self) -> PhyAdrCalvlSwizzle0_0_0W<DdrDenaliPhy525Spec> {
        PhyAdrCalvlSwizzle0_0_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_525::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_525::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy525Spec;
impl crate::RegisterSpec for DdrDenaliPhy525Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_525::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy525Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_525::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy525Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_525 to value 0"]
impl crate::Resettable for DdrDenaliPhy525Spec {
    const RESET_VALUE: u32 = 0;
}
