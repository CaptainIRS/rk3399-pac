#[doc = "Register `DENALI_PHY_662` reader"]
pub type R = crate::R<DenaliPhy662Spec>;
#[doc = "Register `DENALI_PHY_662` writer"]
pub type W = crate::W<DenaliPhy662Spec>;
#[doc = "Field `PHY_ADR_CALVL_BG_0_1` reader - CA training background pattern 0 for address slice 1."]
pub type PhyAdrCalvlBg0_1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_BG_0_1` writer - CA training background pattern 0 for address slice 1."]
pub type PhyAdrCalvlBg0_1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training background pattern 0 for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_bg_0_1(&self) -> PhyAdrCalvlBg0_1R {
        PhyAdrCalvlBg0_1R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training background pattern 0 for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_bg_0_1(&mut self) -> PhyAdrCalvlBg0_1W<DenaliPhy662Spec> {
        PhyAdrCalvlBg0_1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_662::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_662::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy662Spec;
impl crate::RegisterSpec for DenaliPhy662Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_662::R`](R) reader structure"]
impl crate::Readable for DenaliPhy662Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_662::W`](W) writer structure"]
impl crate::Writable for DenaliPhy662Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_662 to value 0"]
impl crate::Resettable for DenaliPhy662Spec {
    const RESET_VALUE: u32 = 0;
}
