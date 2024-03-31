#[doc = "Register `TORR` reader"]
pub type R = crate::R<TorrSpec>;
#[doc = "Register `TORR` writer"]
pub type W = crate::W<TorrSpec>;
#[doc = "Timeout period.\n\nThis field is used to select the timeout period from\n\nwhich the watchdog counter restarts. A change of the timeout\n\nperiod takes effect only after the next counter restart (kick).\n\nThe range of values available for a 32-bit watchdog counter are:\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum TimeoutPeriod {
    #[doc = "0: 0x0000ffff"]
    B0000 = 0,
    #[doc = "1: 0x0001ffff"]
    B0001 = 1,
    #[doc = "2: 0x0003ffff"]
    B0010 = 2,
    #[doc = "3: 0x0007ffff"]
    B0011 = 3,
    #[doc = "4: 0x000fffff"]
    B0100 = 4,
    #[doc = "5: 0x001fffff"]
    B0101 = 5,
    #[doc = "6: 0x003fffff"]
    B0110 = 6,
    #[doc = "7: 0x007fffff"]
    B0111 = 7,
    #[doc = "8: 0x00ffffff"]
    B1000 = 8,
    #[doc = "9: 0x01ffffff"]
    B1001 = 9,
    #[doc = "10: 0x03ffffff"]
    B1010 = 10,
    #[doc = "11: 0x07ffffff"]
    B1011 = 11,
    #[doc = "12: 0x0fffffff"]
    B1100 = 12,
    #[doc = "13: 0x1fffffff"]
    B1101 = 13,
    #[doc = "14: 0x3fffffff"]
    B1110 = 14,
    #[doc = "15: 0x7fffffff"]
    B1111 = 15,
}
impl From<TimeoutPeriod> for u8 {
    #[inline(always)]
    fn from(variant: TimeoutPeriod) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for TimeoutPeriod {
    type Ux = u8;
}
#[doc = "Field `TIMEOUT_PERIOD` reader - Timeout period.\n\nThis field is used to select the timeout period from\n\nwhich the watchdog counter restarts. A change of the timeout\n\nperiod takes effect only after the next counter restart (kick).\n\nThe range of values available for a 32-bit watchdog counter are:"]
pub type TimeoutPeriodR = crate::FieldReader<TimeoutPeriod>;
impl TimeoutPeriodR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TimeoutPeriod {
        match self.bits {
            0 => TimeoutPeriod::B0000,
            1 => TimeoutPeriod::B0001,
            2 => TimeoutPeriod::B0010,
            3 => TimeoutPeriod::B0011,
            4 => TimeoutPeriod::B0100,
            5 => TimeoutPeriod::B0101,
            6 => TimeoutPeriod::B0110,
            7 => TimeoutPeriod::B0111,
            8 => TimeoutPeriod::B1000,
            9 => TimeoutPeriod::B1001,
            10 => TimeoutPeriod::B1010,
            11 => TimeoutPeriod::B1011,
            12 => TimeoutPeriod::B1100,
            13 => TimeoutPeriod::B1101,
            14 => TimeoutPeriod::B1110,
            15 => TimeoutPeriod::B1111,
            _ => unreachable!(),
        }
    }
    #[doc = "0x0000ffff"]
    #[inline(always)]
    pub fn is_b0000(&self) -> bool {
        *self == TimeoutPeriod::B0000
    }
    #[doc = "0x0001ffff"]
    #[inline(always)]
    pub fn is_b0001(&self) -> bool {
        *self == TimeoutPeriod::B0001
    }
    #[doc = "0x0003ffff"]
    #[inline(always)]
    pub fn is_b0010(&self) -> bool {
        *self == TimeoutPeriod::B0010
    }
    #[doc = "0x0007ffff"]
    #[inline(always)]
    pub fn is_b0011(&self) -> bool {
        *self == TimeoutPeriod::B0011
    }
    #[doc = "0x000fffff"]
    #[inline(always)]
    pub fn is_b0100(&self) -> bool {
        *self == TimeoutPeriod::B0100
    }
    #[doc = "0x001fffff"]
    #[inline(always)]
    pub fn is_b0101(&self) -> bool {
        *self == TimeoutPeriod::B0101
    }
    #[doc = "0x003fffff"]
    #[inline(always)]
    pub fn is_b0110(&self) -> bool {
        *self == TimeoutPeriod::B0110
    }
    #[doc = "0x007fffff"]
    #[inline(always)]
    pub fn is_b0111(&self) -> bool {
        *self == TimeoutPeriod::B0111
    }
    #[doc = "0x00ffffff"]
    #[inline(always)]
    pub fn is_b1000(&self) -> bool {
        *self == TimeoutPeriod::B1000
    }
    #[doc = "0x01ffffff"]
    #[inline(always)]
    pub fn is_b1001(&self) -> bool {
        *self == TimeoutPeriod::B1001
    }
    #[doc = "0x03ffffff"]
    #[inline(always)]
    pub fn is_b1010(&self) -> bool {
        *self == TimeoutPeriod::B1010
    }
    #[doc = "0x07ffffff"]
    #[inline(always)]
    pub fn is_b1011(&self) -> bool {
        *self == TimeoutPeriod::B1011
    }
    #[doc = "0x0fffffff"]
    #[inline(always)]
    pub fn is_b1100(&self) -> bool {
        *self == TimeoutPeriod::B1100
    }
    #[doc = "0x1fffffff"]
    #[inline(always)]
    pub fn is_b1101(&self) -> bool {
        *self == TimeoutPeriod::B1101
    }
    #[doc = "0x3fffffff"]
    #[inline(always)]
    pub fn is_b1110(&self) -> bool {
        *self == TimeoutPeriod::B1110
    }
    #[doc = "0x7fffffff"]
    #[inline(always)]
    pub fn is_b1111(&self) -> bool {
        *self == TimeoutPeriod::B1111
    }
}
#[doc = "Field `TIMEOUT_PERIOD` writer - Timeout period.\n\nThis field is used to select the timeout period from\n\nwhich the watchdog counter restarts. A change of the timeout\n\nperiod takes effect only after the next counter restart (kick).\n\nThe range of values available for a 32-bit watchdog counter are:"]
pub type TimeoutPeriodW<'a, REG> = crate::FieldWriterSafe<'a, REG, 4, TimeoutPeriod>;
impl<'a, REG> TimeoutPeriodW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "0x0000ffff"]
    #[inline(always)]
    pub fn b0000(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B0000)
    }
    #[doc = "0x0001ffff"]
    #[inline(always)]
    pub fn b0001(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B0001)
    }
    #[doc = "0x0003ffff"]
    #[inline(always)]
    pub fn b0010(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B0010)
    }
    #[doc = "0x0007ffff"]
    #[inline(always)]
    pub fn b0011(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B0011)
    }
    #[doc = "0x000fffff"]
    #[inline(always)]
    pub fn b0100(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B0100)
    }
    #[doc = "0x001fffff"]
    #[inline(always)]
    pub fn b0101(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B0101)
    }
    #[doc = "0x003fffff"]
    #[inline(always)]
    pub fn b0110(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B0110)
    }
    #[doc = "0x007fffff"]
    #[inline(always)]
    pub fn b0111(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B0111)
    }
    #[doc = "0x00ffffff"]
    #[inline(always)]
    pub fn b1000(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B1000)
    }
    #[doc = "0x01ffffff"]
    #[inline(always)]
    pub fn b1001(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B1001)
    }
    #[doc = "0x03ffffff"]
    #[inline(always)]
    pub fn b1010(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B1010)
    }
    #[doc = "0x07ffffff"]
    #[inline(always)]
    pub fn b1011(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B1011)
    }
    #[doc = "0x0fffffff"]
    #[inline(always)]
    pub fn b1100(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B1100)
    }
    #[doc = "0x1fffffff"]
    #[inline(always)]
    pub fn b1101(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B1101)
    }
    #[doc = "0x3fffffff"]
    #[inline(always)]
    pub fn b1110(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B1110)
    }
    #[doc = "0x7fffffff"]
    #[inline(always)]
    pub fn b1111(self) -> &'a mut crate::W<REG> {
        self.variant(TimeoutPeriod::B1111)
    }
}
impl R {
    #[doc = "Bits 0:3 - Timeout period.\n\nThis field is used to select the timeout period from\n\nwhich the watchdog counter restarts. A change of the timeout\n\nperiod takes effect only after the next counter restart (kick).\n\nThe range of values available for a 32-bit watchdog counter are:"]
    #[inline(always)]
    pub fn timeout_period(&self) -> TimeoutPeriodR {
        TimeoutPeriodR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timeout period.\n\nThis field is used to select the timeout period from\n\nwhich the watchdog counter restarts. A change of the timeout\n\nperiod takes effect only after the next counter restart (kick).\n\nThe range of values available for a 32-bit watchdog counter are:"]
    #[inline(always)]
    #[must_use]
    pub fn timeout_period(&mut self) -> TimeoutPeriodW<TorrSpec> {
        TimeoutPeriodW::new(self, 0)
    }
}
#[doc = "Timeout range Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`torr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`torr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TorrSpec;
impl crate::RegisterSpec for TorrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`torr::R`](R) reader structure"]
impl crate::Readable for TorrSpec {}
#[doc = "`write(|w| ..)` method takes [`torr::W`](W) writer structure"]
impl crate::Writable for TorrSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TORR to value 0"]
impl crate::Resettable for TorrSpec {
    const RESET_VALUE: u32 = 0;
}
