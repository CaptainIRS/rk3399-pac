#[doc = "Register `VBUS_STOP_DISCHARGE_THRESHOLD` reader"]
pub type R = crate::R<VbusStopDischargeThresholdSpec>;
#[doc = "Register `VBUS_STOP_DISCHARGE_THRESHOLD` writer"]
pub type W = crate::W<VbusStopDischargeThresholdSpec>;
#[doc = "Field `Voltage_trip_point` reader - 10-bit for voltage threshold with \n\n25mV LSB.(Default vSafe0V) \n\n+/- 5%accuracy. \n\nThe TCPC may ignore B0 or B1 &amp; B0 \n\ndepending upon \n\nDEVICE_CAPABILITIES_2.VbusVoltageAl\n\narmLsb."]
pub type VoltageTripPointR = crate::FieldReader<u16>;
#[doc = "Field `Voltage_trip_point` writer - 10-bit for voltage threshold with \n\n25mV LSB.(Default vSafe0V) \n\n+/- 5%accuracy. \n\nThe TCPC may ignore B0 or B1 &amp; B0 \n\ndepending upon \n\nDEVICE_CAPABILITIES_2.VbusVoltageAl\n\narmLsb."]
pub type VoltageTripPointW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore \n\nread value."]
pub type NotUsedR = crate::FieldReader<u16>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and ignore \n\nread value."]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - 10-bit for voltage threshold with \n\n25mV LSB.(Default vSafe0V) \n\n+/- 5%accuracy. \n\nThe TCPC may ignore B0 or B1 &amp; B0 \n\ndepending upon \n\nDEVICE_CAPABILITIES_2.VbusVoltageAl\n\narmLsb."]
    #[inline(always)]
    pub fn voltage_trip_point(&self) -> VoltageTripPointR {
        VoltageTripPointR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and ignore \n\nread value."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - 10-bit for voltage threshold with \n\n25mV LSB.(Default vSafe0V) \n\n+/- 5%accuracy. \n\nThe TCPC may ignore B0 or B1 &amp; B0 \n\ndepending upon \n\nDEVICE_CAPABILITIES_2.VbusVoltageAl\n\narmLsb."]
    #[inline(always)]
    #[must_use]
    pub fn voltage_trip_point(&mut self) -> VoltageTripPointW<VbusStopDischargeThresholdSpec> {
        VoltageTripPointW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and ignore \n\nread value."]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<VbusStopDischargeThresholdSpec> {
        NotUsedW::new(self, 16)
    }
}
#[doc = "Vbus Stop Discharge Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_stop_discharge_threshold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_stop_discharge_threshold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusStopDischargeThresholdSpec;
impl crate::RegisterSpec for VbusStopDischargeThresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_stop_discharge_threshold::R`](R) reader structure"]
impl crate::Readable for VbusStopDischargeThresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_stop_discharge_threshold::W`](W) writer structure"]
impl crate::Writable for VbusStopDischargeThresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_STOP_DISCHARGE_THRESHOLD to value 0x20"]
impl crate::Resettable for VbusStopDischargeThresholdSpec {
    const RESET_VALUE: u32 = 0x20;
}
