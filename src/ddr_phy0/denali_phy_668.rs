#[doc = "Register `DENALI_PHY_668` reader"]
pub type R = crate::R<DenaliPhy668Spec>;
#[doc = "Register `DENALI_PHY_668` writer"]
pub type W = crate::W<DenaliPhy668Spec>;
#[doc = "Field `PHY_ADR_CALVL_BG_3_1` reader - CA training background pattern 3 for address slice 1."]
pub type PhyAdrCalvlBg3_1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_BG_3_1` writer - CA training background pattern 3 for address slice 1."]
pub type PhyAdrCalvlBg3_1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training background pattern 3 for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_bg_3_1(&self) -> PhyAdrCalvlBg3_1R {
        PhyAdrCalvlBg3_1R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training background pattern 3 for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_bg_3_1(&mut self) -> PhyAdrCalvlBg3_1W<DenaliPhy668Spec> {
        PhyAdrCalvlBg3_1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_668::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_668::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy668Spec;
impl crate::RegisterSpec for DenaliPhy668Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_668::R`](R) reader structure"]
impl crate::Readable for DenaliPhy668Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_668::W`](W) writer structure"]
impl crate::Writable for DenaliPhy668Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_668 to value 0"]
impl crate::Resettable for DenaliPhy668Spec {
    const RESET_VALUE: u32 = 0;
}
