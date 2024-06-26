#[doc = "Register `DENALI_PHY_539` reader"]
pub type R = crate::R<DenaliPhy539Spec>;
#[doc = "Register `DENALI_PHY_539` writer"]
pub type W = crate::W<DenaliPhy539Spec>;
#[doc = "Field `PHY_ADR_CALVL_FG_3_0` reader - CA training foreground pattern 3 for address slice 0."]
pub type PhyAdrCalvlFg3_0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_FG_3_0` writer - CA training foreground pattern 3 for address slice 0."]
pub type PhyAdrCalvlFg3_0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training foreground pattern 3 for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_fg_3_0(&self) -> PhyAdrCalvlFg3_0R {
        PhyAdrCalvlFg3_0R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training foreground pattern 3 for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_fg_3_0(&mut self) -> PhyAdrCalvlFg3_0W<DenaliPhy539Spec> {
        PhyAdrCalvlFg3_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_539::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_539::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy539Spec;
impl crate::RegisterSpec for DenaliPhy539Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_539::R`](R) reader structure"]
impl crate::Readable for DenaliPhy539Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_539::W`](W) writer structure"]
impl crate::Writable for DenaliPhy539Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_539 to value 0"]
impl crate::Resettable for DenaliPhy539Spec {
    const RESET_VALUE: u32 = 0;
}
