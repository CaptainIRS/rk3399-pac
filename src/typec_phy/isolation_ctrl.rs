#[doc = "Register `ISOLATION_CTRL` reader"]
pub type R = crate::R<IsolationCtrlSpec>;
#[doc = "Register `ISOLATION_CTRL` writer"]
pub type W = crate::W<IsolationCtrlSpec>;
#[doc = "Field `FIELD5` reader - PHY/PMA lane isolation enable (ln_isolation_en) - When in PHY Macro \n\nIsolation Mode, the selected PHY \n\nlane(s) \n\nisolation registers are \n\nselected. When in PMA Isolation Mode, the selected PMA lane(s) \n\nisolation registers are selected."]
pub type Field5R = crate::FieldReader;
#[doc = "Field `FIELD5` writer - PHY/PMA lane isolation enable (ln_isolation_en) - When in PHY Macro \n\nIsolation Mode, the selected PHY \n\nlane(s) \n\nisolation registers are \n\nselected. When in PMA Isolation Mode, the selected PMA lane(s) \n\nisolation registers are selected."]
pub type Field5W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `FIELD4` reader - Reserved"]
pub type Field4R = crate::FieldReader;
#[doc = "Field `FIELD3` reader - PHY/PMA \n\nisolation mode \n\nselect \n\n(isolation_mode_sel) \n\n- When \n\nisolation_en is set, this bit selects between PHY isolation and PMA \n\nisolation mode. 0 = PHY isolation mode; 1 = PMA isolation mode."]
pub type Field3R = crate::BitReader;
#[doc = "Field `FIELD3` writer - PHY/PMA \n\nisolation mode \n\nselect \n\n(isolation_mode_sel) \n\n- When \n\nisolation_en is set, this bit selects between PHY isolation and PMA \n\nisolation mode. 0 = PHY isolation mode; 1 = PMA isolation mode."]
pub type Field3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD2` reader - Reserved"]
pub type Field2R = crate::BitReader;
#[doc = "Field `FIELD1` reader - PHY/PMA common isolation enable (cmn_isolation_en) - When in PHY \n\nMacro Isolation Mode, the PHY common isolation register(s) are \n\nselected. When in PMA Isolation Mode, the PMA common isolation \n\nreg- ister(s) are selected."]
pub type Field1R = crate::BitReader;
#[doc = "Field `FIELD1` writer - PHY/PMA common isolation enable (cmn_isolation_en) - When in PHY \n\nMacro Isolation Mode, the PHY common isolation register(s) are \n\nselected. When in PMA Isolation Mode, the PMA common isolation \n\nreg- ister(s) are selected."]
pub type Field1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIELD0` reader - PHY/PMA \n\nisolation \n\nenable \n\n(isolation_en) \n\n- When \n\nset, \n\nenables \n\nisolation (PHY or PMA)."]
pub type Field0R = crate::BitReader;
#[doc = "Field `FIELD0` writer - PHY/PMA \n\nisolation \n\nenable \n\n(isolation_en) \n\n- When \n\nset, \n\nenables \n\nisolation (PHY or PMA)."]
pub type Field0W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - PHY/PMA lane isolation enable (ln_isolation_en) - When in PHY Macro \n\nIsolation Mode, the selected PHY \n\nlane(s) \n\nisolation registers are \n\nselected. When in PMA Isolation Mode, the selected PMA lane(s) \n\nisolation registers are selected."]
    #[inline(always)]
    pub fn field5(&self) -> Field5R {
        Field5R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:11 - Reserved"]
    #[inline(always)]
    pub fn field4(&self) -> Field4R {
        Field4R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - PHY/PMA \n\nisolation mode \n\nselect \n\n(isolation_mode_sel) \n\n- When \n\nisolation_en is set, this bit selects between PHY isolation and PMA \n\nisolation mode. 0 = PHY isolation mode; 1 = PMA isolation mode."]
    #[inline(always)]
    pub fn field3(&self) -> Field3R {
        Field3R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Reserved"]
    #[inline(always)]
    pub fn field2(&self) -> Field2R {
        Field2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - PHY/PMA common isolation enable (cmn_isolation_en) - When in PHY \n\nMacro Isolation Mode, the PHY common isolation register(s) are \n\nselected. When in PMA Isolation Mode, the PMA common isolation \n\nreg- ister(s) are selected."]
    #[inline(always)]
    pub fn field1(&self) -> Field1R {
        Field1R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - PHY/PMA \n\nisolation \n\nenable \n\n(isolation_en) \n\n- When \n\nset, \n\nenables \n\nisolation (PHY or PMA)."]
    #[inline(always)]
    pub fn field0(&self) -> Field0R {
        Field0R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:7 - PHY/PMA lane isolation enable (ln_isolation_en) - When in PHY Macro \n\nIsolation Mode, the selected PHY \n\nlane(s) \n\nisolation registers are \n\nselected. When in PMA Isolation Mode, the selected PMA lane(s) \n\nisolation registers are selected."]
    #[inline(always)]
    #[must_use]
    pub fn field5(&mut self) -> Field5W<IsolationCtrlSpec> {
        Field5W::new(self, 0)
    }
    #[doc = "Bit 12 - PHY/PMA \n\nisolation mode \n\nselect \n\n(isolation_mode_sel) \n\n- When \n\nisolation_en is set, this bit selects between PHY isolation and PMA \n\nisolation mode. 0 = PHY isolation mode; 1 = PMA isolation mode."]
    #[inline(always)]
    #[must_use]
    pub fn field3(&mut self) -> Field3W<IsolationCtrlSpec> {
        Field3W::new(self, 12)
    }
    #[doc = "Bit 14 - PHY/PMA common isolation enable (cmn_isolation_en) - When in PHY \n\nMacro Isolation Mode, the PHY common isolation register(s) are \n\nselected. When in PMA Isolation Mode, the PMA common isolation \n\nreg- ister(s) are selected."]
    #[inline(always)]
    #[must_use]
    pub fn field1(&mut self) -> Field1W<IsolationCtrlSpec> {
        Field1W::new(self, 14)
    }
    #[doc = "Bit 15 - PHY/PMA \n\nisolation \n\nenable \n\n(isolation_en) \n\n- When \n\nset, \n\nenables \n\nisolation (PHY or PMA)."]
    #[inline(always)]
    #[must_use]
    pub fn field0(&mut self) -> Field0W<IsolationCtrlSpec> {
        Field0W::new(self, 15)
    }
}
#[doc = "Isolation control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`isolation_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`isolation_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsolationCtrlSpec;
impl crate::RegisterSpec for IsolationCtrlSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`isolation_ctrl::R`](R) reader structure"]
impl crate::Readable for IsolationCtrlSpec {}
#[doc = "`write(|w| ..)` method takes [`isolation_ctrl::W`](W) writer structure"]
impl crate::Writable for IsolationCtrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets ISOLATION_CTRL to value 0"]
impl crate::Resettable for IsolationCtrlSpec {
    const RESET_VALUE: u16 = 0;
}
