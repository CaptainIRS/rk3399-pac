#[doc = "Register `DENALI_PHY_524` reader"]
pub type R = crate::R<DenaliPhy524Spec>;
#[doc = "Register `DENALI_PHY_524` writer"]
pub type W = crate::W<DenaliPhy524Spec>;
#[doc = "Field `PHY_ADR_CALVL_QTR_0` reader - CA training DLL quarter cycle delay value for address slice 0."]
pub type PhyAdrCalvlQtr0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CALVL_QTR_0` writer - CA training DLL quarter cycle delay value for address slice 0."]
pub type PhyAdrCalvlQtr0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - CA training DLL quarter cycle delay value for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_qtr_0(&self) -> PhyAdrCalvlQtr0R {
        PhyAdrCalvlQtr0R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - CA training DLL quarter cycle delay value for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_qtr_0(&mut self) -> PhyAdrCalvlQtr0W<DenaliPhy524Spec> {
        PhyAdrCalvlQtr0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_524::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_524::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy524Spec;
impl crate::RegisterSpec for DenaliPhy524Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_524::R`](R) reader structure"]
impl crate::Readable for DenaliPhy524Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_524::W`](W) writer structure"]
impl crate::Writable for DenaliPhy524Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_524 to value 0"]
impl crate::Resettable for DenaliPhy524Spec {
    const RESET_VALUE: u32 = 0;
}
