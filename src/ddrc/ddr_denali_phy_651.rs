#[doc = "Register `DDR_DENALI_PHY_651` reader"]
pub type R = crate::R<DdrDenaliPhy651Spec>;
#[doc = "Register `DDR_DENALI_PHY_651` writer"]
pub type W = crate::W<DdrDenaliPhy651Spec>;
#[doc = "Field `PHY_ADR_CALVL_START_1` reader - CA training DLL start value for address slice 1."]
pub type PhyAdrCalvlStart1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CALVL_START_1` writer - CA training DLL start value for address slice 1."]
pub type PhyAdrCalvlStart1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR_CALVL_COARSE_DLY_1` reader - Coarse CA training DLL increment value for address slice 1."]
pub type PhyAdrCalvlCoarseDly1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CALVL_COARSE_DLY_1` writer - Coarse CA training DLL increment value for address slice 1."]
pub type PhyAdrCalvlCoarseDly1W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - CA training DLL start value for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_start_1(&self) -> PhyAdrCalvlStart1R {
        PhyAdrCalvlStart1R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Coarse CA training DLL increment value for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_calvl_coarse_dly_1(&self) -> PhyAdrCalvlCoarseDly1R {
        PhyAdrCalvlCoarseDly1R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - CA training DLL start value for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_start_1(&mut self) -> PhyAdrCalvlStart1W<DdrDenaliPhy651Spec> {
        PhyAdrCalvlStart1W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Coarse CA training DLL increment value for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_coarse_dly_1(&mut self) -> PhyAdrCalvlCoarseDly1W<DdrDenaliPhy651Spec> {
        PhyAdrCalvlCoarseDly1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_651::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_651::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy651Spec;
impl crate::RegisterSpec for DdrDenaliPhy651Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_651::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy651Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_651::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy651Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_651 to value 0"]
impl crate::Resettable for DdrDenaliPhy651Spec {
    const RESET_VALUE: u32 = 0;
}
