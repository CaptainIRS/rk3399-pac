#[doc = "Register `DDR_DENALI_PHY_323` reader"]
pub type R = crate::R<DdrDenaliPhy323Spec>;
#[doc = "Register `DDR_DENALI_PHY_323` writer"]
pub type W = crate::W<DdrDenaliPhy323Spec>;
#[doc = "Field `PHY_RDDQ6_SLAVE_DELAY_2` reader - Read DQ6 slave delay setting for slice 2."]
pub type PhyRddq6SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ6_SLAVE_DELAY_2` writer - Read DQ6 slave delay setting for slice 2."]
pub type PhyRddq6SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQ7_SLAVE_DELAY_2` reader - Read DQ7 slave delay setting for slice 2."]
pub type PhyRddq7SlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ7_SLAVE_DELAY_2` writer - Read DQ7 slave delay setting for slice 2."]
pub type PhyRddq7SlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DQ6 slave delay setting for slice 2."]
    #[inline(always)]
    pub fn phy_rddq6_slave_delay_2(&self) -> PhyRddq6SlaveDelay2R {
        PhyRddq6SlaveDelay2R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQ7 slave delay setting for slice 2."]
    #[inline(always)]
    pub fn phy_rddq7_slave_delay_2(&self) -> PhyRddq7SlaveDelay2R {
        PhyRddq7SlaveDelay2R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQ6 slave delay setting for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq6_slave_delay_2(&mut self) -> PhyRddq6SlaveDelay2W<DdrDenaliPhy323Spec> {
        PhyRddq6SlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQ7 slave delay setting for slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq7_slave_delay_2(&mut self) -> PhyRddq7SlaveDelay2W<DdrDenaliPhy323Spec> {
        PhyRddq7SlaveDelay2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_323::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_323::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy323Spec;
impl crate::RegisterSpec for DdrDenaliPhy323Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_323::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy323Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_323::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy323Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_323 to value 0"]
impl crate::Resettable for DdrDenaliPhy323Spec {
    const RESET_VALUE: u32 = 0;
}
