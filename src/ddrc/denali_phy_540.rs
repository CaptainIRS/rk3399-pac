#[doc = "Register `DENALI_PHY_540` reader"]
pub type R = crate::R<DenaliPhy540Spec>;
#[doc = "Register `DENALI_PHY_540` writer"]
pub type W = crate::W<DenaliPhy540Spec>;
#[doc = "Field `PHY_ADR_CALVL_BG_3_0` reader - CA training background pattern 3 for address slice 0."]
pub type PhyAdrCalvlBg3_0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_BG_3_0` writer - CA training background pattern 3 for address slice 0."]
pub type PhyAdrCalvlBg3_0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training background pattern 3 for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_bg_3_0(&self) -> PhyAdrCalvlBg3_0R {
        PhyAdrCalvlBg3_0R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training background pattern 3 for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_bg_3_0(&mut self) -> PhyAdrCalvlBg3_0W<DenaliPhy540Spec> {
        PhyAdrCalvlBg3_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_540::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_540::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy540Spec;
impl crate::RegisterSpec for DenaliPhy540Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_540::R`](R) reader structure"]
impl crate::Readable for DenaliPhy540Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_540::W`](W) writer structure"]
impl crate::Writable for DenaliPhy540Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_540 to value 0"]
impl crate::Resettable for DenaliPhy540Spec {
    const RESET_VALUE: u32 = 0;
}
