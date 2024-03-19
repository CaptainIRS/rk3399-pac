#[doc = "Register `DDR_DENALI_PHY_534` reader"]
pub type R = crate::R<DdrDenaliPhy534Spec>;
#[doc = "Register `DDR_DENALI_PHY_534` writer"]
pub type W = crate::W<DdrDenaliPhy534Spec>;
#[doc = "Field `PHY_ADR_CALVL_BG_0_0` reader - CA training background pattern 0 for address slice 0."]
pub type PhyAdrCalvlBg0_0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_BG_0_0` writer - CA training background pattern 0 for address slice 0."]
pub type PhyAdrCalvlBg0_0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training background pattern 0 for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_bg_0_0(&self) -> PhyAdrCalvlBg0_0R {
        PhyAdrCalvlBg0_0R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training background pattern 0 for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_bg_0_0(&mut self) -> PhyAdrCalvlBg0_0W<DdrDenaliPhy534Spec> {
        PhyAdrCalvlBg0_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_534::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_534::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy534Spec;
impl crate::RegisterSpec for DdrDenaliPhy534Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_534::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy534Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_534::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy534Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_534 to value 0"]
impl crate::Resettable for DdrDenaliPhy534Spec {
    const RESET_VALUE: u32 = 0;
}
