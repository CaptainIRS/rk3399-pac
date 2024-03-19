#[doc = "Register `DDR_DENALI_PHY_779` reader"]
pub type R = crate::R<DdrDenaliPhy779Spec>;
#[doc = "Register `DDR_DENALI_PHY_779` writer"]
pub type W = crate::W<DdrDenaliPhy779Spec>;
#[doc = "Field `PHY_ADR_CALVL_START_2` reader - CA training DLL start value for address slice 2."]
pub type PhyAdrCalvlStart2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CALVL_START_2` writer - CA training DLL start value for address slice 2."]
pub type PhyAdrCalvlStart2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR_CALVL_COARSE_DLY_2` reader - Coarse CA training DLL increment value for address slice 2."]
pub type PhyAdrCalvlCoarseDly2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CALVL_COARSE_DLY_2` writer - Coarse CA training DLL increment value for address slice 2."]
pub type PhyAdrCalvlCoarseDly2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - CA training DLL start value for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_start_2(&self) -> PhyAdrCalvlStart2R {
        PhyAdrCalvlStart2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Coarse CA training DLL increment value for address slice 2."]
    #[inline(always)]
    pub fn phy_adr_calvl_coarse_dly_2(&self) -> PhyAdrCalvlCoarseDly2R {
        PhyAdrCalvlCoarseDly2R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - CA training DLL start value for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_start_2(&mut self) -> PhyAdrCalvlStart2W<DdrDenaliPhy779Spec> {
        PhyAdrCalvlStart2W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Coarse CA training DLL increment value for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_coarse_dly_2(&mut self) -> PhyAdrCalvlCoarseDly2W<DdrDenaliPhy779Spec> {
        PhyAdrCalvlCoarseDly2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_779::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_779::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy779Spec;
impl crate::RegisterSpec for DdrDenaliPhy779Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_779::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy779Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_779::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy779Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_779 to value 0"]
impl crate::Resettable for DdrDenaliPhy779Spec {
    const RESET_VALUE: u32 = 0;
}
