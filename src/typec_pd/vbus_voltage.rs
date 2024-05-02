#[doc = "Register `VBUS_VOLTAGE` reader"]
pub type R = crate::R<VbusVoltageSpec>;
#[doc = "Field `VBUS_voltage_measurement` reader - 10-bit measurement of (VBUS / ScaleFactor) \n\nTCPM multiplies this value by the scale factor \n\nto obtain the voltage measurement. Voltages \n\ngreater than or equal to 4V shall meet +/- 2% \n\nabsolute value or +/- 50mV, whichever is \n\ngreater. The LSB is25mV. \n\nThis \n\nregister \n\nrefreshes \n\nbased \n\non \n\nthe \n\nvbus_voltage\\[9:0\\]
input at an approximate \n\nrate of once every120us. \n\nIt is therefore recommended that the register \n\nis read twice to obtain a stable value, with the \n\ninterval \n\nbetween \n\nthe \n\nreads \n\ngreater \n\nthan120us."]
pub type VbusVoltageMeasurementR = crate::FieldReader<u16>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ScaleFactor {
    #[doc = "0: VBUS measurement not scaled. 01: VBUS measurement divided by2"]
    B00 = 0,
    #[doc = "2: VBUS measurement divided by4"]
    B10 = 2,
    #[doc = "3: reserved. This field is fixed for any given configuration of the TCPC. Currently fixed at00."]
    B11 = 3,
}
impl From<ScaleFactor> for u8 {
    #[inline(always)]
    fn from(variant: ScaleFactor) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ScaleFactor {
    type Ux = u8;
}
#[doc = "Field `Scale_Factor` reader - "]
pub type ScaleFactorR = crate::FieldReader<ScaleFactor>;
impl ScaleFactorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ScaleFactor> {
        match self.bits {
            0 => Some(ScaleFactor::B00),
            2 => Some(ScaleFactor::B10),
            3 => Some(ScaleFactor::B11),
            _ => None,
        }
    }
    #[doc = "VBUS measurement not scaled. 01: VBUS measurement divided by2"]
    #[inline(always)]
    pub fn is_b00(&self) -> bool {
        *self == ScaleFactor::B00
    }
    #[doc = "VBUS measurement divided by4"]
    #[inline(always)]
    pub fn is_b10(&self) -> bool {
        *self == ScaleFactor::B10
    }
    #[doc = "reserved. This field is fixed for any given configuration of the TCPC. Currently fixed at00."]
    #[inline(always)]
    pub fn is_b11(&self) -> bool {
        *self == ScaleFactor::B11
    }
}
#[doc = "Field `not_used` reader - Not used. Always write as 0 and ignore \n\nread value. Always reads0."]
pub type NotUsedR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:9 - 10-bit measurement of (VBUS / ScaleFactor) \n\nTCPM multiplies this value by the scale factor \n\nto obtain the voltage measurement. Voltages \n\ngreater than or equal to 4V shall meet +/- 2% \n\nabsolute value or +/- 50mV, whichever is \n\ngreater. The LSB is25mV. \n\nThis \n\nregister \n\nrefreshes \n\nbased \n\non \n\nthe \n\nvbus_voltage\\[9:0\\]
input at an approximate \n\nrate of once every120us. \n\nIt is therefore recommended that the register \n\nis read twice to obtain a stable value, with the \n\ninterval \n\nbetween \n\nthe \n\nreads \n\ngreater \n\nthan120us."]
    #[inline(always)]
    pub fn vbus_voltage_measurement(&self) -> VbusVoltageMeasurementR {
        VbusVoltageMeasurementR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 10:11"]
    #[inline(always)]
    pub fn scale_factor(&self) -> ScaleFactorR {
        ScaleFactorR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 16:31 - Not used. Always write as 0 and ignore \n\nread value. Always reads0."]
    #[inline(always)]
    pub fn not_used(&self) -> NotUsedR {
        NotUsedR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Vbus Voltage Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`vbus_voltage::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VbusVoltageSpec;
impl crate::RegisterSpec for VbusVoltageSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`vbus_voltage::R`](R) reader structure"]
impl crate::Readable for VbusVoltageSpec {}
#[doc = "`reset()` method sets VBUS_VOLTAGE to value 0"]
impl crate::Resettable for VbusVoltageSpec {
    const RESET_VALUE: u32 = 0;
}
