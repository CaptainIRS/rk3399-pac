#[doc = "Register `DENALI_PHY_791` reader"]
pub type R = crate::R<DenaliPhy791Spec>;
#[doc = "Register `DENALI_PHY_791` writer"]
pub type W = crate::W<DenaliPhy791Spec>;
#[doc = "Field `PHY_ADR_CALVL_FG_1_2` reader - CA training foreground pattern 1 for address slice 2."]
pub type PhyAdrCalvlFg1_2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_FG_1_2` writer - CA training foreground pattern 1 for address slice 2."]
pub type PhyAdrCalvlFg1_2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training foreground pattern 1 for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_fg_1_2(&self) -> PhyAdrCalvlFg1_2R {
        PhyAdrCalvlFg1_2R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training foreground pattern 1 for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_fg_1_2(&mut self) -> PhyAdrCalvlFg1_2W<DenaliPhy791Spec> {
        PhyAdrCalvlFg1_2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_791::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_791::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy791Spec;
impl crate::RegisterSpec for DenaliPhy791Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_791::R`](R) reader structure"]
impl crate::Readable for DenaliPhy791Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_791::W`](W) writer structure"]
impl crate::Writable for DenaliPhy791Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_791 to value 0"]
impl crate::Resettable for DenaliPhy791Spec {
    const RESET_VALUE: u32 = 0;
}
