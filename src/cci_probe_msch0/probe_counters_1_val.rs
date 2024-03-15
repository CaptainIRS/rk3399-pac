#[doc = "Register `PROBE_Counters_1_Val` reader"]
pub type R = crate::R<ProbeCounters1ValSpec>;
#[doc = "Field `COUNTERS_0_VAL` reader - Register Val is a read-only register that is always present. The register containsthe statistics counter value either pending StatAlarm output, or when statisticscollection is suspended subsequent to triggers or signal statSuspend."]
pub type Counters0ValR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Register Val is a read-only register that is always present. The register containsthe statistics counter value either pending StatAlarm output, or when statisticscollection is suspended subsequent to triggers or signal statSuspend."]
    #[inline(always)]
    pub fn counters_0_val(&self) -> Counters0ValR {
        Counters0ValR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "Registers Counters_M_Val contain the statistics counter values.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`probe_counters_1_val::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ProbeCounters1ValSpec;
impl crate::RegisterSpec for ProbeCounters1ValSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`probe_counters_1_val::R`](R) reader structure"]
impl crate::Readable for ProbeCounters1ValSpec {}
#[doc = "`reset()` method sets PROBE_Counters_1_Val to value 0"]
impl crate::Resettable for ProbeCounters1ValSpec {
    const RESET_VALUE: u32 = 0;
}
