#[doc = "Register `VBUS_SINK_DISCONNECT_THRESHOLD` reader"]
pub type R = crate::R<VbusSinkDisconnectThresholdSpec>;
#[doc = "Register `VBUS_SINK_DISCONNECT_THRESHOLD` writer"]
pub type W = crate::W<VbusSinkDisconnectThresholdSpec>;
#[doc = "Field `Voltage_trip_point` reader - 10-bit for voltage threshold with \n\n25mV LSB.(Default vSafe5V) \n\n+/- 5%accuracy. \n\nA value of B9:0=000h disables this \n\nthreshold \n\nThe TCPC may ignore B0 or B1 &amp; B0 \n\ndepending upon \n\nDEVICE_CAPABILITIES_2.VbusVoltageAl\n\narmLsb."]
pub type VoltageTripPointR = crate::FieldReader<u16>;
#[doc = "Field `Voltage_trip_point` writer - 10-bit for voltage threshold with \n\n25mV LSB.(Default vSafe5V) \n\n+/- 5%accuracy. \n\nA value of B9:0=000h disables this \n\nthreshold \n\nThe TCPC may ignore B0 or B1 &amp; B0 \n\ndepending upon \n\nDEVICE_CAPABILITIES_2.VbusVoltageAl\n\narmLsb."]
pub type VoltageTripPointW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore \n\nread value."]
pub type NotUsedR = crate::FieldReader<u16>;
#[doc = "Field `not_used` writer - Not used. Always write as 0 and ignore \n\nread value."]
pub type NotUsedW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:9 - 10-bit for voltage threshold with \n\n25mV LSB.(Default vSafe5V) \n\n+/- 5%accuracy. \n\nA value of B9:0=000h disables this \n\nthreshold \n\nThe TCPC may ignore B0 or B1 &amp; B0 \n\ndepending upon \n\nDEVICE_CAPABILITIES_2.VbusVoltageAl\n\narmLsb."]
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
    #[doc = "Bits 0:9 - 10-bit for voltage threshold with \n\n25mV LSB.(Default vSafe5V) \n\n+/- 5%accuracy. \n\nA value of B9:0=000h disables this \n\nthreshold \n\nThe TCPC may ignore B0 or B1 &amp; B0 \n\ndepending upon \n\nDEVICE_CAPABILITIES_2.VbusVoltageAl\n\narmLsb."]
    #[inline(always)]
    #[must_use]
    pub fn voltage_trip_point(&mut self) -> VoltageTripPointW<VbusSinkDisconnectThresholdSpec> {
        VoltageTripPointW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and ignore \n\nread value."]
    #[inline(always)]
    #[must_use]
    pub fn not_used(&mut self) -> NotUsedW<VbusSinkDisconnectThresholdSpec> {
        NotUsedW::new(self, 16)
    }
}
#[doc = "Vbus Sink Disconnect Threshold Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_sink_disconnect_threshold::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`vbus_sink_disconnect_threshold::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusSinkDisconnectThresholdSpec;
impl crate::RegisterSpec for VbusSinkDisconnectThresholdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_sink_disconnect_threshold::R`](R) reader structure"]
impl crate::Readable for VbusSinkDisconnectThresholdSpec {}
#[doc = "`write(|w| ..)` method takes [`vbus_sink_disconnect_threshold::W`](W) writer structure"]
impl crate::Writable for VbusSinkDisconnectThresholdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets VBUS_SINK_DISCONNECT_THRESHOLD to value 0xa2"]
impl crate::Resettable for VbusSinkDisconnectThresholdSpec {
    const RESET_VALUE: u32 = 0xa2;
}
