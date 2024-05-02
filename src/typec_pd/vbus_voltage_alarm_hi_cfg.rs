#[doc = "Register `VBUS_VOLTAGE_ALARM_HI_CFG` reader"]
pub type R = crate::R<VbusVoltageAlarmHiCfgSpec>;
#[doc = "Register `VBUS_VOLTAGE_ALARM_HI_CFG` writer"]
pub type W = crate::W<VbusVoltageAlarmHiCfgSpec>;
#[doc = "Field `Voltage_trip_point` reader - 10-bit for voltage threshold with 25mVLSB. \n\n+/- 5%accuracy. \n\nThe TCPC may ignore B0 or B1 &amp; B0 depending \n\nupon \n\nDEVICE_CAPABILITIES_2.VbusVoltageAlarmLsb."]
pub type VoltageTripPointR = crate::FieldReader<u16>;
#[doc = "Field `Voltage_trip_point` writer - 10-bit for voltage threshold with 25mVLSB. \n\n+/- 5%accuracy. \n\nThe TCPC may ignore B0 or B1 &amp; B0 depending \n\nupon \n\nDEVICE_CAPABILITIES_2.VbusVoltageAlarmLsb."]
pub type VoltageTripPointW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and \n\nignore read \n\nvalue."]
pub type NotUsedR = crate::FieldReader<u16>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and \n\nignore read \n\nvalue."]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - 10-bit for voltage threshold with 25mVLSB. \n\n+/- 5%accuracy. \n\nThe TCPC may ignore B0 or B1 &amp; B0 depending \n\nupon \n\nDEVICE_CAPABILITIES_2.VbusVoltageAlarmLsb."]
    #[inline(always)]
    pub fn voltage_trip_point(&self) -> VoltageTripPointR {
        VoltageTripPointR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and \n\nignore read \n\nvalue."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 10-bit for voltage threshold with 25mVLSB. \n\n+/- 5%accuracy. \n\nThe TCPC may ignore B0 or B1 &amp; B0 depending \n\nupon \n\nDEVICE_CAPABILITIES_2.VbusVoltageAlarmLsb."]
    #[inline(always)]
    #[must_use]
    pub fn voltage_trip_point(&mut self) -> VoltageTripPointW<VbusVoltageAlarmHiCfgSpec> {
        VoltageTripPointW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and \n\nignore read \n\nvalue."]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<VbusVoltageAlarmHiCfgSpec> {
        NotUsedW::new(self, 16)
    }
}
#[doc = "Vbus Voltage Alarm High Cfg Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_voltage_alarm_hi_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_voltage_alarm_hi_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusVoltageAlarmHiCfgSpec;
impl crate::RegisterSpec for VbusVoltageAlarmHiCfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_voltage_alarm_hi_cfg::R`](R) reader structure"]
impl crate::Readable for VbusVoltageAlarmHiCfgSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_voltage_alarm_hi_cfg::W`](W) writer structure"]
impl crate::Writable for VbusVoltageAlarmHiCfgSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_VOLTAGE_ALARM_HI_CFG to value 0"]
impl crate::Resettable for VbusVoltageAlarmHiCfgSpec {
    const RESET_VALUE: u32 = 0;
}
