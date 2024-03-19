#[doc = "Register `DDR_DENALI_PHY_520` reader"]
pub type R = crate::R<DdrDenaliPhy520Spec>;
#[doc = "Register `DDR_DENALI_PHY_520` writer"]
pub type W = crate::W<DdrDenaliPhy520Spec>;
#[doc = "Field `PHY_ADR_DDL_MODE_0` reader - DDL mode for address slice 0."]
pub type PhyAdrDdlMode0R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR_DDL_MODE_0` writer - DDL mode for address slice 0."]
pub type PhyAdrDdlMode0W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - DDL mode for address slice 0."]
    #[inline(always)]
    pub fn phy_adr_ddl_mode_0(&self) -> PhyAdrDdlMode0R {
        PhyAdrDdlMode0R::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - DDL mode for address slice 0."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_ddl_mode_0(&mut self) -> PhyAdrDdlMode0W<DdrDenaliPhy520Spec> {
        PhyAdrDdlMode0W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_520::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_520::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy520Spec;
impl crate::RegisterSpec for DdrDenaliPhy520Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_520::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy520Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_520::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy520Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_520 to value 0"]
impl crate::Resettable for DdrDenaliPhy520Spec {
    const RESET_VALUE: u32 = 0;
}
