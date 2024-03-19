#[doc = "Register `DDR_DENALI_PHY_669` reader"]
pub type R = crate::R<DdrDenaliPhy669Spec>;
#[doc = "Register `DDR_DENALI_PHY_669` writer"]
pub type W = crate::W<DdrDenaliPhy669Spec>;
#[doc = "Field `PHY_ADR_ADDR_SEL_1` reader - Mux select to map in LPDDR4 addressing for address slice 1."]
pub type PhyAdrAddrSel1R = crate::FieldReader<u32>;
#[doc = "Field `PHY_ADR_ADDR_SEL_1` writer - Mux select to map in LPDDR4 addressing for address slice 1."]
pub type PhyAdrAddrSel1W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bits 0:29 - Mux select to map in LPDDR4 addressing for address slice 1."]
    #[inline(always)]
    pub fn phy_adr_addr_sel_1(&self) -> PhyAdrAddrSel1R {
        PhyAdrAddrSel1R::new(self.bits & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:29 - Mux select to map in LPDDR4 addressing for address slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_addr_sel_1(&mut self) -> PhyAdrAddrSel1W<DdrDenaliPhy669Spec> {
        PhyAdrAddrSel1W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_669::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_669::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy669Spec;
impl crate::RegisterSpec for DdrDenaliPhy669Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_669::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy669Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_669::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy669Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_669 to value 0"]
impl crate::Resettable for DdrDenaliPhy669Spec {
    const RESET_VALUE: u32 = 0;
}
