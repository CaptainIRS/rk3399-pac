#[doc = "Register `DENALI_PHY_451` reader"]
pub type R = crate::R<DenaliPhy451Spec>;
#[doc = "Register `DENALI_PHY_451` writer"]
pub type W = crate::W<DenaliPhy451Spec>;
#[doc = "Field `PHY_RDDQ6_SLAVE_DELAY_3` reader - Read DQ6 slave delay setting for slice 3."]
pub type PhyRddq6SlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ6_SLAVE_DELAY_3` writer - Read DQ6 slave delay setting for slice 3."]
pub type PhyRddq6SlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `PHY_RDDQ7_SLAVE_DELAY_3` reader - Read DQ7 slave delay setting for slice 3."]
pub type PhyRddq7SlaveDelay3R = crate::FieldReader<u16>;
#[doc = "Field `PHY_RDDQ7_SLAVE_DELAY_3` writer - Read DQ7 slave delay setting for slice 3."]
pub type PhyRddq7SlaveDelay3W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - Read DQ6 slave delay setting for slice 3."]
    #[inline(always)]
    pub fn phy_rddq6_slave_delay_3(&self) -> PhyRddq6SlaveDelay3R {
        PhyRddq6SlaveDelay3R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - Read DQ7 slave delay setting for slice 3."]
    #[inline(always)]
    pub fn phy_rddq7_slave_delay_3(&self) -> PhyRddq7SlaveDelay3R {
        PhyRddq7SlaveDelay3R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Read DQ6 slave delay setting for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq6_slave_delay_3(&mut self) -> PhyRddq6SlaveDelay3W<DenaliPhy451Spec> {
        PhyRddq6SlaveDelay3W::new(self, 0)
    }
    #[doc = "Bits 16:25 - Read DQ7 slave delay setting for slice 3."]
    #[inline(always)]
    #[must_use]
    pub fn phy_rddq7_slave_delay_3(&mut self) -> PhyRddq7SlaveDelay3W<DenaliPhy451Spec> {
        PhyRddq7SlaveDelay3W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`denali_phy_451::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`denali_phy_451::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DenaliPhy451Spec;
impl crate::RegisterSpec for DenaliPhy451Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`denali_phy_451::R`](R) reader structure"]
impl crate::Readable for DenaliPhy451Spec {}
#[doc = "`write(|w| ..)` method takes [`denali_phy_451::W`](W) writer structure"]
impl crate::Writable for DenaliPhy451Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DENALI_PHY_451 to value 0"]
impl crate::Resettable for DenaliPhy451Spec {
    const RESET_VALUE: u32 = 0;
}
