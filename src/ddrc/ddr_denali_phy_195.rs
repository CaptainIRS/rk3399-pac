#[doc = "Register `DDR_DENALI_PHY_195` reader"]
pub type R = crate::R<DdrDenaliPhy195Spec>;
#[doc = "Register `DDR_DENALI_PHY_195` writer"]
pub type W = crate::W<DdrDenaliPhy195Spec>;
#[doc = "Field `PHY_RDDQ6_SLAVE_DELAY_1` reader - Read DQ6 slave delay setting for slice 1."]
pub type PhyRddq6SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ6_SLAVE_DELAY_1` writer - Read DQ6 slave delay setting for slice 1."]
pub type PhyRddq6SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQ7_SLAVE_DELAY_1` reader - Read DQ7 slave delay setting for slice 1."]
pub type PhyRddq7SlaveDelay1R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ7_SLAVE_DELAY_1` writer - Read DQ7 slave delay setting for slice 1."]
pub type PhyRddq7SlaveDelay1W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DQ6 slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_rddq6_slave_delay_1(&self) -> PhyRddq6SlaveDelay1R {
        PhyRddq6SlaveDelay1R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQ7 slave delay setting for slice 1."]
    #[inline(always)]
    pub fn phy_rddq7_slave_delay_1(&self) -> PhyRddq7SlaveDelay1R {
        PhyRddq7SlaveDelay1R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQ6 slave delay setting for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq6_slave_delay_1(&mut self) -> PhyRddq6SlaveDelay1W<DdrDenaliPhy195Spec> {
        PhyRddq6SlaveDelay1W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQ7 slave delay setting for slice 1."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq7_slave_delay_1(&mut self) -> PhyRddq7SlaveDelay1W<DdrDenaliPhy195Spec> {
        PhyRddq7SlaveDelay1W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_195::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_195::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy195Spec;
impl crate::RegisterSpec for DdrDenaliPhy195Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_195::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy195Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_195::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy195Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_195 to value 0"]
impl crate::Resettable for DdrDenaliPhy195Spec {
    const RESET_VALUE: u32 = 0;
}
