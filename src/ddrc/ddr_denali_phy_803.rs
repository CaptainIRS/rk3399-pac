#[doc = "Register `DDR_DENALI_PHY_803` reader"]
pub type R = crate::R<DdrDenaliPhy803Spec>;
#[doc = "Register `DDR_DENALI_PHY_803` writer"]
pub type W = crate::W<DdrDenaliPhy803Spec>;
#[doc = "Field `PHY_ADR5_CLK_WR_SLAVE_DELAY_2` reader - Address slice slave delay setting for address slice 2."]
pub type PhyAdr5ClkWrSlaveDelay2R = crate::FieldReader<u16>;
#[doc = "Field `PHY_ADR5_CLK_WR_SLAVE_DELAY_2` writer - Address slice slave delay setting for address slice 2."]
pub type PhyAdr5ClkWrSlaveDelay2W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `PHY_ADR_SW_MASTER_MODE_2` reader - Master delay line override settings for address slice 2. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
pub type PhyAdrSwMasterMode2R = crate::FieldReader;
#[doc = "Field `PHY_ADR_SW_MASTER_MODE_2` writer - Master delay line override settings for address slice 2. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
pub type PhyAdrSwMasterMode2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:10 - Address slice slave delay setting for address slice 2."]
    #[inline(always)]
    pub fn phy_adr5_clk_wr_slave_delay_2(&self) -> PhyAdr5ClkWrSlaveDelay2R {
        PhyAdr5ClkWrSlaveDelay2R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:19 - Master delay line override settings for address slice 2. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
    #[inline(always)]
    pub fn phy_adr_sw_master_mode_2(&self) -> PhyAdrSwMasterMode2R {
        PhyAdrSwMasterMode2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:10 - Address slice slave delay setting for address slice 2."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr5_clk_wr_slave_delay_2(
        &mut self,
    ) -> PhyAdr5ClkWrSlaveDelay2W<DdrDenaliPhy803Spec> {
        PhyAdr5ClkWrSlaveDelay2W::new(self, 0)
    }
    #[doc = "Bits 16:19 - Master delay line override settings for address slice 2. Bit (0) enables software half clock mode. Bit (1) is the software half clock mode value. Bit (2) enables software bypass mode. Bit (3) is the software bypass mode value."]
    #[inline(always)]
    #[must_use]
    pub fn phy_adr_sw_master_mode_2(&mut self) -> PhyAdrSwMasterMode2W<DdrDenaliPhy803Spec> {
        PhyAdrSwMasterMode2W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ddr_denali_phy_803::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ddr_denali_phy_803::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DdrDenaliPhy803Spec;
impl crate::RegisterSpec for DdrDenaliPhy803Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ddr_denali_phy_803::R`](R) reader structure"]
impl crate::Readable for DdrDenaliPhy803Spec {}
#[doc = "`write(|w| ..)` method takes [`ddr_denali_phy_803::W`](W) writer structure"]
impl crate::Writable for DdrDenaliPhy803Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DDR_DENALI_PHY_803 to value 0"]
impl crate::Resettable for DdrDenaliPhy803Spec {
    const RESET_VALUE: u32 = 0;
}
