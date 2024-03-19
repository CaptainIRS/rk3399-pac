#[doc = "Register `DDR_DENALI_PHY_666` reader"]
pub type R = crate::R<DdrDenaliPhy666Spec>;
#[doc = "Register `DDR_DENALI_PHY_666` writer"]
pub type W = crate::W<DdrDenaliPhy666Spec>;
#[doc = "Field `PHY_ADR_CALVL_BG_2_1` reader - CA training background pattern 2 for address slice 1."]
pub type PhyAdrCalvlBg2_1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_BG_2_1` writer - CA training background pattern 2 for address slice 1."]
pub type PhyAdrCalvlBg2_1W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training background pattern 2 for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_bg_2_1(&self) -> PhyAdrCalvlBg2_1R {
        PhyAdrCalvlBg2_1R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training background pattern 2 for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_bg_2_1(&mut self) -> PhyAdrCalvlBg2_1W<DdrDenaliPhy666Spec> {
        PhyAdrCalvlBg2_1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_666::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_666::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy666Spec;
impl crate::RegisterSpec for DdrDenaliPhy666Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_666::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy666Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_666::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy666Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_666 to value 0"]
impl crate::Resettable for DdrDenaliPhy666Spec {
    const RESET_VALUE: u32 = 0;
}
