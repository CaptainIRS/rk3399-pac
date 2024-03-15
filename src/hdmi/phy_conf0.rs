#[doc = "Register `PHY_CONF0` reader"]
pub type R = crate::R<PhyConf0Spec>;
#[doc = "Register `PHY_CONF0` writer"]
pub type W = crate::W<PhyConf0Spec>;
#[doc = "Field `SELDIPIF` reader - Select interface control."]
pub type SeldipifR = crate::BitReader;
#[doc = "Field `SELDIPIF` writer - Select interface control."]
pub type SeldipifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SELDATAENPOL` reader - Select data enable polarity."]
pub type SeldataenpolR = crate::BitReader;
#[doc = "Field `SELDATAENPOL` writer - Select data enable polarity."]
pub type SeldataenpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENHPDRXSENSE` reader - PHY ENHPDRXSENSE signal. Otherwise, this field is a \"spare\" bit with no associated functionality."]
pub type EnhpdrxsenseR = crate::BitReader;
#[doc = "Field `ENHPDRXSENSE` writer - PHY ENHPDRXSENSE signal. Otherwise, this field is a \"spare\" bit with no associated functionality."]
pub type EnhpdrxsenseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPWRON` reader - PHY TXPWRON signal. Otherwise, this field is a \"spare\" bit with no associated functionality."]
pub type TxpwronR = crate::BitReader;
#[doc = "Field `TXPWRON` writer - PHY TXPWRON signal. Otherwise, this field is a \"spare\" bit with no associated functionality."]
pub type TxpwronW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDDQ` reader - PHY PDDQ signal. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (PHY_MHL_COMBO== 1) ? 1 :"]
pub type PddqR = crate::BitReader;
#[doc = "Field `PDDQ` writer - PHY PDDQ signal. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (PHY_MHL_COMBO== 1) ? 1 :"]
pub type PddqW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SVSRET` reader - PHY SVSRET signal. Otherwise, this field is a \"spare\" bit with no associated functionality."]
pub type SvsretR = crate::BitReader;
#[doc = "Field `SVSRET` writer - PHY SVSRET signal. Otherwise, this field is a \"spare\" bit with no associated functionality."]
pub type SvsretW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENTMDS` reader - Enable TMDS drivers, bias, and TMDS digital logic. Otherwise, this field is a \"spare\" bit with no associated functionality."]
pub type EntmdsR = crate::BitReader;
#[doc = "Field `ENTMDS` writer - Enable TMDS drivers, bias, and TMDS digital logic. Otherwise, this field is a \"spare\" bit with no associated functionality."]
pub type EntmdsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDZ` reader - Power-down enable (active low 0b). Otherwise, this field is a \"spare\" bit with no associated functionality."]
pub type PdzR = crate::BitReader;
#[doc = "Field `PDZ` writer - Power-down enable (active low 0b). Otherwise, this field is a \"spare\" bit with no associated functionality."]
pub type PdzW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Select interface control."]
    #[inline(always)]
    pub fn seldipif(&self) -> SeldipifR {
        SeldipifR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Select data enable polarity."]
    #[inline(always)]
    pub fn seldataenpol(&self) -> SeldataenpolR {
        SeldataenpolR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PHY ENHPDRXSENSE signal. Otherwise, this field is a \"spare\" bit with no associated functionality."]
    #[inline(always)]
    pub fn enhpdrxsense(&self) -> EnhpdrxsenseR {
        EnhpdrxsenseR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PHY TXPWRON signal. Otherwise, this field is a \"spare\" bit with no associated functionality."]
    #[inline(always)]
    pub fn txpwron(&self) -> TxpwronR {
        TxpwronR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PHY PDDQ signal. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (PHY_MHL_COMBO== 1) ? 1 :"]
    #[inline(always)]
    pub fn pddq(&self) -> PddqR {
        PddqR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PHY SVSRET signal. Otherwise, this field is a \"spare\" bit with no associated functionality."]
    #[inline(always)]
    pub fn svsret(&self) -> SvsretR {
        SvsretR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable TMDS drivers, bias, and TMDS digital logic. Otherwise, this field is a \"spare\" bit with no associated functionality."]
    #[inline(always)]
    pub fn entmds(&self) -> EntmdsR {
        EntmdsR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Power-down enable (active low 0b). Otherwise, this field is a \"spare\" bit with no associated functionality."]
    #[inline(always)]
    pub fn pdz(&self) -> PdzR {
        PdzR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Select interface control."]
    #[inline(always)]
    #[must_use]
    pub fn seldipif(&mut self) -> SeldipifW<PhyConf0Spec> {
        SeldipifW::new(self, 0)
    }
    #[doc = "Bit 1 - Select data enable polarity."]
    #[inline(always)]
    #[must_use]
    pub fn seldataenpol(&mut self) -> SeldataenpolW<PhyConf0Spec> {
        SeldataenpolW::new(self, 1)
    }
    #[doc = "Bit 2 - PHY ENHPDRXSENSE signal. Otherwise, this field is a \"spare\" bit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn enhpdrxsense(&mut self) -> EnhpdrxsenseW<PhyConf0Spec> {
        EnhpdrxsenseW::new(self, 2)
    }
    #[doc = "Bit 3 - PHY TXPWRON signal. Otherwise, this field is a \"spare\" bit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn txpwron(&mut self) -> TxpwronW<PhyConf0Spec> {
        TxpwronW::new(self, 3)
    }
    #[doc = "Bit 4 - PHY PDDQ signal. Otherwise, this field is a \"spare\" bit with no associated functionality. Reset Value: (PHY_MHL_COMBO== 1) ? 1 :"]
    #[inline(always)]
    #[must_use]
    pub fn pddq(&mut self) -> PddqW<PhyConf0Spec> {
        PddqW::new(self, 4)
    }
    #[doc = "Bit 5 - PHY SVSRET signal. Otherwise, this field is a \"spare\" bit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn svsret(&mut self) -> SvsretW<PhyConf0Spec> {
        SvsretW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable TMDS drivers, bias, and TMDS digital logic. Otherwise, this field is a \"spare\" bit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn entmds(&mut self) -> EntmdsW<PhyConf0Spec> {
        EntmdsW::new(self, 6)
    }
    #[doc = "Bit 7 - Power-down enable (active low 0b). Otherwise, this field is a \"spare\" bit with no associated functionality."]
    #[inline(always)]
    #[must_use]
    pub fn pdz(&mut self) -> PdzW<PhyConf0Spec> {
        PdzW::new(self, 7)
    }
}
#[doc = "Select interface control.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`phy_conf0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`phy_conf0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PhyConf0Spec;
impl crate::RegisterSpec for PhyConf0Spec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`phy_conf0::R`](R) reader structure"]
impl crate::Readable for PhyConf0Spec {}
#[doc = "`write(|w| ..)` method takes [`phy_conf0::W`](W) writer structure"]
impl crate::Writable for PhyConf0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u8 = 0;
}
#[doc = "`reset()` method sets PHY_CONF0 to value 0x06"]
impl crate::Resettable for PhyConf0Spec {
    const RESET_VALUE: u8 = 0x06;
}
