#[doc = "Register `DDR_DENALI_PHY_538` reader"]
pub type R = crate::R<DdrDenaliPhy538Spec>;
#[doc = "Register `DDR_DENALI_PHY_538` writer"]
pub type W = crate::W<DdrDenaliPhy538Spec>;
#[doc = "Field `PHY_ADR_CALVL_BG_2_0` reader - CA training background pattern 2 for address slice 0."]
pub type PhyAdrCalvlBg2_0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_BG_2_0` writer - CA training background pattern 2 for address slice 0."]
pub type PhyAdrCalvlBg2_0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training background pattern 2 for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_bg_2_0(&self) -> PhyAdrCalvlBg2_0R {
        PhyAdrCalvlBg2_0R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training background pattern 2 for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_bg_2_0(&mut self) -> PhyAdrCalvlBg2_0W<DdrDenaliPhy538Spec> {
        PhyAdrCalvlBg2_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_538::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_538::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy538Spec;
impl crate::RegisterSpec for DdrDenaliPhy538Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_538::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy538Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_538::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy538Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_538 to value 0"]
impl crate::Resettable for DdrDenaliPhy538Spec {
    const RESET_VALUE: u32 = 0;
}
