#[doc = "Register `DDR_DENALI_PHY_796` reader"]
pub type R = crate::R<DdrDenaliPhy796Spec>;
#[doc = "Register `DDR_DENALI_PHY_796` writer"]
pub type W = crate::W<DdrDenaliPhy796Spec>;
#[doc = "Field `PHY_ADR_CALVL_BG_3_2` reader - CA training background pattern 3 for address slice 2."]
pub type PhyAdrCalvlBg3_2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_BG_3_2` writer - CA training background pattern 3 for address slice 2."]
pub type PhyAdrCalvlBg3_2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training background pattern 3 for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_bg_3_2(&self) -> PhyAdrCalvlBg3_2R {
        PhyAdrCalvlBg3_2R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training background pattern 3 for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_bg_3_2(&mut self) -> PhyAdrCalvlBg3_2W<DdrDenaliPhy796Spec> {
        PhyAdrCalvlBg3_2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_796::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_796::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy796Spec;
impl crate::RegisterSpec for DdrDenaliPhy796Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_796::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy796Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_796::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy796Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_796 to value 0"]
impl crate::Resettable for DdrDenaliPhy796Spec {
    const RESET_VALUE: u32 = 0;
}
