#[doc = "Register `DDR_DENALI_PHY_536` reader"]
pub type R = crate::R<DdrDenaliPhy536Spec>;
#[doc = "Register `DDR_DENALI_PHY_536` writer"]
pub type W = crate::W<DdrDenaliPhy536Spec>;
#[doc = "Field `PHY_ADR_CALVL_BG_1_0` reader - CA training background pattern 1 for address slice 0."]
pub type PhyAdrCalvlBg1_0R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_CALVL_BG_1_0` writer - CA training background pattern 1 for address slice 0."]
pub type PhyAdrCalvlBg1_0W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
impl R {
    #[doc = "Bits 0:19 - CA training background pattern 1 for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_bg_1_0(&self) -> PhyAdrCalvlBg1_0R {
        PhyAdrCalvlBg1_0R::new(self.bits & 0x000f_ffff)
    }
}
impl W {
    #[doc = "Bits 0:19 - CA training background pattern 1 for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_bg_1_0(&mut self) -> PhyAdrCalvlBg1_0W<DdrDenaliPhy536Spec> {
        PhyAdrCalvlBg1_0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_536::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_536::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy536Spec;
impl crate::RegisterSpec for DdrDenaliPhy536Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_536::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy536Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_536::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy536Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_536 to value 0"]
impl crate::Resettable for DdrDenaliPhy536Spec {
    const RESET_VALUE: u32 = 0;
}
