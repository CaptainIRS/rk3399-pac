#[doc = "Register `DENALI_PHY_665` reader"]
pub type R = crate::R<DenaliPhy665Spec>;
#[doc = "Register `DENALI_PHY_665` writer"]
pub type W = crate::W<DenaliPhy665Spec>;
#[doc = "Field `PHY_ADR_CALVL_FG_2_1` reader - CA training foreground pattern 2 for address slice 1."]
pub type PhyAdrCalvlFg2_1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_FG_2_1` writer - CA training foreground pattern 2 for address slice 1."]
pub type PhyAdrCalvlFg2_1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training foreground pattern 2 for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_fg_2_1(&self) -> PhyAdrCalvlFg2_1R {
        PhyAdrCalvlFg2_1R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training foreground pattern 2 for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_fg_2_1(&mut self) -> PhyAdrCalvlFg2_1W<DenaliPhy665Spec> {
        PhyAdrCalvlFg2_1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_665::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_665::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy665Spec;
impl crate::RegisterSpec for DenaliPhy665Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_665::R`](R) reader structure"]
impl crate::Readable for DenaliPhy665Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_665::W`](W) writer structure"]
impl crate::Writable for DenaliPhy665Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_665 to value 0"]
impl crate::Resettable for DenaliPhy665Spec {
    const RESET_VALUE: u32 = 0;
}
