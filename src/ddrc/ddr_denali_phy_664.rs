#[doc = "Register `DDR_DENALI_PHY_664` reader"]
pub type R = crate::R<DdrDenaliPhy664Spec>;
#[doc = "Register `DDR_DENALI_PHY_664` writer"]
pub type W = crate::W<DdrDenaliPhy664Spec>;
#[doc = "Field `PHY_ADR_CALVL_BG_1_1` reader - CA training background pattern 1 for address slice 1."]
pub type PhyAdrCalvlBg1_1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_BG_1_1` writer - CA training background pattern 1 for address slice 1."]
pub type PhyAdrCalvlBg1_1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training background pattern 1 for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_bg_1_1(&self) -> PhyAdrCalvlBg1_1R {
        PhyAdrCalvlBg1_1R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training background pattern 1 for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_bg_1_1(&mut self) -> PhyAdrCalvlBg1_1W<DdrDenaliPhy664Spec> {
        PhyAdrCalvlBg1_1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_664::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_664::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy664Spec;
impl crate::RegisterSpec for DdrDenaliPhy664Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_664::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy664Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_664::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy664Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_664 to value 0"]
impl crate::Resettable for DdrDenaliPhy664Spec {
    const RESET_VALUE: u32 = 0;
}
