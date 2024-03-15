#[doc = "Register `DENALI_PHY_523` reader"]
pub type R = crate::R<DenaliPhy523Spec>;
#[doc = "Register `DENALI_PHY_523` writer"]
pub type W = crate::W<DenaliPhy523Spec>;
#[doc = "Field `PHY_ADR_CALVL_START_0` reader - CA training DLL start value for address slice 0."]
pub type PhyAdrCalvlStart0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CALVL_START_0` writer - CA training DLL start value for address slice 0."]
pub type PhyAdrCalvlStart0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR_CALVL_COARSE_DLY_0` reader - Coarse CA training DLL increment value for address slice 0."]
pub type PhyAdrCalvlCoarseDly0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_CALVL_COARSE_DLY_0` writer - Coarse CA training DLL increment value for address slice 0."]
pub type PhyAdrCalvlCoarseDly0W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - CA training DLL start value for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_start_0(&self) -> PhyAdrCalvlStart0R {
        PhyAdrCalvlStart0R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - Coarse CA training DLL increment value for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_calvl_coarse_dly_0(&self) -> PhyAdrCalvlCoarseDly0R {
        PhyAdrCalvlCoarseDly0R::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - CA training DLL start value for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_start_0(&mut self) -> PhyAdrCalvlStart0W<DenaliPhy523Spec> {
        PhyAdrCalvlStart0W::new(self, 0)
    }
    #[doc = "Bits 16:26 - Coarse CA training DLL increment value for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_calvl_coarse_dly_0(&mut self) -> PhyAdrCalvlCoarseDly0W<DenaliPhy523Spec> {
        PhyAdrCalvlCoarseDly0W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_523::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_523::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy523Spec;
impl crate::RegisterSpec for DenaliPhy523Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_523::R`](R) reader structure"]
impl crate::Readable for DenaliPhy523Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_523::W`](W) writer structure"]
impl crate::Writable for DenaliPhy523Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_523 to value 0"]
impl crate::Resettable for DenaliPhy523Spec {
    const RESET_VALUE: u32 = 0;
}
