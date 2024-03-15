#[doc = "Register `PROBE_MainCtl` reader"]
pub type R = crate::R<ProbeMainCtlSpec>;
#[doc = "Register `PROBE_MainCtl` writer"]
pub type W = crate::W<ProbeMainCtlSpec>;
#[doc = "Field `ERREN` reader - Register field ErrEn enables the probe to send on the ObsTx output any packet with Error status, independently of filtering mechanisms, thus constituting a simple supplementary global filter."]
pub type ErrenR = crate::BitReader;
#[doc = "Field `ERREN` writer - Register field ErrEn enables the probe to send on the ObsTx output any packet with Error status, independently of filtering mechanisms, thus constituting a simple supplementary global filter."]
pub type ErrenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRACEEN` reader - Register field TraceEn enables the probe to send filtered packets (Trace) on the ObsTx observation output."]
pub type TraceenR = crate::BitReader;
#[doc = "Field `PAYLOADEN` reader - Register field PayloadEn, when set to 1, enables traces to contain headers and payload. When set ot 0, only headers are reported."]
pub type PayloadenR = crate::BitReader;
#[doc = "Field `PAYLOADEN` writer - Register field PayloadEn, when set to 1, enables traces to contain headers and payload. When set ot 0, only headers are reported."]
pub type PayloadenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATEN` reader - When set to 1, register field StatEn enables statistics profiling. The probe sendS statistics results to the output for signal ObsTx. All statistics counters are cleared when the StatEn bit goes from 0 to 1. When set to 0, counters are disabled."]
pub type StatenR = crate::BitReader;
#[doc = "Field `STATEN` writer - When set to 1, register field StatEn enables statistics profiling. The probe sendS statistics results to the output for signal ObsTx. All statistics counters are cleared when the StatEn bit goes from 0 to 1. When set to 0, counters are disabled."]
pub type StatenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STATCONDDUMP` reader - When set, register field StatCondDump enables the dump of a statistics frame to the range of counter values set for registers StatAlarmMin, StatAlarmMax, and AlarmMode. This field also renders register StatAlarmStatus inoperative. When parameter statisticsCounterAlarm is set to False, the StatCondDump register bit is reserved."]
pub type StatconddumpR = crate::BitReader;
#[doc = "Field `STATCONDDUMP` writer - When set, register field StatCondDump enables the dump of a statistics frame to the range of counter values set for registers StatAlarmMin, StatAlarmMax, and AlarmMode. This field also renders register StatAlarmStatus inoperative. When parameter statisticsCounterAlarm is set to False, the StatCondDump register bit is reserved."]
pub type StatconddumpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTRUSIVEMODE` reader - When set to 1, register field IntrusiveMode enables trace operation in Intrusive flow-control mode. When set to 0, the register enables trace operation in Overflow flow-control mode"]
pub type IntrusivemodeR = crate::BitReader;
#[doc = "Field `FILTBYTEALWAYSCHAINABLEEN` reader - When set to 0, filters are mapped to all statistic counters when counting bytes or enabled bytes. Therefore, only filter events mapped to even counters can be counted using a pair of chained counters.When set to 1, filters are mapped only to even statistic counters when counting bytes or enabled bytes. Thus events from any filter can be counted using a pair of chained counters."]
pub type FiltbytealwayschainableenR = crate::BitReader;
#[doc = "Field `FILTBYTEALWAYSCHAINABLEEN` writer - When set to 0, filters are mapped to all statistic counters when counting bytes or enabled bytes. Therefore, only filter events mapped to even counters can be counted using a pair of chained counters.When set to 1, filters are mapped only to even statistic counters when counting bytes or enabled bytes. Thus events from any filter can be counted using a pair of chained counters."]
pub type FiltbytealwayschainableenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Register field ErrEn enables the probe to send on the ObsTx output any packet with Error status, independently of filtering mechanisms, thus constituting a simple supplementary global filter."]
    #[inline(always)]
    pub fn erren(&self) -> ErrenR {
        ErrenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Register field TraceEn enables the probe to send filtered packets (Trace) on the ObsTx observation output."]
    #[inline(always)]
    pub fn traceen(&self) -> TraceenR {
        TraceenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Register field PayloadEn, when set to 1, enables traces to contain headers and payload. When set ot 0, only headers are reported."]
    #[inline(always)]
    pub fn payloaden(&self) -> PayloadenR {
        PayloadenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - When set to 1, register field StatEn enables statistics profiling. The probe sendS statistics results to the output for signal ObsTx. All statistics counters are cleared when the StatEn bit goes from 0 to 1. When set to 0, counters are disabled."]
    #[inline(always)]
    pub fn staten(&self) -> StatenR {
        StatenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - When set, register field StatCondDump enables the dump of a statistics frame to the range of counter values set for registers StatAlarmMin, StatAlarmMax, and AlarmMode. This field also renders register StatAlarmStatus inoperative. When parameter statisticsCounterAlarm is set to False, the StatCondDump register bit is reserved."]
    #[inline(always)]
    pub fn statconddump(&self) -> StatconddumpR {
        StatconddumpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - When set to 1, register field IntrusiveMode enables trace operation in Intrusive flow-control mode. When set to 0, the register enables trace operation in Overflow flow-control mode"]
    #[inline(always)]
    pub fn intrusivemode(&self) -> IntrusivemodeR {
        IntrusivemodeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - When set to 0, filters are mapped to all statistic counters when counting bytes or enabled bytes. Therefore, only filter events mapped to even counters can be counted using a pair of chained counters.When set to 1, filters are mapped only to even statistic counters when counting bytes or enabled bytes. Thus events from any filter can be counted using a pair of chained counters."]
    #[inline(always)]
    pub fn filtbytealwayschainableen(&self) -> FiltbytealwayschainableenR {
        FiltbytealwayschainableenR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Register field ErrEn enables the probe to send on the ObsTx output any packet with Error status, independently of filtering mechanisms, thus constituting a simple supplementary global filter."]
    #[inline(always)]
    #[must_use]
    pub fn erren(&mut self) -> ErrenW<ProbeMainCtlSpec> {
        ErrenW::new(self, 0)
    }
    #[doc = "Bit 2 - Register field PayloadEn, when set to 1, enables traces to contain headers and payload. When set ot 0, only headers are reported."]
    #[inline(always)]
    #[must_use]
    pub fn payloaden(&mut self) -> PayloadenW<ProbeMainCtlSpec> {
        PayloadenW::new(self, 2)
    }
    #[doc = "Bit 3 - When set to 1, register field StatEn enables statistics profiling. The probe sendS statistics results to the output for signal ObsTx. All statistics counters are cleared when the StatEn bit goes from 0 to 1. When set to 0, counters are disabled."]
    #[inline(always)]
    #[must_use]
    pub fn staten(&mut self) -> StatenW<ProbeMainCtlSpec> {
        StatenW::new(self, 3)
    }
    #[doc = "Bit 5 - When set, register field StatCondDump enables the dump of a statistics frame to the range of counter values set for registers StatAlarmMin, StatAlarmMax, and AlarmMode. This field also renders register StatAlarmStatus inoperative. When parameter statisticsCounterAlarm is set to False, the StatCondDump register bit is reserved."]
    #[inline(always)]
    #[must_use]
    pub fn statconddump(&mut self) -> StatconddumpW<ProbeMainCtlSpec> {
        StatconddumpW::new(self, 5)
    }
    #[doc = "Bit 7 - When set to 0, filters are mapped to all statistic counters when counting bytes or enabled bytes. Therefore, only filter events mapped to even counters can be counted using a pair of chained counters.When set to 1, filters are mapped only to even statistic counters when counting bytes or enabled bytes. Thus events from any filter can be counted using a pair of chained counters."]
    #[inline(always)]
    #[must_use]
    pub fn filtbytealwayschainableen(&mut self) -> FiltbytealwayschainableenW<ProbeMainCtlSpec> {
        FiltbytealwayschainableenW::new(self, 7)
    }
}
#[doc = "Register MainCtl contains probe global control bits.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_main_ctl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`probe_main_ctl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProbeMainCtlSpec;
impl crate::RegisterSpec for ProbeMainCtlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`probe_main_ctl::R`](R) reader structure"]
impl crate::Readable for ProbeMainCtlSpec {}
#[doc = "`write(|w| ..)` method takes [`probe_main_ctl::W`](W) writer structure"]
impl crate::Writable for ProbeMainCtlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PROBE_MainCtl to value 0"]
impl crate::Resettable for ProbeMainCtlSpec {
    const RESET_VALUE: u32 = 0;
}
