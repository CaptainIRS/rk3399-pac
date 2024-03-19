#[doc = "Register `DDR_DENALI_PHY_793` reader"]
pub type R = crate::R<DdrDenaliPhy793Spec>;
#[doc = "Register `DDR_DENALI_PHY_793` writer"]
pub type W = crate::W<DdrDenaliPhy793Spec>;
#[doc = "Field `PHY_ADR_CALVL_FG_2_2` reader - CA training foreground pattern 2 for address slice 2."]
pub type PhyAdrCalvlFg2_2R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_FG_2_2` writer - CA training foreground pattern 2 for address slice 2."]
pub type PhyAdrCalvlFg2_2W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training foreground pattern 2 for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_fg_2_2(&self) -> PhyAdrCalvlFg2_2R {
        PhyAdrCalvlFg2_2R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training foreground pattern 2 for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_fg_2_2(&mut self) -> PhyAdrCalvlFg2_2W<DdrDenaliPhy793Spec> {
        PhyAdrCalvlFg2_2W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_793::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_793::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy793Spec;
impl crate::RegisterSpec for DdrDenaliPhy793Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_793::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy793Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_793::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy793Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_793 to value 0"]
impl crate::Resettable for DdrDenaliPhy793Spec {
    const RESET_VALUE: u32 = 0;
}
