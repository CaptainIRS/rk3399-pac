#[doc = "Register `DDR_DENALI_PHY_780` reader"]
pub type R = crate::R<DdrDenaliPhy780Spec>;
#[doc = "Register `DDR_DENALI_PHY_780` writer"]
pub type W = crate::W<DdrDenaliPhy780Spec>;
#[doc = "Field `PHY_ADR_CALVL_QTR_2` reader - CA training DLL quarter cycle delay value for address slice 2."]
pub type PhyAdrCalvlQtr2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CALVL_QTR_2` writer - CA training DLL quarter cycle delay value for address slice 2."]
pub type PhyAdrCalvlQtr2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - CA training DLL quarter cycle delay value for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_qtr_2(&self) -> PhyAdrCalvlQtr2R {
        PhyAdrCalvlQtr2R::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - CA training DLL quarter cycle delay value for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_qtr_2(&mut self) -> PhyAdrCalvlQtr2W<DdrDenaliPhy780Spec> {
        PhyAdrCalvlQtr2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_780::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_780::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy780Spec;
impl crate::RegisterSpec for DdrDenaliPhy780Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_780::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy780Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_780::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy780Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_780 to value 0"]
impl crate::Resettable for DdrDenaliPhy780Spec {
    const RESET_VALUE: u32 = 0;
}
