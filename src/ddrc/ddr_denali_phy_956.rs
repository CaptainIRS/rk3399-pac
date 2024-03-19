#[doc = "Register `DDR_DENALI_PHY_956` reader"]
pub type R = crate::R<DdrDenaliPhy956Spec>;
#[doc = "Register `DDR_DENALI_PHY_956` writer"]
pub type W = crate::W<DdrDenaliPhy956Spec>;
#[doc = "Field `PHY_DDL_AC_ENABLE` reader - PHY Address/Control DDL BIST mode enable."]
pub type PhyDdlAcEnableR = crate::FieldReader<u32>;
#[doc = "Field `PHY_DDL_AC_ENABLE` writer - PHY Address/Control DDL BIST mode enable."]
pub type PhyDdlAcEnableW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - PHY Address/Control DDL BIST mode enable."]
    #[inline(always)]
    pub fn phy_ddl_ac_enable(&self) -> PhyDdlAcEnableR {
        PhyDdlAcEnableR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - PHY Address/Control DDL BIST mode enable."]
    #[inline(always)]
    #[must_use]
    pub fn phy_ddl_ac_enable(&mut self) -> PhyDdlAcEnableW<DdrDenaliPhy956Spec> {
        PhyDdlAcEnableW::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_956::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_956::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy956Spec;
impl crate::RegisterSpec for DdrDenaliPhy956Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_956::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy956Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_956::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy956Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_956 to value 0"]
impl crate::Resettable for DdrDenaliPhy956Spec {
    const RESET_VALUE: u32 = 0;
}
